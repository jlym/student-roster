# README

## Async

  * Rust Async code relies on us supplying a runtime.
  * Libraries depend on specific versions of specific runtimes.
  * Unfortunately different runtimes, and even different versions
    of a runtime are not compatible with each other.
  * We have to carefully pick versions of libraries so they all use the same version
    of a runtime that they support.

## Errors

  * Use [anyhow](https://docs.rs/anyhow/1.0.34/anyhow/) to propogate errors in applications
  * Use [thiserror](https://docs.rs/thiserror/1.0.22/thiserror/) to create new errors
