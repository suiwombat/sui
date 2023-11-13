// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeBackup`](crate::operation::describe_backup::builders::DescribeBackupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`backup_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_backup::builders::DescribeBackupFluentBuilder::backup_arn) / [`set_backup_arn(Option<String>)`](crate::operation::describe_backup::builders::DescribeBackupFluentBuilder::set_backup_arn): <p>The Amazon Resource Name (ARN) associated with the backup.</p>
    /// - On success, responds with [`DescribeBackupOutput`](crate::operation::describe_backup::DescribeBackupOutput) with field(s):
    ///   - [`backup_description(Option<BackupDescription>)`](crate::operation::describe_backup::DescribeBackupOutput::backup_description): <p>Contains the description of the backup created for the table.</p>
    /// - On failure, responds with [`SdkError<DescribeBackupError>`](crate::operation::describe_backup::DescribeBackupError)
    pub fn describe_backup(&self) -> crate::operation::describe_backup::builders::DescribeBackupFluentBuilder {
        crate::operation::describe_backup::builders::DescribeBackupFluentBuilder::new(self.handle.clone())
    }
}
