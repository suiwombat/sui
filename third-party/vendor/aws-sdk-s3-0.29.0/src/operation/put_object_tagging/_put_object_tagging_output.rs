// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub version_id: ::std::option::Option<::std::string::String>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl PutObjectTaggingOutput {
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub fn version_id(&self) -> ::std::option::Option<&str> {
        self.version_id.as_deref()
    }
}
impl crate::s3_request_id::RequestIdExt for PutObjectTaggingOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for PutObjectTaggingOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutObjectTaggingOutput {
    /// Creates a new builder-style object to manufacture [`PutObjectTaggingOutput`](crate::operation::put_object_tagging::PutObjectTaggingOutput).
    pub fn builder() -> crate::operation::put_object_tagging::builders::PutObjectTaggingOutputBuilder {
        crate::operation::put_object_tagging::builders::PutObjectTaggingOutputBuilder::default()
    }
}

/// A builder for [`PutObjectTaggingOutput`](crate::operation::put_object_tagging::PutObjectTaggingOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutObjectTaggingOutputBuilder {
    pub(crate) version_id: ::std::option::Option<::std::string::String>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl PutObjectTaggingOutputBuilder {
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// <p>The versionId of the object the tag-set was added to.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.version_id
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
    /// Consumes the builder and constructs a [`PutObjectTaggingOutput`](crate::operation::put_object_tagging::PutObjectTaggingOutput).
    pub fn build(self) -> crate::operation::put_object_tagging::PutObjectTaggingOutput {
        crate::operation::put_object_tagging::PutObjectTaggingOutput {
            version_id: self.version_id,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
