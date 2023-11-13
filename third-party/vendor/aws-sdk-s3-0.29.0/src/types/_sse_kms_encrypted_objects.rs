// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A container for filter information for the selection of S3 objects encrypted with Amazon Web Services KMS.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SseKmsEncryptedObjects {
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub status: ::std::option::Option<crate::types::SseKmsEncryptedObjectsStatus>,
}
impl SseKmsEncryptedObjects {
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::SseKmsEncryptedObjectsStatus> {
        self.status.as_ref()
    }
}
impl SseKmsEncryptedObjects {
    /// Creates a new builder-style object to manufacture [`SseKmsEncryptedObjects`](crate::types::SseKmsEncryptedObjects).
    pub fn builder() -> crate::types::builders::SseKmsEncryptedObjectsBuilder {
        crate::types::builders::SseKmsEncryptedObjectsBuilder::default()
    }
}

/// A builder for [`SseKmsEncryptedObjects`](crate::types::SseKmsEncryptedObjects).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SseKmsEncryptedObjectsBuilder {
    pub(crate) status: ::std::option::Option<crate::types::SseKmsEncryptedObjectsStatus>,
}
impl SseKmsEncryptedObjectsBuilder {
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub fn status(mut self, input: crate::types::SseKmsEncryptedObjectsStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::SseKmsEncryptedObjectsStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies whether Amazon S3 replicates objects created with server-side encryption using an Amazon Web Services KMS key stored in Amazon Web Services Key Management Service.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::SseKmsEncryptedObjectsStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`SseKmsEncryptedObjects`](crate::types::SseKmsEncryptedObjects).
    pub fn build(self) -> crate::types::SseKmsEncryptedObjects {
        crate::types::SseKmsEncryptedObjects { status: self.status }
    }
}
