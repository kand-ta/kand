use crate::{KandError, TAFloat};

/// Returns the lookback period required for True Range (TR) calculation
///
/// # Description
/// The lookback period indicates the minimum number of data points needed before
/// the first valid True Range value can be calculated.
///
/// # Arguments
/// * None
///
/// # Returns
/// * `Result<usize, KandError>` - Returns 1, as TR calculation requires the previous close price
///
/// # Errors
/// * None
///
/// # Example
/// ```
/// use kand::ohlcv::trange;
/// let lookback = trange::lookback().unwrap();
/// assert_eq!(lookback, 1);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(1)
}

/// Calculates True Range (TR) values for a series of price data
///
/// # Description
/// True Range is a technical indicator that measures market volatility by taking into account
/// gaps and limit moves between trading periods. It expands the regular high-low range when
/// there are price gaps between periods.
///
/// # Mathematical Formula
/// ```text
/// TR = max(
///     high[i] - low[i],                 // Current period's range
///     |high[i] - close[i-1]|,           // Current high to previous close
///     |low[i] - close[i-1]|             // Current low to previous close
/// )
/// ```
///
/// # Calculation Principles
/// 1. Calculate the current period's high-low range
/// 2. Calculate the absolute difference between current high and previous close
/// 3. Calculate the absolute difference between current low and previous close
/// 4. Take the maximum of these three values
///
/// # Arguments
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `input_close` - Array of closing prices for each period
/// * `output_trange` - Array to store calculated TR values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok value on success
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::trange;
///
/// let high = vec![10.0, 12.0, 15.0];
/// let low = vec![8.0, 9.0, 11.0];
/// let close = vec![9.0, 11.0, 14.0];
/// let mut tr = vec![0.0; 3];
///
/// trange::trange(&high, &low, &close, &mut tr).unwrap();
/// assert!(tr[0].is_nan()); // First value is NaN
/// assert_eq!(tr[1], 3.0); // max(3, 3, 2)
/// assert_eq!(tr[2], 4.0); // max(4, 4, 3)
/// ```
pub fn trange(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    output_trange: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback()?;
    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len != input_low.len() || len != input_close.len() || len != output_trange.len() {
            return Err(KandError::LengthMismatch);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
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

    // First value is NAN since we need previous close
    output_trange[0] = TAFloat::NAN;

    // Calculate True Range for remaining values
    for i in 1..len {
        let h_l = input_high[i] - input_low[i];
        let h_pc = (input_high[i] - input_close[i - 1]).abs();
        let l_pc = (input_low[i] - input_close[i - 1]).abs();
        output_trange[i] = h_l.max(h_pc).max(l_pc);
    }

    // Fill initial values with NAN
    for value in output_trange.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single True Range value for the most recent period
///
/// # Description
/// This function provides an optimized way to calculate TR for real-time data updates,
/// requiring only the current period's prices and previous close.
///
/// # Mathematical Formula
/// ```text
/// TR = max(
///     high - low,                // Current period's range
///     |high - prev_close|,       // High to previous close
///     |low - prev_close|         // Low to previous close
/// )
/// ```
///
/// # Arguments
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `prev_close` - Previous period's closing price
///
/// # Returns
/// * `Result<TAFloat, KandError>` - Calculated TR value
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::trange;
///
/// let tr = trange::trange_inc(12.0, 9.0, 11.0).unwrap();
/// assert_eq!(tr, 3.0); // max(3, 1, 2)
/// ```
pub fn trange_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_close: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_high.is_nan() || input_low.is_nan() || prev_close.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let h_l = input_high - input_low;
    let h_pc = (input_high - prev_close).abs();
    let l_pc = (input_low - prev_close).abs();
    Ok(h_l.max(h_pc).max(l_pc))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_trange_calculation() {
        let input_high = vec![35266.0, 35247.5, 35235.7, 35190.8, 35182.0];
        let input_low = vec![35216.1, 35206.5, 35180.0, 35130.7, 35153.6];
        let input_close = vec![35216.1, 35221.4, 35190.7, 35170.0, 35181.5];
        let mut output_trange = vec![0.0; 5];

        trange(&input_high, &input_low, &input_close, &mut output_trange).unwrap();

        assert!(output_trange[0].is_nan());
        assert_relative_eq!(output_trange[1], 41.0, epsilon = 0.00001);
        assert_relative_eq!(output_trange[2], 55.7, epsilon = 0.00001);
        assert_relative_eq!(output_trange[3], 60.1, epsilon = 0.00001);
        assert_relative_eq!(output_trange[4], 28.4, epsilon = 0.00001);

        // Test each incremental step matches regular calculation
        for i in 1..5 {
            let result = trange_inc(input_high[i], input_low[i], input_close[i - 1]).unwrap();
            assert_relative_eq!(result, output_trange[i], epsilon = 0.00001);
        }
    }
}
