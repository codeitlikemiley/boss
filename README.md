# Rust with Bazel

This is a simple example of a Rust application using Bazel.

## Build

```sh
bazel build //:server
```

## Run

```sh
bazel run //:server
```

## Test

```sh
bazel test //...
```


## Rust analyzer

run this command everytime you update your dependencies

```sh
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project
```


.vscode/settings.json

```json
{
    "rust-analyzer.linkedProjects": [
        "rust-project.json"
    ]
}
```

You will know this is working if you go to `src/main.rs`
and click the Run Icon on top of fn main and it run 

```sh
 *  Executing task: bazel run :server 

INFO: Analyzed target //:server (207 packages loaded, 5985 targets configured).
INFO: Found 1 target...
Target //:server up-to-date:
  bazel-bin/server
INFO: Elapsed time: 3.103s, Critical Path: 0.21s
INFO: 1 process: 383 action cache hit, 1 internal.
INFO: Build completed successfully, 1 total action
INFO: Running command line: bazel-bin/server
🚀 Server running on http://127.0.0.1:3000
   Try http://127.0.0.1:3000/ping
```


## This is Repo to test [Cargo Runner](https://github.com/codeitlikemiley/cargo-runner)

Latest version of Cargo Runner Introduce different Binary / Test Framework which can totally replace Cargo

## Known Limitation

Currently Bazel has no support for `rustdoc` for running individual doctests we can run all doctest 

```bazel
# Doc tests
rust_doc_test(
    name = "doc",
    crate = ":boss", 
)
```

`:boss` is the name of the library