// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetObjectLegalHoldOutput {
    /// <p>The current legal hold status for the specified object.</p>
    pub legal_hold: ::std::option::Option<crate::types::ObjectLockLegalHold>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetObjectLegalHoldOutput {
    /// <p>The current legal hold status for the specified object.</p>
    pub fn legal_hold(&self) -> ::std::option::Option<&crate::types::ObjectLockLegalHold> {
        self.legal_hold.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetObjectLegalHoldOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetObjectLegalHoldOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetObjectLegalHoldOutput {
    /// Creates a new builder-style object to manufacture [`GetObjectLegalHoldOutput`](crate::operation::get_object_legal_hold::GetObjectLegalHoldOutput).
    pub fn builder() -> crate::operation::get_object_legal_hold::builders::GetObjectLegalHoldOutputBuilder {
        crate::operation::get_object_legal_hold::builders::GetObjectLegalHoldOutputBuilder::default()
    }
}

/// A builder for [`GetObjectLegalHoldOutput`](crate::operation::get_object_legal_hold::GetObjectLegalHoldOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetObjectLegalHoldOutputBuilder {
    pub(crate) legal_hold: ::std::option::Option<crate::types::ObjectLockLegalHold>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetObjectLegalHoldOutputBuilder {
    /// <p>The current legal hold status for the specified object.</p>
    pub fn legal_hold(mut self, input: crate::types::ObjectLockLegalHold) -> Self {
        self.legal_hold = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current legal hold status for the specified object.</p>
    pub fn set_legal_hold(mut self, input: ::std::option::Option<crate::types::ObjectLockLegalHold>) -> Self {
        self.legal_hold = input;
        self
    }
    /// <p>The current legal hold status for the specified object.</p>
    pub fn get_legal_hold(&self) -> &::std::option::Option<crate::types::ObjectLockLegalHold> {
        &self.legal_hold
    }
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
    /// Consumes the builder and constructs a [`GetObjectLegalHoldOutput`](crate::operation::get_object_legal_hold::GetObjectLegalHoldOutput).
    pub fn build(self) -> crate::operation::get_object_legal_hold::GetObjectLegalHoldOutput {
        crate::operation::get_object_legal_hold::GetObjectLegalHoldOutput {
            legal_hold: self.legal_hold,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
