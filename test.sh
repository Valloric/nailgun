#!/bin/bash

set -e

rustc --test -o /tmp/test parser.rs \
  && /tmp/test \
  && rustc --test -o /tmp/test generator/unescape.rs \
  && /tmp/test

