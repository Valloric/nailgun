#!/bin/bash

rustc --out-dir /tmp parser.rs \
  && rustc -L /tmp -o /tmp/main main.rs \
  && /tmp/main "$@"
