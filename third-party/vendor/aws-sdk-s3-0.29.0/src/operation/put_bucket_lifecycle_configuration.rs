// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

impl PutBucketLifecycleConfigurationInput {}
/// Orchestration and serialization glue logic for `PutBucketLifecycleConfiguration`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
#[doc(hidden)]
pub struct PutBucketLifecycleConfiguration;
impl PutBucketLifecycleConfiguration {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput,
    ) -> ::std::result::Result<
        crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let map_err = |err: ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >| {
            err.map_service_error(|err| {
                err.downcast::<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError>()
                    .expect("correct error type")
            })
        };
        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
            .await
            .map_err(map_err)?;
        let output = context.finalize().map_err(map_err)?;
        ::std::result::Result::Ok(
            output
                .downcast::<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput>()
                .expect("correct output type"),
        )
    }

    pub(crate) async fn orchestrate_with_stop_point(
        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
        input: crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput,
        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
    ) -> ::std::result::Result<
        ::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext,
        ::aws_smithy_http::result::SdkError<
            ::aws_smithy_runtime_api::client::interceptors::context::Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
            "s3",
            "PutBucketLifecycleConfiguration",
            input,
            runtime_plugins,
            stop_point,
        )
        .await
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
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for PutBucketLifecycleConfiguration {
    fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
        let mut cfg = ::aws_smithy_types::config_bag::Layer::new("PutBucketLifecycleConfiguration");

        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(
            PutBucketLifecycleConfigurationRequestSerializer,
        ));
        cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(
            PutBucketLifecycleConfigurationResponseDeserializer,
        ));

        cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(
            ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new(),
        ));

        cfg.store_put(::aws_smithy_http::operation::Metadata::new("PutBucketLifecycleConfiguration", "s3"));
        let mut signing_options = ::aws_runtime::auth::sigv4::SigningOptions::default();
        signing_options.double_uri_encode = false;
        signing_options.content_sha256_header = true;
        signing_options.normalize_uri_path = false;
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
                crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AmzRetryAfterHeaderClassifier)
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::ModeledAsRetryableClassifier::<
                crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
            >::new())
            .with_classifier(::aws_runtime::retries::classifier::AwsErrorCodeClassifier::<
                crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
            >::new())
            .with_classifier(::aws_smithy_runtime::client::retries::classifier::HttpStatusCodeClassifier::default());

        ::std::borrow::Cow::Owned(
            ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("PutBucketLifecycleConfiguration")
                .with_retry_classifiers(::std::option::Option::Some(retry_classifiers))
                .with_auth_scheme_option_resolver(::std::option::Option::Some(
                    ::aws_smithy_runtime_api::client::auth::SharedAuthSchemeOptionResolver::new(
                        ::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolver::new(vec![
                            ::aws_runtime::auth::sigv4::SCHEME_ID,
                        ]),
                    ),
                ))
                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
                    PutBucketLifecycleConfigurationEndpointParamsInterceptor,
                ) as _)
                .with_interceptor(::aws_smithy_runtime_api::client::interceptors::SharedInterceptor::new(
                    crate::http_request_checksum::RequestChecksumInterceptor::new(
                        |input: &::aws_smithy_runtime_api::client::interceptors::context::Input| {
                            let input: &crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput =
                                input.downcast_ref().expect("correct type");
                            let checksum_algorithm = input.checksum_algorithm();
                            let checksum_algorithm = checksum_algorithm.map(|algorithm| algorithm.as_str()).or(Some("md5"));
                            let checksum_algorithm = match checksum_algorithm {
                                Some(algo) => Some(
                                    algo.parse::<::aws_smithy_checksums::ChecksumAlgorithm>()
                                        .map_err(::aws_smithy_http::operation::error::BuildError::other)?,
                                ),
                                None => None,
                            };
                            ::std::result::Result::<_, ::aws_smithy_runtime_api::box_error::BoxError>::Ok(checksum_algorithm)
                        },
                    ),
                ) as _),
        )
    }
}

