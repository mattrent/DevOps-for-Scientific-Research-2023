# DevOps-for-Scientific-Research-2023

Repository for the exam of the "DevOps for scientific research" PhD course at the University of Bologna.

# The project

This is a very simple (and very incomplete) Rust implementation of the `wc` program with a couple of verbosity options.

The code can be built and run using `cargo`, or run directly using the Docker container.

The documentation (generated with `cargo doc`) for the project is available at https://mattrent.github.io/DevOps-for-Scientific-Research-2023/.

## Running the project with Docker

```
```

## Running the project with cargo

To build the project:

```
cargo build -r
```

To run on a file:

```
./target/release/wcrs <FILE>
```

If no file is given, it defaults to reading from stdin.
