# cargo which-nightly | [![Tests](https://img.shields.io/github/actions/workflow/status/cdown/cargo-which-nightly/ci.yml?branch=master)](https://github.com/cdown/cargo-which-nightly/actions?query=branch%3Amaster)

`cargo which-nightly` tells you which nightly contains a particular set of
features.

## Usage

    % cargo which-nightly miri clippy
    2023-05-04

You can set this nightly as the default with:

    % cargo which-nightly --set-default miri clippy

Or, if you'd prefer to do it yourself:

    % rustup default nightly-"$(cargo which-nightly miri rls clippy)"

The current compiled platform is assumed as the target. If you want to check
another, pass `--target`:

    % cargo which-nightly --target aarch64-unknown-linux-gnu miri clippy
