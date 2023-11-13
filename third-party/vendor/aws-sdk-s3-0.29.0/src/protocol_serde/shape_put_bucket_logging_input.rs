// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bucket_logging_status_http_payload(
    payload: &::std::option::Option<crate::types::BucketLoggingStatus>,
) -> Result<::std::vec::Vec<u8>, ::aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::protocol_serde::rest_xml_unset_payload()),
    };
    Ok(crate::protocol_serde::shape_put_bucket_logging_input::ser_bucket_logging_status_payload(
        payload,
    )?)
}

pub fn ser_bucket_logging_status_payload(
    input: &crate::types::BucketLoggingStatus,
) -> std::result::Result<std::vec::Vec<u8>, ::aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = ::aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("BucketLoggingStatus")
            .write_ns("http://s3.amazonaws.com/doc/2006-03-01/", None);
        crate::protocol_serde::shape_bucket_logging_status::ser_bucket_logging_status(input, root)?
    }
    Ok(out.into_bytes())
}
