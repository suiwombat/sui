// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportTableOutput {
    /// <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
    pub import_table_description: ::std::option::Option<crate::types::ImportTableDescription>,
    _request_id: Option<String>,
}
impl ImportTableOutput {
    /// <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
    pub fn import_table_description(&self) -> ::std::option::Option<&crate::types::ImportTableDescription> {
        self.import_table_description.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for ImportTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ImportTableOutput {
    /// Creates a new builder-style object to manufacture [`ImportTableOutput`](crate::operation::import_table::ImportTableOutput).
    pub fn builder() -> crate::operation::import_table::builders::ImportTableOutputBuilder {
        crate::operation::import_table::builders::ImportTableOutputBuilder::default()
    }
}

/// A builder for [`ImportTableOutput`](crate::operation::import_table::ImportTableOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ImportTableOutputBuilder {
    pub(crate) import_table_description: ::std::option::Option<crate::types::ImportTableDescription>,
    _request_id: Option<String>,
}
impl ImportTableOutputBuilder {
    /// <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
    pub fn import_table_description(mut self, input: crate::types::ImportTableDescription) -> Self {
        self.import_table_description = ::std::option::Option::Some(input);
        self
    }
    /// <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
    pub fn set_import_table_description(mut self, input: ::std::option::Option<crate::types::ImportTableDescription>) -> Self {
        self.import_table_description = input;
        self
    }
    /// <p> Represents the properties of the table created for the import, and parameters of the import. The import parameters include import status, how many items were processed, and how many errors were encountered. </p>
    pub fn get_import_table_description(&self) -> &::std::option::Option<crate::types::ImportTableDescription> {
        &self.import_table_description
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ImportTableOutput`](crate::operation::import_table::ImportTableOutput).
    pub fn build(self) -> crate::operation::import_table::ImportTableOutput {
        crate::operation::import_table::ImportTableOutput {
            import_table_description: self.import_table_description,
            _request_id: self._request_id,
        }
    }
}
