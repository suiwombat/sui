// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_auto_scaling_policy_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AutoScalingPolicyUpdate,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_name {
        object.key("PolicyName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_tracking_scaling_policy_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("TargetTrackingScalingPolicyConfiguration").start_object();
        crate::protocol_serde::shape_auto_scaling_target_tracking_scaling_policy_configuration_update::ser_auto_scaling_target_tracking_scaling_policy_configuration_update(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
