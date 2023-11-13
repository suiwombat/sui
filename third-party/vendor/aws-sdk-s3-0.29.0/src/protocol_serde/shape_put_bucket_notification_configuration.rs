// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_notification_configuration_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationOutput,
    crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_notification_configuration_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationOutput,
    crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::put_bucket_notification_configuration::builders::PutBucketNotificationConfigurationOutputBuilder::default();
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_put_bucket_notification_configuration_headers(
    input: &crate::operation::put_bucket_notification_configuration::PutBucketNotificationConfigurationInput,
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
    if let ::std::option::Option::Some(inner_3) = &input.skip_destination_validation {
        let mut encoder = ::aws_smithy_types::primitive::Encoder::from(*inner_3);
        let formatted_4 = encoder.encode();
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
            let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
                ::aws_smithy_http::operation::error::BuildError::invalid_field(
                    "skip_destination_validation",
                    format!("`{}` cannot be used as a header value: {}", &header_value, err),
                )
            })?;
            builder = builder.header("x-amz-skip-destination-validation", header_value);
        }
    }
    Ok(builder)
}
