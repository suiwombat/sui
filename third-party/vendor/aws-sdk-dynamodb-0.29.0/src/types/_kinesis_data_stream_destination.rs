// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Kinesis data stream destination.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KinesisDataStreamDestination {
    /// <p>The ARN for a specific Kinesis data stream.</p>
    pub stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The current status of replication.</p>
    pub destination_status: ::std::option::Option<crate::types::DestinationStatus>,
    /// <p>The human-readable string that corresponds to the replica status.</p>
    pub destination_status_description: ::std::option::Option<::std::string::String>,
}
impl KinesisDataStreamDestination {
    /// <p>The ARN for a specific Kinesis data stream.</p>
    pub fn stream_arn(&self) -> ::std::option::Option<&str> {
        self.stream_arn.as_deref()
    }
    /// <p>The current status of replication.</p>
    pub fn destination_status(&self) -> ::std::option::Option<&crate::types::DestinationStatus> {
        self.destination_status.as_ref()
    }
    /// <p>The human-readable string that corresponds to the replica status.</p>
    pub fn destination_status_description(&self) -> ::std::option::Option<&str> {
        self.destination_status_description.as_deref()
    }
}
impl KinesisDataStreamDestination {
    /// Creates a new builder-style object to manufacture [`KinesisDataStreamDestination`](crate::types::KinesisDataStreamDestination).
    pub fn builder() -> crate::types::builders::KinesisDataStreamDestinationBuilder {
        crate::types::builders::KinesisDataStreamDestinationBuilder::default()
    }
}

/// A builder for [`KinesisDataStreamDestination`](crate::types::KinesisDataStreamDestination).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct KinesisDataStreamDestinationBuilder {
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) destination_status: ::std::option::Option<crate::types::DestinationStatus>,
    pub(crate) destination_status_description: ::std::option::Option<::std::string::String>,
}
impl KinesisDataStreamDestinationBuilder {
    /// <p>The ARN for a specific Kinesis data stream.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN for a specific Kinesis data stream.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The ARN for a specific Kinesis data stream.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// <p>The current status of replication.</p>
    pub fn destination_status(mut self, input: crate::types::DestinationStatus) -> Self {
        self.destination_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of replication.</p>
    pub fn set_destination_status(mut self, input: ::std::option::Option<crate::types::DestinationStatus>) -> Self {
        self.destination_status = input;
        self
    }
    /// <p>The current status of replication.</p>
    pub fn get_destination_status(&self) -> &::std::option::Option<crate::types::DestinationStatus> {
        &self.destination_status
    }
    /// <p>The human-readable string that corresponds to the replica status.</p>
    pub fn destination_status_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_status_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The human-readable string that corresponds to the replica status.</p>
    pub fn set_destination_status_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_status_description = input;
        self
    }
    /// <p>The human-readable string that corresponds to the replica status.</p>
    pub fn get_destination_status_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_status_description
    }
    /// Consumes the builder and constructs a [`KinesisDataStreamDestination`](crate::types::KinesisDataStreamDestination).
    pub fn build(self) -> crate::types::KinesisDataStreamDestination {
        crate::types::KinesisDataStreamDestination {
            stream_arn: self.stream_arn,
            destination_status: self.destination_status,
            destination_status_description: self.destination_status_description,
        }
    }
}
