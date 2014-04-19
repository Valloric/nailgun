#!/usr/bin/env bash

# Makes apt-get shut up about not being connected to a tty
export DEBIAN_FRONTEND=noninteractive

apt-get update

# needed for add-apt-repository
apt-get install -yqq python-software-properties
apt-get update

add-apt-repository -y ppa:hansjorg/rust
apt-get update

apt-get install -yqq rust-nightly
apt-get install -yqq git
apt-get install -yqq tmux

# apt-get install -yqq python-dev
# apt-get install -yqq python-setuptools
# apt-get install -yqq curl

# curl -O https://raw.github.com/pypa/pip/master/contrib/get-pip.py
# python get-pip.py
#
# pip install httpie
# pip install ipython
