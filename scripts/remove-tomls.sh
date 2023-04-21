#!/bin/bash

pushd $(dirname $0)/.. > /dev/null

mv Cargo.toml Cargotoml
mv hello_world/Cargo.toml hello_world/Cargotoml
mv hello_lib/Cargo.toml hello_lib/Cargotoml
mv hello_macro/Cargo.toml hello_macro/Cargotoml

popd > /dev/null
