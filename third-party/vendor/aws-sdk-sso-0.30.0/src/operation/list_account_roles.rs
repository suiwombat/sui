// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl ListAccountRolesInput {}
/// Orchestration and serialization glue logic for `ListAccountRoles`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct ListAccountRoles;
impl ListAccountRoles {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::list_account_roles::ListAccountRolesInput,
    ) -> ::std::result::Result<
        crate::operation::list_account_roles::ListAccountRolesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::list_account_roles::ListAccountRolesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::list_account_roles::ListAccountRolesError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::list_account_roles::ListAccountRolesOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::list_account_roles::ListAccountRolesInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point("sso", "ListAccountRoles", input, runtime_plugins, stop_point).await
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for ListAccountRoles {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("ListAccountRoles");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            ListAccountRolesRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            ListAccountRolesResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new("ListAccountRoles", "sso"));

        ::std::option::Option::Some(cfg.freeze())
    }

    fn runtime_components(&self) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
        // Retry classifiers are operation-specific because they need to downcast operation-specific error types.
        let retry_classifiers = ::aws_smithy_runtime_api::client::retries::RetryClassifiers::new()
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::SmithyErrorClassifier::<
                crate::operation::list_account_roles::ListAccountRolesError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AmzRetryAfterHeaderClassifier)
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::ModeledAsRetryableClassifier::<
                crate::operation::list_account_roles::ListAccountRolesError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AwsErrorCodeClassifier::<
                crate::operation::list_account_roles::ListAccountRolesError,
            >::new())
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::HttpStatusCodeClassifier::default());

        ::std::borrow::Cow::Owned(
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("ListAccountRoles")
                .with_retry_classifiers(::std::option::Option::Some(retry_classifiers))
                .with_auth_scheme_option_resolver(::std::option::Option::Some(
                    ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(
                        ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver::new(vec![
                            ::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID,
                        ]),
                    ),
                ))
                .with_interceptor(
                    ::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(ListAccountRolesEndpointParamsInterceptor) as _,
                ),
        )
    }
}

#[derive(Debug)]
struct ListAccountRolesResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::ResponseDeserializer for ListAccountRolesResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 {
            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_list_account_roles::de_list_account_roles_http_response(status, headers, body)
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct ListAccountRolesRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::RequestSerializer for ListAccountRolesRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::list_account_roles::ListAccountRolesInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::list_account_roles::ListAccountRolesInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/assignment/roles").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::list_account_roles::ListAccountRolesInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                if let ::std::option::Option::Some(inner_1) = &_input.next_token {
                    {
                        query.push_kv("next_token", &::aws_smithy_http::query::fmt_string(&inner_1));
                    }
                }
                if let ::std::option::Option::Some(inner_2) = &_input.max_results {
                    if *inner_2 != 0 {
                        query.push_kv("max_result", ::aws_smithy_types::primitive::Encoder::from(*inner_2).encode());
                    }
                }
                let inner_3 = &_input.account_id;
                let inner_3 = inner_3
                    .as_ref()
                    .ok_or_else(|| ::aws_smithy_http::operation::error::BuildError::missing_field("account_id", "cannot be empty or unset"))?;
                if inner_3.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_http::operation::error::BuildError::missing_field(
                        "account_id",
                        "cannot be empty or unset",
                    ));
                }
                query.push_kv("account_id", &::aws_smithy_http::query::fmt_string(&inner_3));
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::list_account_roles::ListAccountRolesInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder = crate::protocol_serde::shape_list_account_roles::ser_list_account_roles_headers(input, builder)?;
                ::std::result::Result::Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder
        };
        let body = ::aws_smithy_http::body::SdkBody::from("");

        ::std::result::Result::Ok(request_builder.body(body).expect("valid request"))
    }
}
#[derive(Debug)]
struct ListAccountRolesEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Interceptor for ListAccountRolesEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "ListAccountRolesEndpointParamsInterceptor"
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
            .downcast_ref::<ListAccountRolesInput>()
            .ok_or("failed to downcast to ListAccountRolesInput")?;

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
pub type ListAccountRolesErrorKind = ListAccountRolesError;
/// Error type for the `ListAccountRolesError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum ListAccountRolesError {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequestException(crate::types::error::InvalidRequestException),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequestsException(crate::types::error::TooManyRequestsException),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    UnauthorizedException(crate::types::error::UnauthorizedException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for ListAccountRolesError {
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
impl ::std::fmt::Display for ListAccountRolesError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::InvalidRequestException(_inner) => _inner.fmt(f),
            Self::ResourceNotFoundException(_inner) => _inner.fmt(f),
            Self::TooManyRequestsException(_inner) => _inner.fmt(f),
            Self::UnauthorizedException(_inner) => _inner.fmt(f),
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ListAccountRolesError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::InvalidRequestException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::ResourceNotFoundException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::TooManyRequestsException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::UnauthorizedException(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
            Self::Unhandled(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
        }
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::list_account_roles::ListAccountRolesError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for ListAccountRolesError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ListAccountRolesError {
    /// Creates the `ListAccountRolesError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err).build())
    }

    /// Creates the `ListAccountRolesError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::InvalidRequestException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TooManyRequestsException(e) => e.meta(),
            Self::UnauthorizedException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
        }
    }
    /// Returns `true` if the error kind is `ListAccountRolesError::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(self, Self::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `ListAccountRolesError::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(self, Self::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListAccountRolesError::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(self, Self::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `ListAccountRolesError::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(self, Self::UnauthorizedException(_))
    }
}
impl ::std::error::Error for ListAccountRolesError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::InvalidRequestException(_inner) => ::std::option::Option::Some(_inner),
            Self::ResourceNotFoundException(_inner) => ::std::option::Option::Some(_inner),
            Self::TooManyRequestsException(_inner) => ::std::option::Option::Some(_inner),
            Self::UnauthorizedException(_inner) => ::std::option::Option::Some(_inner),
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::list_account_roles::_list_account_roles_output::ListAccountRolesOutput;

pub use crate::operation::list_account_roles::_list_account_roles_input::ListAccountRolesInput;

mod _list_account_roles_input;

mod _list_account_roles_output;

/// Builders
pub mod builders;

/// Paginator for this operation
pub mod paginator;
