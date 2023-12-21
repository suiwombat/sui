// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the settings for a global table in a Region that will be modified.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicaSettingsUpdate {
    /// <p>The Region of the replica to be added.</p>
    pub region_name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub replica_provisioned_read_capacity_units: ::std::option::Option<i64>,
    /// <p>Auto scaling settings for managing a global table replica's read capacity units.</p>
    pub replica_provisioned_read_capacity_auto_scaling_settings_update: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub replica_global_secondary_index_settings_update:
        ::std::option::Option<::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate>>,
    /// <p>Replica-specific table class. If not specified, uses the source table's table class.</p>
    pub replica_table_class: ::std::option::Option<crate::types::TableClass>,
}
impl ReplicaSettingsUpdate {
    /// <p>The Region of the replica to be added.</p>
    pub fn region_name(&self) -> ::std::option::Option<&str> {
        self.region_name.as_deref()
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub fn replica_provisioned_read_capacity_units(&self) -> ::std::option::Option<i64> {
        self.replica_provisioned_read_capacity_units
    }
    /// <p>Auto scaling settings for managing a global table replica's read capacity units.</p>
    pub fn replica_provisioned_read_capacity_auto_scaling_settings_update(&self) -> ::std::option::Option<&crate::types::AutoScalingSettingsUpdate> {
        self.replica_provisioned_read_capacity_auto_scaling_settings_update.as_ref()
    }
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub fn replica_global_secondary_index_settings_update(
        &self,
    ) -> ::std::option::Option<&[crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate]> {
        self.replica_global_secondary_index_settings_update.as_deref()
    }
    /// <p>Replica-specific table class. If not specified, uses the source table's table class.</p>
    pub fn replica_table_class(&self) -> ::std::option::Option<&crate::types::TableClass> {
        self.replica_table_class.as_ref()
    }
}
impl ReplicaSettingsUpdate {
    /// Creates a new builder-style object to manufacture [`ReplicaSettingsUpdate`](crate::types::ReplicaSettingsUpdate).
    pub fn builder() -> crate::types::builders::ReplicaSettingsUpdateBuilder {
        crate::types::builders::ReplicaSettingsUpdateBuilder::default()
    }
}

/// A builder for [`ReplicaSettingsUpdate`](crate::types::ReplicaSettingsUpdate).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReplicaSettingsUpdateBuilder {
    pub(crate) region_name: ::std::option::Option<::std::string::String>,
    pub(crate) replica_provisioned_read_capacity_units: ::std::option::Option<i64>,
    pub(crate) replica_provisioned_read_capacity_auto_scaling_settings_update: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    pub(crate) replica_global_secondary_index_settings_update:
        ::std::option::Option<::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate>>,
    pub(crate) replica_table_class: ::std::option::Option<crate::types::TableClass>,
}
impl ReplicaSettingsUpdateBuilder {
    /// <p>The Region of the replica to be added.</p>
    pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region of the replica to be added.</p>
    pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region_name = input;
        self
    }
    /// <p>The Region of the replica to be added.</p>
    pub fn get_region_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.region_name
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub fn replica_provisioned_read_capacity_units(mut self, input: i64) -> Self {
        self.replica_provisioned_read_capacity_units = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub fn set_replica_provisioned_read_capacity_units(mut self, input: ::std::option::Option<i64>) -> Self {
        self.replica_provisioned_read_capacity_units = input;
        self
    }
    /// <p>The maximum number of strongly consistent reads consumed per second before DynamoDB returns a <code>ThrottlingException</code>. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/WorkingWithTables.html#ProvisionedThroughput">Specifying Read and Write Requirements</a> in the <i>Amazon DynamoDB Developer Guide</i>. </p>
    pub fn get_replica_provisioned_read_capacity_units(&self) -> &::std::option::Option<i64> {
        &self.replica_provisioned_read_capacity_units
    }
    /// <p>Auto scaling settings for managing a global table replica's read capacity units.</p>
    pub fn replica_provisioned_read_capacity_auto_scaling_settings_update(mut self, input: crate::types::AutoScalingSettingsUpdate) -> Self {
        self.replica_provisioned_read_capacity_auto_scaling_settings_update = ::std::option::Option::Some(input);
        self
    }
    /// <p>Auto scaling settings for managing a global table replica's read capacity units.</p>
    pub fn set_replica_provisioned_read_capacity_auto_scaling_settings_update(
        mut self,
        input: ::std::option::Option<crate::types::AutoScalingSettingsUpdate>,
    ) -> Self {
        self.replica_provisioned_read_capacity_auto_scaling_settings_update = input;
        self
    }
    /// <p>Auto scaling settings for managing a global table replica's read capacity units.</p>
    pub fn get_replica_provisioned_read_capacity_auto_scaling_settings_update(
        &self,
    ) -> &::std::option::Option<crate::types::AutoScalingSettingsUpdate> {
        &self.replica_provisioned_read_capacity_auto_scaling_settings_update
    }
    /// Appends an item to `replica_global_secondary_index_settings_update`.
    ///
    /// To override the contents of this collection use [`set_replica_global_secondary_index_settings_update`](Self::set_replica_global_secondary_index_settings_update).
    ///
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub fn replica_global_secondary_index_settings_update(mut self, input: crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate) -> Self {
        let mut v = self.replica_global_secondary_index_settings_update.unwrap_or_default();
        v.push(input);
        self.replica_global_secondary_index_settings_update = ::std::option::Option::Some(v);
        self
    }
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub fn set_replica_global_secondary_index_settings_update(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate>>,
    ) -> Self {
        self.replica_global_secondary_index_settings_update = input;
        self
    }
    /// <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    pub fn get_replica_global_secondary_index_settings_update(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::ReplicaGlobalSecondaryIndexSettingsUpdate>> {
        &self.replica_global_secondary_index_settings_update
    }
    /// <p>Replica-specific table class. If not specified, uses the source table's table class.</p>
    pub fn replica_table_class(mut self, input: crate::types::TableClass) -> Self {
        self.replica_table_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>Replica-specific table class. If not specified, uses the source table's table class.</p>
    pub fn set_replica_table_class(mut self, input: ::std::option::Option<crate::types::TableClass>) -> Self {
        self.replica_table_class = input;
        self
    }
    /// <p>Replica-specific table class. If not specified, uses the source table's table class.</p>
    pub fn get_replica_table_class(&self) -> &::std::option::Option<crate::types::TableClass> {
        &self.replica_table_class
    }
    /// Consumes the builder and constructs a [`ReplicaSettingsUpdate`](crate::types::ReplicaSettingsUpdate).
    pub fn build(self) -> crate::types::ReplicaSettingsUpdate {
        crate::types::ReplicaSettingsUpdate {
            region_name: self.region_name,
            replica_provisioned_read_capacity_units: self.replica_provisioned_read_capacity_units,
            replica_provisioned_read_capacity_auto_scaling_settings_update: self.replica_provisioned_read_capacity_auto_scaling_settings_update,
            replica_global_secondary_index_settings_update: self.replica_global_secondary_index_settings_update,
            replica_table_class: self.replica_table_class,
        }
    }
}
