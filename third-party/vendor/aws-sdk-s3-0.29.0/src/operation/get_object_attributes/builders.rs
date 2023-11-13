// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_object_attributes::_get_object_attributes_output::GetObjectAttributesOutputBuilder;

pub use crate::operation::get_object_attributes::_get_object_attributes_input::GetObjectAttributesInputBuilder;

impl GetObjectAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_object_attributes::GetObjectAttributesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_object_attributes::GetObjectAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_object_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetObjectAttributes`.
///
/// <p>Retrieves all the metadata from an object without returning the object itself. This action is useful if you're interested only in an object's metadata. To use <code>GetObjectAttributes</code>, you must have READ access to the object.</p>
/// <p> <code>GetObjectAttributes</code> combines the functionality of <code>HeadObject</code> and <code>ListParts</code>. All of the data returned with each of those individual calls can be returned with a single call to <code>GetObjectAttributes</code>.</p>
/// <p>If you encrypt an object by using server-side encryption with customer-provided encryption keys (SSE-C) when you store the object in Amazon S3, then when you retrieve the metadata from the object, you must use the following headers:</p>
/// <ul>
/// <li> <p> <code>x-amz-server-side-encryption-customer-algorithm</code> </p> </li>
/// <li> <p> <code>x-amz-server-side-encryption-customer-key</code> </p> </li>
/// <li> <p> <code>x-amz-server-side-encryption-customer-key-MD5</code> </p> </li>
/// </ul>
/// <p>For more information about SSE-C, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using Customer-Provided Encryption Keys)</a> in the <i>Amazon S3 User Guide</i>.</p> <note>
/// <ul>
/// <li> <p>Encryption request headers, such as <code>x-amz-server-side-encryption</code>, should not be sent for GET requests if your object uses server-side encryption with Amazon Web Services KMS keys stored in Amazon Web Services Key Management Service (SSE-KMS) or server-side encryption with Amazon S3 managed keys (SSE-S3). If your object does use these types of keys, you'll get an HTTP <code>400 Bad Request</code> error.</p> </li>
/// <li> <p> The last modified property in this case is the creation date of the object.</p> </li>
/// </ul>
/// </note>
/// <p>Consider the following when using request headers:</p>
/// <ul>
/// <li> <p> If both of the <code>If-Match</code> and <code>If-Unmodified-Since</code> headers are present in the request as follows, then Amazon S3 returns the HTTP status code <code>200 OK</code> and the data requested:</p>
/// <ul>
/// <li> <p> <code>If-Match</code> condition evaluates to <code>true</code>.</p> </li>
/// <li> <p> <code>If-Unmodified-Since</code> condition evaluates to <code>false</code>.</p> </li>
/// </ul> </li>
/// <li> <p>If both of the <code>If-None-Match</code> and <code>If-Modified-Since</code> headers are present in the request as follows, then Amazon S3 returns the HTTP status code <code>304 Not Modified</code>:</p>
/// <ul>
/// <li> <p> <code>If-None-Match</code> condition evaluates to <code>false</code>.</p> </li>
/// <li> <p> <code>If-Modified-Since</code> condition evaluates to <code>true</code>.</p> </li>
/// </ul> </li>
/// </ul>
/// <p>For more information about conditional requests, see <a href="https://tools.ietf.org/html/rfc7232">RFC 7232</a>.</p>
/// <dl>
/// <dt>
/// Permissions
/// </dt>
/// <dd>
/// <p>The permissions that you need to use this operation depend on whether the bucket is versioned. If the bucket is versioned, you need both the <code>s3:GetObjectVersion</code> and <code>s3:GetObjectVersionAttributes</code> permissions for this operation. If the bucket is not versioned, you need the <code>s3:GetObject</code> and <code>s3:GetObjectAttributes</code> permissions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/using-with-s3-actions.html">Specifying Permissions in a Policy</a> in the <i>Amazon S3 User Guide</i>. If the object that you request does not exist, the error Amazon S3 returns depends on whether you also have the <code>s3:ListBucket</code> permission.</p>
/// <ul>
/// <li> <p>If you have the <code>s3:ListBucket</code> permission on the bucket, Amazon S3 returns an HTTP status code <code>404 Not Found</code> ("no such key") error.</p> </li>
/// <li> <p>If you don't have the <code>s3:ListBucket</code> permission, Amazon S3 returns an HTTP status code <code>403 Forbidden</code> ("access denied") error.</p> </li>
/// </ul>
/// </dd>
/// </dl>
/// <p>The following actions are related to <code>GetObjectAttributes</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectAcl.html">GetObjectAcl</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectLegalHold.html">GetObjectLegalHold</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectLockConfiguration.html">GetObjectLockConfiguration</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectRetention.html">GetObjectRetention</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObjectTagging.html">GetObjectTagging</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_HeadObject.html">HeadObject</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetObjectAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_object_attributes::builders::GetObjectAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetObjectAttributesFluentBuilder {
    /// Creates a new `GetObjectAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetObjectAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::get_object_attributes::builders::GetObjectAttributesInputBuilder {
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
        crate::operation::get_object_attributes::GetObjectAttributesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_object_attributes::GetObjectAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_object_attributes::GetObjectAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_object_attributes::GetObjectAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_object_attributes::GetObjectAttributesOutput,
            crate::operation::get_object_attributes::GetObjectAttributesError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_object_attributes::GetObjectAttributesError>,
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
    /// <p>The name of the bucket that contains the object.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket that contains the object.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>The name of the bucket that contains the object.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    /// <p>The object key.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>The object key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>The object key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key()
    }
    /// <p>The version ID used to reference a specific version of the object.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>The version ID used to reference a specific version of the object.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
    /// <p>The version ID used to reference a specific version of the object.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id()
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
    /// <p>Specifies the algorithm to use when encrypting the object (for example, AES256).</p>
    pub fn sse_customer_algorithm(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_algorithm(input.into());
        self
    }
    /// <p>Specifies the algorithm to use when encrypting the object (for example, AES256).</p>
    pub fn set_sse_customer_algorithm(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_algorithm(input);
        self
    }
    /// <p>Specifies the algorithm to use when encrypting the object (for example, AES256).</p>
    pub fn get_sse_customer_algorithm(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_algorithm()
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn sse_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key(input.into());
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn set_sse_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key(input);
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    pub fn get_sse_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_key()
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn sse_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key_md5(input.into());
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn set_sse_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key_md5(input);
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn get_sse_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sse_customer_key_md5()
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
    /// Appends an item to `ObjectAttributes`.
    ///
    /// To override the contents of this collection use [`set_object_attributes`](Self::set_object_attributes).
    ///
    /// <p>Specifies the fields at the root level that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn object_attributes(mut self, input: crate::types::ObjectAttributes) -> Self {
        self.inner = self.inner.object_attributes(input);
        self
    }
    /// <p>Specifies the fields at the root level that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn set_object_attributes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ObjectAttributes>>) -> Self {
        self.inner = self.inner.set_object_attributes(input);
        self
    }
    /// <p>Specifies the fields at the root level that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn get_object_attributes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ObjectAttributes>> {
        self.inner.get_object_attributes()
    }
}