#[derive(Debug)]
struct PutBucketLifecycleConfigurationResponseDeserializer;
impl ::aws_smithy_runtime_api::client::ser_de::ResponseDeserializer for PutBucketLifecycleConfigurationResponseDeserializer {
    fn deserialize_nonstreaming(
        &self,
        response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    ) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
        let (success, status) = (response.status().is_success(), response.status().as_u16());
        let headers = response.headers();
        let body = response.body().bytes().expect("body loaded");
        ::tracing::debug!(extended_request_id = ?crate::s3_request_id::RequestIdExt::extended_request_id(response));
        ::tracing::debug!(request_id = ?::aws_http::request_id::RequestId::request_id(response));
        let parse_result = if !success && status != 200 {
            crate::protocol_serde::shape_put_bucket_lifecycle_configuration::de_put_bucket_lifecycle_configuration_http_error(status, headers, body)
        } else {
            crate::protocol_serde::shape_put_bucket_lifecycle_configuration::de_put_bucket_lifecycle_configuration_http_response(
                status, headers, body,
            )
        };
        crate::protocol_serde::type_erase_result(parse_result)
    }
}
#[derive(Debug)]
struct PutBucketLifecycleConfigurationRequestSerializer;
impl ::aws_smithy_runtime_api::client::ser_de::RequestSerializer for PutBucketLifecycleConfigurationRequestSerializer {
    #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
    fn serialize_input(
        &self,
        input: ::aws_smithy_runtime_api::client::interceptors::context::Input,
        _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
        let input = input
            .downcast::<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput>()
            .expect("correct type");
        let _header_serialization_settings = _cfg
            .load::<crate::serialization_settings::HeaderSerializationSettings>()
            .cloned()
            .unwrap_or_default();
        let mut request_builder = {
            fn uri_base(
                _input: &crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput,
                output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                use ::std::fmt::Write as _;
                ::std::write!(output, "/").expect("formatting should succeed");
                ::std::result::Result::Ok(())
            }
            fn uri_query(
                _input: &crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput,
                mut output: &mut ::std::string::String,
            ) -> ::std::result::Result<(), ::aws_smithy_http::operation::error::BuildError> {
                let mut query = ::aws_smithy_http::query::Writer::new(output);
                query.push_v("lifecycle");
                ::std::result::Result::Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationInput,
                builder: ::http::request::Builder,
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_http::operation::error::BuildError> {
                let mut uri = ::std::string::String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                let builder =
                    crate::protocol_serde::shape_put_bucket_lifecycle_configuration::ser_put_bucket_lifecycle_configuration_headers(input, builder)?;
                ::std::result::Result::Ok(builder.method("PUT").uri(uri))
            }
            let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
            builder = _header_serialization_settings.set_default_header(builder, ::http::header::CONTENT_TYPE, "application/xml");
            builder
        };
        let body = ::aws_smithy_http::body::SdkBody::from(
            crate::protocol_serde::shape_put_bucket_lifecycle_configuration_input::ser_lifecycle_configuration_http_payload(
                &input.lifecycle_configuration,
            )?,
        );
        if let Some(content_length) = body.content_length() {
            let content_length = content_length.to_string();
            request_builder = _header_serialization_settings.set_default_header(request_builder, ::http::header::CONTENT_LENGTH, &content_length);
        }
        ::std::result::Result::Ok(request_builder.body(body).expect("valid request"))
    }
}
#[derive(Debug)]
struct PutBucketLifecycleConfigurationEndpointParamsInterceptor;

impl ::aws_smithy_runtime_api::client::interceptors::Interceptor for PutBucketLifecycleConfigurationEndpointParamsInterceptor {
    fn name(&self) -> &'static str {
        "PutBucketLifecycleConfigurationEndpointParamsInterceptor"
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
            .downcast_ref::<PutBucketLifecycleConfigurationInput>()
            .ok_or("failed to downcast to PutBucketLifecycleConfigurationInput")?;

