// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPublicAccessBlockOutput {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub public_access_block_configuration: ::std::option::Option<crate::types::PublicAccessBlockConfiguration>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetPublicAccessBlockOutput {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub fn public_access_block_configuration(&self) -> ::std::option::Option<&crate::types::PublicAccessBlockConfiguration> {
        self.public_access_block_configuration.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetPublicAccessBlockOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetPublicAccessBlockOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetPublicAccessBlockOutput {
    /// Creates a new builder-style object to manufacture [`GetPublicAccessBlockOutput`](crate::operation::get_public_access_block::GetPublicAccessBlockOutput).
    pub fn builder() -> crate::operation::get_public_access_block::builders::GetPublicAccessBlockOutputBuilder {
        crate::operation::get_public_access_block::builders::GetPublicAccessBlockOutputBuilder::default()
    }
}

/// A builder for [`GetPublicAccessBlockOutput`](crate::operation::get_public_access_block::GetPublicAccessBlockOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetPublicAccessBlockOutputBuilder {
    pub(crate) public_access_block_configuration: ::std::option::Option<crate::types::PublicAccessBlockConfiguration>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetPublicAccessBlockOutputBuilder {
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub fn public_access_block_configuration(mut self, input: crate::types::PublicAccessBlockConfiguration) -> Self {
        self.public_access_block_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub fn set_public_access_block_configuration(mut self, input: ::std::option::Option<crate::types::PublicAccessBlockConfiguration>) -> Self {
        self.public_access_block_configuration = input;
        self
    }
    /// <p>The <code>PublicAccessBlock</code> configuration currently in effect for this Amazon S3 bucket.</p>
    pub fn get_public_access_block_configuration(&self) -> &::std::option::Option<crate::types::PublicAccessBlockConfiguration> {
        &self.public_access_block_configuration
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
    /// Consumes the builder and constructs a [`GetPublicAccessBlockOutput`](crate::operation::get_public_access_block::GetPublicAccessBlockOutput).
    pub fn build(self) -> crate::operation::get_public_access_block::GetPublicAccessBlockOutput {
        crate::operation::get_public_access_block::GetPublicAccessBlockOutput {
            public_access_block_configuration: self.public_access_block_configuration,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
