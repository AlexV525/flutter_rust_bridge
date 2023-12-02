use crate::generalized_isolate::{IntoDart, SendableChannelHandle};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::api2wire::Api2wire;
use crate::rust2dart::sender::Rust2DartSender;
use std::marker::PhantomData;

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSink<T> {
    sendable_channel_handle: SendableChannelHandle,
    _phantom_data: PhantomData<T>,
}

impl<T> StreamSink<T> {
    /// Create a new sink from a port wrapper.
    pub fn new(sender: Rust2DartSender) -> Self {
        Self {
            sendable_channel_handle: (&sender.channel).into(),
            _phantom_data: PhantomData,
        }
    }

    fn sender(&self) -> Rust2DartSender {
        Rust2DartSender::new((&self.sendable_channel_handle).into())
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add<D: IntoDart>(&self, value: T) -> bool
    where
        T: IntoIntoDart<D>,
    {
        self.sender()
            .send(Api2wire::success(value.into_into_dart().into_dart()))
    }

    /// Close the stream and ignore further messages. Returns false when
    /// the stream could not be closed, or when it has already been closed.
    pub fn close(&self) -> bool {
        self.sender().send(Api2wire::close_stream())
    }
}
