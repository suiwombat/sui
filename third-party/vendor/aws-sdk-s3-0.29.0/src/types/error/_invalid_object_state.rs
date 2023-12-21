// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Object is archived and inaccessible until restored.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InvalidObjectState {
    #[allow(missing_docs)] // documentation missing in model
    pub storage_class: ::std::option::Option<crate::types::StorageClass>,
    #[allow(missing_docs)] // documentation missing in model
    pub access_tier: ::std::option::Option<crate::types::IntelligentTieringAccessTier>,
    #[allow(missing_docs)] // documentation missing in model
    pub message: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl InvalidObjectState {
    #[allow(missing_docs)] // documentation missing in model
    pub fn storage_class(&self) -> ::std::option::Option<&crate::types::StorageClass> {
        self.storage_class.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn access_tier(&self) -> ::std::option::Option<&crate::types::IntelligentTieringAccessTier> {
        self.access_tier.as_ref()
    }
}
impl InvalidObjectState {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for InvalidObjectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "InvalidObjectState")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for InvalidObjectState {}
impl crate::s3_request_id::RequestIdExt for crate::types::error::InvalidObjectState {
    fn extended_request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().extended_request_id()
    }
}
impl ::aws_http::request_id::RequestId for crate::types::error::InvalidObjectState {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidObjectState {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl InvalidObjectState {
    /// Creates a new builder-style object to manufacture [`InvalidObjectState`](crate::types::error::InvalidObjectState).
    pub fn builder() -> crate::types::error::builders::InvalidObjectStateBuilder {
        crate::types::error::builders::InvalidObjectStateBuilder::default()
    }
}

/// A builder for [`InvalidObjectState`](crate::types::error::InvalidObjectState).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InvalidObjectStateBuilder {
    pub(crate) storage_class: ::std::option::Option<crate::types::StorageClass>,
    pub(crate) access_tier: ::std::option::Option<crate::types::IntelligentTieringAccessTier>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl InvalidObjectStateBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn storage_class(mut self, input: crate::types::StorageClass) -> Self {
        self.storage_class = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_storage_class(mut self, input: ::std::option::Option<crate::types::StorageClass>) -> Self {
        self.storage_class = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_storage_class(&self) -> &::std::option::Option<crate::types::StorageClass> {
        &self.storage_class
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn access_tier(mut self, input: crate::types::IntelligentTieringAccessTier) -> Self {
        self.access_tier = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_access_tier(mut self, input: ::std::option::Option<crate::types::IntelligentTieringAccessTier>) -> Self {
        self.access_tier = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_access_tier(&self) -> &::std::option::Option<crate::types::IntelligentTieringAccessTier> {
        &self.access_tier
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(&mut self, meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`InvalidObjectState`](crate::types::error::InvalidObjectState).
    pub fn build(self) -> crate::types::error::InvalidObjectState {
        crate::types::error::InvalidObjectState {
            storage_class: self.storage_class,
            access_tier: self.access_tier,
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
