// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the settings used to enable or disable Time to Live (TTL) for the specified table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TimeToLiveSpecification {
    /// <p>Indicates whether TTL is to be enabled (true) or disabled (false) on the table.</p>
    pub enabled: ::std::option::Option<bool>,
    /// <p>The name of the TTL attribute used to store the expiration time for items in the table.</p>
    pub attribute_name: ::std::option::Option<::std::string::String>,
}
impl TimeToLiveSpecification {
    /// <p>Indicates whether TTL is to be enabled (true) or disabled (false) on the table.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
    /// <p>The name of the TTL attribute used to store the expiration time for items in the table.</p>
    pub fn attribute_name(&self) -> ::std::option::Option<&str> {
        self.attribute_name.as_deref()
    }
}
impl TimeToLiveSpecification {
    /// Creates a new builder-style object to manufacture [`TimeToLiveSpecification`](crate::types::TimeToLiveSpecification).
    pub fn builder() -> crate::types::builders::TimeToLiveSpecificationBuilder {
        crate::types::builders::TimeToLiveSpecificationBuilder::default()
    }
}

/// A builder for [`TimeToLiveSpecification`](crate::types::TimeToLiveSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TimeToLiveSpecificationBuilder {
    pub(crate) enabled: ::std::option::Option<bool>,
    pub(crate) attribute_name: ::std::option::Option<::std::string::String>,
}
impl TimeToLiveSpecificationBuilder {
    /// <p>Indicates whether TTL is to be enabled (true) or disabled (false) on the table.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether TTL is to be enabled (true) or disabled (false) on the table.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Indicates whether TTL is to be enabled (true) or disabled (false) on the table.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// <p>The name of the TTL attribute used to store the expiration time for items in the table.</p>
    pub fn attribute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.attribute_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the TTL attribute used to store the expiration time for items in the table.</p>
    pub fn set_attribute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.attribute_name = input;
        self
    }
    /// <p>The name of the TTL attribute used to store the expiration time for items in the table.</p>
    pub fn get_attribute_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.attribute_name
    }
    /// Consumes the builder and constructs a [`TimeToLiveSpecification`](crate::types::TimeToLiveSpecification).
    pub fn build(self) -> crate::types::TimeToLiveSpecification {
        crate::types::TimeToLiveSpecification {
            enabled: self.enabled,
            attribute_name: self.attribute_name,
        }
    }
}
