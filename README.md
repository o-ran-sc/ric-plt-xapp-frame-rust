# xapp-frame-rust

Framework for development of xApps in Rust for the O-RAN Near-Realtime RIC. This framework primarily targets the O-RAN Software Community's [RIC Platform](https://wiki.o-ran-sc.org/pages/viewpage.action?pageId=1179659).

# Overview

This projects consists of the following crates.

1. `rmr` - Rust bindings for the [RMR](https://gerrit.o-ran-sc.org/r/admin/repos/ric-plt/lib/rmr,general) library.
2. `xapp` - Public API for the XApp framework.
3. `rnib` - A wrapper around the protobuf definitions in the `ric-plt/nodeb-rnib` repository.
4. `sdl` - Implementation of Shared Data Layer for the RIC Platform in Rust, currently supports Redis backend.
5. `subscription-api` - Rust bindings for OpenAPI definitions for Subscription by xApps.
6. `registration-api` - Rust bindings for the OpenAPI definitions for xApps registration with RIC Application Manager.

# Getting Started

## Setup for Building the Code

1. The code uses `bindgen` to generate bindings for `librmr_si.so`, which requires `libclang-dev` installed. Make sure `libclang` is installed on your machine.
2. Make sure that `librmr_si` and it's header files are available at `/usr/local/lib` and `/usr/local/include/rmr` directories respectively.
   - On Debian-based systems, you can install RMR via .deb package as follows:

         export RMRTAG=4.9.4
         wget --content-disposition https://packagecloud.io/o-ran-sc/staging/packages/debian/stretch/rmr_${RMRTAG}_amd64.deb/download.deb && dpkg -i rmr_${RMRTAG}_amd64.deb && rm -rf rmr_${RMRTAG}_amd64.deb
         wget --content-disposition https://packagecloud.io/o-ran-sc/staging/packages/debian/stretch/rmr-dev_${RMRTAG}_amd64.deb/download.deb && dpkg -i rmr-dev_${RMRTAG}_amd64.deb && rm -rf rmr-dev_${RMRTAG}_amd64.deb

   - Alternatively, follow the build instructions in the [RMR repository](https://gerrit.o-ran-sc.org/r/gitweb?p=ric-plt/lib/rmr.git;a=summary).
3. Make sure that the external dependency inside `rnib` is satisfied as a `git submodule`. This adds the protobuf files inside `rnib/external/nodeb-rnib/` directory. The submodules can be updated by -

         git submodule init
         git submodule update

## Using the Framework in xApps

An example [Hello World Rust](https://gerrit.o-ran-sc.org/r/admin/repos/ric-app/hw-rust,general) xApp is provided, which demonstrates the API usage. Please refer to the source code of Hello World Rust application.
