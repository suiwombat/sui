// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_provided_context(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ProvidedContext,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ProviderArn");
    if let Some(var_2) = &input.provider_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ContextAssertion");
    if let Some(var_4) = &input.context_assertion {
        scope_3.string(var_4);
    }
    Ok(())
}
