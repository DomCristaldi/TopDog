#! bin/bash

cargo build --bin top_dog --features "vulkan"
cp -r assets target/debug/
cp -r resources target/debug