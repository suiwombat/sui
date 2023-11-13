// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AssumeRoleWithSamlInput {
    /// <p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p>
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.</p>
    pub principal_arn: ::std::option::Option<::std::string::String>,
    /// <p>The base64 encoded SAML authentication response provided by the IdP.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/create-role-saml-IdP-tasks.html">Configuring a Relying Party and Adding Claims</a> in the <i>IAM User Guide</i>. </p>
    pub saml_assertion: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p>
    /// <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
    pub policy_arns: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
    /// <p>An IAM policy in JSON format that you want to use as an inline session policy.</p>
    /// <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>. </p>
    /// <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    pub policy: ::std::option::Option<::std::string::String>,
    /// <p>The duration, in seconds, of the role session. Your role session lasts for the duration that you specify for the <code>DurationSeconds</code> parameter, or until the time specified in the SAML authentication response's <code>SessionNotOnOrAfter</code> value, whichever is shorter. You can provide a <code>DurationSeconds</code> value from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p>
    /// <p>By default, the value is set to <code>3600</code> seconds. </p> <note>
    /// <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub duration_seconds: ::std::option::Option<i32>,
}
impl AssumeRoleWithSamlInput {
    /// <p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.</p>
    pub fn principal_arn(&self) -> ::std::option::Option<&str> {
        self.principal_arn.as_deref()
    }
    /// <p>The base64 encoded SAML authentication response provided by the IdP.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/create-role-saml-IdP-tasks.html">Configuring a Relying Party and Adding Claims</a> in the <i>IAM User Guide</i>. </p>
    pub fn saml_assertion(&self) -> ::std::option::Option<&str> {
        self.saml_assertion.as_deref()
    }
    /// <p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p>
    /// <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn policy_arns(&self) -> ::std::option::Option<&[crate::types::PolicyDescriptorType]> {
        self.policy_arns.as_deref()
    }
    /// <p>An IAM policy in JSON format that you want to use as an inline session policy.</p>
    /// <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>. </p>
    /// <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    pub fn policy(&self) -> ::std::option::Option<&str> {
        self.policy.as_deref()
    }
    /// <p>The duration, in seconds, of the role session. Your role session lasts for the duration that you specify for the <code>DurationSeconds</code> parameter, or until the time specified in the SAML authentication response's <code>SessionNotOnOrAfter</code> value, whichever is shorter. You can provide a <code>DurationSeconds</code> value from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p>
    /// <p>By default, the value is set to <code>3600</code> seconds. </p> <note>
    /// <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub fn duration_seconds(&self) -> ::std::option::Option<i32> {
        self.duration_seconds
    }
}
impl ::std::fmt::Debug for AssumeRoleWithSamlInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AssumeRoleWithSamlInput");
        formatter.field("role_arn", &self.role_arn);
        formatter.field("principal_arn", &self.principal_arn);
        formatter.field("saml_assertion", &"*** Sensitive Data Redacted ***");
        formatter.field("policy_arns", &self.policy_arns);
        formatter.field("policy", &self.policy);
        formatter.field("duration_seconds", &self.duration_seconds);
        formatter.finish()
    }
}
impl AssumeRoleWithSamlInput {
    /// Creates a new builder-style object to manufacture [`AssumeRoleWithSamlInput`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput).
    pub fn builder() -> crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlInputBuilder {
        crate::operation::assume_role_with_saml::builders::AssumeRoleWithSamlInputBuilder::default()
    }
}

