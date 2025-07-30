use crate::{KandError, TAFloat};

/// Calculates the lookback period required for MEDPRICE calculation.
///
/// # Description
/// Returns the number of data points needed for calculating the Median Price indicator.
/// Since MEDPRICE only requires current high and low prices, the lookback period is 0.
///
/// # Returns
/// * `Result<usize, KandError>` - Returns 0 on success
///
/// # Errors
/// No errors are returned by this function.
///
/// # Example
/// ```
/// use kand::ohlcv::medprice;
/// let lookback = medprice::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates Median Price (MEDPRICE) for a price series.
///
/// # Description
/// The Median Price is a technical analysis indicator that represents the middle point between
/// high and low prices for each period. It helps identify the overall price level and can be
/// used as a basic trend indicator.
///
/// # Mathematical Formula
/// ```text
/// MEDPRICE = (High + Low) / 2
/// ```
///
/// # Calculation Principles
/// 1. For each period, take the high and low prices
/// 2. Sum the high and low prices
/// 3. Divide the sum by 2 to get the median price
///
/// # Arguments
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `output_medprice` - Array to store calculated median price values. Must be same length as inputs.
///
/// # Returns
/// * `Result<(), KandError>` - Returns Ok(()) on success
///
/// # Errors
/// * `KandError::InvalidData` - Input arrays are empty
/// * `KandError::LengthMismatch` - Input arrays have different lengths
/// * `KandError::NaNDetected` - Input contains NaN values (when "`check-nan`" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::medprice;
///
/// let high = vec![10.0, 11.0, 12.0];
/// let low = vec![8.0, 9.0, 10.0];
/// let mut output = vec![0.0; 3];
///
/// medprice::medprice(&high, &low, &mut output).unwrap();
/// assert_eq!(output, vec![9.0, 10.0, 11.0]);
/// ```
pub fn medprice(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    output_medprice: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len != input_low.len() || len != output_medprice.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    for i in 0..len {
        output_medprice[i] = f64::midpoint(input_high[i], input_low[i]);
    }

    Ok(())
}

/// Calculates a single MEDPRICE value incrementally.
///
/// # Description
/// This function provides an optimized way to calculate the median price for a single period,
/// making it suitable for real-time calculations without processing the entire data series.
///
/// # Arguments
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
///
/// # Returns
/// * `Result<TAFloat, KandError>` - Returns calculated median price on success
///
/// # Errors
/// * `KandError::NaNDetected` - Input contains NaN values (when "`check-nan`" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::medprice;
///
/// let high = 10.0;
/// let low = 8.0;
/// let result = medprice::medprice_inc(high, low).unwrap();
/// assert_eq!(result, 9.0);
/// ```
pub const fn medprice_inc(input_high: TAFloat, input_low: TAFloat) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan() || input_low.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(f64::midpoint(input_high, input_low))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_medprice_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1,
        ];
        let mut output_medprice = vec![0.0; input_high.len()];

        medprice(&input_high, &input_low, &mut output_medprice).unwrap();

        // Compare with known values
        let expected_values = [
            35241.05, 35227.0, 35207.85, 35160.75, 35167.8, 35216.35, 35232.75, 35242.5, 35215.5,
            35188.0, 35178.15, 35192.05, 35213.5, 35181.0, 35146.35,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_medprice[i], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        for i in 0..input_high.len() {
            let result = medprice_inc(input_high[i], input_low[i]).unwrap();
            assert_relative_eq!(result, output_medprice[i], epsilon = 0.0001);
        }
    }
}
