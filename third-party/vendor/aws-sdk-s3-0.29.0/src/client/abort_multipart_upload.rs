// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AbortMultipartUpload`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::set_bucket): <p>The bucket name to which the upload was taking place. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(impl ::std::convert::Into<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::set_key): <p>Key of the object for which the multipart upload was initiated.</p>
    ///   - [`upload_id(impl ::std::convert::Into<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::set_upload_id): <p>Upload ID that identifies the multipart upload.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`AbortMultipartUploadOutput`](crate::operation::abort_multipart_upload::AbortMultipartUploadOutput) with field(s):
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::abort_multipart_upload::AbortMultipartUploadOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
    /// - On failure, responds with [`SdkError<AbortMultipartUploadError>`](crate::operation::abort_multipart_upload::AbortMultipartUploadError)
    pub fn abort_multipart_upload(&self) -> crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder {
        crate::operation::abort_multipart_upload::builders::AbortMultipartUploadFluentBuilder::new(self.handle.clone())
    }
}
