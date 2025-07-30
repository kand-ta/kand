use crate::{KandError, TAFloat};

/// Returns the lookback period required for ROC (Rate of Change) calculation
///
/// # Description
/// Calculates the minimum number of data points needed before the first valid ROC value can be computed.
/// The lookback period equals the ROC period parameter since we need that many previous prices to calculate
/// the first value.
///
/// # Parameters
/// * `param_period` - The time period used for ROC calculation (usize)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if parameters are valid
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 1 (when "check" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::roc;
/// let lookback = roc::lookback(14).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 1 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period)
}

/// Calculates Rate of Change (ROC) technical indicator for a price series
///
/// # Description
/// The Rate of Change (ROC) is a momentum oscillator that measures the percentage change in price
/// between the current price and the price n periods ago. ROC indicates both the speed and magnitude
/// of price movements, making it useful for identifying overbought/oversold conditions and divergences.
///
/// # Mathematical Formula
/// ```text
/// ROC = ((Current Price - Price n periods ago) / Price n periods ago) * 100
/// ```
///
/// # Calculation Principles
/// 1. For each data point after the lookback period:
///    - Take current price and price from n periods ago
///    - Calculate percentage change between these prices
///    - Multiply by 100 to get percentage value
/// 2. Initial values within lookback period are set to NaN
///
/// # Parameters
/// * `input_price` - Array of price values (slice of type `TAFloat`)
/// * `param_period` - Number of periods to look back (usize)
/// * `output_roc` - Array to store calculated ROC values, must be same length as `input_price` (mutable slice of type `TAFloat`)
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` < 1
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If input contains NaN values (with "`check-nan`")
/// * `KandError::InvalidData` - If division by zero occurs (with "`check-nan`")
///
/// # Example
/// ```
/// use kand::ohlcv::roc;
///
/// let input_price = vec![10.0, 10.5, 11.2, 10.8, 11.5];
/// let param_period = 2;
/// let mut output_roc = vec![0.0; 5];
///
/// roc::roc(&input_price, param_period, &mut output_roc).unwrap();
/// ```
pub fn roc(
    input_price: &[TAFloat],
    param_period: usize,
    output_roc: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_price.len();
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
        if len != output_roc.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for price in input_price {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate ROC values
    for i in lookback..len {
        let current_price = input_price[i];
        let prev_price = input_price[i - param_period];

        #[cfg(feature = "check-nan")]
        {
            if prev_price == 0.0 {
                return Err(KandError::InvalidData);
            }
        }

        output_roc[i] = (current_price - prev_price) / prev_price * 100.0;
    }

    // Fill initial values with NAN
    for value in output_roc.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single ROC value incrementally for streaming data
///
/// # Description
/// Provides an optimized way to calculate the latest ROC value when new data arrives,
/// without recalculating the entire series. This is particularly useful for real-time
/// data processing and streaming applications.
///
/// # Mathematical Formula
/// ```text
/// ROC = ((Current Price - Price n periods ago) / Price n periods ago) * 100
/// ```
///
/// # Parameters
/// * `current_price` - The most recent price value (type `TAFloat`)
/// * `prev_price` - The price from n periods ago (type `TAFloat`)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The calculated ROC value if successful
///
/// # Errors
/// * `KandError::NaNDetected` - If either input is NaN (with "`check-nan`")
/// * `KandError::InvalidData` - If `prev_price` is zero (with "`check-nan`")
///
/// # Example
/// ```
/// use kand::ohlcv::roc::roc_inc;
///
/// let current_price = 11.5;
/// let prev_price = 10.0;
///
/// let roc_value = roc_inc(current_price, prev_price).unwrap();
/// assert_eq!(roc_value, 15.0); // ((11.5 - 10.0) / 10.0) * 100
/// ```
pub fn roc_inc(current_price: TAFloat, prev_price: TAFloat) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if current_price.is_nan() || prev_price.is_nan() {
            return Err(KandError::NaNDetected);
        }
        // Division by zero check
        if prev_price == 0.0 {
            return Err(KandError::InvalidData);
        }
    }

    Ok((current_price - prev_price) / prev_price * 100.0)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_roc_calculation() {
        let input_price = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];
        let param_period = 14;
        let mut output_roc = vec![0.0; input_price.len()];

        roc(&input_price, param_period, &mut output_roc).unwrap();

        // First 13 values should be NaN
        for value in output_roc.iter().take(14) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            -0.357_222_974_718_940_4,
            -0.511_620_776_005_505_8,
            -0.543_893_699_187_547_6,
            -0.445_265_851_578_047_2,
            -0.319_770_333_840_230_24,
            -0.652_397_133_991_022_8,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_roc[i + 14], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        for i in 15..20 {
            let result = roc_inc(input_price[i], input_price[i - param_period]).unwrap();
            assert_relative_eq!(result, output_roc[i], epsilon = 0.0001);
        }
    }
}
