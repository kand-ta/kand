use crate::{KandError, TAFloat, helper::period_to_k};

/// Returns the lookback period required for EMA calculation.
///
/// # Description
/// Calculates the minimum number of historical data points needed before generating the first valid EMA value.
/// For EMA, this equals period - 1 since the first value requires a complete period for SMA calculation.
///
/// # Arguments
/// * `param_period` - The time period for EMA calculation. Must be >= 2.
///
/// # Returnss
/// * `Result<usize, KandError>` - The lookback period on success, or error on failure.
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2.
///
/// # Example
/// ```
/// use kand::ohlcv::ema;
/// let period = 14;
/// let lookback = ema::lookback(period).unwrap();
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

/// Calculates Exponential Moving Average (EMA) for a price series.
///
/// # Description
/// EMA is a type of moving average that places a greater weight and significance on the most recent data points.
/// The weighting given to each data point decreases exponentially with time.
///
/// # Mathematical Formula
/// ```text
/// Initial EMA = SMA(first n prices)
/// EMA = Price * k + EMA(previous) * (1 - k)
/// where:
/// k = smoothing factor (default is 2/(period+1))
/// ```
///
/// # Calculation Steps
/// 1. Calculate Simple Moving Average (SMA) for initial period
/// 2. Apply EMA formula using smoothing factor k for remaining periods
/// 3. Fill initial values before lookback period with NaN
///
/// # Arguments
/// * `input_prices` - Array of price values to calculate EMA
/// * `param_period` - The time period for EMA calculation (must be >= 2)
/// * `param_k` - Optional custom smoothing factor. If None, uses 2/(period+1)
/// * `output_ema` - Array to store calculated EMA values. Must match input length
///
/// # Returns
/// * `Result<(), KandError>` - Unit on success, or error on failure
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If output length doesn't match input
/// * `KandError::InvalidParameter` - If period < 2
/// * `KandError::InsufficientData` - If input length < period
/// * `KandError::NaNDetected` - If any input price is NaN (with "`check-nan`")
///
/// # Example
/// ```
/// use kand::ohlcv::ema;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let period = 3;
/// let mut ema_values = vec![0.0; prices.len()];
///
/// // Calculate EMA with default smoothing
/// ema::ema(&prices, period, None, &mut ema_values).unwrap();
/// ```
pub fn ema(
    input_prices: &[TAFloat],
    param_period: usize,
    param_k: Option<TAFloat>,
    output_ema: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_prices.len();
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
        if output_ema.len() != len {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for price in input_prices {
            // NaN check
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate initial SMA
    let mut sum = input_prices[0];
    for value in input_prices.iter().take(param_period).skip(1) {
        sum += *value;
    }
    let mut prev_ma = sum / (param_period as TAFloat);
    output_ema[lookback] = prev_ma;

    // Get multiplier - either custom or default
    let multiplier = match param_k {
        Some(k) => k,
        None => period_to_k(param_period)?,
    };

    // Calculate EMA
    for i in param_period..len {
        prev_ma = (input_prices[i] - prev_ma).mul_add(multiplier, prev_ma);
        output_ema[i] = prev_ma;
    }

    // Fill initial values with NAN
    for value in output_ema.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single EMA value incrementally using the previous EMA.
///
/// # Description
/// Provides an efficient way to update EMA calculations when new data arrives, without reprocessing
/// the entire dataset. Uses the previous EMA value and current price to compute the new EMA.
///
/// # Mathematical Formula
/// ```text
/// EMA = Price * k + EMA(previous) * (1 - k)
/// where:
/// k = smoothing factor (default is 2/(period+1))
/// ```
///
/// # Arguments
/// * `input_price` - The current period's price value
/// * `prev_ema` - The previous period's EMA value
/// * `param_period` - The time period for EMA calculation (must be >= 2)
/// * `param_k` - Optional custom smoothing factor. If None, uses 2/(period+1)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The new EMA value on success, or error on failure
///
/// # Errors
/// * `KandError::InvalidParameter` - If period < 2
/// * `KandError::NaNDetected` - If price or previous EMA is NaN (with "`check-nan`")
///
/// # Example
/// ```
/// use kand::ohlcv::ema;
/// let current_price = 15.0;
/// let prev_ema = 14.5;
/// let period = 14;
///
/// // Calculate next EMA with default smoothing
/// let new_ema = ema::ema_inc(current_price, prev_ema, period, None).unwrap();
/// ```
pub fn ema_inc(
    input_price: TAFloat,
    prev_ema: TAFloat,
    param_period: usize,
    param_k: Option<TAFloat>,
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
        if input_price.is_nan() || prev_ema.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let multiplier = match param_k {
        Some(k) => k,
        None => period_to_k(param_period)?,
    };
    Ok((input_price - prev_ema).mul_add(multiplier, prev_ema))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_ema_calculation() {
        let input_prices = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
        ];
        let param_period = 14;
        let mut output_ema = vec![0.0; input_prices.len()];

        ema(&input_prices, param_period, None, &mut output_ema).unwrap();

        // First 13 values should be NaN
        for value in output_ema.iter().take(13) {
            assert!(value.is_nan());
        }

        // Test first valid value
        let expected_values = [
            35_203.535_714_285_72,
            35_188.437_619_047_625,
            35_168.805_936_507_94,
            35_146.205_144_973_545,
            35_128.497_792_310_41,
            35_120.564_753_335_69,
            35_107.769_452_890_934,
            35_085.333_525_838_81,
            35_067.635_722_393_636,
            35_058.617_626_074_48,
            35_056.375_275_931_22,
            35_059.525_239_140_39,
            35_066.855_207_255_01,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_ema[i + 13], *expected, epsilon = 0.00001);
        }

        let param_period = 14;
        let mut output_ema = vec![0.0; input_prices.len()];

        ema(&input_prices, param_period, None, &mut output_ema).unwrap();

        // Now test incremental calculation matches regular calculation
        let mut prev_ema = output_ema[13]; // First valid EMA value

        // Test each incremental step
        for i in 14..18 {
            let result = ema_inc(input_prices[i], prev_ema, param_period, None).unwrap();
            assert_relative_eq!(result, output_ema[i], epsilon = 0.00001);
            prev_ema = result;
        }
    }
}
