// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_acl_http_error(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_bucket_acl::GetBucketAclOutput, crate::operation::get_bucket_acl::GetBucketAclError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_bucket_acl::GetBucketAclError::unhandled)?;
    generic_builder = crate::s3_request_id::apply_extended_request_id(generic_builder, _response_headers);
    generic_builder = ::aws_http::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_bucket_acl::GetBucketAclError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_acl_http_response(
    _response_status: u16,
    _response_headers: &::http::header::HeaderMap,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_bucket_acl::GetBucketAclOutput, crate::operation::get_bucket_acl::GetBucketAclError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bucket_acl::builders::GetBucketAclOutputBuilder::default();
        output = crate::protocol_serde::shape_get_bucket_acl::de_get_bucket_acl(_response_body, output)
            .map_err(crate::operation::get_bucket_acl::GetBucketAclError::unhandled)?;
        output._set_extended_request_id(crate::s3_request_id::RequestIdExt::extended_request_id(_response_headers).map(str::to_string));
        output._set_request_id(::aws_http::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_bucket_acl_headers(
    input: &crate::operation::get_bucket_acl::GetBucketAclInput,
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
pub fn de_get_bucket_acl(
    inp: &[u8],
    mut builder: crate::operation::get_bucket_acl::builders::GetBucketAclOutputBuilder,
) -> Result<crate::operation::get_bucket_acl::builders::GetBucketAclOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("AccessControlPolicy") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected AccessControlPolicy but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Owner") /* Owner com.amazonaws.s3.synthetic#GetBucketAclOutput$Owner */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_owner::de_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_3);
            }
            ,
            s if s.matches("AccessControlList") /* Grants com.amazonaws.s3.synthetic#GetBucketAclOutput$Grants */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_grants::de_grants(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_grants(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
