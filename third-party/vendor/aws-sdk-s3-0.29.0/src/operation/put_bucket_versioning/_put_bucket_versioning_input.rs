// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutBucketVersioningInput {
    /// <p>The bucket name.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub content_md5: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub checksum_algorithm: ::std::option::Option<crate::types::ChecksumAlgorithm>,
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub mfa: ::std::option::Option<::std::string::String>,
    /// <p>Container for setting the versioning state.</p>
    pub versioning_configuration: ::std::option::Option<crate::types::VersioningConfiguration>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl PutBucketVersioningInput {
    /// <p>The bucket name.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(&self) -> ::std::option::Option<&str> {
        self.content_md5.as_deref()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(&self) -> ::std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn mfa(&self) -> ::std::option::Option<&str> {
        self.mfa.as_deref()
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn versioning_configuration(&self) -> ::std::option::Option<&crate::types::VersioningConfiguration> {
        self.versioning_configuration.as_ref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl PutBucketVersioningInput {
    /// Creates a new builder-style object to manufacture [`PutBucketVersioningInput`](crate::operation::put_bucket_versioning::PutBucketVersioningInput).
    pub fn builder() -> crate::operation::put_bucket_versioning::builders::PutBucketVersioningInputBuilder {
        crate::operation::put_bucket_versioning::builders::PutBucketVersioningInputBuilder::default()
    }
}

/// A builder for [`PutBucketVersioningInput`](crate::operation::put_bucket_versioning::PutBucketVersioningInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutBucketVersioningInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) content_md5: ::std::option::Option<::std::string::String>,
    pub(crate) checksum_algorithm: ::std::option::Option<crate::types::ChecksumAlgorithm>,
    pub(crate) mfa: ::std::option::Option<::std::string::String>,
    pub(crate) versioning_configuration: ::std::option::Option<crate::types::VersioningConfiguration>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
}
impl PutBucketVersioningInputBuilder {
    /// <p>The bucket name.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The bucket name.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The bucket name.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_md5 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_md5 = input;
        self
    }
    /// <p>&gt;The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn get_content_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_md5
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.checksum_algorithm = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn set_checksum_algorithm(mut self, input: ::std::option::Option<crate::types::ChecksumAlgorithm>) -> Self {
        self.checksum_algorithm = input;
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn get_checksum_algorithm(&self) -> &::std::option::Option<crate::types::ChecksumAlgorithm> {
        &self.checksum_algorithm
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn mfa(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mfa = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn set_mfa(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mfa = input;
        self
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device.</p>
    pub fn get_mfa(&self) -> &::std::option::Option<::std::string::String> {
        &self.mfa
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn versioning_configuration(mut self, input: crate::types::VersioningConfiguration) -> Self {
        self.versioning_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn set_versioning_configuration(mut self, input: ::std::option::Option<crate::types::VersioningConfiguration>) -> Self {
        self.versioning_configuration = input;
        self
    }
    /// <p>Container for setting the versioning state.</p>
    pub fn get_versioning_configuration(&self) -> &::std::option::Option<crate::types::VersioningConfiguration> {
        &self.versioning_configuration
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
    /// Consumes the builder and constructs a [`PutBucketVersioningInput`](crate::operation::put_bucket_versioning::PutBucketVersioningInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::put_bucket_versioning::PutBucketVersioningInput, ::aws_smithy_http::operation::error::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::put_bucket_versioning::PutBucketVersioningInput {
            bucket: self.bucket,
            content_md5: self.content_md5,
            checksum_algorithm: self.checksum_algorithm,
            mfa: self.mfa,
            versioning_configuration: self.versioning_configuration,
            expected_bucket_owner: self.expected_bucket_owner,
        })
    }
}
