// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_replica_global_secondary_index_settings_description<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::ReplicaGlobalSecondaryIndexSettingsDescription>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ReplicaGlobalSecondaryIndexSettingsDescriptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "IndexName" => {
                            builder = builder.set_index_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "IndexStatus" => {
                            builder = builder.set_index_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::IndexStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ProvisionedReadCapacityUnits" => {
                            builder = builder.set_provisioned_read_capacity_units(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "ProvisionedReadCapacityAutoScalingSettings" => {
                            builder = builder.set_provisioned_read_capacity_auto_scaling_settings(
                                crate::protocol_serde::shape_auto_scaling_settings_description::de_auto_scaling_settings_description(tokens)?,
                            );
                        }
                        "ProvisionedWriteCapacityUnits" => {
                            builder = builder.set_provisioned_write_capacity_units(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "ProvisionedWriteCapacityAutoScalingSettings" => {
                            builder = builder.set_provisioned_write_capacity_auto_scaling_settings(
                                crate::protocol_serde::shape_auto_scaling_settings_description::de_auto_scaling_settings_description(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
