use crate::{TAFloat, error::KandError, helper::period_to_k};

/// Returns the lookback period required for DEMA calculation.
///
/// # Description
/// Calculates the minimum number of historical data points needed before generating the first valid DEMA value.
/// For DEMA, this equals 2 * (`param_period` - 1) due to the double exponential smoothing process.
///
/// # Arguments
/// * `param_period` - The smoothing period used for EMA calculations. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The required lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::dema;
/// let lookback = dema::lookback(5).unwrap();
/// assert_eq!(lookback, 8); // 2 * (5-1) = 8
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(2 * (param_period - 1))
}

/// Calculates Double Exponential Moving Average (DEMA) for a price series.
///
/// # Description
/// DEMA is designed to reduce the lag in traditional moving averages by applying double exponential smoothing.
/// It puts more weight on recent data while maintaining smoothness and reducing noise.
///
/// # Mathematical Formula
/// ```text
/// α = 2/(period + 1)
/// EMA1(t) = α * price(t) + (1 - α) * EMA1(t-1)
/// EMA2(t) = α * EMA1(t) + (1 - α) * EMA2(t-1)
/// DEMA(t) = 2 * EMA1(t) - EMA2(t)
/// ```
///
/// # Calculation Steps
/// 1. Calculate first EMA (EMA1) of the price series
/// 2. Calculate second EMA (EMA2) using the EMA1 values
/// 3. Calculate DEMA as: 2 * EMA1 - EMA2
///
/// # Arguments
/// * `input` - Array of price values to calculate DEMA
/// * `param_period` - The smoothing period for EMA calculations. Must be >= 2.
/// * `output_dema` - Array to store calculated DEMA values
/// * `output_ema1` - Array to store first EMA values
/// * `output_ema2` - Array to store second EMA values
///
/// # Returns
/// * `Result<(), KandError>` - Empty result if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - Input array is empty
/// * `KandError::LengthMismatch` - Output arrays don't match input length
/// * `KandError::InvalidParameter` - Period is less than 2
/// * `KandError::InsufficientData` - Input length <= lookback period
/// * `KandError::NaNDetected` - Input contains NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::dema;
/// let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
/// let mut output_dema = vec![0.0; input.len()];
/// let mut output_ema1 = vec![0.0; input.len()];
/// let mut output_ema2 = vec![0.0; input.len()];
///
/// dema::dema(
///     &input,
///     3,
///     &mut output_dema,
///     &mut output_ema1,
///     &mut output_ema2,
/// )
/// .unwrap();
/// ```
pub fn dema(
    input: &[TAFloat],
    param_period: usize,
    output_dema: &mut [TAFloat],
    output_ema1: &mut [TAFloat],
    output_ema2: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Insufficient data check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Length mismatch check
        if len != output_dema.len() || len != output_ema1.len() || len != output_ema2.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for price in input {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let alpha = period_to_k(param_period)?;

    // Calculate initial SMA for first EMA
    let mut sum = input[0];
    for price in input.iter().take(param_period).skip(1) {
        sum += *price;
    }
    let mut prev_ema1 = sum / param_period as TAFloat;
    output_ema1[param_period - 1] = prev_ema1;

    // Calculate first EMA series
    for i in param_period..len {
        prev_ema1 = input[i].mul_add(alpha, prev_ema1 * (1.0 - alpha));
        output_ema1[i] = prev_ema1;
    }
    // Initialize second EMA with SMA of first EMA
    let mut sum = output_ema1[param_period - 1];
    for value in output_ema1
        .iter()
        .take(param_period * 2 - 1)
        .skip(param_period)
    {
        sum += *value;
    }
    let mut prev_ema2 = sum / param_period as TAFloat;
    output_ema2[param_period * 2 - 2] = prev_ema2;

    // Calculate second EMA series (EMA of EMA)
    for i in param_period * 2 - 1..len {
        prev_ema2 = output_ema1[i].mul_add(alpha, prev_ema2 * (1.0 - alpha));
        output_ema2[i] = prev_ema2;
    }

    // Calculate DEMA = 2 * EMA1 - EMA2
    for i in 0..len {
        output_dema[i] = 2.0f64.mul_add(output_ema1[i], -output_ema2[i]);
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_dema[i] = TAFloat::NAN;
        output_ema1[i] = TAFloat::NAN;
        output_ema2[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single new DEMA value incrementally using previous EMAs.
///
/// # Description
/// Enables real-time DEMA calculation by computing only the latest value based on the current price
/// and previous EMA values. This is more efficient than recalculating the entire series for streaming data.
///
/// # Mathematical Formula
/// ```text
/// α = 2/(period + 1)
/// new_ema1 = α * price + (1 - α) * prev_ema1
/// new_ema2 = α * new_ema1 + (1 - α) * prev_ema2
/// dema = 2 * new_ema1 - new_ema2
/// ```
///
/// # Arguments
/// * `input_price` - Current price value
/// * `prev_ema1` - Previous value of first EMA
/// * `prev_ema2` - Previous value of second EMA
/// * `param_period` - The smoothing period. Must be >= 2.
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing (DEMA, `new_ema1`, `new_ema2`) if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - Period is less than 2
/// * `KandError::NaNDetected` - Any input is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::dema;
/// let (dema_value, new_ema1, new_ema2) = dema::dema_inc(
///     10.0, // current price
///     9.5,  // previous EMA1
///     9.0,  // previous EMA2
///     3,    // period
/// )
/// .unwrap();
/// ```
pub fn dema_inc(
    input_price: TAFloat,
    prev_ema1: TAFloat,
    prev_ema2: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_price.is_nan() || prev_ema1.is_nan() || prev_ema2.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let alpha = period_to_k(param_period)?;

    let new_ema1 = input_price.mul_add(alpha, prev_ema1 * (1.0 - alpha));
    let new_ema2 = new_ema1.mul_add(alpha, prev_ema2 * (1.0 - alpha));
    let dema = 2.0f64.mul_add(new_ema1, -new_ema2);

    Ok((dema, new_ema1, new_ema2))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_dema_calculation() {
        let input = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];
        let param_period = 5;
        let mut output_dema = vec![0.0; input.len()];
        let mut output_ema1 = vec![0.0; input.len()];
        let mut output_ema2 = vec![0.0; input.len()];

        dema(
            &input,
            param_period,
            &mut output_dema,
            &mut output_ema1,
            &mut output_ema2,
        )
        .unwrap();

        // First 8 values should be NaN (lookback = 2 * (period - 1) = 8)
        for value in output_dema.iter().take(8) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            35_218.829_037_037_04,
            35_200.555_187_928_67,
            35_185.338_456_332_87,
            35_207.882_302_697_76,
            35_210.681_534_115_734,
            35_183.349_910_955_31,
            35_129.574_755_000_09,
            35_074.033_046_242_2,
            35_022.421_948_322_895,
            35_004.747_910_545_1,
            35_028.743_014_805_514,
            35_019.213_837_276_19,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_dema[i + 8], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        let mut prev_ema1 = output_ema1[9];
        let mut prev_ema2 = output_ema2[9];

        // Test each incremental step
        for i in 10..15 {
            let (dema_val, new_ema1, new_ema2) =
                dema_inc(input[i], prev_ema1, prev_ema2, param_period).unwrap();

            assert_relative_eq!(dema_val, output_dema[i], epsilon = 0.0001);
            assert_relative_eq!(new_ema1, output_ema1[i], epsilon = 0.0001);
            assert_relative_eq!(new_ema2, output_ema2[i], epsilon = 0.0001);

            prev_ema1 = new_ema1;
            prev_ema2 = new_ema2;
        }
    }
}
