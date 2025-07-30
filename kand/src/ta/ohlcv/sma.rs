use crate::{KandError, TAFloat, TAPeriod};

/// Returns the lookback period required for Simple Moving Average (SMA) calculation.
///
/// The lookback period is the number of data points needed before the first valid SMA value, equal to the period minus one.
///
/// # Errors
///
/// - [`KandError::InvalidParameter`] if period is less than 2 (enabled by "check" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::sma;
/// let lookback = sma::lookback(14).unwrap();
/// assert_eq!(lookback, 13);
/// ```
#[must_use]
pub const fn lookback(param_period: TAPeriod) -> Result<TAPeriod, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates the Simple Moving Average (SMA) for the entire price series.
///
/// The SMA smooths price data by computing the arithmetic mean over a specified period, commonly used to identify trends.
///
/// # Formula
///
/// ```text
/// SMA = (P1 + P2 + ... + Pn) / n
/// ```
///
/// Where `P1, P2, ..., Pn` are the input values, and `n` is the period.
///
/// # Calculation
///
/// 1. Sum the prices over the period.
/// 2. Divide by the period to compute the mean.
/// 3. Slide the window forward and repeat.
/// 4. Set the first `period - 1` values to NaN, as they lack sufficient data.
///
/// # Errors
///
/// - [`KandError::InvalidData`] if input is empty (enabled by "check" feature).
/// - [`KandError::LengthMismatch`] if input and output lengths differ (enabled by "check" feature).
/// - [`KandError::InvalidParameter`] if period is less than 2 (propagated from lookback).
/// - [`KandError::InsufficientData`] if input length is less than or equal to lookback (enabled by "check" feature).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::sma;
/// let prices = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// let period = 3;
/// let mut sma_values = vec![0.0; 5];
///
/// sma::sma(&prices, period, &mut sma_values).unwrap();
/// ```
pub fn sma(
    input: &[TAFloat],
    param_period: TAPeriod,
    output_sma: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        if output_sma.len() != len {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input.iter().any(|&price| price.is_nan()) {
            return Err(KandError::NaNDetected);
        }
    }

    let mut sum = input.iter().take(param_period).sum::<TAFloat>();
    let period_float = param_period as TAFloat; // TODO: find a safe way to convert TAPeriod to TAFloat
    output_sma[lookback] = sum / period_float;

    for i in lookback + 1..len {
        sum = sum + input[i] - input[i - param_period];
        output_sma[i] = sum / period_float;
    }

    #[cfg(feature = "allow-nan")]
    {
        for value in output_sma.iter_mut().take(lookback) {
            *value = TAFloat::NAN;
        }
    }

    Ok(())
}

/// Calculates the next SMA value incrementally using the previous SMA value.
///
/// This optimized version computes only the latest SMA value, avoiding recalculation of the entire series.
///
/// # Formula
///
/// ```text
/// Latest SMA = Previous SMA + (New Price - Old Price) / period
/// ```
///
/// # Errors
///
/// - [`KandError::InvalidParameter`] if period is less than 2 (enabled by "check" feature).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::sma;
/// let prev_sma = 4.0;
/// let new_price = 10.0;
/// let old_price = 2.0;
/// let period = 3;
///
/// let next_sma = sma::sma_inc(prev_sma, new_price, old_price, period).unwrap();
/// ```
#[must_use]
pub fn sma_inc(
    prev_sma: TAFloat,
    input_new_price: TAFloat,
    input_old_price: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if prev_sma.is_nan() || input_new_price.is_nan() || input_old_price.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(prev_sma + (input_new_price - input_old_price) / param_period as TAFloat) // TODO: find a safe way to convert TAPeriod to TAFloat
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
        let mut output_sma = vec![0.0; INPUT_DATA.len()];
        sma(&INPUT_DATA, PERIOD, &mut output_sma).unwrap();

        // Check that initial values are NaN
        for &val in output_sma.iter().take(LOOKBACK) {
            assert!(val.is_nan());
        }

        // Check the calculated SMA values
        for (i, &expected) in EXPECTED_VALUES.iter().enumerate() {
            assert_relative_eq!(output_sma[LOOKBACK + i], expected, epsilon = EPSILON);
        }

        // Check incremental calculation
        let mut prev_sma = output_sma[LOOKBACK];
        for i in (LOOKBACK + 1)..INPUT_DATA.len() {
            let result = sma_inc(prev_sma, INPUT_DATA[i], INPUT_DATA[i - PERIOD], PERIOD).unwrap();
            assert_relative_eq!(result, output_sma[i], epsilon = EPSILON);
            prev_sma = result;
        }
    }

    /// Tests SMA without `allow-nan` feature.
    #[test]
    #[cfg(not(feature = "allow-nan"))]
    fn test_sma_without_nan() {
        let mut output_sma = vec![0.0; INPUT_DATA.len()];
        sma(&INPUT_DATA, PERIOD, &mut output_sma).unwrap();

        // Check that initial values are 0.0
        for &val in output_sma.iter().take(LOOKBACK) {
            assert_eq!(val, 0.0);
        }

        // Check the calculated SMA values
        for (i, &expected) in EXPECTED_VALUES.iter().enumerate() {
            assert_relative_eq!(output_sma[LOOKBACK + i], expected, epsilon = EPSILON);
        }

        // Check incremental calculation
        let mut prev_sma = output_sma[LOOKBACK];
        for i in (LOOKBACK + 1)..INPUT_DATA.len() {
            let result = sma_inc(prev_sma, INPUT_DATA[i], INPUT_DATA[i - PERIOD], PERIOD).unwrap();
            assert_relative_eq!(result, output_sma[i], epsilon = EPSILON);
            prev_sma = result;
        }
    }
}
