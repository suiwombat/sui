# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# TODO(cjhopman): This was generated by scripts/hacks/rules_shim_with_docs.py,
# but should be manually edited going forward. There may be some errors in
# the generated docs, and so those should be verified to be accurate and
# well-formatted (and then delete this TODO)

load(":common.bzl", "LinkableDepType")

def _link_style():
    return {
        "link_style": attrs.option(attrs.enum(LinkableDepType), default = None, doc = """
    Determines whether to build and link this rule's dependencies statically or dynamically. Can be
     either `static`, `static_pic` or `shared`.
"""),
    }

def _link_whole(link_whole_type):
    return {
        "link_whole": link_whole_type,
    }

def _preferred_linkage(preferred_linkage_type):
    return {
        "preferred_linkage": preferred_linkage_type,
    }

def _link_group_deps():
    return {
        "link_group_deps": attrs.list(attrs.dep(), default = [], doc = """
    Additional targets to traverse when building link groups, but which should not
     be direct dependencies of the main executable.
"""),
    }

def _link_group_public_deps_label():
    return {
        "link_group_public_deps_label": attrs.option(attrs.string(), default = None, doc = """
    Surface nodes with this label as "public" nodes in the main executable when
     linking with with link groups.
"""),
    }

native_common = struct(
    link_group_deps = _link_group_deps,
    link_group_public_deps_label = _link_group_public_deps_label,
    link_style = _link_style,
    link_whole = _link_whole,
    preferred_linkage = _preferred_linkage,
)
