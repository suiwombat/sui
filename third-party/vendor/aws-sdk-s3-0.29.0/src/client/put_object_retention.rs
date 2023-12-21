// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutObjectRetention`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_bucket): <p>The bucket name that contains the object you want to apply this Object Retention configuration to. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(impl ::std::convert::Into<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_key): <p>The key name for the object that you want to apply this Object Retention configuration to.</p>
    ///   - [`retention(ObjectLockRetention)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::retention) / [`set_retention(Option<ObjectLockRetention>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_retention): <p>The container element for the Object Retention configuration.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`version_id(impl ::std::convert::Into<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_version_id): <p>The version ID for the object that you want to apply this Object Retention configuration to.</p>
    ///   - [`bypass_governance_retention(bool)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::bypass_governance_retention) / [`set_bypass_governance_retention(Option<bool>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_bypass_governance_retention): <p>Indicates whether this action should bypass Governance-mode restrictions.</p>
    ///   - [`content_md5(impl ::std::convert::Into<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::content_md5) / [`set_content_md5(Option<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_content_md5): <p>The MD5 hash for the request body.</p>  <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`PutObjectRetentionOutput`](crate::operation::put_object_retention::PutObjectRetentionOutput) with field(s):
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::put_object_retention::PutObjectRetentionOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
    /// - On failure, responds with [`SdkError<PutObjectRetentionError>`](crate::operation::put_object_retention::PutObjectRetentionError)
    pub fn put_object_retention(&self) -> crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder {
        crate::operation::put_object_retention::builders::PutObjectRetentionFluentBuilder::new(self.handle.clone())
    }
}
