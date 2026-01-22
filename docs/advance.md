# Advanced Configuration Guide

This guide explains how to customize `kand` for numerical precision, optimization, and cross-platform use. Custom configurations require building from source with `maturin`, as the default `pip install kand` provides a pre-built package with fixed settings (`f64`, `i64`, `check`).

## Customization Options

### Numerical Precision

Choose the floating-point precision for your needs:

- **`f32` (32-bit)**: Lower memory usage, great for large datasets or constrained systems.
- **`f64` (64-bit)**: Default, higher precision for complex calculations.

| Precision | Memory Usage | Precision | Use Case                  |
|-----------|--------------|-----------|---------------------------|
| `f32`     | Low          | Lower     | Embedded systems, big data |
| `f64`     | Medium       | Higher    | Scientific computing      |

### Integer Types

Select the integer type for indexing:

- **`i32` (32-bit)**: Memory-efficient, suits smaller applications.
- **`i64` (64-bit)**: Default, handles larger datasets.

| Type  | Description     | Best For                  |
|-------|-----------------|---------------------------|
| `i32` | 32-bit integers | Small-scale applications  |
| `i64` | 64-bit integers | Large datasets            |

### Validation Levels

Adjust validation for safety vs. performance:

- **`check`**: Basic validation, ideal for production.
- **`check-nan`**: Detailed checks, best for debugging.
- **None**: No validation, fastest but risky.

| Level        | Safety | Performance | Use Case            |
|--------------|--------|-------------|---------------------|
| `check`      | High   | Medium      | Production          |
| `check-nan` | Highest| Slowest     | Debugging           |
| None         | Low    | Fastest     | Tested environments |

!!! warning "Performance vs. Safety"
    Skipping validation boosts speed but may cause issues with invalid inputs. Use cautiously.

---

## Building from Source

Customizing `kand` requires a local build with `maturin`.

### Prerequisites

- **Python**: 3.8+
- **Rust**: 1.80+
- **maturin**: `pip install maturin`

### Build Steps

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/kand-ta/kand.git
   cd kand
   ```

2. **Build with Custom Features**:
    For development (editable install):

    ```bash
    maturin develop --features f32,i64,check
    ```

!!! tip "High-Performance Build Example"
    Use `--release` for an optimized build:
    ```bash
    maturin build --release --features f64,i64,check
    ```

---

## Troubleshooting

- **Feature Not Working**: Ensure `--features` matches your desired configuration (e.g., `f32,i64,check`).
- **Build Fails**: Check Rust/Python versions or run `cargo build` to debug.
- **Help**: Visit [GitHub Discussions](https://github.com/kand-ta/kand/discussions).

---

## Performance Trade-offs

| Configuration         | Memory | Speed  | Precision |
|-----------------------|--------|--------|-----------|
| `f32, i32, no checks` | Lowest | Fastest| Lowest    |
| `f32, i64, check`     | Low    | Fast   | Low       |
| `f64, i64, check`     | Medium | Medium | High      |
| `f64, i64, check-nan` | High   | Slowest| Highest   |

See the [Performance Guide](performance.md) for benchmarks.
