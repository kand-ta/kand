use crate::{KandError, TAFloat};

/// Returns the lookback period for Rate of Change Percentage (ROCP) calculation.
///
/// # Description
/// Calculates the minimum number of data points needed before the first valid ROCP value can be computed.
/// The lookback period equals the ROCP period parameter since we need that many previous prices to calculate
/// the first value.
///
/// # Arguments
/// * `opt_period` - The time period used for ROCP calculation (usize)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if parameters are valid
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 1 (when "check" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::rocp;
/// let lookback = rocp::lookback(14).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if opt_period < 1 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period)
}

/// Calculates Rate of Change Percentage (ROCP) technical indicator for a price series.
///
/// # Description
/// The Rate of Change Percentage (ROCP) is a momentum indicator that measures the percentage change
/// between the current price and a price n periods ago. ROCP indicates both the speed and magnitude
/// of price movements, making it useful for identifying overbought/oversold conditions and divergences.
///
/// # Mathematical Formula
/// ```text
/// ROCP = (Current Price - Price n periods ago) / Price n periods ago
/// ```
///
/// # Calculation Principles
/// 1. For each data point after the lookback period:
///    - Take current price and price from n periods ago
///    - Calculate percentage change between these prices
/// 2. Initial values within lookback period are set to NaN
///
/// # Arguments
/// * `input_price` - Array of price values
/// * `opt_period` - Number of periods to look back, must be >= 1
/// * `output_rocp` - Array to store calculated ROCP values. Must be same length as `input_price`.
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds, Err otherwise
///
/// # Errors
/// Returns error if:
/// * Input arrays are empty (with "check" feature)
/// * Input and output arrays have different lengths (with "check" feature)
/// * `opt_period` < 1 (with "check" feature)
/// * Insufficient data points (with "check" feature)
/// * Input contains NaN values (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::rocp;
///
/// let input_price = vec![10.0, 10.5, 11.2, 10.8, 11.5];
/// let opt_period = 2;
/// let mut output_rocp = vec![0.0; 5];
///
/// rocp::rocp(&input_price, opt_period, &mut output_rocp).unwrap();
/// ```
pub fn rocp(
    input_price: &[TAFloat],
    opt_period: usize,
    output_rocp: &mut [TAFloat],
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
        if len != output_rocp.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for price in input_price {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate ROCP values
    for i in lookback..len {
        output_rocp[i] =
            (input_price[i] - input_price[i - opt_period]) / input_price[i - opt_period];
    }

    // Fill initial values with NAN
    for value in output_rocp.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single ROCP value incrementally.
///
/// # Description
/// This function provides an optimized way to calculate the latest ROCP value when
/// streaming data is received, without recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// ROCP = (Current Price - Previous Price) / Previous Price
/// ```
///
/// # Arguments
/// * `input` - The most recent price value
/// * `prev` - The price from n periods ago
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The calculated ROCP value if successful, error otherwise
///
/// # Errors
/// Returns error if (with "`check-nan`" feature):
/// * Either input is NaN
///
/// # Example
/// ```
/// use kand::ohlcv::rocp::rocp_inc;
///
/// let current_price = 11.5;
/// let prev_price = 10.0;
///
/// let output_rocp = rocp_inc(current_price, prev_price).unwrap();
/// ```
pub fn rocp_inc(input: TAFloat, prev: TAFloat) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        if input.is_nan() || prev.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok((input - prev) / prev)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_rocp_calculation() {
        let input_price = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];
        let opt_period = 10;
        let mut output_rocp = vec![0.0; input_price.len()];

        rocp(&input_price, opt_period, &mut output_rocp).unwrap();

        // First 10 values should be NaN
        for value in output_rocp.iter().take(10) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            -0.001_164_240_219_672_252_2,
            0.000_241_330_554_719_573_87,
            0.000_619_481_851_739_320_6,
            -0.000_264_429_911_856_778_8,
            -0.002_592_271_506_331_37,
            -0.006_053_110_799_725_468,
            -0.005_780_790_164_418_739,
            -0.006_765_592_776_559_561,
            -0.003_653_658_203_968_411_3,
            -0.004_550_273_272_189_292,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_rocp[i + 10], *expected, epsilon = 0.000_000_1);
        }

        // Test incremental calculation matches regular calculation
        for i in opt_period..input_price.len() {
            let result = rocp_inc(input_price[i], input_price[i - opt_period]).unwrap();
            assert_relative_eq!(result, output_rocp[i], epsilon = 0.000_000_1);
        }
    }
}
