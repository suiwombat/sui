// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_metrics_filter(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::MetricsFilter, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut base: Option<crate::types::MetricsFilter> = None;
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Prefix") /* Prefix com.amazonaws.s3#MetricsFilter$Prefix */ =>  {
                let tmp =
                    Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        .into()
                    )
                    ?
                ;
                base = Some(crate::types::MetricsFilter::Prefix(tmp));
            }
            ,
            s if s.matches("Tag") /* Tag com.amazonaws.s3#MetricsFilter$Tag */ =>  {
                let tmp =
                    crate::protocol_serde::shape_tag::de_tag(&mut tag)
                    ?
                ;
                base = Some(crate::types::MetricsFilter::Tag(tmp));
            }
            ,
            s if s.matches("AccessPointArn") /* AccessPointArn com.amazonaws.s3#MetricsFilter$AccessPointArn */ =>  {
                let tmp =
                    Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        .into()
                    )
                    ?
                ;
                base = Some(crate::types::MetricsFilter::AccessPointArn(tmp));
            }
            ,
            s if s.matches("And") /* And com.amazonaws.s3#MetricsFilter$And */ =>  {
                let tmp =
                    crate::protocol_serde::shape_metrics_and_operator::de_metrics_and_operator(&mut tag)
                    ?
                ;
                base = Some(crate::types::MetricsFilter::And(tmp));
            }
            ,
            _unknown => base = Some(crate::types::MetricsFilter::Unknown),
        }
    }
    base.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom("expected union, got nothing"))
}

pub fn ser_metrics_filter(
    input: &crate::types::MetricsFilter,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    let mut scope_writer = writer.finish();
    match input {
        crate::types::MetricsFilter::Prefix(inner) => {
            let mut inner_writer = scope_writer.start_el("Prefix").finish();
            inner_writer.data(inner.as_str());
        }
        crate::types::MetricsFilter::Tag(inner) => {
            let inner_writer = scope_writer.start_el("Tag");
            crate::protocol_serde::shape_tag::ser_tag(inner, inner_writer)?
        }
        crate::types::MetricsFilter::AccessPointArn(inner) => {
            let mut inner_writer = scope_writer.start_el("AccessPointArn").finish();
            inner_writer.data(inner.as_str());
        }
        crate::types::MetricsFilter::And(inner) => {
            let inner_writer = scope_writer.start_el("And");
            crate::protocol_serde::shape_metrics_and_operator::ser_metrics_and_operator(inner, inner_writer)?
        }
        crate::types::MetricsFilter::Unknown => {
            return Err(::aws_smithy_http::operation::error::SerializationError::unknown_variant("MetricsFilter"))
        }
    }
    Ok(())
}