/// A builder for [`AssumeRoleWithSamlInput`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AssumeRoleWithSamlInputBuilder {
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) principal_arn: ::std::option::Option<::std::string::String>,
    pub(crate) saml_assertion: ::std::option::Option<::std::string::String>,
    pub(crate) policy_arns: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>,
    pub(crate) policy: ::std::option::Option<::std::string::String>,
    pub(crate) duration_seconds: ::std::option::Option<i32>,
}
impl AssumeRoleWithSamlInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the role that the caller is assuming.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.</p>
    pub fn principal_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.</p>
    pub fn set_principal_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.</p>
    pub fn get_principal_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.principal_arn
    }
    /// <p>The base64 encoded SAML authentication response provided by the IdP.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/create-role-saml-IdP-tasks.html">Configuring a Relying Party and Adding Claims</a> in the <i>IAM User Guide</i>. </p>
    pub fn saml_assertion(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.saml_assertion = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64 encoded SAML authentication response provided by the IdP.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/create-role-saml-IdP-tasks.html">Configuring a Relying Party and Adding Claims</a> in the <i>IAM User Guide</i>. </p>
    pub fn set_saml_assertion(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.saml_assertion = input;
        self
    }
    /// <p>The base64 encoded SAML authentication response provided by the IdP.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/create-role-saml-IdP-tasks.html">Configuring a Relying Party and Adding Claims</a> in the <i>IAM User Guide</i>. </p>
    pub fn get_saml_assertion(&self) -> &::std::option::Option<::std::string::String> {
        &self.saml_assertion
    }
    /// Appends an item to `policy_arns`.
    ///
    /// To override the contents of this collection use [`set_policy_arns`](Self::set_policy_arns).
    ///
    /// <p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p>
    /// <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn policy_arns(mut self, input: crate::types::PolicyDescriptorType) -> Self {
        let mut v = self.policy_arns.unwrap_or_default();
        v.push(input);
        self.policy_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p>
    /// <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>>) -> Self {
        self.policy_arns = input;
        self
    }
    /// <p>The Amazon Resource Names (ARNs) of the IAM managed policies that you want to use as managed session policies. The policies must exist in the same account as the role.</p>
    /// <p>This parameter is optional. You can provide up to 10 managed policy ARNs. However, the plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the Amazon Web Services General Reference.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    /// <p>Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn get_policy_arns(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PolicyDescriptorType>> {
        &self.policy_arns
    }
    /// <p>An IAM policy in JSON format that you want to use as an inline session policy.</p>
    /// <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>. </p>
    /// <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An IAM policy in JSON format that you want to use as an inline session policy.</p>
    /// <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>. </p>
    /// <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy = input;
        self
    }
    /// <p>An IAM policy in JSON format that you want to use as an inline session policy.</p>
    /// <p>This parameter is optional. Passing policies to this operation returns new temporary credentials. The resulting session's permissions are the intersection of the role's identity-based policy and the session policies. You can use the role's temporary credentials in subsequent Amazon Web Services API calls to access resources in the account that owns the role. You cannot use session policies to grant more permissions than those allowed by the identity-based policy of the role that is being assumed. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#policies_session">Session Policies</a> in the <i>IAM User Guide</i>. </p>
    /// <p>The plaintext that you use for both inline and managed session policies can't exceed 2,048 characters. The JSON policy characters can be any ASCII character from the space character to the end of the valid character list (\u0020 through \u00FF). It can also include the tab (\u0009), linefeed (\u000A), and carriage return (\u000D) characters.</p> <note>
    /// <p>An Amazon Web Services conversion compresses the passed inline session policy, managed policy ARNs, and session tags into a packed binary format that has a separate limit. Your request can fail for this limit even if your plaintext meets the other requirements. The <code>PackedPolicySize</code> response element indicates by percentage how close the policies and tags for your request are to the upper size limit.</p>
    /// </note>
    pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy
    }
    /// <p>The duration, in seconds, of the role session. Your role session lasts for the duration that you specify for the <code>DurationSeconds</code> parameter, or until the time specified in the SAML authentication response's <code>SessionNotOnOrAfter</code> value, whichever is shorter. You can provide a <code>DurationSeconds</code> value from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p>
    /// <p>By default, the value is set to <code>3600</code> seconds. </p> <note>
    /// <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub fn duration_seconds(mut self, input: i32) -> Self {
        self.duration_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>The duration, in seconds, of the role session. Your role session lasts for the duration that you specify for the <code>DurationSeconds</code> parameter, or until the time specified in the SAML authentication response's <code>SessionNotOnOrAfter</code> value, whichever is shorter. You can provide a <code>DurationSeconds</code> value from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p>
    /// <p>By default, the value is set to <code>3600</code> seconds. </p> <note>
    /// <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub fn set_duration_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.duration_seconds = input;
        self
    }
    /// <p>The duration, in seconds, of the role session. Your role session lasts for the duration that you specify for the <code>DurationSeconds</code> parameter, or until the time specified in the SAML authentication response's <code>SessionNotOnOrAfter</code> value, whichever is shorter. You can provide a <code>DurationSeconds</code> value from 900 seconds (15 minutes) up to the maximum session duration setting for the role. This setting can have a value from 1 hour to 12 hours. If you specify a value higher than this setting, the operation fails. For example, if you specify a session duration of 12 hours, but your administrator set the maximum session duration to 6 hours, your operation fails. To learn how to view the maximum value for your role, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_use.html#id_roles_use_view-role-max-session">View the Maximum Session Duration Setting for a Role</a> in the <i>IAM User Guide</i>.</p>
    /// <p>By default, the value is set to <code>3600</code> seconds. </p> <note>
    /// <p>The <code>DurationSeconds</code> parameter is separate from the duration of a console session that you might request using the returned credentials. The request to the federation endpoint for a console sign-in token takes a <code>SessionDuration</code> parameter that specifies the maximum length of the console session. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_providers_enable-console-custom-url.html">Creating a URL that Enables Federated Users to Access the Amazon Web Services Management Console</a> in the <i>IAM User Guide</i>.</p>
    /// </note>
    pub fn get_duration_seconds(&self) -> &::std::option::Option<i32> {
        &self.duration_seconds
    }
    /// Consumes the builder and constructs a [`AssumeRoleWithSamlInput`](crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput, ::aws_smithy_http::operation::error::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::assume_role_with_saml::AssumeRoleWithSamlInput {
            role_arn: self.role_arn,
            principal_arn: self.principal_arn,
            saml_assertion: self.saml_assertion,
            policy_arns: self.policy_arns,
            policy: self.policy,
            duration_seconds: self.duration_seconds,
        })
    }
}
impl ::std::fmt::Debug for AssumeRoleWithSamlInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AssumeRoleWithSamlInputBuilder");
        formatter.field("role_arn", &self.role_arn);
        formatter.field("principal_arn", &self.principal_arn);
        formatter.field("saml_assertion", &"*** Sensitive Data Redacted ***");
        formatter.field("policy_arns", &self.policy_arns);
        formatter.field("policy", &self.policy);
        formatter.field("duration_seconds", &self.duration_seconds);
        formatter.finish()
    }
}
