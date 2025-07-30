use crate::{
    TAFloat,
    error::KandError,
    helper::{highest_bars, lowest_bars},
};

/// Calculates the lookback period required for Midpoint Price calculation.
///
/// # Arguments
/// * `param_period` - The time period used for calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - Returns `param_period - 1` on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::midprice;
/// let lookback = midprice::lookback(14).unwrap();
/// assert_eq!(lookback, 13);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates Midpoint Price for a price series.
///
/// The Midpoint Price is a technical indicator that represents the mean value between the highest high
/// and lowest low prices over a specified period.
///
/// # Mathematical Formula
/// ```text
/// MIDPRICE[i] = (Highest High[i-n+1...i] + Lowest Low[i-n+1...i]) / 2
/// ```
/// Where:
/// - n is the period
/// - i is the current index
///
/// # Calculation Steps
/// 1. Find highest high price in the period window
/// 2. Find lowest low price in the period window
/// 3. Calculate arithmetic mean of these values
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_period` - Calculation period (must be >= 2)
/// * `output_midprice` - Buffer to store calculated midpoint prices
/// * `output_highest_high` - Buffer to store highest highs
/// * `output_lowest_low` - Buffer to store lowest lows
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok on success
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output array lengths differ
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input contains NaN (with `check-nan`)
///
/// # Example
/// ```
/// use kand::ohlcv::midprice;
///
/// let high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let mut midprice = vec![0.0; 5];
/// let mut highest = vec![0.0; 5];
/// let mut lowest = vec![0.0; 5];
///
/// midprice::midprice(&high, &low, 3, &mut midprice, &mut highest, &mut lowest).unwrap();
/// ```
pub fn midprice(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    param_period: usize,
    output_midprice: &mut [TAFloat],
    output_highest_high: &mut [TAFloat],
    output_lowest_low: &mut [TAFloat],
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
            || len != output_midprice.len()
            || len != output_highest_high.len()
            || len != output_lowest_low.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            // NaN check
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate midpoint price for each window
    for i in lookback..len {
        let highest_idx = highest_bars(input_high, i, param_period)?;
        let lowest_idx = lowest_bars(input_low, i, param_period)?;

        let highest_high = input_high[i - highest_idx];
        let lowest_low = input_low[i - lowest_idx];

        output_highest_high[i] = highest_high;
        output_lowest_low[i] = lowest_low;
        output_midprice[i] = f64::midpoint(highest_high, lowest_low);
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_midprice[i] = TAFloat::NAN;
        output_highest_high[i] = TAFloat::NAN;
        output_lowest_low[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Incrementally calculates the next Midpoint Price value.
///
/// Provides optimized calculation of the next value when new data arrives, avoiding
/// recalculation of the entire series.
///
/// # Arguments
/// * `input_high` - Current high price
/// * `input_low` - Current low price
/// * `prev_highest_high` - Previous period's highest high
/// * `prev_lowest_low` - Previous period's lowest low
/// * `param_period` - Calculation period (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Returns (midprice, `new_highest_high`, `new_lowest_low`)
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::NaNDetected` - If any input contains NaN (with `check-nan`)
///
/// # Example
/// ```
/// use kand::ohlcv::midprice;
///
/// let (midprice, highest, lowest) = midprice::midprice_inc(
///     10.5, // current high
///     9.8,  // current low
///     10.2, // previous highest high
///     9.5,  // previous lowest low
///     14,   // period
/// )
/// .unwrap();
/// ```
pub const fn midprice_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_highest_high: TAFloat,
    prev_lowest_low: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan()
            || input_low.is_nan()
            || prev_highest_high.is_nan()
            || prev_lowest_low.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let new_highest_high = input_high.max(prev_highest_high);
    let new_lowest_low = input_low.min(prev_lowest_low);
    let midprice = f64::midpoint(new_highest_high, new_lowest_low);

    Ok((midprice, new_highest_high, new_lowest_low))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_midprice_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0,
        ];
        let param_period = 14;
        let mut output_midprice = vec![0.0; input_high.len()];
        let mut output_highest_high = vec![0.0; input_high.len()];
        let mut output_lowest_low = vec![0.0; input_high.len()];

        midprice(
            &input_high,
            &input_low,
            param_period,
            &mut output_midprice,
            &mut output_highest_high,
            &mut output_lowest_low,
        )
        .unwrap();

        // First 13 values should be NaN
        for i in 0..13 {
            assert!(output_midprice[i].is_nan());
            assert!(output_highest_high[i].is_nan());
            assert!(output_lowest_low[i].is_nan());
        }

        // Compare with known values from the CSV data
        let expected_values = [
            35206.1, 35180.8, 35151.3, 35115.8, 35115.8, 35115.8, 35115.8, 35106.55, 35083.5,
            35076.0, 35076.0, 35076.0,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_midprice[i + 13], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_highest_high = output_highest_high[13];
        let mut prev_lowest_low = output_lowest_low[13];

        for i in 14..19 {
            let (midprice, new_highest_high, new_lowest_low) = midprice_inc(
                input_high[i],
                input_low[i],
                prev_highest_high,
                prev_lowest_low,
                param_period,
            )
            .unwrap();

            assert_relative_eq!(midprice, output_midprice[i], epsilon = 0.0001);
            prev_highest_high = new_highest_high;
            prev_lowest_low = new_lowest_low;
        }
    }
}
