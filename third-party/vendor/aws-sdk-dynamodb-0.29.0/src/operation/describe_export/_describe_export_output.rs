// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExportOutput {
    /// <p>Represents the properties of the export.</p>
    pub export_description: ::std::option::Option<crate::types::ExportDescription>,
    _request_id: Option<String>,
}
impl DescribeExportOutput {
    /// <p>Represents the properties of the export.</p>
    pub fn export_description(&self) -> ::std::option::Option<&crate::types::ExportDescription> {
        self.export_description.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeExportOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeExportOutput {
    /// Creates a new builder-style object to manufacture [`DescribeExportOutput`](crate::operation::describe_export::DescribeExportOutput).
    pub fn builder() -> crate::operation::describe_export::builders::DescribeExportOutputBuilder {
        crate::operation::describe_export::builders::DescribeExportOutputBuilder::default()
    }
}

/// A builder for [`DescribeExportOutput`](crate::operation::describe_export::DescribeExportOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeExportOutputBuilder {
    pub(crate) export_description: ::std::option::Option<crate::types::ExportDescription>,
    _request_id: Option<String>,
}
impl DescribeExportOutputBuilder {
    /// <p>Represents the properties of the export.</p>
    pub fn export_description(mut self, input: crate::types::ExportDescription) -> Self {
        self.export_description = ::std::option::Option::Some(input);
        self
    }
    /// <p>Represents the properties of the export.</p>
    pub fn set_export_description(mut self, input: ::std::option::Option<crate::types::ExportDescription>) -> Self {
        self.export_description = input;
        self
    }
    /// <p>Represents the properties of the export.</p>
    pub fn get_export_description(&self) -> &::std::option::Option<crate::types::ExportDescription> {
        &self.export_description
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeExportOutput`](crate::operation::describe_export::DescribeExportOutput).
    pub fn build(self) -> crate::operation::describe_export::DescribeExportOutput {
        crate::operation::describe_export::DescribeExportOutput {
            export_description: self.export_description,
            _request_id: self._request_id,
        }
    }
}
