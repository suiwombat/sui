// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_output_serialization(
    input: &crate::types::OutputSerialization,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.csv {
        let inner_writer = scope.start_el("CSV");
        crate::protocol_serde::shape_csv_output::ser_csv_output(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.json {
        let inner_writer = scope.start_el("JSON");
        crate::protocol_serde::shape_json_output::ser_json_output(var_2, inner_writer)?
    }
    scope.finish();
    Ok(())
}
