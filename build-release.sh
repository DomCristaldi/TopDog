#! bin/bash

cargo build --release --bin top_dog --features "vulkan"
cp -r assets target/release/
cp -r resources target/release