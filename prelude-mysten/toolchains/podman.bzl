PodmanToolchainInfo = provider(
    fields={
        "builder": provider_field(typing.Any, default=""),
        "builder_script": provider_field(typing.Any, default=""),
    }
)


def _podman_toolchain_impl(ctx: AnalysisContext):
    return [
        DefaultInfo(),
        PodmanToolchainInfo(
            builder=ctx.attrs.builder,
            builder_script=ctx.attrs._builder_script,
        ),
    ]


system_podman_toolchain = rule(
    impl=_podman_toolchain_impl,
    attrs={
        "builder": attrs.string(),
        "_builder_script": attrs.dep(default="prelude-mysten//podman:build"),
    },
    is_toolchain_rule=True,
)
