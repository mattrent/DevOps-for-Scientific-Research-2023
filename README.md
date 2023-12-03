# DevOps-for-Scientific-Research-2023

Repository for the exam of the "DevOps for scientific research" PhD course at the University of Bologna.

# The project

This is a very simple (and very incomplete) Rust implementation of the `wc` program with a couple of verbosity options.

The code can be built and run using `cargo`, or run directly using the Docker container.

The documentation (generated with `cargo doc`) for the project is available at https://mattrent.github.io/DevOps-for-Scientific-Research-2023/.

## How this works

Like `wc`, if no file is given, it defaults to reading from stdin.

The output is, in order, lines, words and bytes.

## Running the project with Docker

The image can be pulled from Docker Hub:
```
docker pull mattrent/wcrs:latest
```

Reading from stdin:
```
$ echo -en "something\nsomething" | sudo docker run -i mattrent/wcrs
1        2        19
$ echo -en "something\nsomething" | sudo docker run -i mattrent/wcrs -l
1
$ echo -en "something\nsomething" | sudo docker run -i mattrent/wcrs -c
19
$ echo -en "something\nsomething" | sudo docker run -i mattrent/wcrs -w
2
```

Reading from file:
```
$ cat LICENSE | sudo docker run -i mattrent/wcrs
21       169      1071
$ cat LICENSE | sudo docker run -i mattrent/wcrs -l
21
$ cat LICENSE | sudo docker run -i mattrent/wcrs -c
1071
$ cat LICENSE | sudo docker run -i mattrent/wcrs -w
169
```

Running interactively (the `wcrs` executable is in `/bin`):
```
$ sudo docker run -it --entrypoint /bin/sh wcrs
```

Any external file should be mounted as a volume inside the container.

## Running the project with cargo

To build the project:
```
cargo build -r
```

To run on a file:
```
./target/release/wcrs <FILE>
```
