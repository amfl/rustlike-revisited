#!/bin/sh
set -e

BASEDIR=$(readlink -f "$(dirname $0)")
DOCKER_IMG="rustlike-revisited"
docker build -t ${DOCKER_IMG} .
docker run -it -u $(id -u) -v "${BASEDIR}/include:/rlr" --rm ${DOCKER_IMG} bash
