OciToolChainInfo = provider(
    fields={
        builder: attrs.string(),
    }
)


def _oci_toolchain_impl(
    ctx: AnalysisContext,
) -> typing.List[[DefaultInfo, OciToolchainInfo]]:
    return [
        DefaultInfo(),
        OciToolchainInfo(
            builder=ctx.attrs.builder,
        ),
    ]


system_oci_toolchain = rule(
    impl=_oci_toolchain_impl,
    attrs={
        "builder": attrs.exec_dep(providers=[RunInfo]),
    },
)
