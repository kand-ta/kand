import numpy as np
import pandas as pd
import talib
import kand
import time
from typing import List, Dict
import matplotlib.pyplot as plt

def pure_python_ema(data: List[float], span: int) -> List[float]:
    """Calculates EMA using a pure Python implementation.

    Args:
        data: List of float values for EMA calculation.
        span: Time period for the EMA.

    Returns:
        List of EMA values.
    """
    alpha = 2 / (span + 1)
    result = [data[0]]
    for n in range(1, len(data)):
        result.append(alpha * data[n] + (1 - alpha) * result[n - 1])
    return result

def pandas_ema(data: pd.Series, span: int) -> pd.Series:
    """Calculates EMA using Pandas EWM (Exponential Weighted Moving Average).

    Args:
        data: Pandas Series of float values for EMA calculation.
        span: Time period for the EMA.

    Returns:
        Pandas Series of EMA values.
    """
    return data.ewm(span=span, adjust=False).mean()

def plot_performance_comparison(results: Dict[str, float]) -> plt.Figure:
    """Plots a horizontal bar chart comparing EMA implementation performance.

    The chart is styled for academic publication with a clean, minimal design,
    using orange bars, a grid, and concise labels.

    Args:
        results: Dictionary mapping implementation names to execution times (ms).

    Returns:
        Matplotlib Figure object.
    """
    # Sort data by performance (shortest to longest time)
    sorted_items = sorted(results.items(), key=lambda x: x[1])
    categories = [item[0] for item in sorted_items]
    values = [item[1] for item in sorted_items]

    # Create figure with tight layout for publication-quality appearance
    plt.figure(figsize=(6, 3), dpi=300)  # Smaller size but keep high DPI
    bars = plt.barh(categories, values, color='#FF8C00',  # Soft orange for readability
                    height=0.5,  # Thinner bars for minimalism
                    edgecolor='black', linewidth=0.8)  # Slightly thicker outlines for clarity

    # Set limits and remove unnecessary padding
    plt.xlim(0, max(values) * 1.05)
    plt.grid(axis='x', linestyle=':', alpha=0.3, color='gray')  # Light, subtle grid

    # Format x-axis ticks with 'ms' unit
    ax = plt.gca()
    ax.xaxis.set_major_formatter(plt.FuncFormatter(lambda x, p: f'{x:.0f} ms'))

    # Add value labels with adjusted font size
    for bar in bars:
        width = bar.get_width()
        plt.text(width, bar.get_y() + bar.get_height() / 2,
                 f'{width:.1f} ms', ha='left', va='center',
                 fontsize=8, color='black', fontweight='normal')

    # Customize labels and title for clarity and minimalism
    plt.title('EMA Performance Comparison', fontsize=10, pad=10)
    plt.xlabel('Execution Time (ms)', fontsize=9)

    # Remove top and right spines, adjust tick parameters for simplicity
    plt.gca().spines['top'].set_visible(False)
    plt.gca().spines['right'].set_visible(False)
    plt.tick_params(axis='both', which='both', length=0, labelsize=8)

    # Ensure tight layout for publication
    plt.tight_layout(pad=0.2)
    return plt

def benchmark_ema(size: int = 10_000_000) -> Dict[str, float]:
    """Benchmarks the performance of different EMA implementations.

    Generates random data and measures execution time for each implementation
    in milliseconds.

    Args:
        size: Number of data points to generate for testing (default: 10,000,000).

    Returns:
        Dictionary mapping implementation names to execution times (ms).
    """
    # Generate test data
    print(f"Generating {size:,} random data points...")
    data = np.random.random(size)
    span = 10
    results = {}

    # Benchmark pure Python implementation
    print("\nBenchmarking pure Python implementation...")
    start = time.perf_counter()
    pure_python_result = pure_python_ema(data.tolist(), span)
    results['pure python'] = (time.perf_counter() - start) * 1000
    print(f"Pure Python time: {results['pure python']:.2f} ms")

    # Benchmark Pandas implementation
    print("\nBenchmarking Pandas implementation...")
    start = time.perf_counter()
    pandas_result = pandas_ema(pd.Series(data), span)
    results['pandas'] = (time.perf_counter() - start) * 1000
    print(f"Pandas time: {results['pandas']:.2f} ms")

    # Benchmark TA-Lib implementation
    print("\nBenchmarking TA-Lib implementation...")
    start = time.perf_counter()
    talib_result = talib.EMA(data, timeperiod=span)
    results['ta-lib'] = (time.perf_counter() - start) * 1000
    print(f"TA-Lib time: {results['ta-lib']:.2f} ms")

    # Benchmark Kand implementation
    print("\nBenchmarking Kand implementation...")
    start = time.perf_counter()
    kand_result = kand.ema(data, span)
    results['kand'] = (time.perf_counter() - start) * 1000
    print(f"Kand time: {results['kand']:.2f} ms")

    return results

if __name__ == "__main__":
    # Run performance benchmarks
    results = benchmark_ema()

    # Plot and display results
    plt = plot_performance_comparison(results)
    plt.show()
