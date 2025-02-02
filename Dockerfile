# File:         DockerFile

# Created by:   VPR
# Created:      February 12th, 2024

# Updated by:   VPR
# Updated:      May 2nd, 2024

FROM ubuntu:24.04

# Set env to avoid user input interruption during installation
ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Install required packages
RUN apt-get update && apt upgrade -y
RUN apt-get install -y --no-install-recommends \
    curl \
    ca-certificates \
    build-essential \
    vim \
    mingw-w64 \
    mingw-w64-common \
    mingw-w64-i686-dev \
    mingw-w64-x86-64-dev \
    mingw-w64-tools
RUN update-ca-certificates

# Create working environment
WORKDIR /var/opt/concept

# Copy sources
COPY res res
COPY src src
COPY config.toml config.toml
COPY build.rs build.rs
COPY rust-toolchain.toml rust-toolchain.toml
COPY Cargo.toml Cargo.toml
COPY Makefile Makefile

# Set docker user to local user uid:gid
ARG LOCAL_USER
ARG LOCAL_UID
ARG LOCAL_GID

RUN if getent group $LOCAL_GID > /dev/null; \
    then \
        groupmod -n $LOCAL_USER `getent group $LOCAL_GID | cut -f1 -d:`; \
    else \
        groupadd -g $LOCAL_GID $LOCAL_USER; \
    fi

RUN if getent group $LOCAL_UID > /dev/null; \
    then \
        usermod -l $LOCAL_USER -d /home/$LOCAL_USER -m `getent passwd 1000 | cut -f1 -d:`; \
    else \
        useradd -m -u $LOCAL_UID -g $LOCAL_GID -s /bin/bash $LOCAL_USER; \
    fi

RUN chown -R $LOCAL_USER:$LOCAL_USER /var/opt/concept

# Become user
USER $LOCAL_USER
ENV HOME=/home/$LOCAL_USER

# Install & configure Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . $HOME/.profile && rustup install nightly
RUN . $HOME/.profile && rustup default nightly
RUN . $HOME/.profile && rustup target add x86_64-pc-windows-gnu --toolchain nightly
RUN . $HOME/.profile && rustup toolchain add nightly-x86_64-pc-windows-gnu
RUN . $HOME/.profile && rustup component add rust-src --toolchain nightly-x86_64-pc-windows-gnu
