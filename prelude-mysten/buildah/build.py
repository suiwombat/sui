"""
Run buildah to create an image and export it locally as an artifact/output
"""

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import List, Tuple
from contextlib import contextmanager
import uuid
import shutil


def arg_parse():
    parser = argparse.ArgumentParser(description="Run Buildah build script")
    parser.add_argument("--buildah", type=str, required=True)
    parser.add_argument("--image_name", type=str, required=True)
    parser.add_argument("--registry", type=str, required=False)
    parser.add_argument("--gcloud", type=str, required=False)
    parser.add_argument("--docker_root", type=str, required=True)
    parser.add_argument("--build-arg", type=str, action="append", nargs="*")
    parser.add_argument("--out", type=str, required=True)
    return parser


@contextmanager
def set_directory(path: Path):
    origin = Path().absolute()
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(origin)


def run(cmd: List[str]) -> str:
    try:
        # Run the subprocess and capture stdout and stderr
        result = subprocess.run(
            cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=os.environ.copy()
        )
    except Exception as e:
        print(f"Failed to run {cmd} because {e}", file=sys.stderr)
        sys.exit(1)

    if result.returncode != 0:
        print(f"stdout: {result.stdout}")
        print(f"stderr: {result.stderr}")
        sys.exit(result.returncode)

    return result.stdout


def split_registry(registry: str) -> Tuple[str, str, str]:
    # assuming registry is in the form us-central1-docker.pkg.dev/YOUR_PROJECT_ID/PROJECT_URI
    split = registry.split("/")
    if len(split) != 3:
        print(f"failed to split registry string {registry}", file=sys.stderr)
        sys.exit(1)
    (prefix, project_id, project_uri) = split[0], split[1], split[2]
    return (prefix, project_id, project_uri)


def valid_container_registry(registry: str) -> bool:
    """our known valid remote registries"""
    # google artifact registry has some wack urls, try to extract domains
    allowed = {"docker.pkg.dev": push_to_gcr}
    (registry_uri, _, _) = split_registry(registry)
    domain = "-".join(registry_uri.rsplit("-", 1)[1:])
    print(
        f"attempt to extract match for registry prefix: {registry_uri} with sub match as {domain}"
    )
    # try the split string first, then the unsplit and then fail
    return allowed.get(domain, allowed.get(registry_uri, None))


def login_to_gcr(buildah: str, gcloud: str, registry: str) -> None:
    (registry_uri, _, _) = split_registry(registry)
    gcloud = [gcloud, "auth", "print-access-token"]
    # decode the token and trim newlines.
    gcloud_access_token = run(gcloud).decode("utf-8").strip()
    cmd = [
        buildah,
        "login",
        "-v",
        "--tls-verify=true",
        "-u",
        "oauth2accesstoken",
        "--password",
        gcloud_access_token,
        registry_uri,
    ]
    run(cmd)


def push_to_gcr(image_id: str, args: argparse.Namespace):
    # buildah push --tls-verify=false localhost:5000/my-image:latest us-central1-docker.pkg.dev/YOUR_PROJECT_ID/PROJECT_URI/myimage:tag
    login_to_gcr(args.buildah, args.gcloud, args.registry)
    (registry_uri, project_id, project_uri) = split_registry(args.registry)
    cmd = [
        args.buildah,
        "push",
        "--tls-verify=true",
        image_id,
        f"{registry_uri}/{project_id}/{project_uri}/{args.image_name}:{image_id}",
    ]
    print(" ".join(cmd))
    run(cmd)


def parse_ids(ids: str, uuid: str) -> str:
    """
    parse_ids parses stdout data from get_image_id and returns that id
    """
    for line in ids.splitlines():
        line = line.decode("utf-8")
        if uuid in line:
            (image_id, _) = line.split()
            return image_id
    print(f"Failed to extract image_id: {ids}", file=sys.stderr)
    sys.exit(1)


def build(
    build_id: str, image_name: str, build_args: List[List[str]], buildah: str
) -> None:
    """
    build will use buildah to create an image locally
    we do not push to remote repos in this stage
    """
    cmd = [
        buildah,
        "bud",
        "-t",
        f"localhost/{image_name}:{build_id}",
        "--logfile",
        "build_logfile.log",
        "--pull",
    ]
    # buildah wants docker args as build-args.  re-map this from buck
    for ba in build_args:
        ba.insert(0, "--build-arg")
        cmd.extend(ba)
    run(cmd)


def get_image_id(build_id: str, buildah: str) -> str:
    """
    extract the image id buildah assigned to our recent image build
    """
    cmd = [buildah, "images", "--format", "{{.ID}} {{.Tag}}"]
    ids = run(cmd)
    image_id = parse_ids(ids, build_id)
    print(f"discovered image_id {image_id}")
    return image_id


def push_tar_image_localhost(image_id: str, buildah: str, out: str):
    """
    export our image from the local container registry.  we always store
    a container locally so we can push it to multiple other locations easily.
    it also allows us to use podman locally after the fact.
    """
    dirname, filename = os.path.split(out)
    print(f"using dirname {dirname} filename {filename}")
    # important to use the .. here to place the tar in our parent directory
    # buck needs it there bc of the _buildah_image_impl design
    # TODO possibly refactor that design
    exported_image = os.path.join(os.getcwd(), "..", filename)
    export_cmd = [
        buildah,
        "push",
        image_id,
        f"oci-archive:{exported_image}",
    ]
    run(export_cmd)


def main(args: argparse.Namespace) -> None:
    build_id = uuid.uuid4().hex
    # terrible things happen if we don't execute buildah in the same directory as the
    # Dockerfile resides in.  Probabaly better to invest in a go bin instead of
    # horsing around with buildah directly
    with set_directory(Path(args.docker_root)):
        build(build_id, args.image_name, args.build_arg, args.buildah)
        image_id = get_image_id(build_id, args.buildah)
        push_tar_image_localhost(image_id, args.buildah, args.out)
        if args.registry:
            fn = valid_container_registry(args.registry)
            if not fn:
                print(
                    f"Failed to match registry with export func: {args.registry}",
                    file=sys.stderr,
                )
                sys.exit(1)
            fn(image_id, args)


if __name__ == "__main__":
    args = arg_parse().parse_args()
    main(args)
