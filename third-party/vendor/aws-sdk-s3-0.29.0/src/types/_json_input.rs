// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies JSON as object's input serialization format.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub r#type: ::std::option::Option<crate::types::JsonType>,
}
impl JsonInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::JsonType> {
        self.r#type.as_ref()
    }
}
impl JsonInput {
    /// Creates a new builder-style object to manufacture [`JsonInput`](crate::types::JsonInput).
    pub fn builder() -> crate::types::builders::JsonInputBuilder {
        crate::types::builders::JsonInputBuilder::default()
    }
}

/// A builder for [`JsonInput`](crate::types::JsonInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct JsonInputBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::JsonType>,
}
impl JsonInputBuilder {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub fn r#type(mut self, input: crate::types::JsonType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::JsonType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::JsonType> {
        &self.r#type
    }
    /// Consumes the builder and constructs a [`JsonInput`](crate::types::JsonInput).
    pub fn build(self) -> crate::types::JsonInput {
        crate::types::JsonInput { r#type: self.r#type }
    }
}
