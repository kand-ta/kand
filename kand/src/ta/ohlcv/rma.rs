use crate::{KandError, TAFloat};

/// Calculates the lookback period required for RMA calculation.
///
/// Returns the number of data points needed before RMA can start producing valid values.
/// The lookback period equals the period minus 1, since RMA requires a full period of data
/// to calculate the first value.
///
/// # Arguments
/// * `param_period` - The period length used for RMA calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::rma;
/// let period = 14;
/// let lookback = rma::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // lookback is period - 1
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

/// Calculates the Running Moving Average (RMA) for a price series.
///
/// RMA is a type of moving average that gives more weight to recent prices while still maintaining
/// some influence from all past prices. It is similar to EMA but uses a different smoothing factor.
///
/// # Mathematical Formula
/// ```text
/// RMA = (Current Price * α) + Previous RMA * (1 - α)
/// where α = 1/period
/// ```
///
/// # Calculation Steps
/// 1. Calculate initial SMA value using first `period` prices
/// 2. For remaining values, apply RMA formula using smoothing factor α = 1/period
/// 3. Fill initial values before period with NaN
///
/// # Arguments
/// * `input` - Array of price values to calculate RMA
/// * `param_period` - The smoothing period (must be >= 2)
/// * `output_rma` - Array to store calculated RMA values
///
/// # Returns
/// * `Result<(), KandError>` - Empty result on success
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::InsufficientData` - If input length is less than period
/// * `KandError::NaNDetected` - If input contains NaN values (with "`deep-check`" feature)
///
/// # Examples
/// ```
/// use kand::ohlcv::rma;
/// let prices = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let period = 3;
/// let mut rma_values = vec![0.0; 5];
/// rma::rma(&prices, period, &mut rma_values).unwrap();
/// ```
pub fn rma(
    input: &[TAFloat],
    param_period: usize,
    output_rma: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
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
        if len != output_rma.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for price in input.iter().take(len) {
            // NaN check
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate first SMA value
    let mut sum = input[0];
    for value in input.iter().take(param_period).skip(1) {
        sum += *value;
    }
    let alpha = 1.0 / param_period as TAFloat;
    output_rma[param_period - 1] = sum / param_period as TAFloat;

    // Calculate RMA for remaining values
    for i in param_period..input.len() {
        output_rma[i] = input[i].mul_add(alpha, output_rma[i - 1] * (1.0 - alpha));
    }

    // Fill initial values with NAN
    for value in output_rma.iter_mut().take(param_period - 1) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single new RMA value incrementally.
///
/// This function enables real-time RMA calculation by computing the next value
/// using only the current price and previous RMA, without requiring historical data.
///
/// # Mathematical Formula
/// ```text
/// RMA = (Current Price * α) + Previous RMA * (1 - α)
/// where α = 1/period
/// ```
///
/// # Arguments
/// * `input_current` - The current price value
/// * `prev_rma` - The previous RMA value
/// * `param_period` - The smoothing period (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The new RMA value on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::NaNDetected` - If any input is NaN (with "`deep-check`" feature)
///
/// # Examples
/// ```
/// use kand::ohlcv::rma;
/// let current_price = 10.0;
/// let prev_rma = 9.5;
/// let period = 14;
/// let new_rma = rma::rma_inc(current_price, prev_rma, period).unwrap();
/// ```
pub fn rma_inc(
    input_current: TAFloat,
    prev_rma: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        // NaN check
        if input_current.is_nan() || prev_rma.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let alpha = 1.0 / param_period as TAFloat;
    Ok(input_current.mul_add(alpha, prev_rma * (1.0 - alpha)))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_rma_calculation() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let param_period = 5;
        let mut output_rma = vec![0.0; input.len()];

        rma(&input, param_period, &mut output_rma).unwrap();

        // First (period-1) values should be NaN
        for value in output_rma.iter().take(param_period - 1) {
            assert!(value.is_nan());
        }

        // First valid value should be SMA of first 5 values
        assert_relative_eq!(output_rma[4], 3.0, epsilon = 1e-12); // (1+2+3+4+5)/5 = 3.0

        // Subsequent values follow RMA formula with alpha = 1/5 = 0.2
        // RMA[5] = 6.0*0.2 + 3.0*0.8 = 1.2 + 2.4 = 3.6
        assert_relative_eq!(output_rma[5], 3.6, epsilon = 1e-12);

        // RMA[6] = 7.0*0.2 + 3.6*0.8 = 1.4 + 2.88 = 4.28
        assert_relative_eq!(output_rma[6], 4.28, epsilon = 1e-12);

        // RMA[7] = 8.0*0.2 + 4.28*0.8 = 1.6 + 3.424 = 5.024
        assert_relative_eq!(output_rma[7], 5.024, epsilon = 1e-12);

        // RMA[8] = 9.0*0.2 + 5.024*0.8 = 1.8 + 4.0192 = 5.8192
        assert_relative_eq!(output_rma[8], 5.8192, epsilon = 1e-12);

        // RMA[9] = 10.0*0.2 + 5.8192*0.8 = 2.0 + 4.65536 = 6.65536
        assert_relative_eq!(output_rma[9], 6.65536, epsilon = 1e-12);
    }

    #[test]
    fn test_rma_incremental() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let param_period = 4;
        let mut output_rma = vec![0.0; input.len()];

        rma(&input, param_period, &mut output_rma).unwrap();

        // Test incremental calculation matches regular calculation
        // Start from the first valid RMA value (after the lookback period)
        let lookback = lookback(param_period).unwrap();

        // Start with the first valid RMA value
        let mut prev_rma = output_rma[lookback];

        // Test each incremental step
        for i in lookback + 1..input.len() {
            // Calculate next RMA using incremental method
            let next_rma = rma_inc(input[i], prev_rma, param_period).unwrap();

            // Verify incremental result matches the regular calculation
            assert_relative_eq!(next_rma, output_rma[i], epsilon = 1e-12);

            // Update prev_rma for next iteration
            prev_rma = next_rma;
        }
    }

    #[test]
    fn test_rma_edge_cases() {
        // Test edge case: period = 2 (minimum allowed)
        let input = vec![10.0, 20.0, 30.0, 40.0];
        let period = 2;
        let mut output = vec![0.0; input.len()];

        rma(&input, period, &mut output).unwrap();

        // Verify first value is NaN (lookback = 1)
        assert!(output[0].is_nan());

        // First valid value should be SMA of first 2 values
        assert_relative_eq!(output[1], 15.0, epsilon = 1e-12); // (10+20)/2 = 15

        // RMA[2] = 30*0.5 + 15*0.5 = 15 + 7.5 = 22.5
        assert_relative_eq!(output[2], 22.5, epsilon = 1e-12);

        // RMA[3] = 40*0.5 + 22.5*0.5 = 20 + 11.25 = 31.25
        assert_relative_eq!(output[3], 31.25, epsilon = 1e-12);
    }

    #[test]
    fn test_rma_with_extended_data() {
        // More extensive test with a larger dataset
        let prices = vec![
            35.25, 35.70, 36.10, 36.25, 36.50, 36.75, 36.70, 36.55, 36.80, 36.90, 37.05, 37.15,
            37.25, 37.40, 37.50, 37.60, 37.55, 37.35, 37.20, 37.10,
        ];
        let period = 7;
        let mut output_full = vec![0.0; prices.len()];

        // Calculate RMA using the rma function
        rma(&prices, period, &mut output_full).unwrap();

        // Manually compute the full expected RMA for comparison
        let alpha = 1.0 / period as TAFloat;
        let mut expected_rma = vec![TAFloat::NAN; prices.len()];

        // First valid value is the SMA of the first `period` elements
        let mut sum = 0.0;
        for i in 0..period {
            sum += prices[i];
        }
        expected_rma[period - 1] = sum / period as TAFloat;

        // Remaining values follow the RMA formula
        for i in period..prices.len() {
            expected_rma[i] = prices[i].mul_add(alpha, expected_rma[i - 1] * (1.0 - alpha));
        }

        // Check NaNs for the first (period-1) values
        for i in 0..period - 1 {
            assert!(output_full[i].is_nan());
        }

        // Assert all subsequent values match expected RMA
        for i in period - 1..prices.len() {
            assert_relative_eq!(output_full[i], expected_rma[i], epsilon = 1e-12);
        }

        // Also confirm incremental RMA matches
        let lookback = lookback(period).unwrap();
        let mut prev_rma = output_full[lookback];
        for i in lookback + 1..prices.len() {
            let next_rma = rma_inc(prices[i], prev_rma, period).unwrap();
            assert_relative_eq!(next_rma, output_full[i], epsilon = 1e-12);
            prev_rma = next_rma;
        }
    }

    #[test]
    fn test_rma_error_conditions() {
        let input = vec![1.0, 2.0, 3.0];
        let mut output = vec![0.0; 3];

        // Test invalid period
        assert!(matches!(
            rma(&input, 1, &mut output),
            Err(KandError::InvalidParameter)
        ));

        // Test length mismatch
        let mut short_output = vec![0.0; 2];
        assert!(matches!(
            rma(&input, 2, &mut short_output),
            Err(KandError::LengthMismatch)
        ));

        // Test insufficient data
        assert!(matches!(
            rma(&input, 4, &mut output),
            Err(KandError::InsufficientData)
        ));

        // Test empty data
        let empty: Vec<TAFloat> = vec![];
        let mut empty_output: Vec<TAFloat> = vec![];
        assert!(matches!(
            rma(&empty, 2, &mut empty_output),
            Err(KandError::InvalidData)
        ));
    }
}
