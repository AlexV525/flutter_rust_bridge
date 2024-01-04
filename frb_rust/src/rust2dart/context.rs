use crate::codec::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::stream_sink::{StreamSinkBase, StreamSinkCloser};
use std::marker::PhantomData;
use std::sync::Arc;

/// A context for task execution related to Rust2Dart
pub struct TaskRust2DartContext<Rust2DartCodec: BaseCodec> {
    sender: Rust2DartSender,
    stream_sink_closer: Option<Arc<StreamSinkCloser<Rust2DartCodec>>>,
    _phantom: PhantomData<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> TaskRust2DartContext<Rust2DartCodec> {
    /// Create a new context.
    pub fn new(
        sender: Rust2DartSender,
        stream_sink_closer: Option<Arc<StreamSinkCloser<Rust2DartCodec>>>,
    ) -> Self {
        Self {
            sender,
            stream_sink_closer,
            _phantom: Default::default(),
        }
    }

    /// Create a new [StreamSinkBase] of the specified type.
    pub fn stream_sink<T, D>(&self) -> StreamSinkBase<T, Rust2DartCodec>
    where
        T: IntoIntoDart<D>,
        D: IntoDart,
    {
        StreamSinkBase::<T, Rust2DartCodec>::new(
            self.sender.clone(),
            self.stream_sink_closer.clone().unwrap(),
        )
    }
}
