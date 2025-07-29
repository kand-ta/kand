<h1 align="center">
  <img src="docs/assets/logo.png" alt="Kand Logo" width="250">
</h1>
<div align="center">
  <a href="https://crates.io/crates/kand">
    <img src="https://img.shields.io/crates/v/kand.svg" alt="Crates.io"/>
  </a>
  <a href="https://pypi.python.org/pypi/kand">
    <img src="https://img.shields.io/pypi/v/kand.svg" alt="PyPI Version"/>
  </a>
  <a href="https://www.npmjs.com/package/kand">
    <img src="https://img.shields.io/npm/v/kand.svg" alt="NPM Version"/>
  </a>
  <a href="https://pypi.python.org/pypi/kand">
    <img src="https://img.shields.io/pypi/pyversions/kand.svg" alt="Python Versions"/>
  </a>
  <a href="https://github.com/kand-ta/kand/actions/workflows/CI.yml">
    <img src="https://github.com/kand-ta/kand/actions/workflows/CI.yml/badge.svg" alt="CI Status"/>
  </a>
  <a href="https://docs.rs/kand">
    <img src="https://docs.rs/kand/badge.svg" alt="Docs.rs"/>
  </a>
  <a href="https://pypi.python.org/pypi/kand">
    <img src="https://img.shields.io/pypi/l/kand.svg" alt="License"/>
  </a>
</div>
<p align="center">
  <b>Documentation</b>:
  <a href="https://docs.rs/kand">Rust</a>
  -
  <a href="https://kand-ta.github.io/kand/">Python</a>
  |
  <b>Repository</b>:
  <a href="https://github.com/kand-ta/kand">GitHub</a>
</p>
<h2 align="center">
  <b>Kand: A Modern, High-Performance Technical Analysis Library</b>
</h2>

> [!WARNING]
> This project is under active development. APIs may change, and some features might not be fully implemented or tested yet. Contributions and feedback are welcome!

<p align="center">
  <picture align="center">
    <img alt="EMA Performance Comparison" src="docs/assets/bench_ema.png" width="600">
  </picture>
</p>

<p align="center">
  <i>EMA calculation performance comparison across different implementations.</i>
</p>

## Why Kand?

`Kand` is engineered as a modern replacement for `TA-Lib`, addressing its core limitations—such as single-threaded execution, Python GIL constraints, memory overhead, and inefficient real-time processing—while preserving its strengths in comprehensive indicator support and ease of integration. Built in Rust, `Kand` delivers superior performance, safety, and flexibility for quantitative trading, data science, and financial analysis.

- **⚡ Superior Performance with Memory Safety**
  Leveraging Rust's efficiency, `Kand` achieves speeds rivaling or exceeding `TA-Lib`'s peak performance, but with built-in memory safety that eliminates common vulnerabilities and reduces overhead in `TA-Lib`'s C-based implementation.

- **🔓 True Multithreading Capabilities**
  Unlike `TA-Lib`, which is hindered by Python's GIL and single-threaded design, `Kand` enables seamless parallel processing across multiple cores, unlocking significant gains in multi-threaded environments for large-scale computations.

- **⚙️ Efficient Real-Time Incremental Updates**
  `Kand` introduces O(1) complexity for incremental calculations, ideal for streaming data and real-time applications—overcoming `TA-Lib`'s reliance on batch processing, which introduces latency and inefficiency in dynamic scenarios.

- **🚀 Zero-Copy NumPy Integration**
  With native, zero-copy data sharing via Rust-NumPy bindings, `Kand` ensures lossless, high-speed data flow between Python and Rust, addressing `TA-Lib`'s memory copying overhead and enabling ultra-low latency (~7ns) operations.

- **📊 Expanded Indicator Suite**
  Kand supports a wide array of standard indicators (e.g., EMA, RSI, MACD) like ``TA-Lib``, while pioneering advanced ones such as Vegas, VWAP, and Supertrend, extending analytical capabilities beyond `TA-Lib`'s traditional scope.

- **📦 Streamlined Installation and Lightweight Design**
  Install with a single `pip install kand` command, featuring precompiled wheels and no complex C dependencies—solving `TA-Lib`'s notoriously cumbersome setup and reducing package bloat for effortless deployment.

- **💻 Broad Cross-Platform Compatibility**
  Seamlessly runs on macOS, Linux, and Windows, with additional support for JavaScript/TypeScript via `WebAssembly`, providing greater universality than `TA-Lib`'s platform-specific challenges.

> *If you truly understand `TA-Lib`'s limitations, you'll appreciate Kand's innovations.* `Kand` isn't just about fixing what's broken—it's about enabling what's possible. Dive deeper at [**why `kand`**](https://kand-ta.github.io/kand/about).

### Python API

The Python interface of `kand` leverages PyO3 for ultra-low latency bindings to the Rust core, seamlessly integrating with NumPy for zero-copy operations and true thread-safe calculations. Below are examples for batch and incremental usage.

