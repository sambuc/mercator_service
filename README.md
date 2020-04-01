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

### Software

 * Rust: https://www.rust-lang.org

## Building from sources

To build this project, you will need to run the following:

```sh
cargo build --release
```

## Installation

To install the software on the system, after checking out the
dependencies you can use:

```sh
cargo install --path .
```

## Usage

In order to configure the behavior of the service, there is couple of
environment variables, in bold their default values:

* `RUST_LOG` = `info`:

   Set the level of logging, for example (**error**, **warn**, **info**,
   **debug**, **trace**). This can be controlled per subsystem of the
   service, or globally by specifying th subsystem and the level in a
   list, or omitting the subsystem part.

   For example:

   ```sh
   # Set actix_web to debug, mercator_service to trace,
   # fall back to info for everything else:
   RUST_LOG="info,actix_web=debug,mercator_service=trace"

   # Set everything to trace
   RUST_LOG="trace"
   ```

   [More details](https://epfl-dias.github.io/mercator_service/env_logger/index.html)
   on how to control the logs.

* `MERCATOR_HOST` = **0.0.0.0** :

   Name or IP address to bind to.

* `MERCATOR_PORT` = **8888** :

   Port on which to listen.

* `MERCATOR_BASE` = **/spatial-search** :

   Web service URL prefix.

* `MERCATOR_ALLOWED_ORIGINS` = **http://localhost:3200** :

   Allowed origins for CORS requests.

* `MERCATOR_DATA` = **.**:

   Provide the root folder of the data sets to expose.

### Example

```sh
RUST_LOG="warn,actix_web=info,mercator_service=trace" \
    MERCATOR_HOST="mercator.example.org" \
    MERCATOR_PORT="1234" \
    MERCATOR_BASE="/" \
    MERCATOR_DATA="../mercator_indexer" \
    MERCATOR_ALLOWED_ORIGINS="http://localhost:3200,http://localhost:3201, http://localhost:3202" \
    mercator_service
```

## Documentation

### User documentation

By this, we mean the REST API documentation. To access it and be able
to test it live you can use the following procedure:

 1. Install [docker](https://www.docker.com).
 2. Start mercator_service, making sure there is the `static` folder,
    or a symlink to it, in the current working directory.
 3. Start swagger to have access to the live documentation:

    ```sh
    docker run \
        --rm \
        -p 3200:8080 \
        -e API_URL='http://127.0.0.1:8888/spatial-search/static/api/v1.0.yaml' \
        swaggerapi/swagger-ui
    ```

 4. Using your web navigator, got to [http://localhost:3200](http://localhost:3200).
 5. You have the whole user documentation accessible, and you can
    trigger actions on your live mercator_service instance, running
    queries on your data.

Otherwise you can read `static/api/v1.0.yaml` directly.

### Developer documentation

For people looking at the internal structure, you can use the
[developer documentation](https://epfl-dias.github.io/mercator_service/).

If you want to build this documentation and access it locally, you can
use:

```sh
cargo doc --open
```

## Acknowledgements

This open source software code was developed in part or in whole in the
Human Brain Project, funded from the European Union’s Horizon 2020
Framework Programme for Research and Innovation under the Specific Grant
Agreement No. 785907 (Human Brain Project SGA2).
