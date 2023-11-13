// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_replication_rule(
    input: &crate::types::ReplicationRule,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.id {
        let mut inner_writer = scope.start_el("ID").finish();
        inner_writer.data(var_1.as_str());
    }
    if input.priority != 0 {
        let mut inner_writer = scope.start_el("Priority").finish();
        inner_writer.data(::aws_smithy_types::primitive::Encoder::from(input.priority).encode());
    }
    if let Some(var_2) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_2.as_str());
    }
    if let Some(var_3) = &input.filter {
        let inner_writer = scope.start_el("Filter");
        crate::protocol_serde::shape_replication_rule_filter::ser_replication_rule_filter(var_3, inner_writer)?
    }
    if let Some(var_4) = &input.status {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.source_selection_criteria {
        let inner_writer = scope.start_el("SourceSelectionCriteria");
        crate::protocol_serde::shape_source_selection_criteria::ser_source_selection_criteria(var_5, inner_writer)?
    }
    if let Some(var_6) = &input.existing_object_replication {
        let inner_writer = scope.start_el("ExistingObjectReplication");
        crate::protocol_serde::shape_existing_object_replication::ser_existing_object_replication(var_6, inner_writer)?
    }
    if let Some(var_7) = &input.destination {
        let inner_writer = scope.start_el("Destination");
        crate::protocol_serde::shape_destination::ser_destination(var_7, inner_writer)?
    }
    if let Some(var_8) = &input.delete_marker_replication {
        let inner_writer = scope.start_el("DeleteMarkerReplication");
        crate::protocol_serde::shape_delete_marker_replication::ser_delete_marker_replication(var_8, inner_writer)?
    }
    scope.finish();
    Ok(())
}

pub fn de_replication_rule(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ReplicationRule, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ReplicationRule::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ID") /* ID com.amazonaws.s3#ReplicationRule$ID */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_9);
            }
            ,
            s if s.matches("Priority") /* Priority com.amazonaws.s3#ReplicationRule$Priority */ =>  {
                let var_10 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.s3#Priority`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_priority(var_10);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#ReplicationRule$Prefix */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_11);
            }
            ,
            s if s.matches("Filter") /* Filter com.amazonaws.s3#ReplicationRule$Filter */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_replication_rule_filter::de_replication_rule_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_filter(var_12);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.s3#ReplicationRule$Status */ =>  {
                let var_13 =
                    Some(
                        Result::<crate::types::ReplicationRuleStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ReplicationRuleStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_13);
            }
            ,
            s if s.matches("SourceSelectionCriteria") /* SourceSelectionCriteria com.amazonaws.s3#ReplicationRule$SourceSelectionCriteria */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_source_selection_criteria::de_source_selection_criteria(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_source_selection_criteria(var_14);
            }
            ,
            s if s.matches("ExistingObjectReplication") /* ExistingObjectReplication com.amazonaws.s3#ReplicationRule$ExistingObjectReplication */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_existing_object_replication::de_existing_object_replication(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_existing_object_replication(var_15);
            }
            ,
            s if s.matches("Destination") /* Destination com.amazonaws.s3#ReplicationRule$Destination */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_destination::de_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination(var_16);
            }
            ,
            s if s.matches("DeleteMarkerReplication") /* DeleteMarkerReplication com.amazonaws.s3#ReplicationRule$DeleteMarkerReplication */ =>  {
                let var_17 =
                    Some(
                        crate::protocol_serde::shape_delete_marker_replication::de_delete_marker_replication(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_delete_marker_replication(var_17);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
