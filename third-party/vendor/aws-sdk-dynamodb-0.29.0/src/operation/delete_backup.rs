// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl DeleteBackupInput {}
/// Orchestration and serialization glue logic for `DeleteBackup`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct DeleteBackup;
impl DeleteBackup {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::delete_backup::DeleteBackupInput,
    ) -> ::std::result::Result<
        crate::operation::delete_backup::DeleteBackupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_backup::DeleteBackupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::delete_backup::DeleteBackupError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::delete_backup::DeleteBackupOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::delete_backup::DeleteBackupInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("dynamodb", "DeleteBackup", input, runtime_plugins, stop_point).await
    }

    pub(crate) fn operation_runtime_plugins(
        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        client_config: &crate::config::Config,
        config_override: ::std::option::Option<crate::config::Builder>,
    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());

        if let ::std::option::Option::Some(config_override) = config_override {
            for plugin in config_override.runtime_plugins.iter().cloned() {
                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
            }
            runtime_plugins = runtime_plugins.with_operation_plugin(crate::config::ConfigOverrideRuntimePlugin::new(
                config_override,
                client_config.config.clone(),
                &client_config.runtime_components,
            ));
        }
        runtime_plugins
    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for DeleteBackup {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("DeleteBackup");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            DeleteBackupRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            DeleteBackupResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new("DeleteBackup", "dynamodb"));
        let mut signing_options = ::aws_runtime::auth::sigv4::SigningOptions::default();
        signing_options.double_uri_encode = true;
        signing_options.content_sha256_header = false;
        signing_options.normalize_uri_path = true;
        signing_options.payload_override = None;

        cfg.store_put(::aws_runtime::auth::sigv4::SigV4OperationSigningConfig {
            region: None,
            service: None,
            signing_options,
        });

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(&self) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        // Retry classifiers are operation-specific because they need to downcast operation-specific error types.
        let retry_classifiers = ::aws_smithy_runtime_api::client::retries::RetryClassifiers::new()
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::SmithyErrorClassifier::<
                crate::operation::delete_backup::DeleteBackupError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AmzRetryAfterHeaderClassifier)
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::ModeledAsRetryableClassifier::<
                crate::operation::delete_backup::DeleteBackupError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AwsErrorCodeClassifier::<
                crate::operation::delete_backup::DeleteBackupError,
            >::new())
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::HttpStatusCodeClassifier::default());

        ::std::borrow::Cow::Owned(
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("DeleteBackup")
                .with_retry_classifiers(::std::option::Option::Some(retry_classifiers))
                .with_auth_scheme_option_resolver(::std::option::Option::Some(
                    ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(
                        ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver::new(vec![
                            ::aws_runtime::auth::sigv4::SCHEME_ID,
                        ]),
                    ),
                ))
                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(DeleteBackupEndpointParamsInterceptor) as _),
        )
    }
}

