// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_tagging_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::put_object_tagging::PutObjectTaggingOutput, crate::operation::put_object_tagging::PutObjectTaggingError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_object_tagging::PutObjectTaggingError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::put_object_tagging::PutObjectTaggingError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_object_tagging_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::put_object_tagging::PutObjectTaggingOutput, crate::operation::put_object_tagging::PutObjectTaggingError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_object_tagging::builders::PutObjectTaggingOutputBuilder::default();
        output = output.set_version_id(
            crate::protocol_serde::shape_put_object_tagging_output::de_version_id_header(_response_headers).map_err(|_| {
                crate::operation::put_object_tagging::PutObjectTaggingError::unhandled("Failed to parse VersionId from header `x-amz-version-id")
            })?,
        );
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_put_object_tagging_headers(
    input: &crate::operation::put_object_tagging::PutObjectTaggingInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.content_md5 {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "content_md5",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("Content-MD5", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_3) = &input.checksum_algorithm {
        let formatted_4 = inner_3.as_str();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "checksum_algorithm",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-sdk-checksum-algorithm", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_5) = &input.expected_bucket_owner {
        let formatted_6 = inner_5.as_str();
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "expected_bucket_owner",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-expected-bucket-owner", header_value);
        }
    }
    if let ::std::option::Option::Some(inner_7) = &input.request_payer {
        let formatted_8 = inner_7.as_str();
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "request_payer",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-request-payer", header_value);
        }
    }
    Ok(builder)
}
