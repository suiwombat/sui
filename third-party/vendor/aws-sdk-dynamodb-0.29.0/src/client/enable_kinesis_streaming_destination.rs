// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableKinesisStreamingDestination`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl ::std::convert::Into<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_table_name): <p>The name of the DynamoDB table.</p>
    ///   - [`stream_arn(impl ::std::convert::Into<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::set_stream_arn): <p>The ARN for a Kinesis data stream.</p>
    /// - On success, responds with [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput) with field(s):
    ///   - [`table_name(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::table_name): <p>The name of the table being modified.</p>
    ///   - [`stream_arn(Option<String>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::stream_arn): <p>The ARN for the specific Kinesis data stream.</p>
    ///   - [`destination_status(Option<DestinationStatus>)`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput::destination_status): <p>The current status of the replication.</p>
    /// - On failure, responds with [`SdkError<EnableKinesisStreamingDestinationError>`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationError)
    pub fn enable_kinesis_streaming_destination(
        &self,
    ) -> crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder {
        crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationFluentBuilder::new(self.handle.clone())
    }
}
