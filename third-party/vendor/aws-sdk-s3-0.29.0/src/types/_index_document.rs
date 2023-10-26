// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the <code>Suffix</code> element.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IndexDocument {
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (for example,if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub suffix: ::std::option::Option<::std::string::String>,
}
impl IndexDocument {
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (for example,if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn suffix(&self) -> ::std::option::Option<&str> {
        self.suffix.as_deref()
    }
}
impl IndexDocument {
    /// Creates a new builder-style object to manufacture [`IndexDocument`](crate::types::IndexDocument).
    pub fn builder() -> crate::types::builders::IndexDocumentBuilder {
        crate::types::builders::IndexDocumentBuilder::default()
    }
}

/// A builder for [`IndexDocument`](crate::types::IndexDocument).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IndexDocumentBuilder {
    pub(crate) suffix: ::std::option::Option<::std::string::String>,
}
impl IndexDocumentBuilder {
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (for example,if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn suffix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.suffix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (for example,if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn set_suffix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.suffix = input;
        self
    }
    /// <p>A suffix that is appended to a request that is for a directory on the website endpoint (for example,if the suffix is index.html and you make a request to samplebucket/images/ the data that is returned will be for the object with the key name images/index.html) The suffix must not be empty and must not include a slash character.</p> <important>
    /// <p>Replacement must be made for object keys containing special characters (such as carriage returns) when using XML requests. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-keys.html#object-key-xml-related-constraints"> XML related object key constraints</a>.</p>
    /// </important>
    pub fn get_suffix(&self) -> &::std::option::Option<::std::string::String> {
        &self.suffix
    }
    /// Consumes the builder and constructs a [`IndexDocument`](crate::types::IndexDocument).
    pub fn build(self) -> crate::types::IndexDocument {
        crate::types::IndexDocument { suffix: self.suffix }
    }
}
