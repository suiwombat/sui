// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the schedule for generating inventory results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InventorySchedule {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub frequency: ::std::option::Option<crate::types::InventoryFrequency>,
}
impl InventorySchedule {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub fn frequency(&self) -> ::std::option::Option<&crate::types::InventoryFrequency> {
        self.frequency.as_ref()
    }
}
impl InventorySchedule {
    /// Creates a new builder-style object to manufacture [`InventorySchedule`](crate::types::InventorySchedule).
    pub fn builder() -> crate::types::builders::InventoryScheduleBuilder {
        crate::types::builders::InventoryScheduleBuilder::default()
    }
}

/// A builder for [`InventorySchedule`](crate::types::InventorySchedule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InventoryScheduleBuilder {
    pub(crate) frequency: ::std::option::Option<crate::types::InventoryFrequency>,
}
impl InventoryScheduleBuilder {
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub fn frequency(mut self, input: crate::types::InventoryFrequency) -> Self {
        self.frequency = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub fn set_frequency(mut self, input: ::std::option::Option<crate::types::InventoryFrequency>) -> Self {
        self.frequency = input;
        self
    }
    /// <p>Specifies how frequently inventory results are produced.</p>
    pub fn get_frequency(&self) -> &::std::option::Option<crate::types::InventoryFrequency> {
        &self.frequency
    }
    /// Consumes the builder and constructs a [`InventorySchedule`](crate::types::InventorySchedule).
    pub fn build(self) -> crate::types::InventorySchedule {
        crate::types::InventorySchedule { frequency: self.frequency }
    }
}
