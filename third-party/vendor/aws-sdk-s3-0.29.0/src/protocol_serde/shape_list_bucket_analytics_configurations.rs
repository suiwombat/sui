// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_bucket_analytics_configurations_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsOutput,
    crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_bucket_analytics_configurations_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsOutput,
    crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_bucket_analytics_configurations::builders::ListBucketAnalyticsConfigurationsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_bucket_analytics_configurations::de_list_bucket_analytics_configurations(_response_body, output)
            .map_err(crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsError::unhandled)?;
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_bucket_analytics_configurations_headers(
    input: &crate::operation::list_bucket_analytics_configurations::ListBucketAnalyticsConfigurationsInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.expected_bucket_owner {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    Ok(builder)
}

#[allow(unused_mut)]
pub fn de_list_bucket_analytics_configurations(
    inp: &[u8],
    mut builder: crate::operation::list_bucket_analytics_configurations::builders::ListBucketAnalyticsConfigurationsOutputBuilder,
) -> Result<
    crate::operation::list_bucket_analytics_configurations::builders::ListBucketAnalyticsConfigurationsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ListBucketAnalyticsConfigurationResult") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected ListBucketAnalyticsConfigurationResult but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NextContinuationToken") /* NextContinuationToken com.amazonaws.s3.synthetic#ListBucketAnalyticsConfigurationsOutput$NextContinuationToken */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_continuation_token(var_3);
            }
            ,
            s if s.matches("ContinuationToken") /* ContinuationToken com.amazonaws.s3.synthetic#ListBucketAnalyticsConfigurationsOutput$ContinuationToken */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_continuation_token(var_4);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.s3.synthetic#ListBucketAnalyticsConfigurationsOutput$IsTruncated */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#IsTruncated`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_5);
            }
            ,
            s if s.matches("AnalyticsConfiguration") /* AnalyticsConfigurationList com.amazonaws.s3.synthetic#ListBucketAnalyticsConfigurationsOutput$AnalyticsConfigurationList */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::vec::Vec<crate::types::AnalyticsConfiguration>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_7 = builder.analytics_configuration_list.take().unwrap_or_default();
                            list_7.push(
                                crate::protocol_serde::shape_analytics_configuration::de_analytics_configuration(&mut tag)
                                ?
                            );
                            list_7
                        })
                        ?
                    )
                ;
                builder = builder.set_analytics_configuration_list(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
