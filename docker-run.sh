#!/bin/sh
set -e

DOCKER_IMG="rustlike-revisited"
docker build -t ${DOCKER_IMG} .
docker run -it --rm ${DOCKER_IMG}