```python
import numpy as np
from kand import ema

# Batch EMA computation with zero-copy NumPy integration
prices = np.array([10.0, 11.0, 12.0, 13.0, 14.0], dtype=np.float64)
ema_values = ema(prices, period=3)

# Incremental EMA update for streaming data
prev_ema = 13.5
new_price = 15.0
new_ema = ema_inc(new_price, prev_ema, period=3)
```

**Key Features:**

- **Zero-Copy**: Operates directly on NumPy arrays, avoiding memory duplication.
- **GIL-Free**: Rust backend releases the Python GIL, enabling parallel execution.
- **Incremental Updates**: O(1) complexity for real-time applications.

---

### Rust API

The Rust interface in `kand` provides a high-performance, type-safe implementation of EMA with flexible parameter control. It supports both Vec and ndarray inputs for batch and incremental calculations, as shown below.

```rust
use kand::ohlcv::ema;
use ndarray::Array1;

// Batch EMA calculation over a price series
let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
let mut ema_values = vec![0.0; prices.len()];
ema::ema(&prices, 3, None, &mut ema_values)?;

// Batch EMA with ndarray for scientific workflows
let prices = Array1::from_vec(vec![10.0, 11.0, 12.0, 13.0, 14.0]);
let mut ema_values = Array1::zeros(prices.len());
ema::ema(&prices, 3, None, &mut ema_values)?;

// Constant-time incremental EMA update
let prev_ema = 13.5;
let new_price = 15.0;
let new_ema = ema::ema_inc(new_price, prev_ema, 3, None)?;
```

**Key Features:**

- **Memory Efficiency**: Leverages mutable buffers (`&mut Vec<f64>` or `&mut Array1<f64>`) to store results, slashing memory allocations.
- **Error Handling**: Returns `Result<(), KandError>` or `Result<f64, KandError>` for reliable failure detection (e.g., invalid period, NaN inputs).
- **Incremental Design**: O(1) updates tailored for real-time systems.

---

### JavaScript/TypeScript API

The JavaScript/TypeScript interface provides WebAssembly bindings for high-performance technical analysis in web applications and Node.js projects. It delivers near-native performance with a clean, synchronous API.

```typescript
import { ema, emaInc } from 'kand';

// Batch EMA computation for price series
const prices = new Float64Array([10.0, 11.0, 12.0, 13.0, 14.0]);
const emaValues = ema(prices, 3, null);
console.log(emaValues); // [NaN, NaN, 11.5, 12.25, 13.125]

// Incremental EMA update for streaming data
const prevEma = 13.5;
const newPrice = 15.0;
const newEma = emaInc(newPrice, prevEma, 3, null);
console.log(newEma); // 14.25

// Custom smoothing factor
const customK = 0.5;
const customEma = emaInc(newPrice, prevEma, 3, customK);
```

**Key Features:**

- **WebAssembly Performance**: Near-native speed through optimized WASM bindings.
- **Type Safety**: Full TypeScript definitions with comprehensive JSDoc documentation.
- **Universal**: Works in both browser environments and Node.js applications.

---

## Setup

### Python

Get started with Kand in one command - no extra configuration needed:

```bash
pip install kand
```

### Rust

