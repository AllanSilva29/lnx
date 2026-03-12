#!/bin/bash

# Simple debug script for lnx
# Usage: ./debug.sh [command]

COMMAND=${1:-"check"}

case $COMMAND in
    "check")
        echo "🔍 Checking lnx-server compilation..."
        docker compose exec lnx-debug bash -c "cd /code && PATH=\"/root/.cargo/bin:\$PATH\" cargo check --package lnx-server"
        ;;
    "build")
        echo "🔨 Building lnx-server..."
        docker compose exec lnx-debug bash -c "cd /code && PATH=\"/root/.cargo/bin:\$PATH\" cargo build --package lnx-server"
        ;;
    "run")
        echo "🚀 Starting lnx-dev server..."
        docker compose exec lnx-dev bash -c "cd /code && PATH=\"/root/.cargo/bin:\$PATH\" cargo run --package lnx -- run"
        ;;
    "shell")
        echo "🐚 Opening debug shell..."
        docker compose exec lnx-debug bash
        ;;
    "dev-shell")
        echo "🐚 Opening dev shell..."
        docker compose exec lnx-dev bash
        ;;
    *)
        echo "Usage: $0 {check|build|run|shell|dev-shell}"
        echo ""
        echo "Commands:"
        echo "  check      - Check compilation errors"
        echo "  build      - Build the project"
        echo "  run        - Run the dev server"
        echo "  shell      - Open debug shell"
        echo "  dev-shell  - Open dev shell"
        ;;
esac
