package(default_visibility = ["//visibility:public"])

load(
    "@rules_rust//rust:defs.bzl",
    "rust_library",
    "rust_binary",
    "rust_test",
    "rust_doc",
)

filegroup(
    name = "test_data",
    srcs = glob([
        "tests/**",
    ]),
)


rust_library(
    name = "asset",
    srcs = ["src/asset.rs"],
)

rust_test(
    name = "asset_test",
    crate = ":asset",
)

rust_library(
    name = "build",
    srcs = ["src/build.rs"],
    deps = [
        ":config",
        ":item",
    ],
)

rust_test(
    name = "build_test",
    crate = ":build",
)

rust_library(
    name = "compile",
    srcs = ["src/compile.rs"],
    deps = [
        ":asset",
        ":config",
        ":page",
        ":template",
        "@crates//:comrak",
    ]
)

rust_test(
    name = "compile_test",
    crate = ":compile",
    data = [":test_data"],
)

rust_library(
    name = "config",
    srcs = ["src/config.rs"],
    deps = [
        "@crates//:serde",
        "@crates//:toml",
    ],
    proc_macro_deps = [
        "@crates//:serde_derive",
    ],
)

rust_library(
    name = "item",
    srcs = ["src/item.rs"],
    deps = [
        ":page",
        ":asset",
        "@crates//:walkdir",
    ]
)

rust_library(
    name = "meta",
    srcs = ["src/meta.rs"],
    deps = [
        "@crates//:chrono",
        "@crates//:dateparser",
        "@crates//:regex",
        "@crates//:serde",
        "@crates//:serde_yaml",
    ],
    proc_macro_deps = [
        "@crates//:serde_derive",
    ],
)

rust_library(
    name = "page",
    srcs = ["src/page.rs"],
    deps = [
        ":meta",
        "@crates//:chrono",
        "@crates//:regex",
        "@crates//:tera",
    ],
)

rust_test(
    name = "page_test",
    crate = ":page",
    data = ["//:test_data"],
)

rust_doc(
    name = "page_doc",
    crate = ":page",
)

rust_library(
    name = "template",
    srcs = ["src/template.rs"],
    deps = [
        ":page",
        "@crates//:tera",
    ],
)

rust_test(
    name = "template_test",
    crate = ":template",
    data = ["//:test_data"],
)