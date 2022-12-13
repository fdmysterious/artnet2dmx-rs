FROM rust:bullseye

# Arguments
ARG SERIALGID=20 # GID of group used to access serial devices

RUN apt-get update -y
RUN apt-get install -y sudo

# Tools
RUN apt-get install -y build-essential pkg-config

# Dev. dependencies
RUN apt-get install -y libudev-dev

# Create dev. user
RUN delgroup dialout
RUN addgroup dialout --gid ${SERIALGID}
RUN adduser builder
RUN usermod -aG dialout builder

# Install toolchain
#RUN rustup target install thumbv6m-none-eabi
#
## Install various tools
#USER builder
#ENV CARGO_HOME=/home/builder/.cargo
#RUN cargo install elf2uf2-rs
#RUN cargo install cargo-generate
