// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_inventory_s3_bucket_destination(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InventoryS3BucketDestination, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InventoryS3BucketDestination::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AccountId") /* AccountId com.amazonaws.s3#InventoryS3BucketDestination$AccountId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_account_id(var_1);
            }
            ,
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3#InventoryS3BucketDestination$Bucket */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_2);
            }
            ,
            s if s.matches("Format") /* Format com.amazonaws.s3#InventoryS3BucketDestination$Format */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::InventoryFormat, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InventoryFormat::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_format(var_3);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#InventoryS3BucketDestination$Prefix */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_4);
            }
            ,
            s if s.matches("Encryption") /* Encryption com.amazonaws.s3#InventoryS3BucketDestination$Encryption */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_inventory_encryption::de_inventory_encryption(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_encryption(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_inventory_s3_bucket_destination(
    input: &crate::types::InventoryS3BucketDestination,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_6) = &input.account_id {
        let mut inner_writer = scope.start_el("AccountId").finish();
        inner_writer.data(var_6.as_str());
    }
    if let Some(var_7) = &input.bucket {
        let mut inner_writer = scope.start_el("Bucket").finish();
        inner_writer.data(var_7.as_str());
    }
    if let Some(var_8) = &input.format {
        let mut inner_writer = scope.start_el("Format").finish();
        inner_writer.data(var_8.as_str());
    }
    if let Some(var_9) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_9.as_str());
    }
    if let Some(var_10) = &input.encryption {
        let inner_writer = scope.start_el("Encryption");
        crate::protocol_serde::shape_inventory_encryption::ser_inventory_encryption(var_10, inner_writer)?
    }
    scope.finish();
    Ok(())
}
