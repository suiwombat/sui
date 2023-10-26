// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutObjectAcl`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`acl(ObjectCannedAcl)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::acl) / [`set_acl(Option<ObjectCannedAcl>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_acl): <p>The canned ACL to apply to the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#CannedACL">Canned ACL</a>.</p>
    ///   - [`access_control_policy(AccessControlPolicy)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::access_control_policy) / [`set_access_control_policy(Option<AccessControlPolicy>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_access_control_policy): <p>Contains the elements that set the ACL permissions for an object per grantee.</p>
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_bucket): <p>The bucket name that contains the object to which you want to attach the ACL. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`content_md5(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::content_md5) / [`set_content_md5(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_content_md5): <p>The base64-encoded 128-bit MD5 digest of the data. This header must be used as a message integrity check to verify that the request body was not corrupted in transit. For more information, go to <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864.&gt;</a> </p>  <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    ///   - [`grant_full_control(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::grant_full_control) / [`set_grant_full_control(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_grant_full_control): <p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`grant_read(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::grant_read) / [`set_grant_read(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_grant_read): <p>Allows grantee to list the objects in the bucket.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`grant_read_acp(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::grant_read_acp) / [`set_grant_read_acp(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_grant_read_acp): <p>Allows grantee to read the bucket ACL.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`grant_write(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::grant_write) / [`set_grant_write(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_grant_write): <p>Allows grantee to create new objects in the bucket.</p>  <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p>
    ///   - [`grant_write_acp(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::grant_write_acp) / [`set_grant_write_acp(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_grant_write_acp): <p>Allows grantee to write the ACL for the applicable bucket.</p>  <p>This action is not supported by Amazon S3 on Outposts.</p>
    ///   - [`key(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_key): <p>Key for which the PUT action was initiated.</p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`version_id(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_version_id): <p>VersionId used to reference a specific version of the object.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`PutObjectAclOutput`](crate::operation::put_object_acl::PutObjectAclOutput) with field(s):
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::put_object_acl::PutObjectAclOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
    /// - On failure, responds with [`SdkError<PutObjectAclError>`](crate::operation::put_object_acl::PutObjectAclError)
    pub fn put_object_acl(&self) -> crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder {
        crate::operation::put_object_acl::builders::PutObjectAclFluentBuilder::new(self.handle.clone())
    }
}
