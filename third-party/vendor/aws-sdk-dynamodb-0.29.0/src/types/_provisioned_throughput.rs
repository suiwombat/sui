// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the provisioned throughput settings for a specified table or index. The settings can be modified using the <code>UpdateTable</code> operation.</p>
/// <p>For current minimum and maximum provisioned throughput values, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Limits.html">Service, Account, and Table Quotas</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvisionedThroughput {
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub read_capacity_units: ::std::option::Option<i64>,
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub write_capacity_units: ::std::option::Option<i64>,
}
impl ProvisionedThroughput {
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn read_capacity_units(&self) -> ::std::option::Option<i64> {
        self.read_capacity_units
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn write_capacity_units(&self) -> ::std::option::Option<i64> {
        self.write_capacity_units
    }
}
impl ProvisionedThroughput {
    /// Creates a new builder-style object to manufacture [`ProvisionedThroughput`](crate::types::ProvisionedThroughput).
    pub fn builder() -> crate::types::builders::ProvisionedThroughputBuilder {
        crate::types::builders::ProvisionedThroughputBuilder::default()
    }
}

/// A builder for [`ProvisionedThroughput`](crate::types::ProvisionedThroughput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ProvisionedThroughputBuilder {
    pub(crate) read_capacity_units: ::std::option::Option<i64>,
    pub(crate) write_capacity_units: ::std::option::Option<i64>,
}
impl ProvisionedThroughputBuilder {
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn read_capacity_units(mut self, input: i64) -> Self {
        self.read_capacity_units = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn set_read_capacity_units(mut self, input: ::std::option::Option<i64>) -> Self {
        self.read_capacity_units = input;
        self
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn get_read_capacity_units(&self) -> &::std::option::Option<i64> {
        &self.read_capacity_units
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn write_capacity_units(mut self, input: i64) -> Self {
        self.write_capacity_units = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn set_write_capacity_units(mut self, input: ::std::option::Option<i64>) -> Self {
        self.write_capacity_units = input;
        self
    }
    /// <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/ProvisionedThroughput.html">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    /// <p>If read/write capacity mode is <code>PAY_PER_REQUEST</code> the value is set to 0.</p>
    pub fn get_write_capacity_units(&self) -> &::std::option::Option<i64> {
        &self.write_capacity_units
    }
    /// Consumes the builder and constructs a [`ProvisionedThroughput`](crate::types::ProvisionedThroughput).
    pub fn build(self) -> crate::types::ProvisionedThroughput {
        crate::types::ProvisionedThroughput {
            read_capacity_units: self.read_capacity_units,
            write_capacity_units: self.write_capacity_units,
        }
    }
}
