use crate::{KandError, TAFloat};

/// Returns the lookback period required for Weighted Close Price (WCLPRICE) calculation.
///
/// # Description
/// The WCLPRICE indicator does not require any historical data since it only uses current price data.
///
/// # Returns
/// * `Result<usize, KandError>` - Returns `Ok(0)` since no historical data is needed
///
/// # Errors
/// This function does not return any errors.
///
/// # Example
/// ```
/// use kand::ohlcv::wclprice;
/// let lookback = wclprice::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates the Weighted Close Price (WCLPRICE) for a series of price data.
///
/// # Description
/// The Weighted Close Price is a price indicator that assigns more weight to the closing price
/// compared to high and low prices. It provides a single value that reflects price action
/// with emphasis on the closing price.
///
/// # Mathematical Formula
/// ```text
/// WCLPRICE = (High + Low + (Close × 2)) ÷ 4
/// ```
///
/// # Calculation Principles
/// 1. Takes high, low and closing prices for each period
/// 2. Multiplies closing price by 2 to give it more weight
/// 3. Adds high and low prices
/// 4. Divides the sum by 4 to get weighted average
///
/// # Arguments
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `input_close` - Array of closing prices for each period
/// * `output` - Array to store the calculated WCLPRICE values
///
/// # Returns
/// * `Result<(), KandError>` - `Ok(())` if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::wclprice;
///
/// let high = vec![10.0, 12.0, 15.0];
/// let low = vec![8.0, 9.0, 11.0];
/// let close = vec![9.0, 11.0, 14.0];
/// let mut output = vec![0.0; 3];
///
/// wclprice::wclprice(&high, &low, &close, &mut output).unwrap();
/// ```
pub fn wclprice(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    output: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }
        if len != input_low.len() || len != input_close.len() || len != output.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() || input_close[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    for i in 0..len {
        output[i] = input_close[i].mul_add(2.0, input_high[i] + input_low[i]) / 4.0;
    }

    Ok(())
}

/// Calculates a single Weighted Close Price (WCLPRICE) value from the latest price data.
///
/// # Description
/// This function provides an optimized way to calculate WCLPRICE for the most recent data point
/// without requiring historical values. It is useful for real-time calculations.
///
/// # Arguments
/// * `input_high` - Latest high price value
/// * `input_low` - Latest low price value
/// * `input_close` - Latest closing price value
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The calculated WCLPRICE value if successful
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::wclprice::wclprice_inc;
///
/// let high = 15.0;
/// let low = 11.0;
/// let close = 14.0;
///
/// let wclprice = wclprice_inc(high, low, close).unwrap();
/// ```
pub fn wclprice_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan() || input_low.is_nan() || input_close.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(input_close.mul_add(2.0, input_high + input_low) / 4.0)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_wclprice_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];

        let mut output = vec![0.0; input_high.len()];
        wclprice(&input_high, &input_low, &input_close, &mut output).unwrap();

        let expected_values = [
            35228.575, 35224.2, 35199.275, 35165.375, 35174.65, 35235.475, 35217.775, 35247.2,
            35206.55, 35186.35, 35176.625, 35210.975, 35213.0, 35170.85, 35118.325, 35058.1,
            34999.1, 35003.075, 35057.275, 35039.1,
        ];

        for i in 0..expected_values.len() {
            assert_relative_eq!(output[i], expected_values[i], epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        for i in 0..input_high.len() {
            let result = wclprice_inc(input_high[i], input_low[i], input_close[i]).unwrap();
            assert_relative_eq!(result, output[i], epsilon = 0.0001);
        }
    }
}
