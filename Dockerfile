# syntax=docker/dockerfile:1
FROM node:16

WORKDIR /home/node

RUN apt-get update && apt-get upgrade -y && apt-get install -y pkg-config build-essential libudev-dev libssl-dev unzip

RUN npm install -g ts-node

USER node

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/home/node/.cargo/bin:$PATH"

RUN cargo install --git https://github.com/project-serum/anchor avm --locked --force

RUN avm install latest

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.14.7/install)" \
    && echo 'export PATH="~/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
