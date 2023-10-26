// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_target_grant(
    input: &crate::types::TargetGrant,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.grantee {
        let inner_writer = scope
            .start_el("Grantee")
            .write_ns("http://www.w3.org/2001/XMLSchema-instance", Some("xsi"));
        crate::protocol_serde::shape_grantee::ser_grantee(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.permission {
        let mut inner_writer = scope.start_el("Permission").finish();
        inner_writer.data(var_2.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_target_grant(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TargetGrant, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TargetGrant::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Grantee") /* Grantee com.amazonaws.s3#TargetGrant$Grantee */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_grantee::de_grantee(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_grantee(var_3);
            }
            ,
            s if s.matches("Permission") /* Permission com.amazonaws.s3#TargetGrant$Permission */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::BucketLogsPermission, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::BucketLogsPermission::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_permission(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
