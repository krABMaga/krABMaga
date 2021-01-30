# Contributing FAQ

Hello, thank you for contributing to this project! Here are some guidelines and helpful info that can help you during
the development phase.

The structure of the project is:

- engine: The Rust-AB framework files are stored here, as well as data structures that can be used in simulation
  development.
- utils: Contains utilities used by the framework such as the dbdashmap data structure.
- visualization: Contains the (optional) visualization framework files that interacts with Rust-AB.

# How to: Test

To test the framework's code, the command to run is `cargo test --lib`.

To test the rustdoc examples, `cargo test --doc`.

To run the integration tests located within the `tests` folder, `cargo test --tests`.

Finally, to run tests for a particular example, `cargo test --example exampleName`.

**WARNING:** By default, `cargo test` with no flags will try building all examples, including the ones with
visualization. This will lead to errors if the correct amethyst feature isn't passed, since it's required for those
examples to build successfully. If you want to test everything, simply add the correct amethyst featured based on your
OS:

- Windows/Linux: `cargo test --features amethyst_vulkan`
- MacOS: `cargo test --features amethyst_metal`comp