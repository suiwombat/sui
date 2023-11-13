// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub intelligent_tiering_configuration: ::std::option::Option<crate::types::IntelligentTieringConfiguration>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketIntelligentTieringConfigurationOutput {
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn intelligent_tiering_configuration(&self) -> ::std::option::Option<&crate::types::IntelligentTieringConfiguration> {
        self.intelligent_tiering_configuration.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for GetBucketIntelligentTieringConfigurationOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for GetBucketIntelligentTieringConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetBucketIntelligentTieringConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`GetBucketIntelligentTieringConfigurationOutput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput).
    pub fn builder() -> crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationOutputBuilder
    {
        crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationOutputBuilder::default()
    }
}

/// A builder for [`GetBucketIntelligentTieringConfigurationOutput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetBucketIntelligentTieringConfigurationOutputBuilder {
    pub(crate) intelligent_tiering_configuration: ::std::option::Option<crate::types::IntelligentTieringConfiguration>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl GetBucketIntelligentTieringConfigurationOutputBuilder {
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn intelligent_tiering_configuration(mut self, input: crate::types::IntelligentTieringConfiguration) -> Self {
        self.intelligent_tiering_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn set_intelligent_tiering_configuration(mut self, input: ::std::option::Option<crate::types::IntelligentTieringConfiguration>) -> Self {
        self.intelligent_tiering_configuration = input;
        self
    }
    /// <p>Container for S3 Intelligent-Tiering configuration.</p>
    pub fn get_intelligent_tiering_configuration(&self) -> &::std::option::Option<crate::types::IntelligentTieringConfiguration> {
        &self.intelligent_tiering_configuration
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
    /// Consumes the builder and constructs a [`GetBucketIntelligentTieringConfigurationOutput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput).
    pub fn build(self) -> crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput {
        crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput {
            intelligent_tiering_configuration: self.intelligent_tiering_configuration,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
