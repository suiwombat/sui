// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_marker_replication(
    input: &crate::types::DeleteMarkerReplication,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.status {
        let mut inner_writer = scope.start_el("Status").finish();
        inner_writer.data(var_1.as_str());
    }
    scope.finish();
    Ok(())
}

pub fn de_delete_marker_replication(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::DeleteMarkerReplication, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DeleteMarkerReplication::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Status") /* Status com.amazonaws.s3#DeleteMarkerReplication$Status */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::DeleteMarkerReplicationStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DeleteMarkerReplicationStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
