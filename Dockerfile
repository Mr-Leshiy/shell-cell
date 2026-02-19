FROM rust:1.93-trixie

ENV PATH="/root/.local/bin:$PATH"

RUN rustup component add clippy
RUN rustup component add rustfmt
RUN apt-get update --fix-missing
RUN apt-get -y install git curl wget
# claude code
RUN curl -fsSL https://claude.ai/install.sh | bash
# zsh
RUN apt install -y zsh