#[derive(Debug)]
struct DeleteBackupResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::ResponseDeserializer for DeleteBackupResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 {
            crate::protocol_serde::shape_delete_backup::de_delete_backup_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_delete_backup::de_delete_backup_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct DeleteBackupRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::RequestSerializer for DeleteBackupRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::delete_backup::DeleteBackupInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::delete_backup::DeleteBackupInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::delete_backup::DeleteBackupInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                ::std::result::Result::Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/x-amz-json-1.0");
            builder = _header_serialization_settings.set_default_header(
                builder,
                ::http::header::HeaderName::from_static("x-amz-target"),
                "DynamoDB_20120810.DeleteBackup",
            );
            builder
        };
        let body = ::aws_smithy_http::body::SdkBody::from(crate::protocol_serde::shape_delete_backup::ser_delete_backup_input(&input)?);
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request"))
    }
}
#[derive(Debug)]
struct DeleteBackupEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Interceptor for DeleteBackupEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "DeleteBackupEndpointParamsInterceptor"
    }

    fn read_before_execution(
        &self,
        context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<
            '_,
            ::aws_smithy_runtime_api::client::interceptors::context::Input,
            ::aws_smithy_runtime_api::client::interceptors::context::Output,
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
        >,
        cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let _input = context
            .input()
            .downcast_ref::<DeleteBackupInput>()
            .ok_or("failed to downcast to DeleteBackupInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type DeleteBackupErrorKind = DeleteBackupError;
/// Error type for the `DeleteBackupError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum DeleteBackupError {
    /// <p>There is another ongoing conflicting backup control plane operation on the table. The backup is either being created, deleted or restored to a table.</p>
    BackupInUseException(crate::types::error::BackupInUseException),
    /// <p>Backup not found for the given BackupARN. </p>
    BackupNotFoundException(crate::types::error::BackupNotFoundException),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(crate::types::error::InternalServerError),
    #[allow(missing_docs)] // documentation missing in model
    InvalidEndpointException(crate::types::error::InvalidEndpointException),
    /// <p>There is no limit to the number of daily on-demand backups that can be taken. </p>
    /// <p>For most purposes, up to 500 simultaneous table operations are allowed per account. These operations include <code>CreateTable</code>, <code>UpdateTable</code>, <code>DeleteTable</code>,<code>UpdateTimeToLive</code>, <code>RestoreTableFromBackup</code>, and <code>RestoreTableToPointInTime</code>. </p>
    /// <p>When you are creating a table with one or more secondary indexes, you can have up to 250 such requests running at a time. However, if the table or index specifications are complex, then DynamoDB might temporarily reduce the number of concurrent operations.</p>
    /// <p>When importing into DynamoDB, up to 50 simultaneous import table operations are allowed per account.</p>
    /// <p>There is a soft account quota of 2,500 tables.</p>
    /// <p>GetRecords was called with a value of more than 1000 for the limit request parameter.</p>
    /// <p>More than 2 processes are reading from the same streams shard at the same time. Exceeding this limit may result in request throttling.</p>
    LimitExceededException(crate::types::error::LimitExceededException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for DeleteBackupError {
    fn create_unhandled_error(
        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> Self {
        Self::Unhandled({
            let mut builder = ::aws_smithy_types::error::Unhandled::builder().source(source);
            builder.set_meta(meta);
            builder.build()
        })
    }
}
impl ::std::fmt::Display for DeleteBackupError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::BackupInUseException(_inner) => _inner.fmt(f),
            Self::BackupNotFoundException(_inner) => _inner.fmt(f),
            Self::InternalServerError(_inner) => _inner.fmt(f),
            Self::InvalidEndpointException(_inner) => _inner.fmt(f),
            Self::LimitExceededException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for DeleteBackupError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::BackupInUseException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::BackupNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InternalServerError(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::InvalidEndpointException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::LimitExceededException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::delete_backup::DeleteBackupError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for DeleteBackupError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl DeleteBackupError {
    /// Creates the `DeleteBackupError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err).build())
    }

    /// Creates the `DeleteBackupError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err.clone()).meta(err).build())
    }
    ///
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    ///
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        match self {
            Self::BackupInUseException(e) => e.meta(),
            Self::BackupNotFoundException(e) => e.meta(),
            Self::InternalServerError(e) => e.meta(),
            Self::InvalidEndpointException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `DeleteBackupError::BackupInUseException`.
    pub fn is_backup_in_use_exception(&self) -> bool {
        matches!(self, Self::BackupInUseException(_))
    }
    /// Returns `true` if the error kind is `DeleteBackupError::BackupNotFoundException`.
    pub fn is_backup_not_found_exception(&self) -> bool {
        matches!(self, Self::BackupNotFoundException(_))
    }
    /// Returns `true` if the error kind is `DeleteBackupError::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(self, Self::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `DeleteBackupError::InvalidEndpointException`.
    pub fn is_invalid_endpoint_exception(&self) -> bool {
        matches!(self, Self::InvalidEndpointException(_))
    }
    /// Returns `true` if the error kind is `DeleteBackupError::LimitExceededException`.
    pub fn is_limit_exceeded_exception(&self) -> bool {
        matches!(self, Self::LimitExceededException(_))
    }
}
impl ::std::error::Error for DeleteBackupError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::BackupInUseException(_inner) => ::std::option::Option::Some(_inner),
            Self::BackupNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::InternalServerError(_inner) => ::std::option::Option::Some(_inner),
            Self::InvalidEndpointException(_inner) => ::std::option::Option::Some(_inner),
            Self::LimitExceededException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::delete_backup::_delete_backup_output::DeleteBackupOutput;

pub use crate::operation::delete_backup::_delete_backup_input::DeleteBackupInput;

mod _delete_backup_input;

mod _delete_backup_output;

/// Builders
pub mod builders;
