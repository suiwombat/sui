// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_versioning::_put_bucket_versioning_output::PutBucketVersioningOutputBuilder;

pub use crate::operation::put_bucket_versioning::_put_bucket_versioning_input::PutBucketVersioningInputBuilder;

impl PutBucketVersioningInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_bucket_versioning::PutBucketVersioningOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_versioning::PutBucketVersioningError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_bucket_versioning();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutBucketVersioning`.
///
/// <p>Sets the versioning state of an existing bucket.</p>
/// <p>You can set the versioning state with one of the following values:</p>
/// <p> <b>Enabled</b>—Enables versioning for the objects in the bucket. All objects added to the bucket receive a unique version ID.</p>
/// <p> <b>Suspended</b>—Disables versioning for the objects in the bucket. All objects added to the bucket receive the version ID null.</p>
/// <p>If the versioning state has never been set on a bucket, it has no versioning state; a <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketVersioning.html">GetBucketVersioning</a> request does not return a versioning state value.</p>
/// <p>In order to enable MFA Delete, you must be the bucket owner. If you are the bucket owner and want to enable MFA Delete in the bucket versioning configuration, you must include the <code>x-amz-mfa request</code> header and the <code>Status</code> and the <code>MfaDelete</code> request elements in a request to set the versioning state of the bucket.</p> <important>
/// <p>If you have an object expiration lifecycle configuration in your non-versioned bucket and you want to maintain the same permanent delete behavior when you enable versioning, you must add a noncurrent expiration policy. The noncurrent expiration lifecycle configuration will manage the deletes of the noncurrent object versions in the version-enabled bucket. (A version-enabled bucket maintains one current and zero or more noncurrent object versions.) For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/object-lifecycle-mgmt.html#lifecycle-and-other-bucket-config">Lifecycle and Versioning</a>.</p>
/// </important>
/// <p>The following operations are related to <code>PutBucketVersioning</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketVersioning.html">GetBucketVersioning</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutBucketVersioningFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_bucket_versioning::builders::PutBucketVersioningInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl PutBucketVersioningFluentBuilder {
    /// Creates a new `PutBucketVersioning`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutBucketVersioning as a reference.
    pub fn as_input(&self) -> &crate::operation::put_bucket_versioning::builders::PutBucketVersioningInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::put_bucket_versioning::PutBucketVersioningOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_versioning::PutBucketVersioningError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_bucket_versioning::PutBucketVersioning::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_bucket_versioning::PutBucketVersioning::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::put_bucket_versioning::PutBucketVersioningOutput,
            crate::operation::put_bucket_versioning::PutBucketVersioningError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::put_bucket_versioning::PutBucketVersioningError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The bucket name.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The bucket name.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The bucket name.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn get_content_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_content_md5()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::ChecksumAlgorithm>) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<crate::types::ChecksumAlgorithm> {
        self.inner.get_checksum_algorithm()
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn mfa(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mfa(input.into());
        self
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn set_mfa(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mfa(input);
        self
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn get_mfa(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mfa()
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn versioning_configuration(mut self, input: crate::types::VersioningConfiguration) -> Self {
        self.inner = self.inner.versioning_configuration(input);
        self
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn set_versioning_configuration(mut self, input: ::std::option::Option<crate::types::VersioningConfiguration>) -> Self {
        self.inner = self.inner.set_versioning_configuration(input);
        self
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn get_versioning_configuration(&self) -> &::std::option::Option<crate::types::VersioningConfiguration> {
        self.inner.get_versioning_configuration()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn get_expected_bucket_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expected_bucket_owner()
    }
}
