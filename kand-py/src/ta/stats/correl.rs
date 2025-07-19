use kand::{TAFloat, stats::correl};
use numpy::{IntoPyArray, PyArray1, PyReadonlyArray1};
use pyo3::prelude::*;

/// Calculate Pearson's Correlation Coefficient between two NumPy arrays
///
/// The Pearson Correlation Coefficient measures the linear correlation between two variables,
/// returning a value between -1 and +1, where:
/// - +1 indicates perfect positive correlation
/// - -1 indicates perfect negative correlation
/// - 0 indicates no linear correlation
///
/// Args:
///   input0: First input series as a 1-D NumPy array of type `TAFloat`.
///   input1: Second input series as a 1-D NumPy array of type `TAFloat`.
///   period: Period for calculation (must be >= 2).
///
/// Returns:
///   A tuple of six 1-D NumPy arrays containing:
///   - Correlation coefficient values
///   - Running sum of series 0
///   - Running sum of series 1
///   - Running sum of squares of series 0
///   - Running sum of squares of series 1
///   - Running sum of products
///   Each array has the same length as the input, with the first (period-1) elements containing NaN values.
///
/// Examples:
///   ```python
///   >>> import numpy as np
///   >>> import kand
///   >>> series1 = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
///   >>> series2 = np.array([2.0, 4.0, 6.0, 8.0, 10.0])
///   >>> correl, sum0, sum1, sum0_sq, sum1_sq, sum01 = kand.correl(series1, series2, 3)
///   ```
#[pyfunction]
#[pyo3(name = "correl", signature = (input0, input1, period))]
pub fn correl_py(
    py: Python,
    input0: PyReadonlyArray1<TAFloat>,
    input1: PyReadonlyArray1<TAFloat>,
    period: usize,
) -> PyResult<(
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
    Py<PyArray1<TAFloat>>,
)> {
    let input0_array = input0.as_slice()?;
    let input1_array = input1.as_slice()?;
    let len = input0_array.len();

    if len != input1_array.len() {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "Input arrays must have the same length",
        ));
    }

    let mut output_correl = vec![0.0; len];
    let mut output_sum_0 = vec![0.0; len];
    let mut output_sum_1 = vec![0.0; len];
    let mut output_sum_0_sq = vec![0.0; len];
    let mut output_sum_1_sq = vec![0.0; len];
    let mut output_sum_01 = vec![0.0; len];

    py.allow_threads(|| {
        correl::correl(
            input0_array,
            input1_array,
            period,
            &mut output_correl,
            &mut output_sum_0,
            &mut output_sum_1,
            &mut output_sum_0_sq,
            &mut output_sum_1_sq,
            &mut output_sum_01,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    Ok((
        output_correl.into_pyarray(py).into(),
        output_sum_0.into_pyarray(py).into(),
        output_sum_1.into_pyarray(py).into(),
        output_sum_0_sq.into_pyarray(py).into(),
        output_sum_1_sq.into_pyarray(py).into(),
        output_sum_01.into_pyarray(py).into(),
    ))
}

/// Calculate the latest Correlation value incrementally
///
/// This function provides an optimized way to update the Correlation value when new data arrives,
/// avoiding full recalculation of the entire series.
///
/// Args:
///   new0: The newest value from series 0 to add
///   new1: The newest value from series 1 to add
///   old0: The oldest value from series 0 to remove
///   old1: The oldest value from series 1 to remove
///   prev_sum0: Previous sum of series 0
///   prev_sum1: Previous sum of series 1
///   prev_sum0_sq: Previous sum of squares of series 0
///   prev_sum1_sq: Previous sum of squares of series 1
///   prev_sum01: Previous sum of products
///   period: Period for calculation (must be >= 2)
///
/// Returns:
///   A tuple containing:
///   - New correlation value
///   - New sum of series 0
///   - New sum of series 1
///   - New sum of squares of series 0
///   - New sum of squares of series 1
///   - New sum of products
///
/// Examples:
///   ```python
///   >>> import kand
///   >>> correl, sum0, sum1, sum0_sq, sum1_sq, sum01 = kand.correl_inc(
///   ...     4.0,    # new value for series 0
///   ...     8.0,    # new value for series 1
///   ...     1.0,    # old value for series 0
///   ...     2.0,    # old value for series 1
///   ...     6.0,    # previous sum of series 0
///   ...     12.0,   # previous sum of series 1
///   ...     14.0,   # previous sum of squares of series 0
///   ...     56.0,   # previous sum of squares of series 1
///   ...     28.0,   # previous sum of products
///   ...     3       # period
///   ... )
///   ```
#[pyfunction]
#[pyo3(name = "correl_inc")]
pub fn correl_inc_py(
    py: Python,
    new0: TAFloat,
    new1: TAFloat,
    old0: TAFloat,
    old1: TAFloat,
    prev_sum0: TAFloat,
    prev_sum1: TAFloat,
    prev_sum0_sq: TAFloat,
    prev_sum1_sq: TAFloat,
    prev_sum01: TAFloat,
    period: usize,
) -> PyResult<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat, TAFloat)> {
    py.allow_threads(|| {
        correl::correl_inc(
            new0,
            new1,
            old0,
            old1,
            prev_sum0,
            prev_sum1,
            prev_sum0_sq,
            prev_sum1_sq,
            prev_sum01,
            period,
        )
    })
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
