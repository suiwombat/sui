// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_intelligent_tiering_and_operator(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::IntelligentTieringAndOperator, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::IntelligentTieringAndOperator::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#IntelligentTieringAndOperator$Prefix */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_1);
            }
            ,
            s if s.matches("Tag") /* Tags com.amazonaws.s3#IntelligentTieringAndOperator$Tags */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::Tag>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_3 = builder.tags.take().unwrap_or_default();
                            list_3.push(
                                crate::protocol_serde::shape_tag::de_tag(&mut tag)
                                ?
                            );
                            list_3
                        })
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_intelligent_tiering_and_operator(
    input: &crate::types::IntelligentTieringAndOperator,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_4) = &input.prefix {
        let mut inner_writer = scope.start_el("Prefix").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        for list_item_6 in var_5 {
            {
                let inner_writer = scope.start_el("Tag");
                crate::protocol_serde::shape_tag::ser_tag(list_item_6, inner_writer)?
            }
        }
    }
    scope.finish();
    Ok(())
}
