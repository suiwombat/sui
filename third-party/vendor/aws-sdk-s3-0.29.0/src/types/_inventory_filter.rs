// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies an inventory filter. The inventory only includes objects that meet the filter's criteria.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InventoryFilter {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub prefix: ::std::option::Option<::std::string::String>,
}
impl InventoryFilter {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub fn prefix(&self) -> ::std::option::Option<&str> {
        self.prefix.as_deref()
    }
}
impl InventoryFilter {
    /// Creates a new builder-style object to manufacture [`InventoryFilter`](crate::types::InventoryFilter).
    pub fn builder() -> crate::types::builders::InventoryFilterBuilder {
        crate::types::builders::InventoryFilterBuilder::default()
    }
}

/// A builder for [`InventoryFilter`](crate::types::InventoryFilter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InventoryFilterBuilder {
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
}
impl InventoryFilterBuilder {
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input;
        self
    }
    /// <p>The prefix that an object must have to be included in the inventory results.</p>
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    /// Consumes the builder and constructs a [`InventoryFilter`](crate::types::InventoryFilter).
    pub fn build(self) -> crate::types::InventoryFilter {
        crate::types::InventoryFilter { prefix: self.prefix }
    }
}
