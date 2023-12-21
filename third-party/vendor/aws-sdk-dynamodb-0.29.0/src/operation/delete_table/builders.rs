// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_table::_delete_table_output::DeleteTableOutputBuilder;

pub use crate::operation::delete_table::_delete_table_input::DeleteTableInputBuilder;

impl DeleteTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_table::DeleteTableOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_table::DeleteTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteTable`.
///
/// <p>The <code>DeleteTable</code> operation deletes a table and all of its items. After a <code>DeleteTable</code> request, the specified table is in the <code>DELETING</code> state until DynamoDB completes the deletion. If the table is in the <code>ACTIVE</code> state, you can delete it. If a table is in <code>CREATING</code> or <code>UPDATING</code> states, then DynamoDB returns a <code>ResourceInUseException</code>. If the specified table does not exist, DynamoDB returns a <code>ResourceNotFoundException</code>. If table is already in the <code>DELETING</code> state, no error is returned. </p> <important>
/// <p>This operation only applies to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/globaltables.V2.html">Version 2019.11.21 (Current)</a> of global tables. </p>
/// </important> <note>
/// <p>DynamoDB might continue to accept data read and write operations, such as <code>GetItem</code> and <code>PutItem</code>, on a table in the <code>DELETING</code> state until the table deletion is complete.</p>
/// </note>
/// <p>When you delete a table, any indexes on that table are also deleted.</p>
/// <p>If you have DynamoDB Streams enabled on the table, then the corresponding stream on that table goes into the <code>DISABLED</code> state, and the stream is automatically deleted after 24 hours.</p>
/// <p>Use the <code>DescribeTable</code> action to check the status of the table. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_table::builders::DeleteTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteTableFluentBuilder {
    /// Creates a new `DeleteTable`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteTable as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_table::builders::DeleteTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_table::DeleteTableOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_table::DeleteTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_table::DeleteTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_table::DeleteTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_table::DeleteTableOutput,
            crate::operation::delete_table::DeleteTableError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_table::DeleteTableError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the table to delete.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_name(input.into());
        self
    }
    /// <p>The name of the table to delete.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_name(input);
        self
    }
    /// <p>The name of the table to delete.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_name()
    }
}
