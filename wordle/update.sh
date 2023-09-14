#!/bin/bash -e
podman build . --tag docker.io/j4cob/wordlyze
podman push docker.io/j4cob/wordlyze
flyctl deploy
