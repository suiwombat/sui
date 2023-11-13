// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type SelectObjectContentEventStreamErrorKind = SelectObjectContentEventStreamError;
/// Error type for the `SelectObjectContentEventStreamError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum SelectObjectContentEventStreamError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for SelectObjectContentEventStreamError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for SelectObjectContentEventStreamError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for SelectObjectContentEventStreamError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
        }
    }
}
impl crate::s3_request_id::RequestIdExt for crate::types::error::SelectObjectContentEventStreamError {
    fn extended_request_id(&self) -> Option<&str> {
        self.meta().extended_request_id()
    }
}
impl ::aws_http::request_id::RequestId for crate::types::error::SelectObjectContentEventStreamError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for SelectObjectContentEventStreamError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl SelectObjectContentEventStreamError {
    /// Creates the `SelectObjectContentEventStreamError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err).build())
    }

    /// Creates the `SelectObjectContentEventStreamError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::Unhandled(e) => e.meta(),
        }
    }
}
impl ::std::error::Error for SelectObjectContentEventStreamError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::types::error::_object_already_in_active_tier_error::ObjectAlreadyInActiveTierError;

pub use crate::types::error::_no_such_key::NoSuchKey;

pub use crate::types::error::_no_such_bucket::NoSuchBucket;

pub use crate::types::error::_not_found::NotFound;

pub use crate::types::error::_invalid_object_state::InvalidObjectState;

pub use crate::types::error::_bucket_already_owned_by_you::BucketAlreadyOwnedByYou;

pub use crate::types::error::_bucket_already_exists::BucketAlreadyExists;

pub use crate::types::error::_object_not_in_active_tier_error::ObjectNotInActiveTierError;

pub use crate::types::error::_no_such_upload::NoSuchUpload;

mod _bucket_already_exists;

mod _bucket_already_owned_by_you;

mod _invalid_object_state;

mod _no_such_bucket;

mod _no_such_key;

mod _no_such_upload;

mod _not_found;

mod _object_already_in_active_tier_error;

mod _object_not_in_active_tier_error;

/// Builders
pub mod builders;
