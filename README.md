# icmp-c2

[![build status](https://github.com/Terryiscool160/icmp-c2/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Terryiscool160/icmp-c2/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/terryiscool160/icmp-c2/status.svg)](https://deps.rs/repo/github/terryiscool160/icmp-c2)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

###

this program is designed to be a working example of using ICMP to communicate between a client and a server to get past (most) firewalls and detections, i decided to try this out after reading on how icmp is often overlooked in security solutions

**WARNING**: i am not responsible for how you use this tool, the unusual responses can easily be noticed by blue teams monitoring packet size

**NOTICE**: command outputs that are too large are not properly parsed and will be cut off, this is a known issue and will be fixed in the future

## Features
- fully encrypted communication
- built in rust

## Running Locally

1. you can install rust by following the [official guide](https://www.rust-lang.org/tools/install)
2. execute `cargo build --release`
3. you will find the `client` and `server` binaries in the `target/release` directory
4) run `./client` and `./server` respectively (root permissions will most likely be required)
