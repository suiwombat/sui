load("@prelude//python:toolchain.bzl", "PythonToolchainInfo")
load("//toolchains/buildah.bzl", "BuildahToolchainInfo")
load("//mypkg:mypkg.bzl", "MypkgInfo")


def _buildah_image_impl(
    ctx: AnalysisContext,
):
    docker_root = ctx.actions.declare_output("docker_root", dir=True)
    deps = {
        "Dockerfile": ctx.attrs.dockerfile,
    }
    for layer in ctx.attrs.layers:
        for dep in layer[DefaultInfo].default_outputs:
            deps[dep.short_path] = dep

    ctx.actions.copied_dir(docker_root, deps)
    buildah = ctx.attrs._buildah_toolchain[BuildahToolchainInfo].bin
    python = ctx.attrs._python_toolchain[PythonToolchainInfo].interpreter
    builder_script = (
        ctx.attrs._buildah_toolchain[BuildahToolchainInfo]
        .builder_script[DefaultInfo]
        .default_outputs
    )

    build_script_output = ctx.actions.declare_output("{}.tar".format(ctx.attrs.name))
    env = {
        "BUILD_DATE": "01-12-2024",
        "GIT_REVISION": "buildah-dirty",
        "PROFILE": "release",
    }
    cmd = cmd_args(
        python,
        builder_script,
        "--buildah",
        buildah,
        "--name",
        ctx.attrs.name,
        "--docker_root",
        docker_root,
        "--build-arg",
        "BUILD_DATE={}".format("01-12-2024"),  # TODO
        "--build-arg",
        "GIT_REVISION={}".format("buildah-dirty"),  # TODO
        "--out",
        build_script_output.as_output(),
    )
    ctx.actions.run(cmd, category="buildah_image_and_export", env=env)
    return [
        DefaultInfo(default_outputs=[build_script_output]),
    ]


buildah_image = rule(
    impl=_buildah_image_impl,
    attrs={
        "layers": attrs.list(attrs.dep()),
        "dockerfile": attrs.source(),
        "_python_toolchain": attrs.toolchain_dep(
            default="toolchains//:python", providers=[PythonToolchainInfo]
        ),
        "_buildah_toolchain": attrs.toolchain_dep(
            default="toolchains//:buildah", providers=[BuildahToolchainInfo]
        ),
    },
)
