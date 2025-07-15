import numpy as np
from kand import sma
import time

data = np.array([float(i) for i in range(10_000_000)])

start_time = time.perf_counter()
result = sma(data, 10)
end_time = time.perf_counter()

print(f"Result: {result}")
print(f"Result type: {type(result)}")
print(f"Result dtype: {result.dtype}")
print(f"Execution time: {end_time - start_time:.4f} seconds")

print(sma.__doc__)
