// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents one of the following:</p>
/// <ul>
/// <li> <p>A new replica to be added to an existing regional table or global table. This request invokes the <code>CreateTableReplica</code> action in the destination Region.</p> </li>
/// <li> <p>New parameters for an existing replica. This request invokes the <code>UpdateTable</code> action in the destination Region.</p> </li>
/// <li> <p>An existing replica to be deleted. The request invokes the <code>DeleteTableReplica</code> action in the destination Region, deleting the replica and all if its items in the destination Region.</p> </li>
/// </ul> <note>
/// <p>When you manually remove a table or global table replica, you do not automatically remove any associated scalable targets, scaling policies, or CloudWatch alarms.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplicationGroupUpdate {
    /// <p>The parameters required for creating a replica for the table.</p>
    pub create: ::std::option::Option<crate::types::CreateReplicationGroupMemberAction>,
    /// <p>The parameters required for updating a replica for the table.</p>
    pub update: ::std::option::Option<crate::types::UpdateReplicationGroupMemberAction>,
    /// <p>The parameters required for deleting a replica for the table.</p>
    pub delete: ::std::option::Option<crate::types::DeleteReplicationGroupMemberAction>,
}
impl ReplicationGroupUpdate {
    /// <p>The parameters required for creating a replica for the table.</p>
    pub fn create(&self) -> ::std::option::Option<&crate::types::CreateReplicationGroupMemberAction> {
        self.create.as_ref()
    }
    /// <p>The parameters required for updating a replica for the table.</p>
    pub fn update(&self) -> ::std::option::Option<&crate::types::UpdateReplicationGroupMemberAction> {
        self.update.as_ref()
    }
    /// <p>The parameters required for deleting a replica for the table.</p>
    pub fn delete(&self) -> ::std::option::Option<&crate::types::DeleteReplicationGroupMemberAction> {
        self.delete.as_ref()
    }
}
impl ReplicationGroupUpdate {
    /// Creates a new builder-style object to manufacture [`ReplicationGroupUpdate`](crate::types::ReplicationGroupUpdate).
    pub fn builder() -> crate::types::builders::ReplicationGroupUpdateBuilder {
        crate::types::builders::ReplicationGroupUpdateBuilder::default()
    }
}

/// A builder for [`ReplicationGroupUpdate`](crate::types::ReplicationGroupUpdate).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReplicationGroupUpdateBuilder {
    pub(crate) create: ::std::option::Option<crate::types::CreateReplicationGroupMemberAction>,
    pub(crate) update: ::std::option::Option<crate::types::UpdateReplicationGroupMemberAction>,
    pub(crate) delete: ::std::option::Option<crate::types::DeleteReplicationGroupMemberAction>,
}
impl ReplicationGroupUpdateBuilder {
    /// <p>The parameters required for creating a replica for the table.</p>
    pub fn create(mut self, input: crate::types::CreateReplicationGroupMemberAction) -> Self {
        self.create = ::std::option::Option::Some(input);
        self
    }
    /// <p>The parameters required for creating a replica for the table.</p>
    pub fn set_create(mut self, input: ::std::option::Option<crate::types::CreateReplicationGroupMemberAction>) -> Self {
        self.create = input;
        self
    }
    /// <p>The parameters required for creating a replica for the table.</p>
    pub fn get_create(&self) -> &::std::option::Option<crate::types::CreateReplicationGroupMemberAction> {
        &self.create
    }
    /// <p>The parameters required for updating a replica for the table.</p>
    pub fn update(mut self, input: crate::types::UpdateReplicationGroupMemberAction) -> Self {
        self.update = ::std::option::Option::Some(input);
        self
    }
    /// <p>The parameters required for updating a replica for the table.</p>
    pub fn set_update(mut self, input: ::std::option::Option<crate::types::UpdateReplicationGroupMemberAction>) -> Self {
        self.update = input;
        self
    }
    /// <p>The parameters required for updating a replica for the table.</p>
    pub fn get_update(&self) -> &::std::option::Option<crate::types::UpdateReplicationGroupMemberAction> {
        &self.update
    }
    /// <p>The parameters required for deleting a replica for the table.</p>
    pub fn delete(mut self, input: crate::types::DeleteReplicationGroupMemberAction) -> Self {
        self.delete = ::std::option::Option::Some(input);
        self
    }
    /// <p>The parameters required for deleting a replica for the table.</p>
    pub fn set_delete(mut self, input: ::std::option::Option<crate::types::DeleteReplicationGroupMemberAction>) -> Self {
        self.delete = input;
        self
    }
    /// <p>The parameters required for deleting a replica for the table.</p>
    pub fn get_delete(&self) -> &::std::option::Option<crate::types::DeleteReplicationGroupMemberAction> {
        &self.delete
    }
    /// Consumes the builder and constructs a [`ReplicationGroupUpdate`](crate::types::ReplicationGroupUpdate).
    pub fn build(self) -> crate::types::ReplicationGroupUpdate {
        crate::types::ReplicationGroupUpdate {
            create: self.create,
            update: self.update,
            delete: self.delete,
        }
    }
}
