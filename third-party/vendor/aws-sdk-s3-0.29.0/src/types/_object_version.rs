// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The version of an object.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ObjectVersion {
    /// <p>The entity tag is an MD5 hash of that version of the object.</p>
    pub e_tag: ::std::option::Option<::std::string::String>,
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub checksum_algorithm: ::std::option::Option<::std::vec::Vec<crate::types::ChecksumAlgorithm>>,
    /// <p>Size in bytes of the object.</p>
    pub size: i64,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: ::std::option::Option<crate::types::ObjectVersionStorageClass>,
    /// <p>The object key.</p>
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>Version ID of an object.</p>
    pub version_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub is_latest: bool,
    /// <p>Date and time the object was last modified.</p>
    pub last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Specifies the owner of the object.</p>
    pub owner: ::std::option::Option<crate::types::Owner>,
    /// <p>Specifies the restoration status of an object. Objects in certain storage classes must be restored before they can be retrieved. For more information about these storage classes and how to work with archived objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/archived-objects.html"> Working with archived objects</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub restore_status: ::std::option::Option<crate::types::RestoreStatus>,
}
impl ObjectVersion {
    /// <p>The entity tag is an MD5 hash of that version of the object.</p>
    pub fn e_tag(&self) -> ::std::option::Option<&str> {
        self.e_tag.as_deref()
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(&self) -> ::std::option::Option<&[crate::types::ChecksumAlgorithm]> {
        self.checksum_algorithm.as_deref()
    }
    /// <p>Size in bytes of the object.</p>
    pub fn size(&self) -> i64 {
        self.size
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn storage_class(&self) -> ::std::option::Option<&crate::types::ObjectVersionStorageClass> {
        self.storage_class.as_ref()
    }
    /// <p>The object key.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>Version ID of an object.</p>
    pub fn version_id(&self) -> ::std::option::Option<&str> {
        self.version_id.as_deref()
    }
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub fn is_latest(&self) -> bool {
        self.is_latest
    }
    /// <p>Date and time the object was last modified.</p>
    pub fn last_modified(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
    /// <p>Specifies the owner of the object.</p>
    pub fn owner(&self) -> ::std::option::Option<&crate::types::Owner> {
        self.owner.as_ref()
    }
    /// <p>Specifies the restoration status of an object. Objects in certain storage classes must be restored before they can be retrieved. For more information about these storage classes and how to work with archived objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/archived-objects.html"> Working with archived objects</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn restore_status(&self) -> ::std::option::Option<&crate::types::RestoreStatus> {
        self.restore_status.as_ref()
    }
}
impl ObjectVersion {
    /// Creates a new builder-style object to manufacture [`ObjectVersion`](crate::types::ObjectVersion).
    pub fn builder() -> crate::types::builders::ObjectVersionBuilder {
        crate::types::builders::ObjectVersionBuilder::default()
    }
}

/// A builder for [`ObjectVersion`](crate::types::ObjectVersion).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ObjectVersionBuilder {
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_algorithm: ::std::option::Option<::std::vec::Vec<crate::types::ChecksumAlgorithm>>,
    pub(crate) size: ::std::option::Option<i64>,
    pub(crate) storage_class: ::std::option::Option<crate::types::ObjectVersionStorageClass>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) version_id: ::std::option::Option<::std::string::String>,
    pub(crate) is_latest: ::std::option::Option<bool>,
    pub(crate) last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) owner: ::std::option::Option<crate::types::Owner>,
    pub(crate) restore_status: ::std::option::Option<crate::types::RestoreStatus>,
}
impl ObjectVersionBuilder {
    /// <p>The entity tag is an MD5 hash of that version of the object.</p>
    pub fn e_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.e_tag = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The entity tag is an MD5 hash of that version of the object.</p>
    pub fn set_e_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.e_tag = input;
        self
    }
    /// <p>The entity tag is an MD5 hash of that version of the object.</p>
    pub fn get_e_tag(&self) -> &::std::option::Option<::std::string::String> {
        &self.e_tag
    }
    /// Appends an item to `checksum_algorithm`.
    ///
    /// To override the contents of this collection use [`set_checksum_algorithm`](Self::set_checksum_algorithm).
    ///
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        let mut v = self.checksum_algorithm.unwrap_or_default();
        v.push(input);
        self.checksum_algorithm = ::std::option::Option::Some(v);
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ChecksumAlgorithm>>) -> Self {
        self.checksum_algorithm = input;
        self
    }
    /// <p>The algorithm that was used to create a checksum of the object.</p>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ChecksumAlgorithm>> {
        &self.checksum_algorithm
    }
    /// <p>Size in bytes of the object.</p>
    pub fn size(mut self, input: i64) -> Self {
        self.size = ::std::option::Option::Some(input);
        self
    }
    /// <p>Size in bytes of the object.</p>
    pub fn set_size(mut self, input: ::std::option::Option<i64>) -> Self {
        self.size = input;
        self
    }
    /// <p>Size in bytes of the object.</p>
    pub fn get_size(&self) -> &::std::option::Option<i64> {
        &self.size
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn storage_class(mut self, input: crate::types::ObjectVersionStorageClass) -> Self {
        self.storage_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn set_storage_class(mut self, input: ::std::option::Option<crate::types::ObjectVersionStorageClass>) -> Self {
        self.storage_class = input;
        self
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn get_storage_class(&self) -> &::std::option::Option<crate::types::ObjectVersionStorageClass> {
        &self.storage_class
    }
    /// <p>The object key.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The object key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The object key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>Version ID of an object.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Version ID of an object.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_id = input;
        self
    }
    /// <p>Version ID of an object.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.version_id
    }
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub fn is_latest(mut self, input: bool) -> Self {
        self.is_latest = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub fn set_is_latest(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_latest = input;
        self
    }
    /// <p>Specifies whether the object is (true) or is not (false) the latest version of an object.</p>
    pub fn get_is_latest(&self) -> &::std::option::Option<bool> {
        &self.is_latest
    }
    /// <p>Date and time the object was last modified.</p>
    pub fn last_modified(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified = ::std::option::Option::Some(input);
        self
    }
    /// <p>Date and time the object was last modified.</p>
    pub fn set_last_modified(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modified = input;
        self
    }
    /// <p>Date and time the object was last modified.</p>
    pub fn get_last_modified(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modified
    }
    /// <p>Specifies the owner of the object.</p>
    pub fn owner(mut self, input: crate::types::Owner) -> Self {
        self.owner = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the owner of the object.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<crate::types::Owner>) -> Self {
        self.owner = input;
        self
    }
    /// <p>Specifies the owner of the object.</p>
    pub fn get_owner(&self) -> &::std::option::Option<crate::types::Owner> {
        &self.owner
    }
    /// <p>Specifies the restoration status of an object. Objects in certain storage classes must be restored before they can be retrieved. For more information about these storage classes and how to work with archived objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/archived-objects.html"> Working with archived objects</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn restore_status(mut self, input: crate::types::RestoreStatus) -> Self {
        self.restore_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the restoration status of an object. Objects in certain storage classes must be restored before they can be retrieved. For more information about these storage classes and how to work with archived objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/archived-objects.html"> Working with archived objects</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_restore_status(mut self, input: ::std::option::Option<crate::types::RestoreStatus>) -> Self {
        self.restore_status = input;
        self
    }
    /// <p>Specifies the restoration status of an object. Objects in certain storage classes must be restored before they can be retrieved. For more information about these storage classes and how to work with archived objects, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/archived-objects.html"> Working with archived objects</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_restore_status(&self) -> &::std::option::Option<crate::types::RestoreStatus> {
        &self.restore_status
    }
    /// Consumes the builder and constructs a [`ObjectVersion`](crate::types::ObjectVersion).
    pub fn build(self) -> crate::types::ObjectVersion {
        crate::types::ObjectVersion {
            e_tag: self.e_tag,
            checksum_algorithm: self.checksum_algorithm,
            size: self.size.unwrap_or_default(),
            storage_class: self.storage_class,
            key: self.key,
            version_id: self.version_id,
            is_latest: self.is_latest.unwrap_or_default(),
            last_modified: self.last_modified,
            owner: self.owner,
            restore_status: self.restore_status,
        }
    }
}
