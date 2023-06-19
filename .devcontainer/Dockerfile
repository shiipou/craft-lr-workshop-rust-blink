ARG VARIANT="bookworm"
FROM mcr.microsoft.com/devcontainers/rust:1-${VARIANT}


RUN apt-get update && \
  apt-get upgrade -y -q && \
  apt-get install -y -q \
    libudev-dev \
    libssl-dev \
    pkg-config &&\
  apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

ARG USERNAME=vscode

RUN adduser $USERNAME plugdev

USER $USERNAME

RUN rustup update

RUN rustup component add llvm-tools-preview &&\
rustup target add thumbv7em-none-eabihf

