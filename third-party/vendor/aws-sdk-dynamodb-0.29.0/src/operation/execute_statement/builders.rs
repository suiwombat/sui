// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::execute_statement::_execute_statement_output::ExecuteStatementOutputBuilder;

pub use crate::operation::execute_statement::_execute_statement_input::ExecuteStatementInputBuilder;

impl ExecuteStatementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::execute_statement::ExecuteStatementOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_statement::ExecuteStatementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.execute_statement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExecuteStatement`.
///
/// <p>This operation allows you to perform reads and singleton writes on data stored in DynamoDB, using PartiQL.</p>
/// <p>For PartiQL reads (<code>SELECT</code> statement), if the total number of processed items exceeds the maximum dataset size limit of 1 MB, the read stops and results are returned to the user as a <code>LastEvaluatedKey</code> value to continue the read in a subsequent operation. If the filter criteria in <code>WHERE</code> clause does not match any data, the read will return an empty result set.</p>
/// <p>A single <code>SELECT</code> statement response can return up to the maximum number of items (if using the Limit parameter) or a maximum of 1 MB of data (and then apply any filtering to the results using <code>WHERE</code> clause). If <code>LastEvaluatedKey</code> is present in the response, you need to paginate the result set. If <code>NextToken</code> is present, you need to paginate the result set and include <code>NextToken</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExecuteStatementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::execute_statement::builders::ExecuteStatementInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ExecuteStatementFluentBuilder {
    /// Creates a new `ExecuteStatement`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ExecuteStatement as a reference.
    pub fn as_input(&self) -> &crate::operation::execute_statement::builders::ExecuteStatementInputBuilder {
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
        crate::operation::execute_statement::ExecuteStatementOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::execute_statement::ExecuteStatementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::execute_statement::ExecuteStatement::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::execute_statement::ExecuteStatement::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::execute_statement::ExecuteStatementOutput,
            crate::operation::execute_statement::ExecuteStatementError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::execute_statement::ExecuteStatementError>,
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
    /// <p>The PartiQL statement representing the operation to run.</p>
    pub fn statement(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement(input.into());
        self
    }
    /// <p>The PartiQL statement representing the operation to run.</p>
    pub fn set_statement(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement(input);
        self
    }
    /// <p>The PartiQL statement representing the operation to run.</p>
    pub fn get_statement(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement()
    }
    /// Appends an item to `Parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The parameters for the PartiQL statement, if any.</p>
    pub fn parameters(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.parameters(input);
        self
    }
    /// <p>The parameters for the PartiQL statement, if any.</p>
    pub fn set_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeValue>>) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// <p>The parameters for the PartiQL statement, if any.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AttributeValue>> {
        self.inner.get_parameters()
    }
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    pub fn consistent_read(mut self, input: bool) -> Self {
        self.inner = self.inner.consistent_read(input);
        self
    }
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    pub fn set_consistent_read(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_consistent_read(input);
        self
    }
    /// <p>The consistency of a read operation. If set to <code>true</code>, then a strongly consistent read is used; otherwise, an eventually consistent read is used.</p>
    pub fn get_consistent_read(&self) -> &::std::option::Option<bool> {
        self.inner.get_consistent_read()
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Set this value to get remaining results, if <code>NextToken</code> was returned in the statement response.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn return_consumed_capacity(mut self, input: crate::types::ReturnConsumedCapacity) -> Self {
        self.inner = self.inner.return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<crate::types::ReturnConsumedCapacity>) -> Self {
        self.inner = self.inner.set_return_consumed_capacity(input);
        self
    }
    /// <p>Determines the level of detail about either provisioned or on-demand throughput consumption that is returned in the response:</p>
    /// <ul>
    /// <li> <p> <code>INDEXES</code> - The response includes the aggregate <code>ConsumedCapacity</code> for the operation, together with <code>ConsumedCapacity</code> for each table and secondary index that was accessed.</p> <p>Note that some operations, such as <code>GetItem</code> and <code>BatchGetItem</code>, do not access any indexes at all. In these cases, specifying <code>INDEXES</code> will only return <code>ConsumedCapacity</code> information for table(s).</p> </li>
    /// <li> <p> <code>TOTAL</code> - The response includes only the aggregate <code>ConsumedCapacity</code> for the operation.</p> </li>
    /// <li> <p> <code>NONE</code> - No <code>ConsumedCapacity</code> details are included in the response.</p> </li>
    /// </ul>
    pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<crate::types::ReturnConsumedCapacity> {
        self.inner.get_return_consumed_capacity()
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of items to evaluate (not necessarily the number of matching items). If DynamoDB processes the number of items up to the limit while processing the results, it stops the operation and returns the matching values up to that point, along with a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation so you can pick up where you left off. Also, if the processed dataset size exceeds 1 MB before DynamoDB reaches this limit, it stops the operation and returns the matching values up to the limit, and a key in <code>LastEvaluatedKey</code> to apply in a subsequent operation to continue the operation. </p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>An optional parameter that returns the item attributes for an <code>ExecuteStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn return_values_on_condition_check_failure(mut self, input: crate::types::ReturnValuesOnConditionCheckFailure) -> Self {
        self.inner = self.inner.return_values_on_condition_check_failure(input);
        self
    }
    /// <p>An optional parameter that returns the item attributes for an <code>ExecuteStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn set_return_values_on_condition_check_failure(
        mut self,
        input: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
    ) -> Self {
        self.inner = self.inner.set_return_values_on_condition_check_failure(input);
        self
    }
    /// <p>An optional parameter that returns the item attributes for an <code>ExecuteStatement</code> operation that failed a condition check.</p>
    /// <p>There is no additional cost associated with requesting a return value aside from the small network and processing overhead of receiving a larger response. No read capacity units are consumed.</p>
    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure> {
        self.inner.get_return_values_on_condition_check_failure()
    }
}
