use crate::{EPSILON, KandError, TAFloat};

/// Calculates the lookback period required for Minimum Value calculation.
///
/// Returns the number of data points needed before the first valid output can be calculated.
/// For MIN, this is one less than the specified period.
///
/// # Arguments
/// * `param_period` - The time period for MIN calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period is less than 2
///
/// # Example
/// ```
/// use kand::stats::min;
/// let period = 14;
/// let lookback = min::lookback(period).unwrap();
/// assert_eq!(lookback, 13);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates the Minimum Value (MIN) for a series of prices over a specified period.
///
/// The MIN indicator finds the lowest price value within a given time period. For each
/// calculation point, it looks back over the specified number of periods and returns
/// the minimum value found.
///
/// # Mathematical Formula
/// ```text
/// MIN[i] = min(price[i], price[i-1], ..., price[i-n+1])
/// ```
/// Where:
/// - i is the current index
/// - n is the time period
/// - price[] represents the input price series
///
/// # Calculation Steps
/// 1. For each point, look back n periods
/// 2. Find the minimum value in that range
/// 3. Store the minimum as the current MIN value
///
/// # Arguments
/// * `input_prices` - Array of input price values
/// * `param_period` - The time period for MIN calculation (must be >= 2)
/// * `output_min` - Array to store the calculated MIN values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success
///
/// # Errors
/// * Returns `KandError::InvalidData` if input array is empty
/// * Returns `KandError::LengthMismatch` if output length doesn't match input
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::InsufficientData` if input length is less than period
/// * Returns `KandError::NaNDetected` if any input value is NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::stats::min;
/// let input = vec![10.0, 8.0, 6.0, 7.0, 9.0];
/// let period = 3;
/// let mut output = vec![0.0; 5];
///
/// min::min(&input, period, &mut output).unwrap();
/// // output = [NaN, NaN, 6.0, 6.0, 6.0]
/// ```
pub fn min(
    input_prices: &[TAFloat],
    param_period: usize,
    output_min: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_prices.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if output_min.len() != len {
            return Err(KandError::LengthMismatch);
        }

        // Parameter range check
        if param_period < 2 {
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

    // Calculate MIN values
    for i in lookback..len {
        let mut min_val = input_prices[i - lookback];
        for price in input_prices.iter().take(i + 1).skip(i - lookback + 1) {
            if *price < min_val {
                min_val = *price;
            }
        }
        output_min[i] = min_val;
    }

    // Fill initial values with NAN
    for value in output_min.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest Minimum Value incrementally using the previous MIN value.
///
/// This function provides an optimized way to calculate the current MIN value
/// when you already have the previous MIN value and are adding a new price point.
///
/// # Arguments
/// * `input_price` - The new price value to include in calculation
/// * `prev_min` - The previous MIN value
/// * `prev_price` - The price value that will drop out of the period
/// * `param_period` - The time period for MIN calculation (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The new MIN value on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::NaNDetected` if any input value is NaN (with "`check-nan`" feature)
/// * Returns `KandError::InsufficientData` if full recalculation is needed
///
/// # Example
/// ```
/// use kand::stats::min;
/// let new_price = 15.0;
/// let prev_min = 12.0;
/// let dropping_price = 14.0;
/// let period = 14;
///
/// let new_min = min::min_inc(new_price, prev_min, dropping_price, period).unwrap();
/// assert_eq!(new_min, 12.0);
/// ```
pub fn min_inc(
    input_price: TAFloat,
    prev_min: TAFloat,
    prev_price: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_price.is_nan() || prev_min.is_nan() || prev_price.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    // If the new price is less than previous min, it becomes the new min
    if input_price < prev_min {
        return Ok(input_price);
    }

    // If the price being removed was the previous min,
    // we need to scan the period for the new min
    if (prev_price - prev_min).abs() < EPSILON {
        // In this case we need the full period data to recalculate
        return Err(KandError::InsufficientData);
    }

    // Otherwise the previous min is still valid
    Ok(prev_min)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_min_calculation() {
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3,
        ];
        let param_period = 14;
        let mut output_min = vec![0.0; input_close.len()];

        min(&input_close, param_period, &mut output_min).unwrap();

        // First 13 values should be NaN
        for value in output_min.iter().take(13) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            35160.7, 35090.3, 35041.2, 34999.3, 34999.3, 34999.3, 34999.3, 34939.5, 34939.5,
            34939.5, 34939.5, 34939.5, 34939.5, 34939.5, 34939.5, 34939.5, 34939.5,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_min[i + 13], *expected, epsilon = 0.0001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_min = output_min[13]; // First valid min value

        // Test each incremental step
        for i in 14..19 {
            let result = min_inc(
                input_close[i],
                prev_min,
                input_close[i - param_period],
                param_period,
            )
            .unwrap();
            assert_relative_eq!(result, output_min[i], epsilon = 0.0001);
            prev_min = result;
        }
    }
}
