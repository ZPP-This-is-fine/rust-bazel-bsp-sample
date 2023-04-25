~~We now use `rules_rust` fork and that requires additional step. Follow
[this tutorial](https://github.com/bazelbuild/rules_rust/blob/main/crate_universe/DEVELOPMENT.md)
and set `CARGO_BAZEL_GENERATOR_URL` environment variable appropriately before
building.~~

~~IntelliJ IDEA must see this variable. On MacOS, `launchctl setenv` may be
helpful. On Linux, look for `~/.config/environment.d/envvars.conf`.~~

Bazel team already released `rules_rust` with changes we require. No need to struggle with the setup.
https://github.com/bazelbuild/rules_rust/releases/tag/0.21.0

