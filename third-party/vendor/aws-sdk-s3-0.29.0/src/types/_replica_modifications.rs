// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A filter that you can specify for selection for modifications on replicas. Amazon S3 doesn't replicate replica modifications by default. In the latest version of replication configuration (when <code>Filter</code> is specified), you can specify this element and set the status to <code>Enabled</code> to replicate modifications on replicas. </p> <note>
/// <p> If you don't specify the <code>Filter</code> element, Amazon S3 assumes that the replication configuration is the earlier version, V1. In the earlier version, this element is not allowed.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicaModifications {
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub status: ::std::option::Option<crate::types::ReplicaModificationsStatus>,
}
impl ReplicaModifications {
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ReplicaModificationsStatus> {
        self.status.as_ref()
    }
}
impl ReplicaModifications {
    /// Creates a new builder-style object to manufacture [`ReplicaModifications`](crate::types::ReplicaModifications).
    pub fn builder() -> crate::types::builders::ReplicaModificationsBuilder {
        crate::types::builders::ReplicaModificationsBuilder::default()
    }
}

/// A builder for [`ReplicaModifications`](crate::types::ReplicaModifications).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReplicaModificationsBuilder {
    pub(crate) status: ::std::option::Option<crate::types::ReplicaModificationsStatus>,
}
impl ReplicaModificationsBuilder {
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub fn status(mut self, input: crate::types::ReplicaModificationsStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ReplicaModificationsStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies whether Amazon S3 replicates modifications on replicas.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ReplicaModificationsStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`ReplicaModifications`](crate::types::ReplicaModifications).
    pub fn build(self) -> crate::types::ReplicaModifications {
        crate::types::ReplicaModifications { status: self.status }
    }
}
