#! /usr/bin/env bash

pnpm run --recursive --parallel build
cp -r ./packages/{frontend/dist,backend/public}
