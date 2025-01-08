#! /usr/bin/env sh

envsubst '$BACKEND_PORT $BACKEND_PORT' < /usr/src/app/sites/nft-ai-generator.lan.conf.template > /usr/src/app/sites/nft-ai-generator.lan.conf

nginx -c /usr/src/app/nginx.conf
