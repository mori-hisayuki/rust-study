# ベースイメージとして公式のRustイメージを使用
FROM rust:latest

# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/rust-app
