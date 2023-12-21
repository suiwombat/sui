// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGlobalTableSettings`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`global_table_name(impl ::std::convert::Into<String>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::global_table_name) / [`set_global_table_name(Option<String>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::set_global_table_name): <p>The name of the global table</p>
    ///   - [`global_table_billing_mode(BillingMode)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::global_table_billing_mode) / [`set_global_table_billing_mode(Option<BillingMode>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::set_global_table_billing_mode): <p>The billing mode of the global table. If <code>GlobalTableBillingMode</code> is not specified, the global table defaults to <code>PROVISIONED</code> capacity billing mode.</p>  <ul>   <li> <p> <code>PROVISIONED</code> - We recommend using <code>PROVISIONED</code> for predictable workloads. <code>PROVISIONED</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.ProvisionedThroughput.Manual">Provisioned Mode</a>.</p> </li>   <li> <p> <code>PAY_PER_REQUEST</code> - We recommend using <code>PAY_PER_REQUEST</code> for unpredictable workloads. <code>PAY_PER_REQUEST</code> sets the billing mode to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/HowItWorks.ReadWriteCapacityMode.html#HowItWorks.OnDemand">On-Demand Mode</a>. </p> </li>  </ul>
    ///   - [`global_table_provisioned_write_capacity_units(i64)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::global_table_provisioned_write_capacity_units) / [`set_global_table_provisioned_write_capacity_units(Option<i64>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::set_global_table_provisioned_write_capacity_units): <p>The maximum number of writes consumed per second before DynamoDB returns a <code>ThrottlingException.</code> </p>
    ///   - [`global_table_provisioned_write_capacity_auto_scaling_settings_update(AutoScalingSettingsUpdate)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::global_table_provisioned_write_capacity_auto_scaling_settings_update) / [`set_global_table_provisioned_write_capacity_auto_scaling_settings_update(Option<AutoScalingSettingsUpdate>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::set_global_table_provisioned_write_capacity_auto_scaling_settings_update): <p>Auto scaling settings for managing provisioned write capacity for the global table.</p>
    ///   - [`global_table_global_secondary_index_settings_update(Vec<GlobalTableGlobalSecondaryIndexSettingsUpdate>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::global_table_global_secondary_index_settings_update) / [`set_global_table_global_secondary_index_settings_update(Option<Vec<GlobalTableGlobalSecondaryIndexSettingsUpdate>>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::set_global_table_global_secondary_index_settings_update): <p>Represents the settings of a global secondary index for a global table that will be modified.</p>
    ///   - [`replica_settings_update(Vec<ReplicaSettingsUpdate>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::replica_settings_update) / [`set_replica_settings_update(Option<Vec<ReplicaSettingsUpdate>>)`](crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::set_replica_settings_update): <p>Represents the settings for a global table in a Region that will be modified.</p>
    /// - On success, responds with [`UpdateGlobalTableSettingsOutput`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput) with field(s):
    ///   - [`global_table_name(Option<String>)`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput::global_table_name): <p>The name of the global table.</p>
    ///   - [`replica_settings(Option<Vec<ReplicaSettingsDescription>>)`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsOutput::replica_settings): <p>The Region-specific settings for the global table.</p>
    /// - On failure, responds with [`SdkError<UpdateGlobalTableSettingsError>`](crate::operation::update_global_table_settings::UpdateGlobalTableSettingsError)
    pub fn update_global_table_settings(&self) -> crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder {
        crate::operation::update_global_table_settings::builders::UpdateGlobalTableSettingsFluentBuilder::new(self.handle.clone())
    }
}
