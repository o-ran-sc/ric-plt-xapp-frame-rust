FROM rust:1.65-bullseye as xapp-base

# libclang is a dependency of the bindgen crate. See https://internals.rust-lang.org/t/consider-shipping-libclang-with-rust-toolchain/12241
RUN apt-get update && apt-get install -y libclang-dev protobuf-compiler redis-server

ARG RMRTAG=4.8.2
# Get RMR library from Packagecloud
# Note: O-RAN-SC only publishes packages for Debian stretch, but stretch does not get official Rust 1.63 updates.
#       Installing this on Debian bullseye seems to work... Should probably request newer packages from O-RAN-SC.
RUN wget --content-disposition https://packagecloud.io/o-ran-sc/staging/packages/debian/stretch/rmr_${RMRTAG}_amd64.deb/download.deb && dpkg -i rmr_${RMRTAG}_amd64.deb && rm -rf rmr_${RMRTAG}_amd64.deb
RUN wget --content-disposition https://packagecloud.io/o-ran-sc/staging/packages/debian/stretch/rmr-dev_${RMRTAG}_amd64.deb/download.deb && dpkg -i rmr-dev_${RMRTAG}_amd64.deb && rm -rf rmr-dev_${RMRTAG}_amd64.deb


ENV PATH=/usr/local/cargo/bin:$PATH

WORKDIR "/"
CMD ["/bin/bash"]

#----------------------------------------------------------
#
#----------------------------------------------------------

FROM xapp-base as xapp-base-testbuild

RUN mkdir -p /ws
WORKDIR "/ws"

# Copy
COPY . /ws

RUN cargo build

RUN cargo test

