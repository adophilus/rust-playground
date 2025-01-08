#! /usr/bin/env sh

envsubst '$BACKEND_PORT $BACKEND_PORT' < /usr/src/app/data/nginx/sites/nft-ai-generator.undo.it.conf.template > /usr/src/app/data/nginx/sites/nft-ai-generator.undo.it.conf
