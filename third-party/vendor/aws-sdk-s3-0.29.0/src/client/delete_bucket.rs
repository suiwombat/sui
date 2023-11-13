// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucket`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::set_bucket): <p>Specifies the bucket being deleted.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`DeleteBucketOutput`](crate::operation::delete_bucket::DeleteBucketOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketError>`](crate::operation::delete_bucket::DeleteBucketError)
    pub fn delete_bucket(&self) -> crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder {
        crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::new(self.handle.clone())
    }
}
