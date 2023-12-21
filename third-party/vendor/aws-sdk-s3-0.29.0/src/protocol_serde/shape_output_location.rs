// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_output_location(
    input: &crate::types::OutputLocation,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.s3 {
        let inner_writer = scope.start_el("S3");
        crate::protocol_serde::shape_s3_location::ser_s3_location(var_1, inner_writer)?
    }
    scope.finish();
    Ok(())
}
