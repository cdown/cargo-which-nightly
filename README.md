# cargo which-nightly | [![Tests](https://img.shields.io/github/actions/workflow/status/cdown/cargo-which-nightly/ci.yml?branch=master)](https://github.com/cdown/cargo-which-nightly/actions?query=branch%3Amaster)

`cargo which-nightly` tells you which nightly contains a particular set of
features.

## Usage

    % cargo which-nightly miri rls clippy
    2023-05-04

You can get this nightly with:

    % rustup default nightly-"$(cargo which-nightly miri rls clippy)"

The current compiled platform is assumed as the target. If you want to check
another, pass `--target`:

    % cargo which-nightly --target aarch64-unknown-linux-gnu miri rls clippy
