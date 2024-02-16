load("@prelude//python:toolchain.bzl", "PythonToolchainInfo")
load("//toolchains/podman.bzl", "PodmanToolchainInfo")
load("//toolchains/mypkg.bzl", "MyPkgToolchainInfo")


def _host_arch() -> str:
    arch = host_info().arch
    if arch.is_x86_64:
        return "x86_64"
    elif host_info().arch.is_aarch64:
        return "arm64"
    else:
        fail("Unsupported host architecture.")


def _host_os() -> str:
    os = host_info().os
    if os.is_linux:
        return "Linux"
    elif os.is_macos:
        return "Darwin"
    elif os.is_windows:
        return "Windows"
    else:
        fail("Unsupported host os.")


def _get_mypkg_impl(ctx: AnalysisContext):
    dst = ctx.actions.declare_output(ctx.attrs.bin)
    mypkg = ctx.attrs._mypkg_toolchain[MyPkgToolchainInfo].bin
    ctx.actions.run(
        [mypkg, cmd_args("fetch", ctx.attrs.build, "-o", dst.as_output())],
        category="mypkg_fetch",
    )

    return [
        DefaultInfo(default_output=dst),
        MypkgInfo(
            build=ctx.attrs.build,
            version=ctx.attrs.version,
            arch=ctx.attrs.arch,
            os=ctx.attrs.os,
        ),
    ]


def get_mypkg(name: str, build: str, arch: [None, str] = None, os: [None, str] = None):
    if arch == None:
        arch = _host_arch()
    if os == None:
        os = _host_os()

    (mypkg_name, mypkg_version) = build.split(":")
    mypkg_artifact(
        name=name,
        build=build,
        version=mypkg_version,
        arch=arch,
        os=os,
        bin="{}_{}_{}".format(mypkg_name, os, arch).lower(),
    )


mypkg_artifact = rule(
    impl=_get_mypkg_impl,
    attrs={
        "build": attrs.string(),
        "version": attrs.string(),
        "arch": attrs.string(default="aarch64"),
        "os": attrs.string(default="macos"),
        "bin": attrs.string(),
        "_mypkg_toolchain": attrs.toolchain_dep(
            default="toolchains//:mypkg", providers=[MyPkgToolchainInfo]
        ),
    },
)

MypkgInfo = provider(
    fields={
        "build": provider_field(typing.Any, default=None),
        "version": provider_field(typing.Any, default=None),
        "arch": provider_field(typing.Any, default=None),
        "os": provider_field(typing.Any, default=None),
    }
)
