use kand::{TAFloat, ohlcv::dx};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Computes the Directional Movement Index (DX) over NumPy arrays.
///
/// The DX indicator measures the strength of a trend by comparing positive and negative directional movements.
///
/// Args:
///   high: High prices as a 1-D NumPy array of type `TAFloat`.
///   low: Low prices as a 1-D NumPy array of type `TAFloat`.
///   close: Close prices as a 1-D NumPy array of type `TAFloat`.
///   period: Window size for DX calculation. Must be positive and less than input length.
///
/// Returns:
///   A tuple of four 1-D NumPy arrays containing:
///   - DX values
///   - Smoothed +DM values
///   - Smoothed -DM values
///   - Smoothed TR values
///   Each array has the same length as the input, with the first `period` elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
///   >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
///   >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
///   >>> dx, plus_dm, minus_dm, tr = kand.dx(high, low, close, 3)
///   ```
#[pyfunction]
#[pyo3(name = "dx", signature = (high, low, close, period))]
pub fn dx_py(
    py: Python,
    high: PyReadonlyArray1<TAFloat>,
    low: PyReadonlyArray1<TAFloat>,
    close: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    // Convert the input NumPy arrays to Rust slices
    let high_input = high.as_slice()?;
    let low_input = low.as_slice()?;
    let close_input = close.as_slice()?;
    let len = high_input.len();

    // Create new output arrays using vec
    let mut output_dx = vec![0.0; len];
    let mut output_smoothed_plus_dm = vec![0.0; len];
    let mut output_smoothed_minus_dm = vec![0.0; len];
    let mut output_smoothed_tr = vec![0.0; len];

    // Perform the DX calculation while releasing the GIL to allow other Python threads to run
    py.allow_threads(|| {
        dx::dx(
            high_input,
            low_input,
            close_input,
            period,
            output_dx.as_mut_slice(),
            output_smoothed_plus_dm.as_mut_slice(),
            output_smoothed_minus_dm.as_mut_slice(),
            output_smoothed_tr.as_mut_slice(),
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Convert the output arrays to Python objects
    Ok((
        output_dx.into_pyarray(py).into(),
        output_smoothed_plus_dm.into_pyarray(py).into(),
        output_smoothed_minus_dm.into_pyarray(py).into(),
        output_smoothed_tr.into_pyarray(py).into(),
    ))
}

/// Calculates the latest DX value incrementally.
///
/// Computes only the most recent DX value using previous smoothed values.
/// Optimized for real-time calculations where only the latest value is needed.
///
/// For the formula, refer to the [`dx`] function documentation.
///
/// Args:
///     input_high (float): Current high price.
///     input_low (float): Current low price.
///     prev_high (float): Previous period's high price.
///     prev_low (float): Previous period's low price.
///     prev_close (float): Previous period's close price.
///     prev_smoothed_plus_dm (float): Previous smoothed +DM value.
///     prev_smoothed_minus_dm (float): Previous smoothed -DM value.
///     prev_smoothed_tr (float): Previous smoothed TR value.
///     opt_period (int): Period for DX calculation (typically 14).
///
/// Returns:
///     tuple: A tuple containing:
///         - Latest DX value (float)
///         - New smoothed +DM (float)
///         - New smoothed -DM (float)
///         - New smoothed TR (float)
///
/// Example:
///     >>> import kand
///     >>> high, low = 24.20, 23.85
///     >>> prev_high, prev_low, prev_close = 24.07, 23.72, 23.95
///     >>> prev_smoothed_plus_dm = 0.5
///     >>> prev_smoothed_minus_dm = 0.3
///     >>> prev_smoothed_tr = 1.2
///     >>> period = 14
///     >>> dx, plus_dm, minus_dm, tr = kand.dx_inc(
///     ...     high, low, prev_high, prev_low, prev_close,
///     ...     prev_smoothed_plus_dm, prev_smoothed_minus_dm,
///     ...     prev_smoothed_tr, period)
#[pyfunction]
#[pyo3(name = "dx_inc", signature = (
    input_high,
    input_low,
    prev_high,
    prev_low,
    prev_close,
    prev_smoothed_plus_dm,
    prev_smoothed_minus_dm,
    prev_smoothed_tr,
    opt_period
))]
pub fn dx_inc_py(
    py: Python,
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    opt_period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat)> {
    // Perform the incremental DX calculation while releasing the GIL
    py.allow_threads(|| {
        dx::dx_inc(
            input_high,
            input_low,
            prev_high,
            prev_low,
            prev_close,
            prev_smoothed_plus_dm,
            prev_smoothed_minus_dm,
            prev_smoothed_tr,
            opt_period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
