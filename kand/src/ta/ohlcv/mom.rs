use crate::{KandError, TAFloat};

/// Returns the lookback period required for Momentum (MOM) calculation
///
/// # Description
/// The lookback period determines how many data points are needed before the first valid output can be calculated.
/// For momentum calculation, this equals the momentum period parameter.
///
/// # Arguments
/// * `opt_period` - The number of periods to look back for momentum calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period on success, or error on failure
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 2 (when "check" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::mom;
///
/// let period = 14;
/// let lookback = mom::lookback(period).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period)
}

/// Calculates Momentum (MOM) for an array of prices
///
/// # Description
/// Momentum is a technical indicator that measures the rate of change in price movement by comparing
/// the current price with the price from n periods ago. It helps identify trend strength and potential
/// reversals.
///
/// # Mathematical Formula
/// ```text
/// MOM[i] = Price[i] - Price[i - n]
/// ```
/// Where:
/// * `i` is the current period
/// * `n` is the momentum period
///
/// # Calculation Principles
/// 1. For each period after the lookback period:
///    - Subtract the price from n periods ago from the current price
/// 2. The first n periods are filled with NaN values
///
/// # Arguments
/// * `input_prices` - Array of input price values
/// * `opt_period` - Number of periods to look back (n)
/// * `output_mom` - Array to store calculated momentum values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success, or error on failure
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If output array length != input array length
/// * `KandError::InvalidParameter` - If `opt_period` < 2
/// * `KandError::InsufficientData` - If input length < lookback period
/// * `KandError::NaNDetected` - If any input price is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::mom;
///
/// let input_prices = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// let period = 2;
/// let mut output_mom = vec![0.0; 5];
///
/// mom::mom(&input_prices, period, &mut output_mom).unwrap();
/// // output_mom = [NaN, NaN, 4.0, 4.0, 4.0]
/// ```
pub fn mom(
    input_prices: &[TAFloat],
    opt_period: usize,
    output_mom: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_prices.len();
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
        if output_mom.len() != len {
            return Err(KandError::LengthMismatch);
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

    // Calculate momentum
    for i in lookback..len {
        output_mom[i] = input_prices[i] - input_prices[i - opt_period];
    }

    // Fill initial values with NAN
    for item in output_mom.iter_mut().take(lookback) {
        *item = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest Momentum (MOM) value incrementally
///
/// # Description
/// This function provides an optimized way to calculate the latest momentum value
/// when streaming data is available, without needing the full price history.
///
/// # Arguments
/// * `input_current_price` - The current period's price value
/// * `input_old_price` - The price value from n periods ago
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The calculated momentum value on success, or error on failure
///
/// # Errors
/// * `KandError::NaNDetected` - If any input price is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::mom;
///
/// let current_price = 10.0;
/// let old_price = 6.0;
/// let momentum = mom::mom_inc(current_price, old_price).unwrap();
/// assert_eq!(momentum, 4.0);
/// ```
pub fn mom_inc(
    input_current_price: TAFloat,
    input_old_price: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        if input_current_price.is_nan() || input_old_price.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(input_current_price - input_old_price)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_mom_calculation() {
        let input_prices = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3,
        ];
        let opt_period = 14;
        let mut output_mom = vec![0.0; input_prices.len()];

        mom(&input_prices, opt_period, &mut output_mom).unwrap();

        // First 14 values should be NaN
        for value in output_mom.iter().take(14) {
            assert!(value.is_nan());
        }

        // Test expected values
        let expected_values = [
            -125.8, -180.2, -191.4, -156.6, -112.5, -230.0, -263.3, -299.3, -197.6, -142.9, -95.1,
            -115.4, -115.3, -68.7, -17.1, 98.1,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_mom[i + 14], *expected, epsilon = 0.1);
        }

        // Test incremental calculation
        for i in opt_period..input_prices.len() {
            let result = mom_inc(input_prices[i], input_prices[i - opt_period]).unwrap();
            assert_relative_eq!(result, output_mom[i], epsilon = 0.00001);
        }
    }
}
