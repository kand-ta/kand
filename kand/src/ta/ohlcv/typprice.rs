use crate::{KandError, TAFloat};

/// Returns the lookback period required for Typical Price calculation.
///
/// Determines the number of data points needed to calculate the first valid Typical Price value.
///
/// # Returns
/// * `Result<usize, KandError>` - Returns Ok(0) since Typical Price requires no historical data
///
/// # Errors
/// * This function does not return any errors
///
/// # Example
/// ```
/// use kand::ohlcv::typprice;
/// let lookback = typprice::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates Typical Price for a series of OHLCV data.
///
/// The Typical Price is a simple average of the high, low and close prices for each period,
/// providing a single representative price for the trading period.
///
/// # Mathematical Formula
/// ```text
/// Typical Price = (High + Low + Close) / 3
/// ```
///
/// # Calculation Principle
/// 1. For each period in the data series:
///    - Sum the high, low and close prices
///    - Divide by 3 to get the arithmetic mean
///
/// # Parameters
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `input_close` - Array of close prices for each period
/// * `output_typprice` - Array to store the calculated Typical Price values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on successful calculation
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::typprice;
/// let input_high = vec![24.20, 24.07, 24.04];
/// let input_low = vec![23.85, 23.72, 23.64];
/// let input_close = vec![23.89, 23.95, 23.67];
/// let mut output_typprice = vec![0.0; 3];
///
/// typprice::typprice(&input_high, &input_low, &input_close, &mut output_typprice).unwrap();
/// // output_typprice ≈ [23.98, 23.91, 23.78]
/// ```
pub fn typprice(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    output_typprice: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len != input_low.len() || len != input_close.len() || len != output_typprice.len() {
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

    // Calculate typical price
    for i in 0..len {
        output_typprice[i] = (input_high[i] + input_low[i] + input_close[i]) / 3.0;
    }

    Ok(())
}

/// Calculates a single Typical Price value incrementally.
///
/// This function provides an optimized way to calculate the Typical Price for a single period,
/// useful in real-time calculations or streaming data scenarios.
///
/// # Parameters
/// * `input_high` - High price for the current period
/// * `input_low` - Low price for the current period
/// * `input_close` - Close price for the current period
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The calculated Typical Price value
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::typprice;
/// let high = 24.20;
/// let low = 23.85;
/// let close = 23.89;
///
/// let typ_price = typprice::typprice_inc(high, low, close).unwrap();
/// // typ_price ≈ 23.98
/// ```
pub fn typprice_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_high.is_nan() || input_low.is_nan() || input_close.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok((input_high + input_low + input_close) / 3.0)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;
    // Basic functionality tests
    #[test]
    fn test_typprice_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5,
        ];
        let mut output_typprice = vec![0.0; input_high.len()];

        typprice(&input_high, &input_low, &input_close, &mut output_typprice).unwrap();

        assert_relative_eq!(output_typprice[0], 35_232.733_333_333_34, epsilon = 0.00001);
        assert_relative_eq!(output_typprice[1], 35_225.133_333_333_33, epsilon = 0.00001);
        assert_relative_eq!(output_typprice[2], 35_202.133_333_333_33, epsilon = 0.00001);
        assert_relative_eq!(
            output_typprice[3],
            35_163.833_333_333_336,
            epsilon = 0.00001
        );
        assert_relative_eq!(output_typprice[4], 35_172.366_666_666_67, epsilon = 0.00001);

        // Test each incremental step matches regular calculation
        for i in 0..input_high.len() {
            let result = typprice_inc(input_high[i], input_low[i], input_close[i]).unwrap();
            assert_relative_eq!(result, output_typprice[i], epsilon = 0.0001);
        }
    }
}
