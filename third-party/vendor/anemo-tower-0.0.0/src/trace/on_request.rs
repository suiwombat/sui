use super::DEFAULT_MESSAGE_LEVEL;
use anemo::Request;
use bytes::Bytes;
use tracing::{Level, Span};

/// Trait used to tell [`Trace`] what to do when a request is received.
///
/// See the [module docs](../trace/index.html#on_request) for details on exactly when the
/// `on_request` callback is called.
///
/// [`Trace`]: super::Trace
pub trait OnRequest {
    /// Do the thing.
    ///
    /// `span` is the `tracing` [`Span`], corresponding to this request, produced by the closure
    /// passed to [`TraceLayer::make_span_with`]. It can be used to [record field values][record]
    /// that weren't known when the span was created.
    ///
    /// [`Span`]: https://docs.rs/tracing/latest/tracing/span/index.html
    /// [record]: https://docs.rs/tracing/latest/tracing/span/struct.Span.html#method.record
    /// [`TraceLayer::make_span_with`]: crate::trace::TraceLayer::make_span_with
    fn on_request(&mut self, request: &Request<Bytes>, span: &Span);
}

impl OnRequest for () {
    #[inline]
    fn on_request(&mut self, _: &Request<Bytes>, _: &Span) {}
}

impl<F> OnRequest for F
where
    F: FnMut(&Request<Bytes>, &Span),
{
    fn on_request(&mut self, request: &Request<Bytes>, span: &Span) {
        self(request, span)
    }
}

/// The default [`OnRequest`] implementation used by [`Trace`].
///
/// [`Trace`]: super::Trace
#[derive(Clone, Debug)]
pub struct DefaultOnRequest {
    level: Level,
}

impl Default for DefaultOnRequest {
    fn default() -> Self {
        Self {
            level: DEFAULT_MESSAGE_LEVEL,
        }
    }
}

impl DefaultOnRequest {
    /// Create a new `DefaultOnRequest`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the [`Level`] used for [tracing events].
    ///
    /// Please note that while this will set the level for the tracing events
    /// themselves, it might cause them to lack expected information, like
    /// request method or path. You can address this using
    /// [`DefaultMakeSpan::level`].
    ///
    /// Defaults to [`Level::DEBUG`].
    ///
    /// [tracing events]: https://docs.rs/tracing/latest/tracing/#events
    /// [`DefaultMakeSpan::level`]: super::DefaultMakeSpan::level
    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }
}

impl OnRequest for DefaultOnRequest {
    fn on_request(&mut self, _: &Request<Bytes>, _: &Span) {
        match self.level {
            Level::ERROR => {
                tracing::event!(Level::ERROR, "started processing request");
            }
            Level::WARN => {
                tracing::event!(Level::WARN, "started processing request");
            }
            Level::INFO => {
                tracing::event!(Level::INFO, "started processing request");
            }
            Level::DEBUG => {
                tracing::event!(Level::DEBUG, "started processing request");
            }
            Level::TRACE => {
                tracing::event!(Level::TRACE, "started processing request");
            }
        }
    }
}
