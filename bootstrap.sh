#!/usr/bin/env bash

# Makes apt-get shut up about not being connected to a tty
export DEBIAN_FRONTEND=noninteractive

apt-get update

# needed for add-apt-repository
apt-get install -y python-software-properties
apt-get update

add-apt-repository -y ppa:ubuntu-toolchain-r/test
add-apt-repository -y ppa:hansjorg/rust
apt-get update

apt-get install -y g++-4.9
apt-get install -y rust-nightly

apt-get install -y git
apt-get install -y tmux

