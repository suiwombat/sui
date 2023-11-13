// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListMultipartUploadsOutput {
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The key at or after which the listing began.</p>
    pub key_marker: ::std::option::Option<::std::string::String>,
    /// <p>Upload ID after which listing began.</p>
    pub upload_id_marker: ::std::option::Option<::std::string::String>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub next_key_marker: ::std::option::Option<::std::string::String>,
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub prefix: ::std::option::Option<::std::string::String>,
    /// <p>Contains the delimiter you specified in the request. If you don't specify a delimiter in your request, this element is absent from the response.</p>
    pub delimiter: ::std::option::Option<::std::string::String>,
    /// <p>When a list is truncated, this element specifies the value that should be used for the <code>upload-id-marker</code> request parameter in a subsequent request.</p>
    pub next_upload_id_marker: ::std::option::Option<::std::string::String>,
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub max_uploads: i32,
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub is_truncated: bool,
    /// <p>Container for elements related to a particular multipart upload. A response can contain zero or more <code>Upload</code> elements.</p>
    pub uploads: ::std::option::Option<::std::vec::Vec<crate::types::MultipartUpload>>,
    /// <p>If you specify a delimiter in the request, then the result returns each distinct key prefix containing the delimiter in a <code>CommonPrefixes</code> element. The distinct key prefixes are returned in the <code>Prefix</code> child element.</p>
    pub common_prefixes: ::std::option::Option<::std::vec::Vec<crate::types::CommonPrefix>>,
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    /// <p>If you specify the <code>encoding-type</code> request parameter, Amazon S3 includes this element in the response, and returns encoded key name values in the following response elements:</p>
    /// <p> <code>Delimiter</code>, <code>KeyMarker</code>, <code>Prefix</code>, <code>NextKeyMarker</code>, <code>Key</code>.</p>
    pub encoding_type: ::std::option::Option<crate::types::EncodingType>,
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl ListMultipartUploadsOutput {
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The key at or after which the listing began.</p>
    pub fn key_marker(&self) -> ::std::option::Option<&str> {
        self.key_marker.as_deref()
    }
    /// <p>Upload ID after which listing began.</p>
    pub fn upload_id_marker(&self) -> ::std::option::Option<&str> {
        self.upload_id_marker.as_deref()
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub fn next_key_marker(&self) -> ::std::option::Option<&str> {
        self.next_key_marker.as_deref()
    }
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
    /// <p>Contains the delimiter you specified in the request. If you don't specify a delimiter in your request, this element is absent from the response.</p>
    pub fn delimiter(&self) -> ::std::option::Option<&str> {
        self.delimiter.as_deref()
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the <code>upload-id-marker</code> request parameter in a subsequent request.</p>
    pub fn next_upload_id_marker(&self) -> ::std::option::Option<&str> {
        self.next_upload_id_marker.as_deref()
    }
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub fn max_uploads(&self) -> i32 {
        self.max_uploads
    }
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>Container for elements related to a particular multipart upload. A response can contain zero or more <code>Upload</code> elements.</p>
    pub fn uploads(&self) -> ::std::option::Option<&[crate::types::MultipartUpload]> {
        self.uploads.as_deref()
    }
    /// <p>If you specify a delimiter in the request, then the result returns each distinct key prefix containing the delimiter in a <code>CommonPrefixes</code> element. The distinct key prefixes are returned in the <code>Prefix</code> child element.</p>
    pub fn common_prefixes(&self) -> ::std::option::Option<&[crate::types::CommonPrefix]> {
        self.common_prefixes.as_deref()
    }
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    /// <p>If you specify the <code>encoding-type</code> request parameter, Amazon S3 includes this element in the response, and returns encoded key name values in the following response elements:</p>
    /// <p> <code>Delimiter</code>, <code>KeyMarker</code>, <code>Prefix</code>, <code>NextKeyMarker</code>, <code>Key</code>.</p>
    pub fn encoding_type(&self) -> ::std::option::Option<&crate::types::EncodingType> {
        self.encoding_type.as_ref()
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(&self) -> ::std::option::Option<&crate::types::RequestCharged> {
        self.request_charged.as_ref()
    }
}
impl crate::s3_request_id::RequestIdExt for ListMultipartUploadsOutput {
    fn extended_request_id(&self) -> Option<&str> {
        self._extended_request_id.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListMultipartUploadsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListMultipartUploadsOutput {
    /// Creates a new builder-style object to manufacture [`ListMultipartUploadsOutput`](crate::operation::list_multipart_uploads::ListMultipartUploadsOutput).
    pub fn builder() -> crate::operation::list_multipart_uploads::builders::ListMultipartUploadsOutputBuilder {
        crate::operation::list_multipart_uploads::builders::ListMultipartUploadsOutputBuilder::default()
    }
}

/// A builder for [`ListMultipartUploadsOutput`](crate::operation::list_multipart_uploads::ListMultipartUploadsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListMultipartUploadsOutputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key_marker: ::std::option::Option<::std::string::String>,
    pub(crate) upload_id_marker: ::std::option::Option<::std::string::String>,
    pub(crate) next_key_marker: ::std::option::Option<::std::string::String>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) delimiter: ::std::option::Option<::std::string::String>,
    pub(crate) next_upload_id_marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_uploads: ::std::option::Option<i32>,
    pub(crate) is_truncated: ::std::option::Option<bool>,
    pub(crate) uploads: ::std::option::Option<::std::vec::Vec<crate::types::MultipartUpload>>,
    pub(crate) common_prefixes: ::std::option::Option<::std::vec::Vec<crate::types::CommonPrefix>>,
    pub(crate) encoding_type: ::std::option::Option<crate::types::EncodingType>,
    pub(crate) request_charged: ::std::option::Option<crate::types::RequestCharged>,
    _extended_request_id: Option<String>,
    _request_id: Option<String>,
}
impl ListMultipartUploadsOutputBuilder {
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated. Does not return the access point ARN or access point alias if used.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The key at or after which the listing began.</p>
    pub fn key_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key at or after which the listing began.</p>
    pub fn set_key_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_marker = input;
        self
    }
    /// <p>The key at or after which the listing began.</p>
    pub fn get_key_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_marker
    }
    /// <p>Upload ID after which listing began.</p>
    pub fn upload_id_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Upload ID after which listing began.</p>
    pub fn set_upload_id_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id_marker = input;
        self
    }
    /// <p>Upload ID after which listing began.</p>
    pub fn get_upload_id_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id_marker
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub fn next_key_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_key_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub fn set_next_key_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_key_marker = input;
        self
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the key-marker request parameter in a subsequent request.</p>
    pub fn get_next_key_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_key_marker
    }
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>When a prefix is provided in the request, this field contains the specified prefix. The result contains only keys starting with the specified prefix.</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// <p>Contains the delimiter you specified in the request. If you don't specify a delimiter in your request, this element is absent from the response.</p>
    pub fn delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.delimiter = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Contains the delimiter you specified in the request. If you don't specify a delimiter in your request, this element is absent from the response.</p>
    pub fn set_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.delimiter = input;
        self
    }
    /// <p>Contains the delimiter you specified in the request. If you don't specify a delimiter in your request, this element is absent from the response.</p>
    pub fn get_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        &self.delimiter
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the <code>upload-id-marker</code> request parameter in a subsequent request.</p>
    pub fn next_upload_id_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_upload_id_marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the <code>upload-id-marker</code> request parameter in a subsequent request.</p>
    pub fn set_next_upload_id_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_upload_id_marker = input;
        self
    }
    /// <p>When a list is truncated, this element specifies the value that should be used for the <code>upload-id-marker</code> request parameter in a subsequent request.</p>
    pub fn get_next_upload_id_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_upload_id_marker
    }
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub fn max_uploads(mut self, input: i32) -> Self {
        self.max_uploads = ::std::option::Option::Some(input);
        self
    }
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub fn set_max_uploads(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_uploads = input;
        self
    }
    /// <p>Maximum number of multipart uploads that could have been included in the response.</p>
    pub fn get_max_uploads(&self) -> &::std::option::Option<i32> {
        &self.max_uploads
    }
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub fn set_is_truncated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>Indicates whether the returned list of multipart uploads is truncated. A value of true indicates that the list was truncated. The list can be truncated if the number of multipart uploads exceeds the limit allowed or specified by max uploads.</p>
    pub fn get_is_truncated(&self) -> &::std::option::Option<bool> {
        &self.is_truncated
    }
    /// Appends an item to `uploads`.
    ///
    /// To override the contents of this collection use [`set_uploads`](Self::set_uploads).
    ///
    /// <p>Container for elements related to a particular multipart upload. A response can contain zero or more <code>Upload</code> elements.</p>
    pub fn uploads(mut self, input: crate::types::MultipartUpload) -> Self {
        let mut v = self.uploads.unwrap_or_default();
        v.push(input);
        self.uploads = ::std::option::Option::Some(v);
        self
    }
    /// <p>Container for elements related to a particular multipart upload. A response can contain zero or more <code>Upload</code> elements.</p>
    pub fn set_uploads(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MultipartUpload>>) -> Self {
        self.uploads = input;
        self
    }
    /// <p>Container for elements related to a particular multipart upload. A response can contain zero or more <code>Upload</code> elements.</p>
    pub fn get_uploads(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MultipartUpload>> {
        &self.uploads
    }
    /// Appends an item to `common_prefixes`.
    ///
    /// To override the contents of this collection use [`set_common_prefixes`](Self::set_common_prefixes).
    ///
    /// <p>If you specify a delimiter in the request, then the result returns each distinct key prefix containing the delimiter in a <code>CommonPrefixes</code> element. The distinct key prefixes are returned in the <code>Prefix</code> child element.</p>
    pub fn common_prefixes(mut self, input: crate::types::CommonPrefix) -> Self {
        let mut v = self.common_prefixes.unwrap_or_default();
        v.push(input);
        self.common_prefixes = ::std::option::Option::Some(v);
        self
    }
    /// <p>If you specify a delimiter in the request, then the result returns each distinct key prefix containing the delimiter in a <code>CommonPrefixes</code> element. The distinct key prefixes are returned in the <code>Prefix</code> child element.</p>
    pub fn set_common_prefixes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CommonPrefix>>) -> Self {
        self.common_prefixes = input;
        self
    }
    /// <p>If you specify a delimiter in the request, then the result returns each distinct key prefix containing the delimiter in a <code>CommonPrefixes</code> element. The distinct key prefixes are returned in the <code>Prefix</code> child element.</p>
    pub fn get_common_prefixes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CommonPrefix>> {
        &self.common_prefixes
    }
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    /// <p>If you specify the <code>encoding-type</code> request parameter, Amazon S3 includes this element in the response, and returns encoded key name values in the following response elements:</p>
    /// <p> <code>Delimiter</code>, <code>KeyMarker</code>, <code>Prefix</code>, <code>NextKeyMarker</code>, <code>Key</code>.</p>
    pub fn encoding_type(mut self, input: crate::types::EncodingType) -> Self {
        self.encoding_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    /// <p>If you specify the <code>encoding-type</code> request parameter, Amazon S3 includes this element in the response, and returns encoded key name values in the following response elements:</p>
    /// <p> <code>Delimiter</code>, <code>KeyMarker</code>, <code>Prefix</code>, <code>NextKeyMarker</code>, <code>Key</code>.</p>
    pub fn set_encoding_type(mut self, input: ::std::option::Option<crate::types::EncodingType>) -> Self {
        self.encoding_type = input;
        self
    }
    /// <p>Encoding type used by Amazon S3 to encode object keys in the response.</p>
    /// <p>If you specify the <code>encoding-type</code> request parameter, Amazon S3 includes this element in the response, and returns encoded key name values in the following response elements:</p>
    /// <p> <code>Delimiter</code>, <code>KeyMarker</code>, <code>Prefix</code>, <code>NextKeyMarker</code>, <code>Key</code>.</p>
    pub fn get_encoding_type(&self) -> &::std::option::Option<crate::types::EncodingType> {
        &self.encoding_type
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn request_charged(mut self, input: crate::types::RequestCharged) -> Self {
        self.request_charged = ::std::option::Option::Some(input);
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn set_request_charged(mut self, input: ::std::option::Option<crate::types::RequestCharged>) -> Self {
        self.request_charged = input;
        self
    }
    /// <p>If present, indicates that the requester was successfully charged for the request.</p>
    pub fn get_request_charged(&self) -> &::std::option::Option<crate::types::RequestCharged> {
        &self.request_charged
    }
    pub(crate) fn _extended_request_id(mut self, extended_request_id: impl Into<String>) -> Self {
        self._extended_request_id = Some(extended_request_id.into());
        self
    }

    pub(crate) fn _set_extended_request_id(&mut self, extended_request_id: Option<String>) -> &mut Self {
        self._extended_request_id = extended_request_id;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListMultipartUploadsOutput`](crate::operation::list_multipart_uploads::ListMultipartUploadsOutput).
    pub fn build(self) -> crate::operation::list_multipart_uploads::ListMultipartUploadsOutput {
        crate::operation::list_multipart_uploads::ListMultipartUploadsOutput {
            bucket: self.bucket,
            key_marker: self.key_marker,
            upload_id_marker: self.upload_id_marker,
            next_key_marker: self.next_key_marker,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_upload_id_marker: self.next_upload_id_marker,
            max_uploads: self.max_uploads.unwrap_or_default(),
            is_truncated: self.is_truncated.unwrap_or_default(),
            uploads: self.uploads,
            common_prefixes: self.common_prefixes,
            encoding_type: self.encoding_type,
            request_charged: self.request_charged,
            _extended_request_id: self._extended_request_id,
            _request_id: self._request_id,
        }
    }
}
