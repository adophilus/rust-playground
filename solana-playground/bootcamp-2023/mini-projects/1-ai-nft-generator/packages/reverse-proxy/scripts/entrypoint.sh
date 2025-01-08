#! /usr/bin/env sh

./substitute-environment-variables.sh

while :; do sleep 6h & wait $${!}; nginx -s reload; done & nginx -c /usr/src/app/nginx.conf
