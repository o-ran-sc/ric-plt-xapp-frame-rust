# Introduction

Rust bindings for `librmr_si.so` from - [https://gerrit.o-ran-sc.org/r/ric-plt/lib/rmr](ORAN SC Project RMR repo)

# Prerequisits

The `rmr` library needs to be built on your machine and installed. It is assumed that it is installed in the prefix path `/usr/local/`. Also make sure during building the library the `-DDEV_PKG=1` option is selected (this will install header files during `make install`).

# Getting Started

Right now simply `cargo build` and `cargo test` can be tried in the repo.

# Examples

You can run the example `simple_client` (this will be renamed later to something sensible! :-) ) as follows -
```
RMR_SEED_RT=./test_route.rt cargo run --example simple_client
```

The environment variable is required to setup 'static' routes. This routing table is shared by the 'other' application (for now we are using `ping_xapp.py` from the Python Xapp examples.)

To run the Python example, you will need to setup the virtual environment and have the `librmr_si.so` installed in the `/usr/local/lib` path along with the development (include) files.

Later on when we have a proper sending rust application, we will not be dependent on the Python Xapp.

