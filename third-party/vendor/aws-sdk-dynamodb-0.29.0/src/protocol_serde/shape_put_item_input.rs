// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_item_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_item::PutItemInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.table_name {
        object.key("TableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.item {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Item").start_object();
        for (key_4, value_5) in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_6 = object_3.key(key_4.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_6, value_5)?;
                object_6.finish();
            }
        }
        object_3.finish();
    }
    if let Some(var_7) = &input.expected {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Expected").start_object();
        for (key_9, value_10) in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_11 = object_8.key(key_9.as_str()).start_object();
                crate::protocol_serde::shape_expected_attribute_value::ser_expected_attribute_value(&mut object_11, value_10)?;
                object_11.finish();
            }
        }
        object_8.finish();
    }
    if let Some(var_12) = &input.return_values {
        object.key("ReturnValues").string(var_12.as_str());
    }
    if let Some(var_13) = &input.return_consumed_capacity {
        object.key("ReturnConsumedCapacity").string(var_13.as_str());
    }
    if let Some(var_14) = &input.return_item_collection_metrics {
        object.key("ReturnItemCollectionMetrics").string(var_14.as_str());
    }
    if let Some(var_15) = &input.conditional_operator {
        object.key("ConditionalOperator").string(var_15.as_str());
    }
    if let Some(var_16) = &input.condition_expression {
        object.key("ConditionExpression").string(var_16.as_str());
    }
    if let Some(var_17) = &input.expression_attribute_names {
        #[allow(unused_mut)]
        let mut object_18 = object.key("ExpressionAttributeNames").start_object();
        for (key_19, value_20) in var_17 {
            {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    if let Some(var_21) = &input.expression_attribute_values {
        #[allow(unused_mut)]
        let mut object_22 = object.key("ExpressionAttributeValues").start_object();
        for (key_23, value_24) in var_21 {
            {
                #[allow(unused_mut)]
                let mut object_25 = object_22.key(key_23.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_25, value_24)?;
                object_25.finish();
            }
        }
        object_22.finish();
    }
    if let Some(var_26) = &input.return_values_on_condition_check_failure {
        object.key("ReturnValuesOnConditionCheckFailure").string(var_26.as_str());
    }
    Ok(())
}
