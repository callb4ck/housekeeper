#!/bin/sh

build() {
    cargo build --release
    strip ./target/release/housekeeper
}

build
echo Requiring root permissions to move executable to /usr/bin/
sudo mv ./target/release/housekeeper /usr/bin/housekeeper && echo Done

