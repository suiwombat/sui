// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_parts::_list_parts_output::ListPartsOutputBuilder;

pub use crate::operation::list_parts::_list_parts_input::ListPartsInputBuilder;

impl ListPartsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_parts::ListPartsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_parts::ListPartsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_parts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListParts`.
///
/// <p>Lists the parts that have been uploaded for a specific multipart upload. This operation must include the upload ID, which you obtain by sending the initiate multipart upload request (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>). This request returns a maximum of 1,000 uploaded parts. The default number of parts returned is 1,000 parts. You can restrict the number of parts returned by specifying the <code>max-parts</code> request parameter. If your multipart upload consists of more than 1,000 parts, the response returns an <code>IsTruncated</code> field with the value of true, and a <code>NextPartNumberMarker</code> element. In subsequent <code>ListParts</code> requests you can include the part-number-marker query string parameter and set its value to the <code>NextPartNumberMarker</code> field value from the previous response.</p>
/// <p>If the upload was created using a checksum algorithm, you will need to have permission to the <code>kms:Decrypt</code> action for the request to succeed. </p>
/// <p>For more information on multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/uploadobjusingmpu.html">Uploading Objects Using Multipart Upload</a>.</p>
/// <p>For information on permissions required to use the multipart upload API, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and Permissions</a>.</p>
/// <p>The following operations are related to <code>ListParts</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPart.html">UploadPart</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAttributes.html">GetObjectAttributes</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListPartsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_parts::builders::ListPartsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ListPartsFluentBuilder {
    /// Creates a new `ListParts`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListParts as a reference.
    pub fn as_input(&self) -> &crate::operation::list_parts::builders::ListPartsInputBuilder {
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
        crate::operation::list_parts::ListPartsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_parts::ListPartsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_parts::ListParts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_parts::ListParts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::list_parts::ListPartsOutput,
            crate::operation::list_parts::ListPartsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::list_parts::ListPartsError>,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_parts::paginator::ListPartsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::list_parts::paginator::ListPartsPaginator {
        crate::operation::list_parts::paginator::ListPartsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the bucket to which the parts are being uploaded. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key()
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn max_parts(mut self, input: i32) -> Self {
        self.inner = self.inner.max_parts(input);
        self
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn set_max_parts(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_parts(input);
        self
    }
    /// <p>Sets the maximum number of parts to return.</p>
    pub fn get_max_parts(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_parts()
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn part_number_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.part_number_marker(input.into());
        self
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn set_part_number_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_part_number_marker(input);
        self
    }
    /// <p>Specifies the part after which listing should begin. Only parts with higher part numbers will be listed.</p>
    pub fn get_part_number_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_part_number_marker()
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.upload_id(input.into());
        self
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id(input);
        self
    }
    /// <p>Upload ID identifying the multipart upload whose parts are being listed.</p>
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_upload_id()
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.inner = self.inner.request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_request_payer(mut self, input: ::std::option::Option<crate::types::RequestPayer>) -> Self {
        self.inner = self.inner.set_request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_request_payer(&self) -> &::std::option::Option<crate::types::RequestPayer> {
        self.inner.get_request_payer()
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
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn sse_customer_algorithm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_algorithm(input.into());
        self
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_sse_customer_algorithm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_algorithm(input);
        self
    }
    /// <p>The server-side encryption (SSE) algorithm used to encrypt the object. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_sse_customer_algorithm(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_algorithm()
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn sse_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key(input.into());
        self
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_sse_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key(input);
        self
    }
    /// <p>The server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_sse_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_key()
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn sse_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key_md5(input.into());
        self
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_sse_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key_md5(input);
        self
    }
    /// <p>The MD5 server-side encryption (SSE) customer managed key. This parameter is needed only when the object was created using a checksum algorithm. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Protecting data using SSE-C keys</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_sse_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_key_md5()
    }
}
