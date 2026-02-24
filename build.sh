#!/usr/bin/env bash

source ./build_env.sh

# 更新依赖锁文件
echo "更新 pnpm 依赖锁文件..."
pnpm install || { echo "pnpm 依赖更新失败"; exit 1; }

source ./scripts/build_webpage.sh

source ./scripts/build_server.sh

# source ./scripts/build_docker.sh
