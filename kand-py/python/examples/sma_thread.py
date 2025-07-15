import numpy as np
import time
from concurrent.futures import ThreadPoolExecutor
from kand import sma

# Generate test data
data = np.array([float(i) for i in range(10_000_000)])  # 10 million data points
period = 100

# Single-threaded test
start_time = time.perf_counter()
_ = sma(data, period)  # First call
_ = sma(data, period)  # Second call
single_thread_time = (time.perf_counter() - start_time) / 2
print(f"Single-threaded execution time: {single_thread_time:.4f} seconds")

# Multi-threaded test (2 threads)
with ThreadPoolExecutor(max_workers=2) as executor:
    start_time = time.perf_counter()
    futures = [executor.submit(sma, data, period) for _ in range(2)]  # Two concurrent calls
    _ = [future.result() for future in futures]
    thread_pool_time = (time.perf_counter() - start_time) / 2

print(f"Multi-threaded execution time: {thread_pool_time:.4f} seconds")
print(f"Speedup: {single_thread_time / thread_pool_time:.2f}x")
