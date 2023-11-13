// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateBackupInput {
    /// <p>The name of the table.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>Specified name for the backup.</p>
    pub backup_name: ::std::option::Option<::std::string::String>,
}
impl CreateBackupInput {
    /// <p>The name of the table.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>Specified name for the backup.</p>
    pub fn backup_name(&self) -> ::std::option::Option<&str> {
        self.backup_name.as_deref()
    }
}
impl CreateBackupInput {
    /// Creates a new builder-style object to manufacture [`CreateBackupInput`](crate::operation::create_backup::CreateBackupInput).
    pub fn builder() -> crate::operation::create_backup::builders::CreateBackupInputBuilder {
        crate::operation::create_backup::builders::CreateBackupInputBuilder::default()
    }
}

/// A builder for [`CreateBackupInput`](crate::operation::create_backup::CreateBackupInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateBackupInputBuilder {
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) backup_name: ::std::option::Option<::std::string::String>,
}
impl CreateBackupInputBuilder {
    /// <p>The name of the table.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the table.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>The name of the table.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>Specified name for the backup.</p>
    pub fn backup_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.backup_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specified name for the backup.</p>
    pub fn set_backup_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.backup_name = input;
        self
    }
    /// <p>Specified name for the backup.</p>
    pub fn get_backup_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.backup_name
    }
    /// Consumes the builder and constructs a [`CreateBackupInput`](crate::operation::create_backup::CreateBackupInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_backup::CreateBackupInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_backup::CreateBackupInput {
            table_name: self.table_name,
            backup_name: self.backup_name,
        })
    }
}
