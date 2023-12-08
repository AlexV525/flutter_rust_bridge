use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::dart_fn::DartFnFuture;
use crate::dart_opaque::DartOpaque;
use crate::generalized_isolate::IntoDart;
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::handler::executor::Executor;
use crate::handler::handler::HandlerConfig;
use crate::handler::handler::{Handler, TaskContext, TaskInfo, TaskRetFutTrait};
use crate::handler::implementation::error_handler::NoOpErrorHandler;
use crate::handler::implementation::executor::SimpleExecutor;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::platform_types::message_port_to_handle;
use crate::platform_types::DartAbi;
use crate::platform_types::MessagePort;
use crate::platform_types::SendableMessagePortHandle;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust_async::{BaseAsyncRuntime, SimpleAsyncRuntime};
use crate::thread_pool::BaseThreadPool;
use allo_isolate::ffi::DartCObject;
use log::warn;
use std::future::Future;
use std::panic;
use std::panic::UnwindSafe;
use std::sync::Mutex;

/// The default handler used by the generated code.
pub type DefaultHandler<TP> =
    SimpleHandler<SimpleExecutor<NoOpErrorHandler, TP, SimpleAsyncRuntime>, NoOpErrorHandler>;

impl<TP: BaseThreadPool> DefaultHandler<TP> {
    pub fn new_simple(thread_pool: TP) -> Self {
        Self::new(
            SimpleExecutor::new(NoOpErrorHandler, thread_pool, Default::default()),
            NoOpErrorHandler,
        )
    }

    pub fn thread_pool(&self) -> &TP {
        self.executor.thread_pool()
    }
}

/// The simple handler uses a simple thread pool to execute tasks.
pub struct SimpleHandler<E: Executor, EH: ErrorHandler> {
    executor: E,
    error_handler: EH,
    config: Mutex<Option<HandlerConfig>>,
}

impl<E: Executor, H: ErrorHandler> SimpleHandler<E, H> {
    /// Create a new default handler.
    pub fn new(executor: E, error_handler: H) -> Self {
        SimpleHandler {
            executor,
            error_handler,
            config: Mutex::new(None),
        }
    }
}

impl<E: Executor, EH: ErrorHandler> Handler for SimpleHandler<E, EH> {
    fn initialize(&self, config: HandlerConfig) {
        // Again, as mentioned below, ensure panics never cross FFI boundary
        let _ = panic::catch_unwind(|| {
            if let Ok(mut self_config) = self.config.lock() {
                if self_config.is_some() {
                    // internal link: https://github.com/fzyzcjy/yplusplus/issues/11352
                    // (If you are interested in more details, create an issue and let me write a doc for it)
                    let msg = "SimpleHandler.initialize is called multiple times.
This is problematic *if* you are running two *live* FRB Dart instances while one FRB Rust instance. Thus:
* If you are hot-restarting Dart (Flutter), it is usually normal and you can ignore this message.
* If you are running `dart test`, try `dart test --concurrency=1` to avoid 'two live instance' problem, and then ignore this message.";
                    warn!("{}", msg);
                    println!("{}", msg); // when users do not enable log
                }

                *self_config = Some(config);
            }
        });
    }

    fn dart_opaque_drop_port(&self) -> SendableMessagePortHandle {
        (self.config.lock().expect("cannot get config lock"))
            .as_ref()
            .expect("no handler config")
            .dart_opaque_drop_port
            .to_owned()
    }

    fn dart_fn_invoke_port(&self) -> SendableMessagePortHandle {
        (self.config.lock().expect("cannot get config lock"))
            .as_ref()
            .expect("no handler config")
            .dart_fn_invoke_port
            .to_owned()
    }

    fn wrap_normal<Rust2DartCodec, PrepareFn, TaskFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> Result<DartCObject, DartCObject>
            + Send
            + UnwindSafe
            + 'static,
        Rust2DartCodec: BaseCodec,
    {
        self.wrap_normal_or_async::<Rust2DartCodec, _, _, _, _>(
            task_info,
            prepare,
            |task_info, task| {
                self.executor
                    .execute_normal::<Rust2DartCodec, _>(task_info, task)
            },
        )
    }

