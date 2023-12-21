// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LogoutOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for LogoutOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl LogoutOutput {
    /// Creates a new builder-style object to manufacture [`LogoutOutput`](crate::operation::logout::LogoutOutput).
    pub fn builder() -> crate::operation::logout::builders::LogoutOutputBuilder {
        crate::operation::logout::builders::LogoutOutputBuilder::default()
    }
}

/// A builder for [`LogoutOutput`](crate::operation::logout::LogoutOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LogoutOutputBuilder {
    _request_id: Option<String>,
}
impl LogoutOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`LogoutOutput`](crate::operation::logout::LogoutOutput).
    pub fn build(self) -> crate::operation::logout::LogoutOutput {
        crate::operation::logout::LogoutOutput {
            _request_id: self._request_id,
        }
    }
}
