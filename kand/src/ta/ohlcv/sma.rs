use crate::{KandError, TAFloat, TAPeriod};

/// Returns the lookback period for Simple Moving Average (SMA) without input validation.
///
/// Assumes valid inputs; returns `opt_period - 1`.
///
/// # Panics
///
/// Panics in debug builds if `opt_period < 1` due to subtraction overflow.
#[inline]
#[must_use]
pub const fn lookback_raw(opt_period: TAPeriod) -> TAPeriod {
    opt_period - 1
}

/// Returns the lookback period for SMA calculation (`opt_period - 1`).
///
/// # Errors
///
/// Returns [`KandError::InvalidParameter`] if `opt_period < 2` (with "check" feature).
#[must_use]
pub const fn lookback(opt_period: TAPeriod) -> Result<TAPeriod, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period - 1)
}

/// Computes SMA without input validation for high performance.
///
/// Sums prices over `opt_period` and divides by period length.
/// Stores results in `output`, leaving first `lookback` values unset.
///
/// # Assumptions
///
/// - `input` and `output` have equal length and sufficient data.
/// - Inputs are valid; no error checking performed.
pub fn sma_raw(input: &[TAFloat], opt_period: TAPeriod, output: &mut [TAFloat]) {
    let len = input.len();
    let lookback = lookback_raw(opt_period);

    let mut sum = input.iter().take(opt_period).sum::<TAFloat>();
    let period_float = opt_period as TAFloat; // TODO: Safely convert TAPeriod to TAFloat
    output[lookback] = sum / period_float;

    for i in lookback + 1..len {
        sum = sum + input[i] - input[i - opt_period];
        output[i] = sum / period_float;
    }
}

/// Computes SMA over a price series, smoothing data to identify trends.
///
/// Formula: `SMA = (P1 + P2 + ... + Pn) / n`, where `n` is `opt_period`.
/// Sets first `period - 1` output values to NaN (with "allow-nan" feature).
///
/// # Errors
///
/// With "check" feature:
/// - [`KandError::InvalidData`] if `input` is empty.
/// - [`KandError::LengthMismatch`] if `input` and `output` lengths differ.
/// - [`KandError::InsufficientData`] if `input.len() <= lookback`.
/// - [`KandError::InvalidParameter`] from `lookback`.
///
/// With "check-nan" feature:
/// - [`KandError::NaNDetected`] if any input is NaN.
pub fn sma(
    input: &[TAFloat],
    opt_period: TAPeriod,
    output: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(opt_period)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
        if output.len() != len {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input.iter().any(|&price| price.is_nan()) {
            return Err(KandError::NaNDetected);
        }
    }

    sma_raw(input, opt_period, output);

    #[cfg(feature = "allow-nan")]
    {
        for value in output.iter_mut().take(lookback) {
            *value = TAFloat::NAN;
        }
    }

    Ok(())
}

/// Computes the next SMA value incrementally without input validation.
///
/// Formula: `Latest SMA = Previous SMA + (New Price - Old Price) / period`.
///
/// # Assumptions
///
/// Inputs are valid; no error checking performed.
#[must_use]
pub fn sma_inc_raw(
    input: TAFloat,
    prev_input: TAFloat,
    prev_sma: TAFloat,
    opt_period: TAPeriod,
) -> TAFloat {
    prev_sma + (input - prev_input) / opt_period as TAFloat // TODO: Safely convert TAPeriod to TAFloat
}

/// Computes the next SMA value incrementally using the previous SMA.
///
/// Formula: `Latest SMA = Previous SMA + (New Price - Old Price) / period`.
///
/// # Errors
///
/// With "check" feature:
/// - [`KandError::InvalidParameter`] if `opt_period < 2`.
///
/// With "check-nan" feature:
/// - [`KandError::NaNDetected`] if any input is NaN.
#[must_use]
pub fn sma_inc(
    input: TAFloat,
    prev_input: TAFloat,
    prev_sma: TAFloat,
    opt_period: TAPeriod,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if prev_sma.is_nan() || input.is_nan() || prev_input.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(sma_inc_raw(input, prev_input, prev_sma, opt_period))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::EPSILON;

    use super::*;

    const PERIOD: TAPeriod = 14;
    const LOOKBACK: TAPeriod = PERIOD - 1;

    const INPUT_DATA: [f64; 39] = [
        35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6, 35184.7,
        35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4, 35069.0, 35024.6,
        34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2, 35092.0, 35073.2, 35139.3,
        35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3, 35154.0, 35216.3, 35211.8,
    ];

    const EXPECTED_VALUES: [f64; 13] = [
        35_203.535_714_285_72,
        35_194.55,
        35_181.678_571_428_57,
        35_168.007_142_857_15,
        35_156.821_428_571_435,
        35_148.785_714_285_72,
        35_132.357_142_857_145,
        35_113.55,
        35_092.171_428_571_43,
        35_078.057_142_857_15,
        35_067.85,
        35_061.057_142_857_15,
        35_052.814_285_714_29,
    ];

    /// Tests SMA with `allow-nan` feature enabled.
    #[test]
    #[cfg(feature = "allow-nan")]
    fn test_sma_with_nan() {
        let mut output = vec![0.0; INPUT_DATA.len()];
        sma(&INPUT_DATA, PERIOD, &mut output).unwrap();

        // Verify initial values are NaN
        for &val in output.iter().take(LOOKBACK) {
            assert!(val.is_nan());
        }

        // Verify calculated SMA values
        for (i, &expected) in EXPECTED_VALUES.iter().enumerate() {
            assert_relative_eq!(output[LOOKBACK + i], expected, epsilon = EPSILON);
        }

        // Verify incremental calculation
        let mut prev_sma = output[LOOKBACK];
        for i in (LOOKBACK + 1)..INPUT_DATA.len() {
            let result = sma_inc(INPUT_DATA[i], INPUT_DATA[i - PERIOD], prev_sma, PERIOD).unwrap();
            assert_relative_eq!(result, output[i], epsilon = EPSILON);
            prev_sma = result;
        }
    }

    /// Tests SMA without `allow-nan` feature.
    #[test]
    #[cfg(not(feature = "allow-nan"))]
    fn test_sma_without_nan() {
        let mut output = vec![0.0; INPUT_DATA.len()];
        sma(&INPUT_DATA, PERIOD, &mut output).unwrap();

        // Verify initial values are 0.0
        for &val in output.iter().take(LOOKBACK) {
            assert_eq!(val, 0.0);
        }

        // Verify calculated SMA values
        for (i, &expected) in EXPECTED_VALUES.iter().enumerate() {
            assert_relative_eq!(output[LOOKBACK + i], expected, epsilon = EPSILON);
        }

        // Verify incremental calculation
        let mut prev_sma = output[LOOKBACK];
        for i in (LOOKBACK + 1)..INPUT_DATA.len() {
            let result = sma_inc(INPUT_DATA[i], INPUT_DATA[i - PERIOD], prev_sma, PERIOD).unwrap();
            assert_relative_eq!(result, output[i], epsilon = EPSILON);
            prev_sma = result;
        }
    }
}
