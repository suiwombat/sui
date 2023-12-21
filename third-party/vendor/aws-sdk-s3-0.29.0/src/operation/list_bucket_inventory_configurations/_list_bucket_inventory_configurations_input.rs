// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListBucketInventoryConfigurationsInput {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub continuation_token: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl ListBucketInventoryConfigurationsInput {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn continuation_token(&self) -> ::std::option::Option<&str> {
        self.continuation_token.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl ListBucketInventoryConfigurationsInput {
    /// Creates a new builder-style object to manufacture [`ListBucketInventoryConfigurationsInput`](crate::operation::list_bucket_inventory_configurations::ListBucketInventoryConfigurationsInput).
    pub fn builder() -> crate::operation::list_bucket_inventory_configurations::builders::ListBucketInventoryConfigurationsInputBuilder {
        crate::operation::list_bucket_inventory_configurations::builders::ListBucketInventoryConfigurationsInputBuilder::default()
    }
}

/// A builder for [`ListBucketInventoryConfigurationsInput`](crate::operation::list_bucket_inventory_configurations::ListBucketInventoryConfigurationsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListBucketInventoryConfigurationsInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) continuation_token: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl ListBucketInventoryConfigurationsInputBuilder {
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket containing the inventory configurations to retrieve.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn continuation_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.continuation_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn set_continuation_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.continuation_token = input;
        self
    }
    /// <p>The marker used to continue an inventory configuration listing that has been truncated. Use the <code>NextContinuationToken</code> from a previously truncated list response to continue the listing. The continuation token is an opaque value that Amazon S3 understands.</p>
    pub fn get_continuation_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.continuation_token
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expected_bucket_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        &self.expected_bucket_owner
    }
    /// Consumes the builder and constructs a [`ListBucketInventoryConfigurationsInput`](crate::operation::list_bucket_inventory_configurations::ListBucketInventoryConfigurationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_bucket_inventory_configurations::ListBucketInventoryConfigurationsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_bucket_inventory_configurations::ListBucketInventoryConfigurationsInput {
                bucket: self.bucket,
                continuation_token: self.continuation_token,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}
