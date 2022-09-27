# xapp-frame-rust

[![Cargo Build & Test](https://github.com/caurus/xapp-frame-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/caurus/xapp-frame-rust/actions/workflows/ci.yml) ![crates.io](https://img.shields.io/crates/v/xapp-frame-rust.svg)

An Open-Source Framework for Rust-based xApps on the O-RAN Near-Realtime RIC. This framework primarily targets the O-RAN Software Community's [RIC Platform](https://wiki.o-ran-sc.org/pages/viewpage.action?pageId=1179659). If you would like to see support for Rust-based xApps on other platforms, please reach out to us on our [discussion page](https://github.com/caurus/xapp-frame-rust/discussions).

This project is maintained by [<img src="https://caur.us/favicon.ico" style="height: 12px" /> Caurus](https://github.com/caurus) for the [O-RAN Software Community](https://wiki.o-ran-sc.org/). All original code in this repository is © 2022 Caurus Technologies, Inc. unless otherwise noted, and made available under the Apache 2.0 license unless otherwise noted.

# Overview

This is a workspace project consisting of following crates -

1. `rmr` - A crate to wrap `librmr_si.so` (`rmr` directory)
2. `xapp` - (`xapp directory) A first implementation of the framework crate for the rust.
   This is an early implementation that demonstrates how the framework is going to take
	 shape.
3. `examples` - (`examples` directory in above crates).
   examples inside individual crates. When our publishable version is ready, we'll have
   some more examples along the lines of other frameworks.

# Getting Started

This is an early implementation and is in active development, the current support includes safe wrapper over the `librmr_si.so` library (in the `rmr` create inside the `rmr` directory) and an initial `XApp` definition (in the `xapp` crate inside `xapp` directory).

## Building The Code

1. The code uses `bindgen` to generate bindings for `librmr_si.so`, which requires `libclang-dev` installed. Make sure `libclang` is installed on your machine.
2. Make sure that `librmr_si` and it's header files are available at `/usr/local/lib` and `/usr/local/include/rmr` directories respectively.
   - On Debian-based systems, you can install RMR via .deb package as follows:

         export RMRTAG=4.8.2
         wget --content-disposition https://packagecloud.io/o-ran-sc/staging/packages/debian/stretch/rmr_${RMRTAG}_amd64.deb/download.deb && dpkg -i rmr_${RMRTAG}_amd64.deb && rm -rf rmr_${RMRTAG}_amd64.deb
         wget --content-disposition https://packagecloud.io/o-ran-sc/staging/packages/debian/stretch/rmr-dev_${RMRTAG}_amd64.deb/download.deb && dpkg -i rmr-dev_${RMRTAG}_amd64.deb && rm -rf rmr-dev_${RMRTAG}_amd64.deb

   - Alternatively, follow the build instructions in the [RMR repository](https://gerrit.o-ran-sc.org/r/gitweb?p=ric-plt/lib/rmr.git;a=summary).

## Running Examples

1. A simple Pong xApp example is available that demonstrates basic use of the `xapp` crate APIs. This can be run as follows -

         $RMR_SEED_RT=test_route.rt cargo run --example basic_pong_xapp

2. A simple Pong example is available that demonstrates the use of client for which uses `rmr` crate APIs. This is very similar to Pong example above, except this only uses the `rmr` APIs. This can be run as follows -

         $RMR_SEED_RT=test_route.rt cargo run --example simple_pong_client