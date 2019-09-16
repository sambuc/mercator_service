# Mercator Service

REST-based HTTP service for Mercator.

## Mercator: Spatial Index

**Mercator** is a spatial *volumetric* index for the [Human Brain Project](http://www.humanbrainproject.eu). It is a component of the [Knowledge Graph](http://www.humanbrainproject.eu/en/explore-the-brain/search/) service, which  provides the spatial anchoring for the metadata registered as well as processes the volumetric queries.

It is build on top of the Iron Sea database toolkit.

## Iron Sea: Database Toolkit

**Iron Sea** provides a set of database engine bricks, which can be combined and applied on arbitrary data structures.

Unlike a traditional database, it does not assume a specific physical structure for the tables nor the records, but relies on the developper to provide a set of extractor functions which are used by the specific indices provided.

This enables the index implementations to be agnostic from the underlying data structure, and re-used.

## Requirements

### Hardware

 * **Processor:** XGHz CPU
 * **RAM:** Y MB per MB of indexed data
 * **Available storage space:** X MB per MB of indexed data

### Software

 * Rust: https://www.rust-lang.org

## Quick start

## Building from sources

To build this project, you will need to run the following:

```sh
cargo build --release
```

### Installation

To install the software on the system you can use:

```sh
cargo install --release
```

### Usage

In order to configure the behavior of the service, there is couple of environment variables:

* `RUST_LOG`: Set the level of logging, for example (**error**, **warn**, **info**, **debug**, **trace**). This can be controlled per subsystem of the service, or globally by specifying th subsystem and the level in a list, or omitting the subsystem part. For example:
  ```sh
  RUST_LOG="actix_web=debug,mercator_service=trace" # Set actix_web to debug, mercator_service to trace
  RUST_LOG="trace" # Set everything to trace
  ```

* `MERCATOR_HOST`: Name or IP address to bind to.
* `MERCATOR_PORT`: Port on which to listen.
* `MERCATOR_BASE`: Prefix du service web.
* `MERCATOR_ALLOWED_ORIGINS`: Allowed origins for CORS requests:
  ```sh
  MERCATOR_ALLOWED_ORIGINS="http://localhost:3200,http://localhost:3201, http://localhost:3202"
  ```

* `MERCATOR_IMPORT_DATA`: Provide the data set to expose.

Complete example of a run:
```sh
RUST_LOG="warn,actix_web=info,mercator_service=trace" MERCATOR_IMPORT_DATA="1000k" MERCATOR_ALLOWED_ORIGINS="http://localhost:3200,http://localhost:3201, http://localhost:3202" cargo run --release
```

## Documentation

For more information, please refer to the [documentation](https://epfl-dias.github.io/mercator_service/).

If you want to build the documentation and access it locally, you can use:

```sh
cargo doc --open
```

## Acknowledgements

This open source software code was developed in part or in whole in the
Human Brain Project, funded from the European Union’s Horizon 2020
Framework Programme for Research and Innovation under the Specific Grant
Agreement No. 785907 (Human Brain Project SGA2).
