// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetObjectRetention`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_bucket): <p>The bucket name containing the object whose retention settings you want to retrieve. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(impl ::std::convert::Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_key): <p>The key name for the object whose retention settings you want to retrieve.</p>
    ///   - [`version_id(impl ::std::convert::Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_version_id): <p>The version ID for the object whose retention settings you want to retrieve.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetObjectRetentionOutput`](crate::operation::get_object_retention::GetObjectRetentionOutput) with field(s):
    ///   - [`retention(Option<ObjectLockRetention>)`](crate::operation::get_object_retention::GetObjectRetentionOutput::retention): <p>The container element for an object's retention settings.</p>
    /// - On failure, responds with [`SdkError<GetObjectRetentionError>`](crate::operation::get_object_retention::GetObjectRetentionError)
    pub fn get_object_retention(&self) -> crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder {
        crate::operation::get_object_retention::builders::GetObjectRetentionFluentBuilder::new(self.handle.clone())
    }
}
