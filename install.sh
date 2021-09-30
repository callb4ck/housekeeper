#!/bin/sh

build() {
    cargo build --release
    strip ./target/release/housekeeper
}

build
printf "\n\nRequiring root permissions to move executable to /usr/bin/\n"
sudo mv ./target/release/housekeeper /usr/bin/housekeeper && echo Done

