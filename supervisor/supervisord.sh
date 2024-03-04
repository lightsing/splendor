#!/bin/bash

set -ex
#docker build -t splendor-supervisor:latest -f Dockerfile ..
docker volume create --name=supervisor-shared
docker run --rm \
    --privileged \
    --name splendor-supervisor \
    -p 50051:50051 \
    -v /var/run/docker.sock:/var/run/docker.sock \
    -v supervisor-shared:/var/run/splendor \
    -e RUST_LOG=info,splendor_supervisor=debug \
    -e RUST_BACKTRACE=1 \
    -e SHARED_VOLUME=supervisor-shared \
    -e SHARED_VOLUME_PATH=/var/run/splendor \
    -e CONTROLLER_ADDR=0.0.0.0:50051 \
    splendor-supervisor:latest
docker volume rm supervisor-shared
