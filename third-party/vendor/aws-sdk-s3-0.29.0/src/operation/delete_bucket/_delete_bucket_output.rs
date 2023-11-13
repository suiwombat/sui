// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBucketOutput {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl crate::s3_request_id::RequestIdExt for DeleteBucketOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteBucketOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteBucketOutput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketOutput`](crate::operation::delete_bucket::DeleteBucketOutput).
    pub fn builder() -> crate::operation::delete_bucket::builders::DeleteBucketOutputBuilder {
        crate::operation::delete_bucket::builders::DeleteBucketOutputBuilder::default()
    }
}

/// A builder for [`DeleteBucketOutput`](crate::operation::delete_bucket::DeleteBucketOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteBucketOutputBuilder {
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl DeleteBucketOutputBuilder {
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteBucketOutput`](crate::operation::delete_bucket::DeleteBucketOutput).
    pub fn build(self) -> crate::operation::delete_bucket::DeleteBucketOutput {
        crate::operation::delete_bucket::DeleteBucketOutput {
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
