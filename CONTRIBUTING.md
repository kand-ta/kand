# Contributing to Kand

First off, thank you for considering contributing to Kand! It's people like you that make Kand such a great tool.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue or assessing patches and features.

## Development Workflow

Our project is structured into two main parts:

1. `kand`: The core library written in Rust, containing all the technical indicator implementations.
2. `kand-py`: The Python bindings for the core library, allowing Kand to be used in Python.

When making changes, please follow this general workflow:

1. **Modify the Rust code**: All logic for technical indicators resides in the `kand/` directory. Make your changes or additions here first.
2. **Ensure tests pass**: Run the Rust test suite to make sure your changes haven't broken anything.
3. **Update Python bindings**: If you've added a new indicator or changed a function signature, update the Python bindings in the `kand-py/` directory accordingly.
4. **Run all checks**: Use the provided `Makefile` to run a full suite of checks, including building, testing, linting, and formatting.

### Using the Makefile

We have a `Makefile` that simplifies the development process. The most important command is:

```bash
make
```

Running `make` by default executes the `pre-commit` target, which will:

- Build the project (`build`)
- Run tests (`test`)
- Run the linter (`clippy`)
- Format the code (`fmt`)
- Generate the changelog (`cliff`)
- Sync the Python environment and generate stubs (`uv-sync`)

Please ensure all checks pass before submitting a pull request.

## Modifying Existing Indicators

1. Locate the indicator's code in the `kand/src/` directory and apply your changes.
2. Run the tests to ensure correctness: `make test`.
3. If you have changed any function signatures, update the corresponding code in `kand-py`.
4. Run `make` to perform all pre-commit checks.

## Adding New Indicators

Adding a new indicator is exciting! To ensure quality and consistency, please follow these steps:

1. Implement the new indicator in the `kand/` directory.
2. Add a new test module for your indicator.
3. **Provide accurate test data**: This is a critical step.
    - **If the indicator exists in TA-Lib**, your test data **must** align with the output of TA-Lib. This ensures compatibility and correctness.
    - **If the indicator is not in TA-Lib**, you must provide a reference to verify the accuracy of your implementation and test data. This can be:
        - A Python implementation of the indicator.
        - A link to a trading website, academic paper, or another reliable source that defines the indicator and provides example calculations.
4. Add the Python bindings for your new indicator in the `kand-py/` directory.
5. Run `make` to ensure everything is in order.

## Pull Request Process

1. Fork the repository and create your branch from `main`.
2. Make your changes, adhering to the workflow described above.
3. Ensure the test suite passes and that all `make` checks are green.
4. Issue that pull request! We'll review it as soon as we can.

Thank you for your contribution!