    fn wrap_sync<Rust2DartCodec, SyncTaskFn>(
        &self,
        task_info: TaskInfo,
        sync_task: SyncTaskFn,
    ) -> <Rust2DartCodec::Message as Rust2DartMessageTrait>::WireSyncType
    where
        SyncTaskFn:
            FnOnce() -> Result<Rust2DartCodec::Message, Rust2DartCodec::Message> + UnwindSafe,
        Rust2DartCodec: BaseCodec,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // For reason, see comments in [wrap]
        panic::catch_unwind(move || {
            let catch_unwind_result = panic::catch_unwind(move || {
                match self
                    .executor
                    .execute_sync::<Rust2DartCodec, _>(task_info, sync_task)
                {
                    Ok(data) => data,
                    Err(err) => {
                        self.error_handler.on_error(Error::CustomError);
                        err
                    }
                }
            });
            catch_unwind_result
                .unwrap_or_else(|error| {
                    self.error_handler.on_error(Error::Panic(error));
                    Rust2DartCodec::encode(error, Rust2DartAction::Panic)
                })
                .into_raw_wire_sync()
        })
        // Deliberately construct simplest possible WireSyncReturn object
        // instead of more realistic things like `WireSyncReturnSrc::new(Panic, ...)`.
        // See comments in [wrap] for why.
        .unwrap_or_else(|_| Rust2DartCodec::Message::simplest().into_raw_wire_sync())
    }

    #[cfg(feature = "rust-async")]
    fn wrap_async<Rust2DartCodec, PrepareFn, TaskFn, TaskRetFut>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRetFut:
            Future<Output = Result<DartCObject, DartCObject>> + TaskRetFutTrait + UnwindSafe,
        Rust2DartCodec: BaseCodec,
    {
        self.wrap_normal_or_async::<Rust2DartCodec, _, _, _, _>(
            task_info,
            prepare,
            |task_info, task| {
                self.executor
                    .execute_async::<Rust2DartCodec, _, _>(task_info, task)
            },
        )
    }

    fn dart_fn_invoke<Ret>(&self, dart_fn_and_args: Vec<DartAbi>) -> DartFnFuture<Ret> {
        todo!()
    }
}

impl<E: Executor, EH: ErrorHandler> SimpleHandler<E, EH> {
    fn wrap_normal_or_async<Rust2DartCodec, PrepareFn, TaskFn, TaskFnRet, ExecuteFn>(
        &self,
        task_info: TaskInfo,
        prepare: PrepareFn,
        execute: ExecuteFn,
    ) where
        PrepareFn: FnOnce() -> TaskFn + UnwindSafe,
        TaskFn: FnOnce(TaskContext<Rust2DartCodec>) -> TaskFnRet,
        ExecuteFn: FnOnce(TaskInfo, TaskFn) + UnwindSafe,
        Rust2DartCodec: BaseCodec,
    {
        // NOTE This extra [catch_unwind] **SHOULD** be put outside **ALL** code!
        // Why do this: As nomicon says, unwind across languages is undefined behavior (UB).
        // Therefore, we should wrap a [catch_unwind] outside of *each and every* line of code
        // that can cause panic. Otherwise we may touch UB.
        // Why do not report error or something like that if this outer [catch_unwind] really
        // catches something: Because if we report error, that line of code itself can cause panic
        // as well. Then that new panic will go across language boundary and cause UB.
        // ref https://doc.rust-lang.org/nomicon/unwinding.html
        let _ = panic::catch_unwind(move || {
            let task_info2 = task_info.clone();
            if let Err(error) = panic::catch_unwind(move || {
                let task = prepare();
                execute(task_info2, task);
            }) {
                self.error_handler
                    .handle_error::<Rust2DartCodec>(task_info.port.unwrap(), Error::Panic(error));
            }
        });
    }
}
