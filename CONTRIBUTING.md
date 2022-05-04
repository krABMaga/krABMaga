# Contributing FAQ

Hello, thank you for contributing to this project! Here are some guidelines and helpful info that can help you during
the development phase.

The structure of the project is:

- engine: The krABMaga framework files are stored here, as well as data structures that can be used in simulation
  development.
- utils: Contains utilities used by the framework such as the DBDashMap data structure.
- visualization: Contains the (optional) visualization framework files that interact with krABMaga.

# How to: Test

To test the framework's code, the command to run is `cargo test --lib`.

To test the rustdoc examples, `cargo test --doc`.

To run the integration tests located within the `tests` folder, `cargo test --tests`.

Finally, to run tests for a particular example, `cargo test --example exampleName`.

**WARNING:** By default, `cargo test` with no flags will try running all the tests and examples, including the ones related to the
visualization framework. This will lead to errors if the correct feature isn't passed, since it's required for those
examples to build successfully. If you want to test everything, simply add the visualization feature flag:

```sh
cargo test --features visualization
```
