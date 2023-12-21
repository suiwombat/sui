// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_delete_marker_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<bool>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-delete-marker").iter();
    let var_1 = ::aws_smithy_http::header::read_many_primitive::<bool>(headers)?;
    if var_1.len() > 1 {
        Err(::aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_1.len()
        )))
    } else {
        let mut var_1 = var_1;
        Ok(var_1.pop())
    }
}

pub(crate) fn de_last_modified_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<::aws_smithy_types::DateTime>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("Last-Modified").iter();
    let var_2: Vec<::aws_smithy_types::DateTime> = ::aws_smithy_http::header::many_dates(headers, ::aws_smithy_types::date_time::Format::HttpDate)?;
    if var_2.len() > 1 {
        Err(::aws_smithy_http::header::ParseError::new(format!(
            "expected one item but found {}",
            var_2.len()
        )))
    } else {
        let mut var_2 = var_2;
        Ok(var_2.pop())
    }
}

pub(crate) fn de_request_charged_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<crate::types::RequestCharged>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-request-charged").iter();
    ::aws_smithy_http::header::one_or_none(headers)
}

pub(crate) fn de_version_id_header(
    header_map: &::http::HeaderMap,
) -> std::result::Result<::std::option::Option<::std::string::String>, ::aws_smithy_http::header::ParseError> {
    let headers = header_map.get_all("x-amz-version-id").iter();
    ::aws_smithy_http::header::one_or_none(headers)
}
