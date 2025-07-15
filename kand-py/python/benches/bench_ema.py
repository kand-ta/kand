import numpy as np
import talib
from kand import ema
import time
import matplotlib.pyplot as plt

# Set modern, professional style for matplotlib
plt.style.use('default')  # Reset to default and customize
plt.rcParams['font.family'] = 'Arial'  # Use Arial for a clean, scientific look
plt.rcParams['font.size'] = 12
plt.rcParams['axes.linewidth'] = 0.8
plt.rcParams['figure.facecolor'] = 'white'
plt.rcParams['axes.facecolor'] = 'white'

def generate_test_data(size):
    """Generate random price data for testing."""
    return np.random.random(size) * 100

def test_batch_performance(data_sizes, period=3, runs=5):
    """Test and compare the batch computation performance between talib and kand."""
    talib_times = []
    kand_times = []

    for size in data_sizes:
        data = generate_test_data(size)
        talib.EMA(data, timeperiod=period)
        ema(data, period=period)

        talib_start = time.perf_counter()
        for _ in range(runs):
            talib_ema = talib.EMA(data, timeperiod=period)
        talib_times.append((time.perf_counter() - talib_start) / runs)

        kand_start = time.perf_counter()
        for _ in range(runs):
            kand_ema = ema(data, period=period)
        kand_times.append((time.perf_counter() - kand_start) / runs)

    return talib_times, kand_times

def plot_batch_results(data_sizes, talib_times, kand_times):
    """Create a professional grouped bar plot for performance comparison."""
    relative_diff = [(t - k) / k * 100 for t, k in zip(talib_times, kand_times)]

    # Create figure with two subplots
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8), height_ratios=[2, 1],
                                   sharex=False)
    fig.subplots_adjust(hspace=0.4, bottom=0.20)  # Increase bottom margin

    # Determine scaling factor for time based on minimum time
    min_time = min(min(talib_times), min(kand_times))
    if min_time < 0.001:
        scale = 1000
        unit = 'ms'
    else:
        scale = 1
        unit = 's'
    scaled_talib_times = [t * scale for t in talib_times]
    scaled_kand_times = [k * scale for k in kand_times]

    # Plot 1: Grouped Bar Plot for Absolute Times
    bar_width = 0.35
    index = np.arange(len(data_sizes))

    bars1 = ax1.bar(index - bar_width/2, scaled_talib_times, bar_width,
                    label='TA-Lib EMA', color='#1f77b4', edgecolor='black', linewidth=0.5)
    bars2 = ax1.bar(index + bar_width/2, scaled_kand_times, bar_width,
                    label='Kand EMA', color='#ff7f0e', edgecolor='black', linewidth=0.5)

    # Dynamically set precision based on scaled time values
    max_time = max(max(scaled_talib_times), max(scaled_kand_times))
    if max_time < 1:
        precision = 5
    elif max_time < 10:
        precision = 4
    else:
        precision = 3

    # Add time labels above bars with slight offset
    for bars in [bars1, bars2]:
        for bar in bars:
            height = bar.get_height()
            ax1.text(bar.get_x() + bar.get_width()/2., height + 0.001 * scale,
                     f'{height:.{precision}f}', ha='center', va='bottom', fontsize=9)

    # Customize Plot 1
    ax1.set_ylabel(f'Time ({unit})', fontsize=12)
    ax1.set_title('EMA Computation Time', fontsize=14, pad=10)
    ax1.set_xticks(index)
    ax1.set_xticklabels([f'{x:,}' for x in data_sizes], rotation=45, ha='right')
    ax1.grid(True, linestyle='--', alpha=0.3, which='both')
    ax1.legend(loc='upper left', fontsize=10, frameon=False)
    ax1.spines['top'].set_visible(False)
    ax1.spines['right'].set_visible(False)
    ax1.spines['left'].set_color('gray')
    ax1.spines['bottom'].set_color('gray')

    # Plot 2: Relative Performance Difference
    bars = ax2.bar(index, relative_diff, bar_width * 1.2,
                   color='#2ca02c', edgecolor='black', linewidth=0.5)

    # Add percentage labels with dynamic positioning
    for bar in bars:
        height = bar.get_height()
        label_y = height + 1 if height >= 0 else height - 1  # Dynamic offset
        ax2.text(bar.get_x() + bar.get_width()/2., label_y,
                 f'{height:.1f}%', ha='center', va='bottom' if height >= 0 else 'top', fontsize=9)

    # Customize Plot 2
    ax2.set_xlabel('Data Size', fontsize=12)
    ax2.set_ylabel('Overhead (%)', fontsize=12)
    ax2.set_xticks(index)
    ax2.set_xticklabels([f'{x:,}' for x in data_sizes], rotation=45, ha='right')
    ax2.grid(True, linestyle='--', alpha=0.3, which='both')
    ax2.spines['top'].set_visible(False)
    ax2.spines['right'].set_visible(False)
    ax2.spines['left'].set_color('gray')
    ax2.spines['bottom'].set_color('gray')

    # Adjust layout and save
    plt.tight_layout()

    plt.savefig('batch_ema_performance.png', dpi=600, bbox_inches='tight',
                facecolor='white', edgecolor='none',
                pil_kwargs={'height': 1920, 'width': 2400})

    # plt.show()

def main():
    """Execute the main benchmark suite."""
    data_sizes = [
        50000,
        100000,
        250000,
        500000,
        1000000,
        2500000,
        5000000,
        10000000
    ]
    period = 30
    runs = 1000

    print("\nRunning performance tests...")
    talib_times, kand_times = test_batch_performance(data_sizes, period, runs)
    plot_batch_results(data_sizes, talib_times, kand_times)

    print("\nBatch Computation Results:")
    print("-" * 60)
    print("   Data Size | TA-Lib (s) |   Kand (s) |  Speedup")
    print("-" * 60)

    for size, t_time, k_time in zip(data_sizes, talib_times, kand_times):
        speedup = t_time / k_time
        print(f"{size:10,d} | {t_time:9.6f} | {k_time:9.6f} | {speedup:7.2f}x")

    print("-" * 60)

if __name__ == "__main__":
    main()
