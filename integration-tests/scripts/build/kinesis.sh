#!/usr/bin/env bash

# This script is used for building the kinesis container locally for testing
# The container we actually use is published by a GitHub Action using a more securit focused base image

set -e

SCRIPT_PATH=$( cd "$(dirname "$0")" ; pwd -P )
ROOT_PATH="$SCRIPT_PATH/../../../"
IMAGES_PATH="$SCRIPT_PATH/../../images"

docker build                                    \
  --tag=test_coverdrop_kinesis:dev              \
  --progress plain                              \
  -f "$IMAGES_PATH/kinesis.Dockerfile"          \
  "$ROOT_PATH"
