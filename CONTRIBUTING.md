# Contributing to Kand

First off, thank you for considering contributing to Kand! It's people like you that make Kand such a great tool.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open source project. In return, they should reciprocate that respect in addressing your issue or assessing patches and features.

## Development Setup

Before you start developing, you need to install several required tools:

### Required Tools

1. **Install Rust development tools**:

   ```bash
   cargo install cargo-udeps git-cliff wasm-pack
   ```

   - `cargo-udeps`: Finds unused dependencies
   - `git-cliff`: Generates changelogs
   - `wasm-pack`: WebAssembly package builder

2. **Install uv** (Python package manager):
   Follow the installation guide at: <https://github.com/astral-sh/uv>

3. **Install maturin** (Python-Rust bindings):

   ```bash
   uv tool install maturin
   ```

4. **Install pre-commit** (Git hooks framework):

   ```bash
   pip install pre-commit
   ```

5. **Install make** (Build automation tool):
   - **Linux/macOS**: Usually pre-installed, or install via package manager
   - **Windows**: Install via one of the following options:
     - Install [Git for Windows](https://git-scm.com/download/win) (includes make)
     - Install [Chocolatey](https://chocolatey.org/) and run `choco install make`
     - Install [MSYS2](https://www.msys2.org/) and run `pacman -S make`

Make sure all these tools are properly installed before proceeding with development.

## Development Workflow

Our project is structured into three main parts:

1. `kand`: The core library written in Rust, containing all the technical indicator implementations.
2. `kand-py`: The Python bindings for the core library, allowing Kand to be used in Python.
3. `kand-wasm`: The WebAssembly bindings for the core library, enabling its use in JavaScript/TypeScript environments (web and Node.js).

When making changes, please follow this general workflow:

1. **Modify the Rust code**: All logic for technical indicators resides in the `kand/` directory. Make your changes or additions here first.
2. **Ensure tests pass**: Run the Rust test suite to make sure your changes haven't broken anything.
3. **Update bindings**: If you've added a new indicator or changed a function signature, update the corresponding bindings in the `kand-py/` and/or `kand-wasm/` directories.
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
- Check for unused dependencies (`udeps-check`)
- Build the Wasm package (`wasm-build`)
- Sync the Python environment and generate stubs (`uv-sync`)

Please ensure all checks pass before submitting a pull request.

## Coding Guidelines

To maintain consistency and readability across the codebase, adhere to the following coding guidelines when implementing or modifying technical indicators:

### Parameter Naming and Order

- **Parameter Order**: Function parameters for technical indicators should follow the `(input data, optimization parameters, output data)` pattern:
  - **Input Data**: Includes raw input data (e.g., `input: &[TAFloat]` for a price series, `input: TAFloat` for a new price, `prev_input: TAFloat` for an old price) and computation state (e.g., `prev_sma: TAFloat` for the previous SMA value). Raw input data should precede state data.
  - **Optimization Parameters**: Configuration parameters like `opt_period: TAPeriod` that control the indicator's behavior.
  - **Output Data**: Typically a mutable output parameter (e.g., `output: &mut [TAFloat]`) for full calculations or the function's return value (e.g., `TAFloat`) for incremental calculations.
  - **Examples**:
    - For full SMA calculation: Use `(input, opt_period, output)` as seen in `sma(input: &[TAFloat], opt_period: TAPeriod, output: &mut [TAFloat])`.
    - For incremental SMA calculation: Use `(input, prev_input, prev_sma, opt_period)` as seen in `sma_inc(input: TAFloat, prev_input: TAFloat, prev_sma: TAFloat, opt_period: TAPeriod)`.
- **Naming Conventions**:
  - Use descriptive names for input data, such as `input` (price series or new price), `prev_input` (old price), or `prev_sma` (previous SMA value).
  - Use domain-specific terms with the `opt_` prefix for optimization parameters, such as `opt_period`, `opt_weight`, or `opt_smoothing`, to clearly indicate their role as configuration parameters.
  - For output data, use clear names like `output` or `sma_values` to indicate the result's purpose.
- **Consistency**: Apply this parameter order and naming style to all technical indicator functions, including full calculation functions (e.g., `sma`) and incremental calculation functions (e.g., `sma_inc`), to ensure a cohesive codebase.

## Modifying Existing Indicators

1. Locate the indicator's code in the `kand/src/` directory and apply your changes.
2. Run the tests to ensure correctness: `make test`.
3. If you have changed any function signatures, update the corresponding code in `kand-py` and/or `kand-wasm`.
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
