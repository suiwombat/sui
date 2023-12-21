// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_encryption::_put_bucket_encryption_output::PutBucketEncryptionOutputBuilder;

pub use crate::operation::put_bucket_encryption::_put_bucket_encryption_input::PutBucketEncryptionInputBuilder;

impl PutBucketEncryptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_bucket_encryption::PutBucketEncryptionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_encryption::PutBucketEncryptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_bucket_encryption();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutBucketEncryption`.
///
/// <p>This action uses the <code>encryption</code> subresource to configure default encryption and Amazon S3 Bucket Keys for an existing bucket.</p>
/// <p>By default, all buckets have a default encryption configuration that uses server-side encryption with Amazon S3 managed keys (SSE-S3). You can optionally configure default encryption for a bucket by using server-side encryption with Key Management Service (KMS) keys (SSE-KMS), dual-layer server-side encryption with Amazon Web Services KMS keys (DSSE-KMS), or server-side encryption with customer-provided keys (SSE-C). If you specify default encryption by using SSE-KMS, you can also configure Amazon S3 Bucket Keys. For information about bucket default encryption, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 bucket default encryption</a> in the <i>Amazon S3 User Guide</i>. For more information about S3 Bucket Keys, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html">Amazon S3 Bucket Keys</a> in the <i>Amazon S3 User Guide</i>.</p> <important>
/// <p>This action requires Amazon Web Services Signature Version 4. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html"> Authenticating Requests (Amazon Web Services Signature Version 4)</a>. </p>
/// </important>
/// <p>To use this operation, you must have permission to perform the <code>s3:PutEncryptionConfiguration</code> action. The bucket owner has this permission by default. The bucket owner can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources">Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing Access Permissions to Your Amazon S3 Resources</a> in the <i>Amazon S3 User Guide</i>. </p>
/// <p>The following operations are related to <code>PutBucketEncryption</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketEncryption.html">GetBucketEncryption</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucketEncryption.html">DeleteBucketEncryption</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutBucketEncryptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_bucket_encryption::builders::PutBucketEncryptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl PutBucketEncryptionFluentBuilder {
    /// Creates a new `PutBucketEncryption`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutBucketEncryption as a reference.
    pub fn as_input(&self) -> &crate::operation::put_bucket_encryption::builders::PutBucketEncryptionInputBuilder {
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
        crate::operation::put_bucket_encryption::PutBucketEncryptionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_encryption::PutBucketEncryptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_bucket_encryption::PutBucketEncryption::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_bucket_encryption::PutBucketEncryption::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::put_bucket_encryption::PutBucketEncryptionOutput,
            crate::operation::put_bucket_encryption::PutBucketEncryptionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::put_bucket_encryption::PutBucketEncryptionError>,
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
    /// <p>Specifies default encryption for a bucket using server-side encryption with different key options. By default, all buckets have a default encryption configuration that uses server-side encryption with Amazon S3 managed keys (SSE-S3). You can optionally configure default encryption for a bucket by using server-side encryption with an Amazon Web Services KMS key (SSE-KMS) or a customer-provided key (SSE-C). For information about the bucket default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Bucket Default Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>Specifies default encryption for a bucket using server-side encryption with different key options. By default, all buckets have a default encryption configuration that uses server-side encryption with Amazon S3 managed keys (SSE-S3). You can optionally configure default encryption for a bucket by using server-side encryption with an Amazon Web Services KMS key (SSE-KMS) or a customer-provided key (SSE-C). For information about the bucket default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Bucket Default Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>Specifies default encryption for a bucket using server-side encryption with different key options. By default, all buckets have a default encryption configuration that uses server-side encryption with Amazon S3 managed keys (SSE-S3). You can optionally configure default encryption for a bucket by using server-side encryption with an Amazon Web Services KMS key (SSE-KMS) or a customer-provided key (SSE-C). For information about the bucket default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Bucket Default Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
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
    /// <p>Specifies the default server-side-encryption configuration.</p>
    pub fn server_side_encryption_configuration(mut self, input: crate::types::ServerSideEncryptionConfiguration) -> Self {
        self.inner = self.inner.server_side_encryption_configuration(input);
        self
    }
    /// <p>Specifies the default server-side-encryption configuration.</p>
    pub fn set_server_side_encryption_configuration(mut self, input: ::std::option::Option<crate::types::ServerSideEncryptionConfiguration>) -> Self {
        self.inner = self.inner.set_server_side_encryption_configuration(input);
        self
    }
    /// <p>Specifies the default server-side-encryption configuration.</p>
    pub fn get_server_side_encryption_configuration(&self) -> &::std::option::Option<crate::types::ServerSideEncryptionConfiguration> {
        self.inner.get_server_side_encryption_configuration()
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
