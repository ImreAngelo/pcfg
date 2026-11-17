#!/bin/sh
# Get data used in testing and benchmarking from releases
set -eu
DATA_TAG=data-v1
mkdir -p data
gh release download "$DATA_TAG" -D data --skip-existing