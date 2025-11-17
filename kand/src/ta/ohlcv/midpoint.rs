use crate::{KandError, TAFloat};

/// Calculates the lookback period required for Midpoint calculation.
///
/// # Description
/// Returns the minimum number of data points needed before the first valid output can be calculated.
/// For the Midpoint indicator, this equals `period - 1`.
///
/// # Arguments
/// * `opt_period` - The time period used for calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::midpoint;
///
/// let period = 14;
/// let lookback = midpoint::lookback(period).unwrap();
/// assert_eq!(lookback, 13);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period - 1)
}

/// Calculates Midpoint values for a price series.
///
/// # Description
/// The Midpoint is a technical indicator that represents the arithmetic mean of the highest and lowest
/// prices over a specified period. It helps identify average price levels and potential support/resistance areas.
///
/// # Mathematical Formula
/// For each period:
/// ```text
/// MIDPOINT = (Highest Price + Lowest Price) / 2
/// ```
/// Where:
/// - Highest Price = Maximum price within the period
/// - Lowest Price = Minimum price within the period
///
/// # Calculation Steps
/// 1. For each period window:
///    - Find the highest and lowest prices
///    - Calculate midpoint as their average
/// 2. Fill initial values before lookback period with NaN
///
/// # Arguments
/// * `input_price` - Array of price values
/// * `opt_period` - Time period for calculation (must be >= 2)
/// * `output_midpoint` - Array to store calculated Midpoint values
/// * `output_highest` - Array to store highest values for each period
/// * `output_lowest` - Array to store lowest values for each period
///
/// # Returns
/// * `Result<(), KandError>` - Unit type on success
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input/output array lengths don't match
/// * `KandError::InvalidParameter` - If `opt_period` is less than 2
/// * `KandError::InsufficientData` - If input length is less than lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Examples
/// ```
/// use kand::ohlcv::midpoint;
///
/// let input_price = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let opt_period = 3;
/// let mut output_midpoint = vec![0.0; 5];
/// let mut output_highest = vec![0.0; 5];
/// let mut output_lowest = vec![0.0; 5];
///
/// midpoint::midpoint(
///     &input_price,
///     opt_period,
///     &mut output_midpoint,
///     &mut output_highest,
///     &mut output_lowest,
/// )
/// .unwrap();
/// ```
pub fn midpoint(
    input_price: &[TAFloat],
    opt_period: usize,
    output_midpoint: &mut [TAFloat],
    output_highest: &mut [TAFloat],
    output_lowest: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_price.len();
    let lookback = lookback(opt_period)?;

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
        if len != output_midpoint.len() || len != output_highest.len() || len != output_lowest.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for &price in input_price.iter().take(len) {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate midpoint for each window
    for i in lookback..len {
        let start_idx = i + 1 - opt_period;
        let mut highest = input_price[start_idx];
        let mut lowest = input_price[start_idx];

        // Find highest and lowest prices in the period
        for &price in input_price.iter().take(i + 1).skip(start_idx + 1) {
            highest = highest.max(price);
            lowest = lowest.min(price);
        }

        output_highest[i] = highest;
        output_lowest[i] = lowest;
        output_midpoint[i] = f64::midpoint(highest, lowest);
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_midpoint[i] = TAFloat::NAN;
        output_highest[i] = TAFloat::NAN;
        output_lowest[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the next Midpoint value incrementally.
///
/// # Description
/// Provides an optimized way to calculate the next Midpoint value when new data arrives,
/// without recalculating the entire series. Updates the highest and lowest values with new price.
///
/// # Mathematical Formula
/// ```text
/// MIDPOINT = (Highest Price + Lowest Price) / 2
/// ```
/// Where:
/// - Highest Price = max(current price, previous highest)
/// - Lowest Price = min(current price, previous lowest)
///
/// # Calculation Steps
/// 1. Compare new price with previous highest/lowest
/// 2. Update highest/lowest values
/// 3. Calculate new midpoint
///
/// # Arguments
/// * `input_price` - Current price value
/// * `prev_highest` - Previous highest value
/// * `prev_lowest` - Previous lowest value
/// * `opt_period` - Time period for calculation (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple (midpoint, `new_highest`, `new_lowest`) on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` is less than 2
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Examples
/// ```
/// use kand::ohlcv::midpoint;
///
/// let current_price = 15.0;
/// let prev_highest = 16.0;
/// let prev_lowest = 14.0;
/// let period = 14;
///
/// let (midpoint, new_highest, new_lowest) =
///     midpoint::midpoint_inc(current_price, prev_highest, prev_lowest, period).unwrap();
/// ```
pub const fn midpoint_inc(
    input_price: TAFloat,
    prev_highest: TAFloat,
    prev_lowest: TAFloat,
    opt_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_price.is_nan() || prev_highest.is_nan() || prev_lowest.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let new_highest = input_price.max(prev_highest);
    let new_lowest = input_price.min(prev_lowest);
    let midpoint = f64::midpoint(new_highest, new_lowest);

    Ok((midpoint, new_highest, new_lowest))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_midpoint_calculation() {
        let input_price = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0,
        ];
        let opt_period = 14;
        let mut output_midpoint = vec![0.0; input_price.len()];
        let mut output_highest = vec![0.0; input_price.len()];
        let mut output_lowest = vec![0.0; input_price.len()];

        midpoint(
            &input_price,
            opt_period,
            &mut output_midpoint,
            &mut output_highest,
            &mut output_lowest,
        )
        .unwrap();

        // First 13 values should be NaN
        for i in 0..13 {
            assert!(output_midpoint[i].is_nan());
            assert!(output_highest[i].is_nan());
            assert!(output_lowest[i].is_nan());
        }

        // Compare with known values from the CSV data
        let expected_values = [
            35207.65, 35172.45, 35147.90, 35126.95, 35126.95, 35126.95, 35125.60, 35095.70,
            35084.70, 35084.70, 35084.70, 35084.70,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_midpoint[i + 13], *expected, epsilon = 0.01);
        }

        // Test incremental calculation
        let mut prev_highest = output_highest[13];
        let mut prev_lowest = output_lowest[13];

        for i in 14..19 {
            let (midpoint, new_highest, new_lowest) =
                midpoint_inc(input_price[i], prev_highest, prev_lowest, opt_period).unwrap();

            assert_relative_eq!(midpoint, output_midpoint[i], epsilon = 0.01);
            prev_highest = new_highest;
            prev_lowest = new_lowest;
        }
    }
}