You can take latest release from [`crates.io`](https://crates.io/crates/kand), or if you want to use the latest features / performance improvements point to the `main` branch of this repo.

```bash
cargo add kand
```

Recommend Rust version `>=1.80`.

### JavaScript/TypeScript

For web applications and Node.js projects, install Kand via npm:

```bash
npm i kand
```

The package provides WebAssembly bindings for high-performance technical analysis in JavaScript and TypeScript environments.

## Functions List

### OHLCV Based

- [x] **AD** - Chaikin A/D Line
- [x] **ADOSC** - Chaikin A/D Oscillator
- [x] **ADR** - Average Daily Range
- [x] **ADX** - Average Directional Movement Index
- [x] **ADXR** - Average Directional Movement Index Rating
- [ ] **APO** - Absolute Price Oscillator
- [x] **AROON** - Aroon
- [x] **AROONOSC** - Aroon Oscillator
- [x] **ATR** - Average True Range
- [x] **BBANDS** - Bollinger Bands
- [x] **BOP** - Balance Of Power
- [x] **CCI** - Commodity Channel Index
- [x] **CDL_DOJI** - Doji
- [x] **CDL_DRAGONFLY_DOJI** - Dragonfly Doji
- [x] **CDL_GRAVESTONE_DOJI** - Gravestone Doji
- [x] **CDL_HAMMER** - Hammer
- [x] **CDL_INVERTED_HAMMER** - Inverted Hammer
- [x] **CDL_LONG_LOWER_SHADOW** - Long Lower Shadow
- [x] **CDL_LONG_UPPER_SHADOW** - Long Upper Shadow
- [x] **CDL_MARUBOZU** - Marubozu
- [ ] **CMO** - Chande Momentum Oscillator
- [x] **DEMA** - Double Exponential Moving Average
- [x] **DX** - Directional Movement Index
- [x] **EMA** - Exponential Moving Average
- [x] **ECL** - Expanded Camarilla Levels **[Untested]**
- [x] **HA** - Heikin Ashi Chart
- [ ] **KAMA** - Kaufman Adaptive Moving Average
- [x] **MACD** - Moving Average Convergence/Divergence **[Unstable]**
- [ ] **MACDEXT** - MACD with controllable MA type
- [ ] **MAMA** - MESA Adaptive Moving Average
- [x] **MEDPRICE** - Median Price
- [x] **MFI** - Money Flow Index **[No Incremental]**
- [x] **MIDPOINT** - MidPoint over period
- [x] **MIDPRICE** - Midpoint Price over period
- [x] **MINUS_DI** - Minus Directional Indicator
- [x] **MINUS_DM** - Minus Directional Movement
- [x] **MOM** - Momentum
- [x] **NATR** - Normalized Average True Range
- [x] **OBV** - On Balance Volume
- [x] **PLUS_DI** - Plus Directional Indicator
- [x] **PLUS_DM** - Plus Directional Movement
- [ ] **PPO** - Percentage Price Oscillator
- [ ] **RENKO** - Renko Chart
- [x] **RMA** - Rolling Moving Average
- [x] **ROC** - Rate of change : ((price/prevPrice)-1)*100
- [x] **ROCP** - Rate of change Percentage: (price-prevPrice)/prevPrice
- [x] **ROCR** - Rate of change ratio: (price/prevPrice)
- [x] **ROCR100** - Rate of change ratio 100 scale: (price/prevPrice)*100
- [x] **RSI** - Relative Strength Index
- [x] **SAR** - Parabolic SAR
- [ ] **SAREXT** - Parabolic SAR - Extended
- [x] **SMA** - Simple Moving Average
- [x] **STOCH** - Stochastic **[No Incremental]**
- [ ] **STOCHF** - Stochastic Fast
- [ ] **STOCHRSI** - Stochastic Relative Strength Index
- [x] **SUPERTREND** - Super Trend Indicator
- [x] **T3** - Triple Exponential Moving Average (T3)
- [x] **TEMA** - Triple Exponential Moving Average
- [x] **TRANGE** - True Range
- [x] **TRIMA** - Triangular Moving Average
- [x] **TRIX** - 1-day Rate-Of-Change (ROC) of a Triple Smooth EMA
- [ ] **TSF** - Time Series Forecast
- [x] **TYPPRICE** - Typical Price
- [ ] **ULTOSC** - Ultimate Oscillator
- [x] **VEGAS** - VEGAS Channel and Trend Boundary EMAs **[Untested]**
- [x] **VWAP** - Volume Weighted Average Price
- [x] **WCLPRICE** - Weighted Close Price
- [x] **WILLR** - Williams' %R
- [x] **WMA** - Weighted Moving Average

### Statistical Analysis

- [ ] **ALPHA** - Alpha: Measures excess returns over market
- [ ] **BETA** - Beta: Measures sensitivity to market volatility
- [ ] **CALMAR** - Calmar Ratio: Annual return to maximum drawdown ratio
- [x] **CORREL** - Pearson's Correlation Coefficient
- [ ] **DRAWDOWN** - Maximum Drawdown: Maximum potential loss
- [ ] **KELLY** - Kelly Criterion: Optimal position sizing
- [x] **MAX** - Highest value over a specified period
- [x] **MIN** - Lowest value over a specified period
- [ ] **SHARPE** - Sharpe Ratio: Risk-adjusted return measure
- [ ] **SORTINO** - Sortino Ratio: Downside risk-adjusted returns
- [x] **STDDEV** - Standard Deviation
- [x] **SUM** - Summation
- [x] **VAR** - Variance
- [ ] **WINRATE** - Win Rate: Strategy success probability

## Contributing

We are passionate about supporting contributors of all levels of experience and would love to see
you get involved in the project. See the
[contributing guide](https://github.com/rust-ta/kand/blob/main/CONTRIBUTING.md) to get started.

## License

This project is licensed under either of the following licenses, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in kand by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any additional terms or conditions.

## Disclaimer

This project is provided "as is" without any warranties or guarantees of any kind, express or implied, including but not limited to warranties of merchantability, fitness for a particular purpose, accuracy, or non-infringement. The authors and contributors of this project are not liable for any damages, losses, or liabilities resulting from the use of this project, including but not limited to financial losses, investment decisions, or trading outcomes.

Trading or investing in financial instruments involves high risks, including the potential loss of some or all of your investment amount, and may not be suitable for all users. Before using this project for any financial purposes, you should carefully consider your investment objectives, level of experience, and risk tolerance, and seek independent professional advice if needed.

The information, data, and calculations provided by this project are for informational purposes only and do not constitute financial, investment, or trading advice, recommendations, or solicitations to buy or sell any securities or assets. We do not guarantee the accuracy, completeness, or timeliness of any output, and users assume all risks and liabilities associated with its use.
