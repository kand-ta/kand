use super::{minus_di, plus_di};
use crate::{KandError, TAFloat};

/// Calculate the lookback period required for DX calculation
///
/// # Description
/// Returns the number of data points needed before the first valid DX value can be calculated.
///
/// # Arguments
/// * `param_period` - The period used for DX calculation (typically 14)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful, or error if parameters are invalid
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::dx;
///
/// let period = 14;
/// let lookback = dx::lookback(period).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period)
}

/// Calculate Directional Movement Index (DX) for the entire input array
///
/// # Description
/// The DX indicator measures the strength of a trend by comparing positive and negative directional movements.
/// It is calculated using +DI and -DI values to determine the relative strength of the trend.
///
/// # Formula
/// ```text
/// DX = 100 * |+DI - -DI| / (+DI + -DI)
/// ```
///
/// # Calculation Steps
/// 1. Calculate +DI and -DI for the given period
/// 2. Calculate DX using the formula above
/// 3. First `param_period` values are set to NaN
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - Period for DX calculation (typically 14)
/// * `output_dx` - Output array to store DX values
/// * `output_smoothed_plus_dm` - Output array for smoothed +DM values
/// * `output_smoothed_minus_dm` - Output array for smoothed -DM values
/// * `output_smoothed_tr` - Output array for smoothed TR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds, Err otherwise
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::dx;
///
/// let input_high = vec![24.20, 24.07, 24.04, 23.87, 23.67, 23.59];
/// let input_low = vec![23.85, 23.72, 23.64, 23.37, 23.46, 23.18];
/// let input_close = vec![23.89, 23.95, 23.67, 23.78, 23.50, 23.32];
/// let param_period = 3;
/// let mut output_dx = vec![0.0; 6];
/// let mut output_smoothed_plus_dm = vec![0.0; 6];
/// let mut output_smoothed_minus_dm = vec![0.0; 6];
/// let mut output_smoothed_tr = vec![0.0; 6];
///
/// dx::dx(
///     &input_high,
///     &input_low,
///     &input_close,
///     param_period,
///     &mut output_dx,
///     &mut output_smoothed_plus_dm,
///     &mut output_smoothed_minus_dm,
///     &mut output_smoothed_tr,
/// )
/// .unwrap();
/// ```
pub fn dx(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    output_dx: &mut [TAFloat],
    output_smoothed_plus_dm: &mut [TAFloat],
    output_smoothed_minus_dm: &mut [TAFloat],
    output_smoothed_tr: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Length consistency check
        if len != input_low.len()
            || len != input_close.len()
            || len != output_dx.len()
            || len != output_smoothed_plus_dm.len()
            || len != output_smoothed_minus_dm.len()
            || len != output_smoothed_tr.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            // NaN check
            if input_high[i].is_nan() || input_low[i].is_nan() || input_close[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut plus_di_values = vec![0.0; len];
    let mut minus_di_values = vec![0.0; len];

    // Calculate +DI and -DI
    plus_di::plus_di(
        input_high,
        input_low,
        input_close,
        param_period,
        &mut plus_di_values,
        output_smoothed_plus_dm,
        output_smoothed_tr,
    )?;
    minus_di::minus_di(
        input_high,
        input_low,
        input_close,
        param_period,
        &mut minus_di_values,
        output_smoothed_minus_dm,
        output_smoothed_tr,
    )?;

    // Calculate DX
    for i in lookback..len {
        let plus_di = plus_di_values[i];
        let minus_di = minus_di_values[i];
        output_dx[i] = 100.0 * (plus_di - minus_di).abs() / (plus_di + minus_di);
    }

    // Fill initial values with NAN
    for item in output_dx.iter_mut().take(lookback) {
        *item = TAFloat::NAN;
    }

    Ok(())
}

/// Calculate the latest DX value incrementally
///
/// # Description
/// Calculates only the most recent DX value using previous smoothed values.
/// This is optimized for real-time calculations where only the latest value is needed.
///
/// # Formula
/// See the formula section in the [`dx`] function documentation.
///
/// # Arguments
/// * `input_high` - Current high price
/// * `input_low` - Current low price
/// * `prev_high` - Previous period's high price
/// * `prev_low` - Previous period's low price
/// * `prev_close` - Previous period's close price
/// * `prev_smoothed_plus_dm` - Previous smoothed +DM value
/// * `prev_smoothed_minus_dm` - Previous smoothed -DM value
/// * `prev_smoothed_tr` - Previous smoothed TR value
/// * `param_period` - Period for DX calculation (typically 14)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing:
///   - Latest DX value
///   - New smoothed +DM
///   - New smoothed -DM
///   - New smoothed TR
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::dx;
///
/// let input_high = 24.20;
/// let input_low = 23.85;
/// let prev_high = 24.07;
/// let prev_low = 23.72;
/// let prev_close = 23.95;
/// let prev_smoothed_plus_dm = 0.5;
/// let prev_smoothed_minus_dm = 0.3;
/// let prev_smoothed_tr = 1.2;
/// let param_period = 14;
///
/// let (output_dx, output_smoothed_plus_dm, output_smoothed_minus_dm, output_smoothed_tr) =
///     dx::dx_inc(
///         input_high,
///         input_low,
///         prev_high,
///         prev_low,
///         prev_close,
///         prev_smoothed_plus_dm,
///         prev_smoothed_minus_dm,
///         prev_smoothed_tr,
///         param_period,
///     )
///     .unwrap();
/// ```
pub fn dx_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        // DX requires at least 2 periods:
        // - One for initial DM and TR calculations (needs previous prices)
        // - One for the current period
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_high.is_nan()
            || input_low.is_nan()
            || prev_high.is_nan()
            || prev_low.is_nan()
            || prev_close.is_nan()
            || prev_smoothed_plus_dm.is_nan()
            || prev_smoothed_minus_dm.is_nan()
            || prev_smoothed_tr.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let (plus_di, output_smoothed_plus_dm, output_smoothed_tr) = plus_di::plus_di_inc(
        input_high,
        input_low,
        prev_high,
        prev_low,
        prev_close,
        prev_smoothed_plus_dm,
        prev_smoothed_tr,
        param_period,
    )?;

    let (minus_di, output_smoothed_minus_dm, _) = minus_di::minus_di_inc(
        input_high,
        input_low,
        prev_high,
        prev_low,
        prev_close,
        prev_smoothed_minus_dm,
        prev_smoothed_tr,
        param_period,
    )?;

    let output_dx = 100.0 * (plus_di - minus_di).abs() / (plus_di + minus_di);
    Ok((
        output_dx,
        output_smoothed_plus_dm,
        output_smoothed_minus_dm,
        output_smoothed_tr,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_dx_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0,
        ];
        let param_period = 14;
        let mut output_dx = vec![0.0; 19];
        let mut output_smoothed_plus_dm = vec![0.0; 19];
        let mut output_smoothed_minus_dm = vec![0.0; 19];
        let mut output_smoothed_tr = vec![0.0; 19];

        dx(
            &input_high,
            &input_low,
            &input_close,
            param_period,
            &mut output_dx,
            &mut output_smoothed_plus_dm,
            &mut output_smoothed_minus_dm,
            &mut output_smoothed_tr,
        )
        .unwrap();

        // First param_period values should be NaN
        for value in output_dx.iter().take(param_period) {
            assert!(value.is_nan());
        }

        // Test specific values
        let expected_values = [
            20.217_627_856_366_71,
            32.157_235_756_576_65,
            43.177_552_482_915_67,
            43.177_552_482_915_65,
            23.711_947_860_846_085,
        ];
        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_dx[i + 14], *expected, epsilon = 0.00001);
        }

        // Calculate and verify incremental values
        for i in param_period + 1..input_high.len() {
            let (result, new_smoothed_plus_dm, new_smoothed_minus_dm, new_smoothed_tr) = dx_inc(
                input_high[i],
                input_low[i],
                input_high[i - 1],
                input_low[i - 1],
                input_close[i - 1],
                output_smoothed_plus_dm[i - 1],
                output_smoothed_minus_dm[i - 1],
                output_smoothed_tr[i - 1],
                param_period,
            )
            .unwrap();

            // Compare with full calculation
            assert_relative_eq!(result, output_dx[i], epsilon = 0.00001);
            assert_relative_eq!(
                new_smoothed_plus_dm,
                output_smoothed_plus_dm[i],
                epsilon = 0.00001
            );
            assert_relative_eq!(
                new_smoothed_minus_dm,
                output_smoothed_minus_dm[i],
                epsilon = 0.00001
            );
            assert_relative_eq!(new_smoothed_tr, output_smoothed_tr[i], epsilon = 0.00001);
        }
    }
}
