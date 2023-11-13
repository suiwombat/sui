// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the <code>MultipartUpload</code> for the Amazon S3 object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MultipartUpload {
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub upload_id: ::std::option::Option<::std::string::String>,
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub initiated: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: ::std::option::Option<crate::types::StorageClass>,
    /// <p>Specifies the owner of the object that is part of the multipart upload. </p>
    pub owner: ::std::option::Option<crate::types::Owner>,
    /// <p>Identifies who initiated the multipart upload.</p>
    pub initiator: ::std::option::Option<crate::types::Initiator>,
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: ::std::option::Option<crate::types::ChecksumAlgorithm>,
}
impl MultipartUpload {
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub fn upload_id(&self) -> ::std::option::Option<&str> {
        self.upload_id.as_deref()
    }
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub fn initiated(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.initiated.as_ref()
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn storage_class(&self) -> ::std::option::Option<&crate::types::StorageClass> {
        self.storage_class.as_ref()
    }
    /// <p>Specifies the owner of the object that is part of the multipart upload. </p>
    pub fn owner(&self) -> ::std::option::Option<&crate::types::Owner> {
        self.owner.as_ref()
    }
    /// <p>Identifies who initiated the multipart upload.</p>
    pub fn initiator(&self) -> ::std::option::Option<&crate::types::Initiator> {
        self.initiator.as_ref()
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(&self) -> ::std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
impl MultipartUpload {
    /// Creates a new builder-style object to manufacture [`MultipartUpload`](crate::types::MultipartUpload).
    pub fn builder() -> crate::types::builders::MultipartUploadBuilder {
        crate::types::builders::MultipartUploadBuilder::default()
    }
}

/// A builder for [`MultipartUpload`](crate::types::MultipartUpload).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MultipartUploadBuilder {
    pub(crate) upload_id: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) initiated: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) storage_class: ::std::option::Option<crate::types::StorageClass>,
    pub(crate) owner: ::std::option::Option<crate::types::Owner>,
    pub(crate) initiator: ::std::option::Option<crate::types::Initiator>,
    pub(crate) checksum_algorithm: ::std::option::Option<crate::types::ChecksumAlgorithm>,
}
impl MultipartUploadBuilder {
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id = input;
        self
    }
    /// <p>Upload ID that identifies the multipart upload.</p>
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id
    }
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>Key of the object for which the multipart upload was initiated.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub fn initiated(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.initiated = ::std::option::Option::Some(input);
        self
    }
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub fn set_initiated(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.initiated = input;
        self
    }
    /// <p>Date and time at which the multipart upload was initiated.</p>
    pub fn get_initiated(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.initiated
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn storage_class(mut self, input: crate::types::StorageClass) -> Self {
        self.storage_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn set_storage_class(mut self, input: ::std::option::Option<crate::types::StorageClass>) -> Self {
        self.storage_class = input;
        self
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn get_storage_class(&self) -> &::std::option::Option<crate::types::StorageClass> {
        &self.storage_class
    }
    /// <p>Specifies the owner of the object that is part of the multipart upload. </p>
    pub fn owner(mut self, input: crate::types::Owner) -> Self {
        self.owner = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the owner of the object that is part of the multipart upload. </p>
    pub fn set_owner(mut self, input: ::std::option::Option<crate::types::Owner>) -> Self {
        self.owner = input;
        self
    }
    /// <p>Specifies the owner of the object that is part of the multipart upload. </p>
    pub fn get_owner(&self) -> &::std::option::Option<crate::types::Owner> {
        &self.owner
    }
    /// <p>Identifies who initiated the multipart upload.</p>
    pub fn initiator(mut self, input: crate::types::Initiator) -> Self {
        self.initiator = ::std::option::Option::Some(input);
        self
    }
    /// <p>Identifies who initiated the multipart upload.</p>
    pub fn set_initiator(mut self, input: ::std::option::Option<crate::types::Initiator>) -> Self {
        self.initiator = input;
        self
    }
    /// <p>Identifies who initiated the multipart upload.</p>
    pub fn get_initiator(&self) -> &::std::option::Option<crate::types::Initiator> {
        &self.initiator
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.checksum_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::ChecksumAlgorithm>) -> Self {
        self.checksum_algorithm = input;
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<crate::types::ChecksumAlgorithm> {
        &self.checksum_algorithm
    }
    /// Consumes the builder and constructs a [`MultipartUpload`](crate::types::MultipartUpload).
    pub fn build(self) -> crate::types::MultipartUpload {
        crate::types::MultipartUpload {
            upload_id: self.upload_id,
            key: self.key,
            initiated: self.initiated,
            storage_class: self.storage_class,
            owner: self.owner,
            initiator: self.initiator,
            checksum_algorithm: self.checksum_algorithm,
        }
    }
}
