#!/bin/bash

pushd $(dirname $0)/.. > /dev/null

mv Cargotoml Cargo.toml
mv hello_world/Cargotoml hello_world/Cargo.toml
mv hello_lib/Cargotoml hello_lib/Cargo.toml
mv hello_macro/Cargotoml hello_macro/Cargo.toml

popd > /dev/null
