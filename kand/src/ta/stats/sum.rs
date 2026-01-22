use crate::{KandError, TAFloat};

/// Calculates the lookback period required for Sum calculation.
///
/// The lookback period represents the number of data points needed before the first valid output
/// can be calculated. For Sum, this equals the period minus 1.
///
/// # Arguments
/// * `opt_period` - The time period for Sum calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success, or error on failure
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period is less than 2
///
/// # Example
/// ```
/// use kand::stats::sum;
/// let period = 14;
/// let lookback = sum::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // lookback is period - 1
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period - 1)
}

/// Calculates the Sum indicator for a price series.
///
/// The Sum indicator represents the rolling sum of values over a specified period.
///
/// # Mathematical Formula
/// ```text
/// SUM[i] = Price[i] + Price[i-1] + ... + Price[i-period+1]
/// ```
/// Where:
/// - SUM\[i\] is the sum value at position i
/// - Price\[i\] represents the input price at position i
/// - period is the calculation timeframe
///
/// # Calculation Principle
/// 1. Calculate initial sum for the first period
/// 2. For subsequent periods, add new price and subtract oldest price
/// 3. Fill initial values before lookback period with NaN
///
/// # Arguments
/// * `input_prices` - Slice of input price values
/// * `opt_period` - The time period for Sum calculation (must be >= 2)
/// * `output_sum` - Mutable slice to store calculated Sum values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success, or error on failure
///
/// # Errors
/// * Returns `KandError::InvalidData` if input data is empty
/// * Returns `KandError::LengthMismatch` if output length doesn't match input
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::InsufficientData` if input length is less than period
/// * Returns `KandError::NaNDetected` if input contains NaN values (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::stats::sum;
/// let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let period = 3;
/// let mut output = vec![0.0; 5];
///
/// sum::sum(&input, period, &mut output).unwrap();
/// // output = [NaN, NaN, 6.0, 9.0, 12.0]
/// ```
pub fn sum(
    input_prices: &[TAFloat],
    opt_period: usize,
    output_sum: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_prices.len();
    let lookback = lookback(opt_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if output_sum.len() != len {
            return Err(KandError::LengthMismatch);
        }

        // Parameter range check
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        for price in input_prices {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate initial sum
    let mut sum_val = 0.0;
    for price in input_prices.iter().take(opt_period) {
        sum_val += *price;
    }
    output_sum[lookback] = sum_val;

    // Calculate subsequent sums incrementally
    for i in opt_period..len {
        sum_val = sum_val + input_prices[i] - input_prices[i - opt_period];
        output_sum[i] = sum_val;
    }

    // Fill initial values with NAN
    for value in output_sum.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest Sum value using incremental calculation.
///
/// This function provides an optimized way to update the Sum value when new data arrives,
/// avoiding full recalculation of the entire series.
///
/// # Arguments
/// * `input_new_price` - The newest price value to add to the sum
/// * `input_old_price` - The oldest price value to remove from the sum
/// * `prev_sum` - The previous sum value
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The new sum value on success, or error on failure
///
/// # Errors
/// * Returns `KandError::NaNDetected` if any input contains NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::stats::sum;
/// let prev_sum = 10.0;
/// let new_price = 5.0;
/// let old_price = 3.0;
///
/// let new_sum = sum::sum_inc(new_price, old_price, prev_sum).unwrap();
/// assert_eq!(new_sum, 12.0); // 10.0 + 5.0 - 3.0
/// ```
pub fn sum_inc(
    input_new_price: TAFloat,
    input_old_price: TAFloat,
    prev_sum: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_new_price.is_nan() || input_old_price.is_nan() || prev_sum.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(prev_sum + input_new_price - input_old_price)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_sum_calculation() {
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0,
        ];
        let opt_period = 14;
        let mut output_sum = vec![0.0; input_close.len()];

        sum(&input_close, opt_period, &mut output_sum).unwrap();

        // First 13 values should be NaN
        for value in output_sum.iter().take(13) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            492_849.500_000_000_06,
            492_723.700_000_000_07,
            492_543.500_000_000_06,
            492_352.100_000_000_03,
            492_195.500_000_000_06,
            492_083.000_000_000_06,
            491_853.000_000_000_06,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_sum[i + 13], *expected, epsilon = 0.0001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_sum = output_sum[13]; // First valid sum value

        // Test each incremental step
        for i in 14..19 {
            let result = sum_inc(input_close[i], input_close[i - opt_period], prev_sum).unwrap();
            assert_relative_eq!(result, output_sum[i], epsilon = 0.0001);
            prev_sum = result;
        }
    }
}
