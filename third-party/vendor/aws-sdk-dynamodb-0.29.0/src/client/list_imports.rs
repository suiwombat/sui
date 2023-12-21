// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListImports`](crate::operation::list_imports::builders::ListImportsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_imports::builders::ListImportsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_arn(impl ::std::convert::Into<String>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::table_arn) / [`set_table_arn(Option<String>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::set_table_arn): <p> The Amazon Resource Name (ARN) associated with the table that was imported to. </p>
    ///   - [`page_size(i32)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::page_size) / [`set_page_size(Option<i32>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::set_page_size): <p> The number of <code>ImportSummary </code>objects returned in a single page. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_imports::builders::ListImportsFluentBuilder::set_next_token): <p> An optional string that, if supplied, must be copied from the output of a previous call to <code>ListImports</code>. When provided in this manner, the API fetches the next page of results. </p>
    /// - On success, responds with [`ListImportsOutput`](crate::operation::list_imports::ListImportsOutput) with field(s):
    ///   - [`import_summary_list(Option<Vec<ImportSummary>>)`](crate::operation::list_imports::ListImportsOutput::import_summary_list): <p> A list of <code>ImportSummary</code> objects. </p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_imports::ListImportsOutput::next_token): <p> If this value is returned, there are additional results to be displayed. To retrieve them, call <code>ListImports</code> again, with <code>NextToken</code> set to this value. </p>
    /// - On failure, responds with [`SdkError<ListImportsError>`](crate::operation::list_imports::ListImportsError)
    pub fn list_imports(&self) -> crate::operation::list_imports::builders::ListImportsFluentBuilder {
        crate::operation::list_imports::builders::ListImportsFluentBuilder::new(self.handle.clone())
    }
}
