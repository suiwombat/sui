// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListObjectsInput {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>A delimiter is a character that you use to group keys.</p>
    pub delimiter: ::std::option::Option<::std::string::String>,
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub encoding_type: ::std::option::Option<crate::types::EncodingType>,
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub max_keys: ::std::option::Option<i32>,
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub request_payer: ::std::option::Option<crate::types::RequestPayer>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub expected_bucket_owner: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub optional_object_attributes: ::std::option::Option<::std::vec::Vec<crate::types::OptionalObjectAttributes>>,
}
impl ListObjectsInput {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>A delimiter is a character that you use to group keys.</p>
    pub fn delimiter(&self) -> ::std::option::Option<&str> {
        self.delimiter.as_deref()
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn encoding_type(&self) -> ::std::option::Option<&crate::types::EncodingType> {
        self.encoding_type.as_ref()
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn max_keys(&self) -> ::std::option::Option<i32> {
        self.max_keys
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn request_payer(&self) -> ::std::option::Option<&crate::types::RequestPayer> {
        self.request_payer.as_ref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> ::std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn optional_object_attributes(&self) -> ::std::option::Option<&[crate::types::OptionalObjectAttributes]> {
        self.optional_object_attributes.as_deref()
    }
}
impl ListObjectsInput {
    /// Creates a new builder-style object to manufacture [`ListObjectsInput`](crate::operation::list_objects::ListObjectsInput).
    pub fn builder() -> crate::operation::list_objects::builders::ListObjectsInputBuilder {
        crate::operation::list_objects::builders::ListObjectsInputBuilder::default()
    }
}

/// A builder for [`ListObjectsInput`](crate::operation::list_objects::ListObjectsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListObjectsInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) delimiter: ::std::option::Option<::std::string::String>,
    pub(crate) encoding_type: ::std::option::Option<crate::types::EncodingType>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_keys: ::std::option::Option<i32>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) request_payer: ::std::option::Option<crate::types::RequestPayer>,
    pub(crate) expected_bucket_owner: ::std::option::Option<::std::string::String>,
    pub(crate) optional_object_attributes: ::std::option::Option<::std::vec::Vec<crate::types::OptionalObjectAttributes>>,
}
impl ListObjectsInputBuilder {
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket containing the objects.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When you use this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When you use this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts access point ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">What is S3 on Outposts?</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>A delimiter is a character that you use to group keys.</p>
    pub fn delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.delimiter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A delimiter is a character that you use to group keys.</p>
    pub fn set_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.delimiter = input;
        self
    }
    /// <p>A delimiter is a character that you use to group keys.</p>
    pub fn get_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        &self.delimiter
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn encoding_type(mut self, input: crate::types::EncodingType) -> Self {
        self.encoding_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn set_encoding_type(mut self, input: ::std::option::Option<crate::types::EncodingType>) -> Self {
        self.encoding_type = input;
        self
    }
    /// <p>Requests Amazon S3 to encode the object keys in the response and specifies the encoding method to use. An object key can contain any Unicode character; however, the XML 1.0 parser cannot parse some characters, such as characters with an ASCII value from 0 to 10. For characters that are not supported in XML 1.0, you can add this parameter to request that Amazon S3 encode the keys in the response.</p>
    pub fn get_encoding_type(&self) -> &::std::option::Option<crate::types::EncodingType> {
        &self.encoding_type
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>Marker is where you want Amazon S3 to start listing from. Amazon S3 starts listing after this specified key. Marker can be any key in the bucket.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.marker
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn max_keys(mut self, input: i32) -> Self {
        self.max_keys = ::std::option::Option::Some(input);
        self
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn set_max_keys(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_keys = input;
        self
    }
    /// <p>Sets the maximum number of keys returned in the response. By default, the action returns up to 1,000 key names. The response might contain fewer keys but will never contain more. </p>
    pub fn get_max_keys(&self) -> &::std::option::Option<i32> {
        &self.max_keys
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>Limits the response to keys that begin with the specified prefix.</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.request_payer = ::std::option::Option::Some(input);
        self
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn set_request_payer(mut self, input: ::std::option::Option<crate::types::RequestPayer>) -> Self {
        self.request_payer = input;
        self
    }
    /// <p>Confirms that the requester knows that she or he will be charged for the list objects request. Bucket owners need not specify this parameter in their requests.</p>
    pub fn get_request_payer(&self) -> &::std::option::Option<crate::types::RequestPayer> {
        &self.request_payer
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
    /// Appends an item to `optional_object_attributes`.
    ///
    /// To override the contents of this collection use [`set_optional_object_attributes`](Self::set_optional_object_attributes).
    ///
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn optional_object_attributes(mut self, input: crate::types::OptionalObjectAttributes) -> Self {
        let mut v = self.optional_object_attributes.unwrap_or_default();
        v.push(input);
        self.optional_object_attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn set_optional_object_attributes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::OptionalObjectAttributes>>) -> Self {
        self.optional_object_attributes = input;
        self
    }
    /// <p>Specifies the optional fields that you want returned in the response. Fields that you do not specify are not returned.</p>
    pub fn get_optional_object_attributes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::OptionalObjectAttributes>> {
        &self.optional_object_attributes
    }
    /// Consumes the builder and constructs a [`ListObjectsInput`](crate::operation::list_objects::ListObjectsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_objects::ListObjectsInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::list_objects::ListObjectsInput {
            bucket: self.bucket,
            delimiter: self.delimiter,
            encoding_type: self.encoding_type,
            marker: self.marker,
            max_keys: self.max_keys,
            prefix: self.prefix,
            request_payer: self.request_payer,
            expected_bucket_owner: self.expected_bucket_owner,
            optional_object_attributes: self.optional_object_attributes,
        })
    }
}
