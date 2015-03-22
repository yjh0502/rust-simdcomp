#!/usr/bin/env bash

rustc --crate-type lib -Z unstable-options --pretty expanded -L target/release/deps/ src/lib.rs
