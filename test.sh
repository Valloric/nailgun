#!/bin/bash

# Runs faster than "./build test", but probably won't for long.

set -e

rustc --test -o /tmp/test parser.rs \
  && /tmp/test \
  && rustc --test -o /tmp/test generator/unescape.rs \
  && /tmp/test

