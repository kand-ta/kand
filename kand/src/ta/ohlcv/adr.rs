use crate::{KandError, TAFloat, ta::ohlcv::sma};

/// Returns the lookback period required for Average Daily Range (ADR) calculation.
///
/// # Description
/// The lookback period represents the number of data points needed before the first valid output
/// can be calculated. For ADR, this equals the specified period minus one.
///
/// # Arguments
/// * `param_period` - The time period used for ADR calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adr;
/// let period = 14;
/// let lookback = adr::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // lookback is period - 1
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    sma::lookback(param_period)
}

/// Calculates Average Daily Range (ADR) for an entire price series.
///
/// # Description
/// The Average Daily Range measures the average price range over a specified period,
/// helping to identify volatility levels in the market.
///
/// # Mathematical Formula
/// ```text
/// Daily Range = High - Low
/// ADR = SMA(Daily Range, period)
/// ```
///
/// # Calculation Steps
/// 1. Calculate daily range for each period: High - Low
/// 2. Apply Simple Moving Average (SMA) to the daily ranges
/// 3. First (period-1) values will be NaN as they require full period data
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_period` - The time period for ADR calculation (must be >= 2)
/// * `output_adr` - Array to store calculated ADR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success, Err on failure
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adr;
/// let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let param_period = 3;
/// let mut output_adr = vec![0.0; 5];
///
/// adr::adr(&input_high, &input_low, param_period, &mut output_adr).unwrap();
/// // First two values are NaN (period-1), followed by calculated ADR values
/// ```
pub fn adr(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    param_period: usize,
    output_adr: &mut [TAFloat],
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
        if len != input_low.len() || len != output_adr.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for i in lookback..len {
            // NaN check
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate daily ranges
    let mut ranges = vec![0.0; len];
    for i in 0..len {
        ranges[i] = input_high[i] - input_low[i];
    }

    // Calculate SMA of ranges
    sma::sma(&ranges, param_period, output_adr)?;

    Ok(())
}

/// Calculates the next Average Daily Range (ADR) value incrementally.
///
/// # Description
/// Optimizes ADR calculation by using previous ADR value and only calculating
/// the latest value, avoiding recalculation of the entire series.
///
/// # Arguments
/// * `prev_adr` - Previous ADR value
/// * `input_new_high` - Latest high price
/// * `input_new_low` - Latest low price
/// * `input_old_high` - Oldest high price to be removed from period
/// * `input_old_low` - Oldest low price to be removed from period
/// * `param_period` - The time period for ADR calculation (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - Latest ADR value on success, Err on failure
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adr;
/// let prev_adr = 3.0;
/// let new_high = 15.0;
/// let new_low = 12.0;
/// let old_high = 10.0;
/// let old_low = 8.0;
/// let period = 14;
///
/// let next_adr = adr::adr_inc(prev_adr, new_high, new_low, old_high, old_low, period).unwrap();
/// ```
pub fn adr_inc(
    prev_adr: TAFloat,
    input_new_high: TAFloat,
    input_new_low: TAFloat,
    input_old_high: TAFloat,
    input_old_low: TAFloat,
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
        if prev_adr.is_nan()
            || input_new_high.is_nan()
            || input_new_low.is_nan()
            || input_old_high.is_nan()
            || input_old_low.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let new_range = input_new_high - input_new_low;
    let old_range = input_old_high - input_old_low;

    sma::sma_inc(prev_adr, new_range, old_range, param_period)
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_adr_calculation() {
        let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
        let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
        let mut output_adr = vec![0.0; input_high.len()];
        let period = 3;

        adr(&input_high, &input_low, period, &mut output_adr).unwrap();

        // First (period-1) values are NaN
        assert!(output_adr[0].is_nan());
        assert!(output_adr[1].is_nan());

        // daily ranges: [2.0, 3.0, 4.0, 4.0, 4.0]
        // output_adr[2] = average of [2.0, 3.0, 4.0] = 3.0
        assert_relative_eq!(output_adr[2], 3.0, max_relative = 1e-12);

        // output_adr[3] = average of [3.0, 4.0, 4.0] = 3.6666...
        assert_relative_eq!(output_adr[3], 3.6666666666666665, max_relative = 1e-12);

        // output_adr[4] = average of [4.0, 4.0, 4.0] = 4.0
        assert_relative_eq!(output_adr[4], 4.0, max_relative = 1e-12);
    }

    #[test]
    fn test_adr_incremental() {
        let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0, 16.0, 17.0, 15.5, 14.2, 13.7];
        let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0, 12.0, 14.0, 13.5, 12.0, 11.3];
        let period = 5;
        let mut output_adr = vec![0.0; input_high.len()];

        // Calculate ADR using the standard method
        adr(&input_high, &input_low, period, &mut output_adr).unwrap();

        // Test incremental calculation matches regular calculation
        // Start from the first valid ADR value (after the lookback period)
        let lookback = lookback(period).unwrap();

        // Start with the first valid ADR value
        let mut prev_adr = output_adr[lookback];

        // Test each incremental step
        for i in lookback + 1..input_high.len() {
            let new_high = input_high[i];
            let new_low = input_low[i];
            let old_high = input_high[i - period];
            let old_low = input_low[i - period];

            // Calculate next ADR using incremental method
            let next_adr = adr_inc(prev_adr, new_high, new_low, old_high, old_low, period).unwrap();

            // Verify incremental result matches the regular calculation
            assert_relative_eq!(next_adr, output_adr[i], epsilon = 1e-12);

            // Update prev_adr for next iteration
            prev_adr = next_adr;
        }
    }

    #[test]
    fn test_adr_with_extended_data() {
        // More extensive test with a larger dataset
        let high_prices = vec![
            35.25, 35.70, 36.10, 36.25, 36.50, 36.75, 36.70, 36.55, 36.80, 36.90, 37.05, 37.15,
            37.25, 37.40, 37.50, 37.60, 37.55, 37.35, 37.20, 37.10,
        ];
        let low_prices = vec![
            34.75, 35.20, 35.70, 35.80, 36.10, 36.20, 36.05, 36.10, 36.40, 36.50, 36.65, 36.80,
            36.90, 37.00, 37.10, 37.20, 37.00, 36.80, 36.70, 36.60,
        ];
        let period = 7;
        let mut output_full = vec![0.0; high_prices.len()];

        adr(&high_prices, &low_prices, period, &mut output_full).unwrap();

        // Test all valid values are calculated correctly
        let lookback = lookback(period).unwrap();
        let mut prev_adr = output_full[lookback];

        for i in lookback + 1..high_prices.len() {
            let result = adr_inc(
                prev_adr,
                high_prices[i],
                low_prices[i],
                high_prices[i - period],
                low_prices[i - period],
                period,
            )
            .unwrap();

            assert_relative_eq!(result, output_full[i], epsilon = 1e-12);
            prev_adr = result;
        }
    }

    #[test]
    fn test_adr_edge_cases() {
        // Test edge case: period = 2 (minimum allowed)
        let high_prices = vec![10.0, 11.0, 12.0, 13.0];
        let low_prices = vec![9.0, 9.5, 10.5, 11.5];
        let period = 2;
        let mut output = vec![0.0; high_prices.len()];

        adr(&high_prices, &low_prices, period, &mut output).unwrap();

        // Verify first value is NaN (lookback = 1)
        assert!(output[0].is_nan());

        // Test incremental calculation for the minimum period
        let mut prev_adr = output[1]; // First valid value with period = 2
        for i in 2..high_prices.len() {
            let result = adr_inc(
                prev_adr,
                high_prices[i],
                low_prices[i],
                high_prices[i - period],
                low_prices[i - period],
                period,
            )
            .unwrap();

            assert_relative_eq!(result, output[i], epsilon = 1e-12);
            prev_adr = result;
        }
    }
}
