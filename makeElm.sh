#!/usr/bin/env bash

set -xe

cd client 

elm make src/Main.elm --output page/index.js
