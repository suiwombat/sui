// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEndpoints`](crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder::send) it.
    /// - On success, responds with [`DescribeEndpointsOutput`](crate::operation::describe_endpoints::DescribeEndpointsOutput) with field(s):
    ///   - [`endpoints(Option<Vec<Endpoint>>)`](crate::operation::describe_endpoints::DescribeEndpointsOutput::endpoints): <p>List of endpoints.</p>
    /// - On failure, responds with [`SdkError<DescribeEndpointsError>`](crate::operation::describe_endpoints::DescribeEndpointsError)
    pub fn describe_endpoints(&self) -> crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder {
        crate::operation::describe_endpoints::builders::DescribeEndpointsFluentBuilder::new(self.handle.clone())
    }
}
