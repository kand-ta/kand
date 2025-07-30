use crate::{KandError, TAFloat, ta::ohlcv::ema};

/// Calculates the lookback period required for Triple Exponential Moving Average (TEMA)
///
/// # Description
/// The lookback period represents the minimum number of data points needed before the first valid TEMA value
/// can be calculated. For TEMA, this equals 3 * (period - 1) due to the triple EMA calculation process.
///
/// # Arguments
/// * `param_period` - The smoothing period used for TEMA calculation. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The required lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - Returned if period < 2
///
/// # Example
/// ```
/// use kand::ohlcv::tema;
/// let period = 14;
/// let lookback = tema::lookback(period).unwrap();
/// assert_eq!(lookback, 39); // 3 * (14 - 1)
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(3 * (param_period - 1))
}

/// Calculates Triple Exponential Moving Average (TEMA) for a price series
///
/// # Description
/// TEMA is an enhanced moving average designed to reduce lag while maintaining smoothing properties.
/// It applies triple exponential smoothing to put more weight on recent data and less on older data.
///
/// # Mathematical Formula
/// ```text
/// EMA1 = EMA(price, period)
/// EMA2 = EMA(EMA1, period)
/// EMA3 = EMA(EMA2, period)
/// TEMA = (3 × EMA1) - (3 × EMA2) + EMA3
/// ```
///
/// # Calculation Steps
/// 1. Calculate first EMA of the input prices
/// 2. Calculate second EMA using the first EMA values
/// 3. Calculate third EMA using the second EMA values
/// 4. Apply the TEMA formula to combine all three EMAs
///
/// # Arguments
/// * `input` - Slice of input price values
/// * `param_period` - Smoothing period for calculations (must be >= 2)
/// * `output_tema` - Mutable slice to store TEMA results (first lookback values will be NaN)
/// * `output_ema1` - Mutable slice to store first EMA series
/// * `output_ema2` - Mutable slice to store second EMA series
/// * `output_ema3` - Mutable slice to store third EMA series
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok value on success
///
/// # Errors
/// * `KandError::InvalidData` - Input slice is empty
/// * `KandError::LengthMismatch` - Output arrays don't match input length
/// * `KandError::InvalidParameter` - Period is less than 2
/// * `KandError::InsufficientData` - Input length is less than required lookback period
/// * `KandError::NaNDetected` - Input contains NaN values (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::tema;
///
/// let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
/// let period = 3;
/// let mut output_tema = vec![0.0; input.len()];
/// let mut ema1 = vec![0.0; input.len()];
/// let mut ema2 = vec![0.0; input.len()];
/// let mut ema3 = vec![0.0; input.len()];
///
/// tema::tema(
///     &input,
///     period,
///     &mut output_tema,
///     &mut ema1,
///     &mut ema2,
///     &mut ema3,
/// )
/// .unwrap();
/// ```
pub fn tema(
    input: &[TAFloat],
    param_period: usize,
    output_tema: &mut [TAFloat],
    output_ema1: &mut [TAFloat],
    output_ema2: &mut [TAFloat],
    output_ema3: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Check if input is empty
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Check if input length is less than required lookback period
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Check if output arrays have the same length as input
        if len != output_tema.len()
            || len != output_ema1.len()
            || len != output_ema2.len()
            || len != output_ema3.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // Check if input contains NaN values
        for value in input {
            if value.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate first EMA series
    ema::ema(input, param_period, None, output_ema1)?;

    // Calculate second EMA series using valid values from first EMA
    ema::ema(
        &output_ema1[param_period - 1..],
        param_period,
        None,
        &mut output_ema2[param_period - 1..],
    )?;

    // Calculate third EMA series
    ema::ema(
        &output_ema2[2 * (param_period - 1)..],
        param_period,
        None,
        &mut output_ema3[2 * (param_period - 1)..],
    )?;

    // Calculate TEMA and store it in the output array (valid only after lookback)
    for i in lookback..len {
        output_tema[i] = 3.0f64.mul_add(output_ema1[i], -(3.0 * output_ema2[i])) + output_ema3[i];
    }

    // Fill initial periods with NAN for all outputs
    for i in 0..lookback {
        output_tema[i] = TAFloat::NAN;
        output_ema1[i] = TAFloat::NAN;
        output_ema2[i] = TAFloat::NAN;
        output_ema3[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates TEMA value incrementally using previous EMA values
///
/// # Description
/// This function enables real-time TEMA calculation by using the previous EMA values and latest price.
/// It avoids recalculating the entire series, making it efficient for streaming data.
///
/// # Arguments
/// * `input` - Latest price value to process
/// * `prev_ema1` - Previous value of first EMA
/// * `prev_ema2` - Previous value of second EMA
/// * `prev_ema3` - Previous value of third EMA
/// * `param_period` - Smoothing period for calculations (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing:
///   - Current TEMA value
///   - Updated first EMA
///   - Updated second EMA
///   - Updated third EMA
///
/// # Errors
/// * `KandError::InvalidParameter` - Period is less than 2
/// * `KandError::NaNDetected` - Any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::tema::tema_inc;
///
/// let new_price = 10.0;
/// let prev_ema1 = 9.0;
/// let prev_ema2 = 8.0;
/// let prev_ema3 = 7.0;
/// let period = 3;
///
/// let (tema, ema1, ema2, ema3) =
///     tema_inc(new_price, prev_ema1, prev_ema2, prev_ema3, period).unwrap();
/// ```
pub fn tema_inc(
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

    let ema1 = ema::ema_inc(input, prev_ema1, param_period, None)?;
    let ema2 = ema::ema_inc(ema1, prev_ema2, param_period, None)?;
    let ema3 = ema::ema_inc(ema2, prev_ema3, param_period, None)?;
    let tema = 3.0f64.mul_add(ema1, -(3.0 * ema2)) + ema3;

    Ok((tema, ema1, ema2, ema3))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_tema_calculation() {
        let input = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];
        let param_period = 3;
        let mut output_tema = vec![0.0; input.len()];
        let mut ema1 = vec![0.0; input.len()];
        let mut ema2 = vec![0.0; input.len()];
        let mut ema3 = vec![0.0; input.len()];

        tema(
            &input,
            param_period,
            &mut output_tema,
            &mut ema1,
            &mut ema2,
            &mut ema3,
        )
        .unwrap();

        // First 6 values should be NaN
        for value in output_tema.iter().take(6) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            35_209.883_333_333_34,
            35_245.566_666_666_68,
            35_206.030_208_333_33,
            35_184.880_729_166_66,
            35_173.019_270_833_32,
            35_220.059_635_416_67,
            35_216.397_591_145_84,
            35_168.941_569_010_41,
            35_096.534_114_583_344,
            35_039.869_694_010_4,
            34_995.421_651_204_42,
            35_003.259_470_621_76,
            35_058.344_179_280_6,
            35_033.424_372_355_13,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_tema[i + 6], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        let mut prev_ema1 = ema1[10];
        let mut prev_ema2 = ema2[10];
        let mut prev_ema3 = ema3[10];

        for i in 11..15 {
            let (tema_val, new_ema1, new_ema2, new_ema3) =
                tema_inc(input[i], prev_ema1, prev_ema2, prev_ema3, param_period).unwrap();

            assert_relative_eq!(tema_val, output_tema[i], epsilon = 0.0001);
            assert_relative_eq!(new_ema1, ema1[i], epsilon = 0.0001);
            assert_relative_eq!(new_ema2, ema2[i], epsilon = 0.0001);
            assert_relative_eq!(new_ema3, ema3[i], epsilon = 0.0001);

            prev_ema1 = new_ema1;
            prev_ema2 = new_ema2;
            prev_ema3 = new_ema3;
        }
    }
}