        let params = crate::config::endpoint::Params::builder()
            .set_region(cfg.load::<::aws_types::region::Region>().map(|r| r.as_ref().to_owned()))
            .set_use_fips(cfg.load::<::aws_types::endpoint_config::UseFips>().map(|ty| ty.0))
            .set_use_dual_stack(cfg.load::<::aws_types::endpoint_config::UseDualStack>().map(|ty| ty.0))
            .set_endpoint(cfg.load::<::aws_types::endpoint_config::EndpointUrl>().map(|ty| ty.0.clone()))
            .set_force_path_style(cfg.load::<crate::config::ForcePathStyle>().map(|ty| ty.0))
            .set_use_arn_region(cfg.load::<crate::config::UseArnRegion>().map(|ty| ty.0))
            .set_disable_multi_region_access_points(cfg.load::<crate::config::DisableMultiRegionAccessPoints>().map(|ty| ty.0))
            .set_accelerate(cfg.load::<crate::config::Accelerate>().map(|ty| ty.0))
            .set_bucket(_input.bucket.clone())
            .build()
            .map_err(|err| {
                ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err)
            })?;
        cfg.interceptor_state()
            .store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
        ::std::result::Result::Ok(())
    }
}
#[allow(unreachable_code, unused_variables)]
#[cfg(test)]
mod put_bucket_lifecycle_configuration_request_test {
    /// This test validates that the content md5 header is set correctly
    /// Test ID: PutBucketLifecycleConfiguration
    #[::tokio::test]
    #[allow(unused_mut)]
    async fn put_bucket_lifecycle_configuration_request() {
        let (conn, request_receiver) = ::aws_smithy_client::test_connection::capture_request(None);
        let config_builder = crate::config::Config::builder()
            .with_test_defaults()
            .endpoint_resolver("https://example.com");
        let config_builder = config_builder.region(::aws_types::region::Region::new("us-east-1"));
        let mut config_builder = config_builder;
        config_builder.set_region(Some(crate::config::Region::new("us-east-1")));
        let config = config_builder.http_connector(conn).build();
        let client = crate::Client::from_conf(config);
        let result = client
            .put_bucket_lifecycle_configuration()
            .set_bucket(::std::option::Option::Some("test-bucket".to_owned()))
            .set_lifecycle_configuration(::std::option::Option::Some(
                crate::types::BucketLifecycleConfiguration::builder()
                    .set_rules(::std::option::Option::Some(vec![crate::types::LifecycleRule::builder()
                        .set_expiration(::std::option::Option::Some(
                            crate::types::LifecycleExpiration::builder()
                                .set_days(::std::option::Option::Some(1))
                                .build(),
                        ))
                        .set_status(::std::option::Option::Some(crate::types::ExpirationStatus::from("Enabled")))
                        .set_id(::std::option::Option::Some("Expire".to_owned()))
                        .build()]))
                    .build(),
            ))
            .send()
            .await;
        let _ = dbg!(result);
        let http_request = request_receiver.expect_request();
        ::pretty_assertions::assert_eq!(http_request.method(), "PUT");
        ::pretty_assertions::assert_eq!(http_request.uri().path(), "/");
        let expected_headers = [("content-md5", "JP8DTuCSH6yDC8wNGg4+mA==")];
        ::aws_smithy_protocol_test::assert_ok(::aws_smithy_protocol_test::validate_headers(http_request.headers(), expected_headers));
        let body = http_request.body().bytes().expect("body should be strict");
        ::aws_smithy_protocol_test::assert_ok(
        ::aws_smithy_protocol_test::validate_body(&body, "<LifecycleConfiguration xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">\n    <Rule>\n        <Expiration>\n            <Days>1</Days>\n        </Expiration>\n        <ID>Expire</ID>\n        <Status>Enabled</Status>\n    </Rule>\n</LifecycleConfiguration>\n", ::aws_smithy_protocol_test::MediaType::from("application/xml"))
        );
    }
}

/// Do not use this.
///
/// Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now).
#[deprecated(
    note = "Operation `*Error/*ErrorKind` types were combined into a single `*Error` enum. The `.kind` field on `*Error` no longer exists and isn't needed anymore (you can just match on the error directly since it's an enum now)."
)]
pub type PutBucketLifecycleConfigurationErrorKind = PutBucketLifecycleConfigurationError;
/// Error type for the `PutBucketLifecycleConfigurationError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum PutBucketLifecycleConfigurationError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    Unhandled(::aws_smithy_types::error::Unhandled),
}
impl ::aws_smithy_http::result::CreateUnhandledError for PutBucketLifecycleConfigurationError {
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
impl ::std::fmt::Display for PutBucketLifecycleConfigurationError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for PutBucketLifecycleConfigurationError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(_inner),
        }
    }
}
impl crate::s3_request_id::RequestIdExt for crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError {
    fn extended_request_id(&self) -> Option<&str> {
        self.meta().extended_request_id()
    }
}
impl ::aws_http::request_id::RequestId for crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError {
    fn request_id(&self) -> Option<&str> {
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for PutBucketLifecycleConfigurationError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl PutBucketLifecycleConfigurationError {
    /// Creates the `PutBucketLifecycleConfigurationError::Unhandled` variant from any error type.
    pub fn unhandled(
        err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>,
    ) -> Self {
        Self::Unhandled(::aws_smithy_types::error::Unhandled::builder().source(err).build())
    }

    /// Creates the `PutBucketLifecycleConfigurationError::Unhandled` variant from a `::aws_smithy_types::error::ErrorMetadata`.
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
            Self::Unhandled(e) => e.meta(),
        }
    }
}
impl ::std::error::Error for PutBucketLifecycleConfigurationError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => ::std::option::Option::Some(_inner),
        }
    }
}

pub use crate::operation::put_bucket_lifecycle_configuration::_put_bucket_lifecycle_configuration_output::PutBucketLifecycleConfigurationOutput;

pub use crate::operation::put_bucket_lifecycle_configuration::_put_bucket_lifecycle_configuration_input::PutBucketLifecycleConfigurationInput;

mod _put_bucket_lifecycle_configuration_input;

mod _put_bucket_lifecycle_configuration_output;

/// Builders
pub mod builders;
