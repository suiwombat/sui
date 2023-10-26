// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBucketAnalyticsConfigurationInput {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The ID that identifies the analytics configuration.</p>
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl DeleteBucketAnalyticsConfigurationInput {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl DeleteBucketAnalyticsConfigurationInput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketAnalyticsConfigurationInput`](crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationInput).
    pub fn builder() -> crate::operation::delete_bucket_analytics_configuration::builders::DeleteBucketAnalyticsConfigurationInputBuilder {
        crate::operation::delete_bucket_analytics_configuration::builders::DeleteBucketAnalyticsConfigurationInputBuilder::default()
    }
}

/// A builder for [`DeleteBucketAnalyticsConfigurationInput`](crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteBucketAnalyticsConfigurationInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl DeleteBucketAnalyticsConfigurationInputBuilder {
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket from which an analytics configuration is deleted.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID that identifies the analytics configuration.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
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
    /// Consumes the builder and constructs a [`DeleteBucketAnalyticsConfigurationInput`](crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_bucket_analytics_configuration::DeleteBucketAnalyticsConfigurationInput {
                bucket: self.bucket,
                id: self.id,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}
