// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketLifecycle`](crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder::set_bucket): <p>The bucket name of the lifecycle to delete.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`DeleteBucketLifecycleOutput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketLifecycleError>`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleError)
    pub fn delete_bucket_lifecycle(&self) -> crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder {
        crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleFluentBuilder::new(self.handle.clone())
    }
}
