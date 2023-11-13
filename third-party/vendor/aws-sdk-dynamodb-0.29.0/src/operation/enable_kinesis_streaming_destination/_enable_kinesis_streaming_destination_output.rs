// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableKinesisStreamingDestinationOutput {
    /// <p>The name of the table being modified.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>The ARN for the specific Kinesis data stream.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The current status of the replication.</p>
    pub destination_status: ::std::option::Option<crate::types::DestinationStatus>,
    _request_id: Option<String>,
}
impl EnableKinesisStreamingDestinationOutput {
    /// <p>The name of the table being modified.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>The ARN for the specific Kinesis data stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
    /// <p>The current status of the replication.</p>
    pub fn destination_status(&self) -> ::std::option::Option<&crate::types::DestinationStatus> {
        self.destination_status.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for EnableKinesisStreamingDestinationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl EnableKinesisStreamingDestinationOutput {
    /// Creates a new builder-style object to manufacture [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput).
    pub fn builder() -> crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder {
        crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationOutputBuilder::default()
    }
}

/// A builder for [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnableKinesisStreamingDestinationOutputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) destination_status: ::std::option::Option<crate::types::DestinationStatus>,
    _request_id: Option<String>,
}
impl EnableKinesisStreamingDestinationOutputBuilder {
    /// <p>The name of the table being modified.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table being modified.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The name of the table being modified.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>The ARN for the specific Kinesis data stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN for the specific Kinesis data stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The ARN for the specific Kinesis data stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// <p>The current status of the replication.</p>
    pub fn destination_status(mut self, input: crate::types::DestinationStatus) -> Self {
        self.destination_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the replication.</p>
    pub fn set_destination_status(mut self, input: ::std::option::Option<crate::types::DestinationStatus>) -> Self {
        self.destination_status = input;
        self
    }
    /// <p>The current status of the replication.</p>
    pub fn get_destination_status(&self) -> &::std::option::Option<crate::types::DestinationStatus> {
        &self.destination_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`EnableKinesisStreamingDestinationOutput`](crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput).
    pub fn build(self) -> crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput {
        crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput {
            table_name: self.table_name,
            stream_arn: self.stream_arn,
            destination_status: self.destination_status,
            _request_id: self._request_id,
        }
    }
}
