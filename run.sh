#!/bin/bash

rustc --out-dir /tmp inlined_parser.rs \
  && rustc -L /tmp -o /tmp/main main.rs \
  && /tmp/main "$@"
