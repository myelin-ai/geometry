#!/usr/bin/env bash

set -e

zip -0 geometry.zip `find . \( -name 'myelin_geometry*.gc*' \) -print`
grcov geometry.zip \
       -t lcov \
       --llvm \
       --branch \
       --ignore-not-existing \
       --ignore-dir '/*' \
       > lcov.info
