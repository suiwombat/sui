// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeGlobalTableOutput {
    /// <p>Contains the details of the global table.</p>
    pub global_table_description: ::std::option::Option<crate::types::GlobalTableDescription>,
    _request_id: Option<String>,
}
impl DescribeGlobalTableOutput {
    /// <p>Contains the details of the global table.</p>
    pub fn global_table_description(&self) -> ::std::option::Option<&crate::types::GlobalTableDescription> {
        self.global_table_description.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeGlobalTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeGlobalTableOutput {
    /// Creates a new builder-style object to manufacture [`DescribeGlobalTableOutput`](crate::operation::describe_global_table::DescribeGlobalTableOutput).
    pub fn builder() -> crate::operation::describe_global_table::builders::DescribeGlobalTableOutputBuilder {
        crate::operation::describe_global_table::builders::DescribeGlobalTableOutputBuilder::default()
    }
}

/// A builder for [`DescribeGlobalTableOutput`](crate::operation::describe_global_table::DescribeGlobalTableOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeGlobalTableOutputBuilder {
    pub(crate) global_table_description: ::std::option::Option<crate::types::GlobalTableDescription>,
    _request_id: Option<String>,
}
impl DescribeGlobalTableOutputBuilder {
    /// <p>Contains the details of the global table.</p>
    pub fn global_table_description(mut self, input: crate::types::GlobalTableDescription) -> Self {
        self.global_table_description = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains the details of the global table.</p>
    pub fn set_global_table_description(mut self, input: ::std::option::Option<crate::types::GlobalTableDescription>) -> Self {
        self.global_table_description = input;
        self
    }
    /// <p>Contains the details of the global table.</p>
    pub fn get_global_table_description(&self) -> &::std::option::Option<crate::types::GlobalTableDescription> {
        &self.global_table_description
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeGlobalTableOutput`](crate::operation::describe_global_table::DescribeGlobalTableOutput).
    pub fn build(self) -> crate::operation::describe_global_table::DescribeGlobalTableOutput {
        crate::operation::describe_global_table::DescribeGlobalTableOutput {
            global_table_description: self.global_table_description,
            _request_id: self._request_id,
        }
    }
}
