load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "hello_world",
    srcs = ["src/main.rs"],
    deps = [
        "//hello_lib",
        "@crate_index//:clap",
    ],
    proc_macro_deps = [
        "//hello_macro",
    ],
    visibility = ["//visibility:public"],
)
