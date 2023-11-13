// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the transition rule that describes when noncurrent objects transition to the <code>STANDARD_IA</code>, <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>, <code>GLACIER_IR</code>, <code>GLACIER</code>, or <code>DEEP_ARCHIVE</code> storage class. If your bucket is versioning-enabled (or versioning is suspended), you can set this action to request that Amazon S3 transition noncurrent object versions to the <code>STANDARD_IA</code>, <code>ONEZONE_IA</code>, <code>INTELLIGENT_TIERING</code>, <code>GLACIER_IR</code>, <code>GLACIER</code>, or <code>DEEP_ARCHIVE</code> storage class at a specific period in the object's lifetime.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NoncurrentVersionTransition {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates How Long an Object Has Been Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub noncurrent_days: i32,
    /// <p>The class of storage used to store the object.</p>
    pub storage_class: ::std::option::Option<crate::types::TransitionStorageClass>,
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub newer_noncurrent_versions: i32,
}
impl NoncurrentVersionTransition {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates How Long an Object Has Been Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn noncurrent_days(&self) -> i32 {
        self.noncurrent_days
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn storage_class(&self) -> ::std::option::Option<&crate::types::TransitionStorageClass> {
        self.storage_class.as_ref()
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn newer_noncurrent_versions(&self) -> i32 {
        self.newer_noncurrent_versions
    }
}
impl NoncurrentVersionTransition {
    /// Creates a new builder-style object to manufacture [`NoncurrentVersionTransition`](crate::types::NoncurrentVersionTransition).
    pub fn builder() -> crate::types::builders::NoncurrentVersionTransitionBuilder {
        crate::types::builders::NoncurrentVersionTransitionBuilder::default()
    }
}

/// A builder for [`NoncurrentVersionTransition`](crate::types::NoncurrentVersionTransition).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct NoncurrentVersionTransitionBuilder {
    pub(crate) noncurrent_days: ::std::option::Option<i32>,
    pub(crate) storage_class: ::std::option::Option<crate::types::TransitionStorageClass>,
    pub(crate) newer_noncurrent_versions: ::std::option::Option<i32>,
}
impl NoncurrentVersionTransitionBuilder {
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates How Long an Object Has Been Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn noncurrent_days(mut self, input: i32) -> Self {
        self.noncurrent_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates How Long an Object Has Been Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_noncurrent_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.noncurrent_days = input;
        self
    }
    /// <p>Specifies the number of days an object is noncurrent before Amazon S3 can perform the associated action. For information about the noncurrent days calculations, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/intro-lifecycle-rules.html#non-current-days-calculations">How Amazon S3 Calculates How Long an Object Has Been Noncurrent</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_noncurrent_days(&self) -> &::std::option::Option<i32> {
        &self.noncurrent_days
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn storage_class(mut self, input: crate::types::TransitionStorageClass) -> Self {
        self.storage_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn set_storage_class(mut self, input: ::std::option::Option<crate::types::TransitionStorageClass>) -> Self {
        self.storage_class = input;
        self
    }
    /// <p>The class of storage used to store the object.</p>
    pub fn get_storage_class(&self) -> &::std::option::Option<crate::types::TransitionStorageClass> {
        &self.storage_class
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn newer_noncurrent_versions(mut self, input: i32) -> Self {
        self.newer_noncurrent_versions = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_newer_noncurrent_versions(mut self, input: ::std::option::Option<i32>) -> Self {
        self.newer_noncurrent_versions = input;
        self
    }
    /// <p>Specifies how many noncurrent versions Amazon S3 will retain. If there are this many more recent noncurrent versions, Amazon S3 will take the associated action. For more information about noncurrent versions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/intro-lifecycle-rules.html">Lifecycle configuration elements</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_newer_noncurrent_versions(&self) -> &::std::option::Option<i32> {
        &self.newer_noncurrent_versions
    }
    /// Consumes the builder and constructs a [`NoncurrentVersionTransition`](crate::types::NoncurrentVersionTransition).
    pub fn build(self) -> crate::types::NoncurrentVersionTransition {
        crate::types::NoncurrentVersionTransition {
            noncurrent_days: self.noncurrent_days.unwrap_or_default(),
            storage_class: self.storage_class,
            newer_noncurrent_versions: self.newer_noncurrent_versions.unwrap_or_default(),
        }
    }
}
