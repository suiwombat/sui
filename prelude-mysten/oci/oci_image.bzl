load("@prelude//decls/toolchains_common.bzl", "toolchains_common")
load(":oci_image_toolchain.bzl", "OCIImageToolchainInfo", "oci_image_toolchain_info")


def oci_image_impl(ctx: AnalysisContext) -> list[Provider]:
    pprint(ctx.attrs)
    # layers = ctx.attrs._layers[OCIImageToolchainInfo]
    # pprint(layers)

    # deps = []
    # for dep in ctx.attrs.deps:
    #     pprint(dep[DefaultInfo])
    #     info = dep[DefaultInfo]
    #     deps.extend(info.default_outputs)
    toolchain_info = oci_image_toolchain_info(ctx)
    pprint(toolchain_info)
    out = ctx.actions.declare_output("build.json")
    pprint(ctx.attrs.layers)
    # pprint([x[OCIImageToolchainInfo] for x in ctx.attrs.layers])
    cmd = ["touch", out.as_output()]
    # cmd = ["/opt/homebrew/bin/python3", "joeman.py", out.as_output()]
    ctx.actions.run(cmd, category="oci_build")
    # outs = []
    # for i, dep in enumerate(deps):
    #     pprint(dir(dep))
    #     out = ctx.actions.declare_output("out_%s" % i)
    #     art = ctx.actions.copy_file(out, dep)
    #     # pprint(art)
    #     outs.append(out)
    #     # outs.append(art)

    return [
        DefaultInfo(default_output=out),
        # RunInfo(args = run_info.args)
    ]


oci_image = rule(
    impl=oci_image_impl,
    attrs={
        # "deps": attrs.list(attrs.dep()),
        # "out": attrs.string(default=""),
        "builder": attrs.string(default="podman"),
        # "src": attrs.source(),
        "layers": attrs.list(attrs.string(), default=[]),
        # "layers": attrs.list(attrs.dep(providers=[OCIImageToolchainInfo])),
        # "dep": attrs.dep(required_providers=[OCIImageToolchainInfo])
    },
)

# attrs.default_only(attrs.toolchain_dep(default = "toolchains//:" + lang, providers = providers))

# OCILayerProvider
# MyPkg = provider(
#     doc = "asdfsfd",
#     fields = {
#         "oci_builder": provider_field(typing.Any, default = None),
#     },
# )
