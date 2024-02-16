"""
Run buildah to create an image and export it locally as an artifact/output
"""

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Any, Dict, IO, NamedTuple, List
from contextlib import contextmanager
import uuid
import shutil


class Args(NamedTuple):
    buildah: str
    name: str
    dockerfile: str
    logfile: str
    out: str


def arg_parse() -> Args:
    parser = argparse.ArgumentParser(description="Run Buildah build script")
    parser.add_argument("--buildah", type=str, required=True)
    parser.add_argument("--name", type=str, required=True)
    # parser.add_argument("--dockerfile", type=str, required=True)
    parser.add_argument("--docker_root", type=str, required=True)
    # parser.add_argument("--logfile", type=str, required=True)
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


def main(args: argparse.Namespace) -> None:
    build_id = uuid.uuid4().hex
    build_cmd = [
        args.buildah,
        "bud",
        "-t",
        f"localhost/{args.name}:{build_id}",
        "--logfile",
        "build_logfile.log",
    ]

    # buildah wants docker args as build-args.  re-map this from buck
    for ba in args.build_arg:
        ba.insert(0, "--build-arg")
        build_cmd.extend(ba)

    image_id_cmd = [args.buildah, "images", "--format", "{{.ID}} {{.Tag}}"]

    # terrible things happen if we don't execute buildah in the same directory as the
    # Dockerfile resides in.  Probabaly better to invest in a go bin instead of
    # horsing around with buildah directly
    with set_directory(Path(args.docker_root)):
        run(build_cmd)
        ids = run(image_id_cmd)
        image_id = parse_ids(ids, build_id)
        print(f"discovered image_id {image_id}")
        dirname, filename = os.path.split(args.out)
        print(f"using dirname {dirname} filename {filename}")
        # important to use the .. here to place the tar in our parent directory
        # buck needs it there bc of the _buildah_image_impl design
        # TODO refactor that design
        exported_image = os.path.join(os.getcwd(), "..", filename)
        export_cmd = [
            args.buildah,
            "push",
            image_id,
            f"oci-archive:{exported_image}",
        ]
        run(export_cmd)


def parse_ids(ids: str, uuid: str) -> str:
    for line in ids.splitlines():
        line = line.decode("utf-8")
        if uuid in line:
            (image_id, _) = line.split()
            return image_id
    print(f"Failed to extract image_id: {ids}", file=sys.stderr)
    sys.exit(1)


if __name__ == "__main__":
    args = arg_parse().parse_args()
    main(args)
