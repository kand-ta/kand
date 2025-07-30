use super::{ema, roc};
use crate::{KandError, TAFloat};

/// Calculates the lookback period required for TRIX calculation
///
/// # Description
/// The lookback period represents the minimum number of data points needed before the first valid TRIX value
/// can be calculated. It is determined by combining the lookback periods of three EMA calculations and one ROC calculation.
///
/// # Arguments
/// * `param_period` - The time period used for EMA calculations in TRIX. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful, or an error if parameters are invalid
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2 (when "check" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::trix;
///
/// let period = 14;
/// let lookback = trix::lookback(period).unwrap();
/// assert_eq!(lookback, 40); // 3 * EMA lookback + 1 ROC lookback
/// ```
pub fn lookback(param_period: usize) -> Result<usize, KandError> {
    Ok(3 * ema::lookback(param_period)? + roc::lookback(1)?)
}

/// Calculates the Triple Exponential Moving Average Oscillator (TRIX)
///
/// # Description
/// TRIX is a momentum oscillator that measures the rate of change of a triple exponentially smoothed moving average.
/// It helps identify oversold and overbought conditions and potential trend reversals through divergences.
///
/// # Calculation Principle
/// 1. Calculate first EMA of the input prices
/// 2. Calculate second EMA of the first EMA values
/// 3. Calculate third EMA of the second EMA values
/// 4. Calculate rate of change (ROC) of the triple EMA
///
/// # Mathematical Formula
/// ```text
/// EMA1 = EMA(price, period)
/// EMA2 = EMA(EMA1, period)
/// EMA3 = EMA(EMA2, period)
/// TRIX = ROC(EMA3, 1) = ((EMA3 - Previous EMA3) / Previous EMA3) * 100
/// ```
///
/// # Arguments
/// * `input` - Array of price values to calculate TRIX
/// * `param_period` - Number of periods for EMA calculations. Must be >= 2
/// * `output` - Array to store calculated TRIX values. First lookback values will be NaN
/// * `ema1_output` - Array to store first EMA values. First lookback values will be NaN
/// * `ema2_output` - Array to store second EMA values. First lookback values will be NaN
/// * `ema3_output` - Array to store third EMA values. First lookback values will be NaN
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok if calculation succeeds, otherwise an error
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty (with "check" feature)
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths (with "check" feature)
/// * `KandError::InvalidParameter` - If `param_period` < 2 (with "check" feature)
/// * `KandError::InsufficientData` - If input length <= lookback period (with "check" feature)
/// * `KandError::NaNDetected` - If input contains NaN values (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::trix;
///
/// let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
/// let period = 3;
/// let mut output = vec![0.0; 9];
/// let mut ema1 = vec![0.0; 9];
/// let mut ema2 = vec![0.0; 9];
/// let mut ema3 = vec![0.0; 9];
///
/// trix::trix(&input, period, &mut output, &mut ema1, &mut ema2, &mut ema3).unwrap();
/// ```
pub fn trix(
    input: &[TAFloat],
    param_period: usize,
    output: &mut [TAFloat],
    ema1_output: &mut [TAFloat],
    ema2_output: &mut [TAFloat],
    ema3_output: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Check if input array is empty
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Check if input length is less than or equal to lookback period
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Check if output, ema1_output, ema2_output, and ema3_output arrays have the same length
        if len != output.len()
            || len != ema1_output.len()
            || len != ema2_output.len()
            || len != ema3_output.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for value in input {
            if value.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate first EMA
    ema::ema(input, param_period, None, ema1_output)?;

    // Calculate second EMA
    ema::ema(
        &ema1_output[param_period - 1..],
        param_period,
        None,
        &mut ema2_output[param_period - 1..],
    )?;

    // Calculate third EMA
    ema::ema(
        &ema2_output[2 * (param_period - 1)..],
        param_period,
        None,
        &mut ema3_output[2 * (param_period - 1)..],
    )?;

    // Calculate TRIX using ROC
    roc::roc(
        &ema3_output[3 * (param_period - 1)..],
        1,
        &mut output[3 * (param_period - 1)..],
    )?;

    // Fill initial values with NAN
    for i in 0..lookback {
        output[i] = TAFloat::NAN;
        ema1_output[i] = TAFloat::NAN;
        ema2_output[i] = TAFloat::NAN;
        ema3_output[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single new TRIX value incrementally using previous EMA values
///
/// # Description
/// This function provides an optimized way to calculate the latest TRIX value when new price data arrives,
/// without recalculating the entire series. It uses the previous EMA values and current price to compute
/// the new TRIX value.
///
/// # Mathematical Formula
/// ```text
/// alpha = 2 / (period + 1)
/// EMA1 = alpha * price + (1 - alpha) * prev_ema1
/// EMA2 = alpha * EMA1 + (1 - alpha) * prev_ema2
/// EMA3 = alpha * EMA2 + (1 - alpha) * prev_ema3
/// TRIX = ((EMA3 - prev_ema3) / prev_ema3) * 100
/// ```
///
/// # Arguments
/// * `input` - Current price value
/// * `prev_ema1` - Previous first EMA value
/// * `prev_ema2` - Previous second EMA value
/// * `prev_ema3` - Previous third EMA value
/// * `param_period` - Period for EMA calculations. Must be >= 2
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing (TRIX, `new_ema1`, `new_ema2`, `new_ema3`) if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2 (with "check" feature)
/// * `KandError::NaNDetected` - If any input value is NaN (with "`check-nan`" feature)
/// * `KandError::InvalidData` - If division by zero occurs during ROC calculation
///
/// # Example
/// ```
/// use kand::ohlcv::trix;
///
/// let price = 100.0;
/// let prev_ema1 = 98.0;
/// let prev_ema2 = 97.0;
/// let prev_ema3 = 96.0;
/// let period = 14;
///
/// let (trix, new_ema1, new_ema2, new_ema3) =
///     trix::trix_inc(price, prev_ema1, prev_ema2, prev_ema3, period).unwrap();
/// ```
pub fn trix_inc(
    input: TAFloat,
    prev_ema1: TAFloat,
    prev_ema2: TAFloat,
    prev_ema3: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input.is_nan() || prev_ema1.is_nan() || prev_ema2.is_nan() || prev_ema3.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let new_ema1 = ema::ema_inc(input, prev_ema1, param_period, None)?;
    let new_ema2 = ema::ema_inc(new_ema1, prev_ema2, param_period, None)?;
    let new_ema3 = ema::ema_inc(new_ema2, prev_ema3, param_period, None)?;
    let trix = roc::roc_inc(new_ema3, prev_ema3)?;

    Ok((trix, new_ema1, new_ema2, new_ema3))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_trix_calculation() {
        let input = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];
        let param_period = 3;
        let mut output = vec![0.0; input.len()];
        let mut ema1 = vec![0.0; input.len()];
        let mut ema2 = vec![0.0; input.len()];
        let mut ema3 = vec![0.0; input.len()];

        trix(
            &input,
            param_period,
            &mut output,
            &mut ema1,
            &mut ema2,
            &mut ema3,
        )
        .unwrap();

        // First 7 values should be NaN (lookback period)
        for value in output.iter().take(7) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            0.023_600_565_750_747_65,
            0.007_581_993_206_917_659,
            -0.008_943_588_952_370_352,
            -0.019_561_690_627_645_234,
            -0.002_233_206_002_377_752_2,
            0.004_028_034_632_819_2,
            -0.013_120_959_352_708_184,
            -0.047_983_857_499_556_14,
            -0.079_103_810_365_388_5,
            -0.099_256_202_780_007_02,
            -0.090_591_666_739_903_15,
            -0.051_535_144_257_075_505,
            -0.037_564_038_843_540_54,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output[i + 7], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        let mut prev_ema1 = ema1[10];
        let mut prev_ema2 = ema2[10];
        let mut prev_ema3 = ema3[10];

        for i in 11..15 {
            let (trix_val, new_ema1, new_ema2, new_ema3) =
                trix_inc(input[i], prev_ema1, prev_ema2, prev_ema3, param_period).unwrap();

            assert_relative_eq!(trix_val, output[i], epsilon = 0.0001);

            prev_ema1 = new_ema1;
            prev_ema2 = new_ema2;
            prev_ema3 = new_ema3;
        }
    }
}
