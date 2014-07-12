#!/usr/bin/env bash

# Makes apt-get shut up about not being connected to a tty
export DEBIAN_FRONTEND=noninteractive

apt-get update

# needed for add-apt-repository
apt-get install -y python-software-properties
apt-get update

apt-get install -y git
apt-get install -y tmux

apt-get dist-upgrade

curl -s http://www.rust-lang.org/rustup.sh | sh

# need to add:
#   export LD_LIBRARY_PATH="/usr/local/lib"
# to .bashrc for rust library loading
