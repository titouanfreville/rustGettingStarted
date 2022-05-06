# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.234.0/containers/rust/.devcontainer/base.Dockerfile

# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}


RUN \
    apt-get update \
    && sudo apt-get install -y gnupg software-properties-common curl \
    && apt-get update \
    && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    gcc zsh wget nano procps htop git zsh tree \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

USER vscode

WORKDIR /home/vscode

ENV TERM xterm

RUN mkdir /home/vscode/commandhistory && touch /home/vscode/commandhistory/.zsh_history
COPY ./.zshrc /home/vscode/.zshrc

WORKDIR /home/vscode/playground

ENV SHELL /bin/zsh

