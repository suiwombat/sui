_oci_image_toolchain_attrs = {
    # builder is the binary we expect to invoke to create an image for us
    "builder": provider_field(typing.Any, default=""),
    # "oneshot": provider_field(typing.Any, default=False),
    # layers is an buck layer target that we use to compose images together.
    # ideally, we want buck to optimize build layers for us so we don't rebuild
    # the world each time.  So, prefer artifact composition that will take this
    # into account.
    "layers": provider_field(list[typing.Any], default=[]),
}
OCIImageToolchainInfo = provider(
    doc="OCIImage toolchain info",
    fields=_oci_image_toolchain_attrs.keys(),
)


def oci_image_toolchain_info(ctx: AnalysisContext) -> OCIImageToolchainInfo:
    attrs = dict()
    for k, default in _oci_image_toolchain_attrs.items():
        v = getattr(ctx.attrs, k)
        attrs[k] = default if v == None else v

    return OCIImageToolchainInfo(**attrs)
