// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_contributor_insights_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_contributor_insights::ListContributorInsightsOutput,
    crate::operation::list_contributor_insights::ListContributorInsightsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_contributor_insights::ListContributorInsightsError::unhandled)?;
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::list_contributor_insights::ListContributorInsightsError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::operation::list_contributor_insights::ListContributorInsightsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::list_contributor_insights::ListContributorInsightsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::list_contributor_insights::ListContributorInsightsError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_contributor_insights::ListContributorInsightsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_contributor_insights::ListContributorInsightsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_contributor_insights_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_contributor_insights::ListContributorInsightsOutput,
    crate::operation::list_contributor_insights::ListContributorInsightsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_contributor_insights::builders::ListContributorInsightsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_contributor_insights::de_list_contributor_insights(_response_body, output)
            .map_err(crate::operation::list_contributor_insights::ListContributorInsightsError::unhandled)?;
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_contributor_insights_input(
    input: &crate::operation::list_contributor_insights::ListContributorInsightsInput,
) -> Result<::aws_smithy_http::body::SdkBody, ::aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_contributor_insights_input::ser_list_contributor_insights_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_http::body::SdkBody::from(out))
}

pub(crate) fn de_list_contributor_insights(
    value: &[u8],
    mut builder: crate::operation::list_contributor_insights::builders::ListContributorInsightsOutputBuilder,
) -> Result<
    crate::operation::list_contributor_insights::builders::ListContributorInsightsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ContributorInsightsSummaries" => {
                    builder = builder.set_contributor_insights_summaries(
                        crate::protocol_serde::shape_contributor_insights_summaries::de_contributor_insights_summaries(tokens)?,
                    );
                }
                "NextToken" => {
                    builder = builder.set_next_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
