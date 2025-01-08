#!/usr/bin/env sh

ROOT_DIR="/usr/src/app"
SCRIPTS_DIR="$ROOT_DIR/scripts"
DATA_DIR="$ROOT_DIR/data"

# Substitute environment variables in configuration
$SCRIPTS_DIR/substitute-environment-variables.sh

# Periodically reload Nginx configuration in the background
while :; do
  sleep 6h
  nginx -s reload
done &

# Start Nginx with the specified configuration
nginx -c "$DATA_DIR/nginx/nginx.conf"
