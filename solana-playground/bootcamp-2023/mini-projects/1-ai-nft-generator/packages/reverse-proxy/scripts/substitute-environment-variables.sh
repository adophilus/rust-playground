#! /usr/bin/env sh

TEMPLATE_DIR="/usr/src/app/data/nginx/sites"
CONFIG_FILE="nft-ai-generator.undo.it.conf"

envsubst '$BACKEND_PORT $BACKEND_PORT' < "$TEMPLATE_DIR/$CONFIG_FILE.template"> "$TEMPLATE_DIR/$CONFIG_FILE"
