#!/usr/bin/env sh

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

wget --no-check-certificate --content-disposition https://github.com/ThizGuy1016/rusty_ducky/releases/download/v0.4.2/rusty_ducky.bin
wget --no-check-certificate --content-disposition https://github.com/ThizGuy1016/rusty_ducky/releases/download/v0.4.2/keyboard_layouts.zip
