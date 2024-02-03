FROM ubuntu:latest

# 必要なパッケージのインストール
RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y curl git build-essential pkg-config libssl-dev \
    # キャッシュのクリア
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Rustのインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 環境変数の設定
ENV PATH="/root/.cargo/bin:${PATH}"

# 作業ディレクトリを設定
WORKDIR /usr/local/app
