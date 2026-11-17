#!/bin/sh
# Adds the pcfg apt repository. Usage:
#   curl -fsSL https://imreangelo.github.io/pcfg/setup.sh | sudo sh
set -eu

URL=https://imreangelo.github.io/pcfg

[ "$(id -u)" -eq 0 ] || { echo "Run with sudo" >&2; exit 1; }

mkdir -p /etc/apt/keyrings
curl -fsSL "$URL/key.asc" -o /etc/apt/keyrings/pcfg.asc

cat > /etc/apt/sources.list.d/pcfg.sources <<EOF
Types: deb
URIs: $URL
Suites: stable
Components: main
Signed-By: /etc/apt/keyrings/pcfg.asc
EOF

echo "Added $URL. Now run: sudo apt update && sudo apt install pcfg"