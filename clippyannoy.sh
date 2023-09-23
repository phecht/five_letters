#!/bin/bash
cargo clippy -- -D clippy::correctness -D clippy::complexity -D clippy::pedantic -D clippy::nursery -D clippy::perf -D clippy::cargo -D clippy::restriction
