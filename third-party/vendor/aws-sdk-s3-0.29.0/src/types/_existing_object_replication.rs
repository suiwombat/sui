// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Optional configuration to replicate existing source bucket objects. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/replication-what-is-isnot-replicated.html#existing-object-replication">Replicating Existing Objects</a> in the <i>Amazon S3 User Guide</i>. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExistingObjectReplication {
    /// <p>Specifies whether Amazon S3 replicates existing source bucket objects. </p>
    pub status: ::std::option::Option<crate::types::ExistingObjectReplicationStatus>,
}
impl ExistingObjectReplication {
    /// <p>Specifies whether Amazon S3 replicates existing source bucket objects. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ExistingObjectReplicationStatus> {
        self.status.as_ref()
    }
}
impl ExistingObjectReplication {
    /// Creates a new builder-style object to manufacture [`ExistingObjectReplication`](crate::types::ExistingObjectReplication).
    pub fn builder() -> crate::types::builders::ExistingObjectReplicationBuilder {
        crate::types::builders::ExistingObjectReplicationBuilder::default()
    }
}

/// A builder for [`ExistingObjectReplication`](crate::types::ExistingObjectReplication).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ExistingObjectReplicationBuilder {
    pub(crate) status: ::std::option::Option<crate::types::ExistingObjectReplicationStatus>,
}
impl ExistingObjectReplicationBuilder {
    /// <p>Specifies whether Amazon S3 replicates existing source bucket objects. </p>
    pub fn status(mut self, input: crate::types::ExistingObjectReplicationStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether Amazon S3 replicates existing source bucket objects. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ExistingObjectReplicationStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies whether Amazon S3 replicates existing source bucket objects. </p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ExistingObjectReplicationStatus> {
        &self.status
    }
    /// Consumes the builder and constructs a [`ExistingObjectReplication`](crate::types::ExistingObjectReplication).
    pub fn build(self) -> crate::types::ExistingObjectReplication {
        crate::types::ExistingObjectReplication { status: self.status }
    }
}
