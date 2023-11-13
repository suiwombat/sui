// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_analytics_configuration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AnalyticsConfiguration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AnalyticsConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Id") /* Id com.amazonaws.s3#AnalyticsConfiguration$Id */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_1);
            }
            ,
            s if s.matches("Filter") /* Filter com.amazonaws.s3#AnalyticsConfiguration$Filter */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_analytics_filter::de_analytics_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_filter(var_2);
            }
            ,
            s if s.matches("StorageClassAnalysis") /* StorageClassAnalysis com.amazonaws.s3#AnalyticsConfiguration$StorageClassAnalysis */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_storage_class_analysis::de_storage_class_analysis(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_storage_class_analysis(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_analytics_configuration(
    input: &crate::types::AnalyticsConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_4) = &input.id {
        let mut inner_writer = scope.start_el("Id").finish();
        inner_writer.data(var_4.as_str());
    }
    if let Some(var_5) = &input.filter {
        let inner_writer = scope.start_el("Filter");
        crate::protocol_serde::shape_analytics_filter::ser_analytics_filter(var_5, inner_writer)?
    }
    if let Some(var_6) = &input.storage_class_analysis {
        let inner_writer = scope.start_el("StorageClassAnalysis");
        crate::protocol_serde::shape_storage_class_analysis::ser_storage_class_analysis(var_6, inner_writer)?
    }
    scope.finish();
    Ok(())
}
