use crate::{KandError, TAFloat};

/// Returns the lookback period required for Expanded Camarilla Levels (ECL) calculation.
///
/// # Description
/// The lookback period represents the minimum number of historical data points needed
/// before generating the first valid output. For ECL, this equals 1 since calculation
/// requires the previous period's data.
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (1)
///
/// # Errors
/// This function does not return any errors.
///
/// # Example
/// ```
/// use kand::ohlcv::ecl;
/// let lookback = ecl::lookback().unwrap();
/// assert_eq!(lookback, 1);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(1)
}

/// Calculates Expanded Camarilla Levels (ECL) for price data.
///
/// # Description
/// ECL provides support and resistance levels based on the previous period's high, low and close prices.
/// The levels are calculated using various ratios of the previous period's range.
///
/// # Mathematical Formula
/// ```text
/// Range = High[t-1] - Low[t-1]
/// H5 = (High[t-1]/Low[t-1]) * Close[t-1]
/// H4 = Close[t-1] + Range * 1.1/2
/// H3 = Close[t-1] + Range * 1.1/4
/// H2 = Close[t-1] + Range * 1.1/6
/// H1 = Close[t-1] + Range * 1.1/12
/// L1 = Close[t-1] - Range * 1.1/12
/// L2 = Close[t-1] - Range * 1.1/6
/// L3 = Close[t-1] - Range * 1.1/4
/// L4 = Close[t-1] - Range * 1.1/2
/// L5 = Close[t-1] - (H5 - Close[t-1])
/// ```
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `output_h5` - Output array for H5 resistance levels
/// * `output_h4` - Output array for H4 resistance levels
/// * `output_h3` - Output array for H3 resistance levels
/// * `output_h2` - Output array for H2 resistance levels
/// * `output_h1` - Output array for H1 resistance levels
/// * `output_l1` - Output array for L1 support levels
/// * `output_l2` - Output array for L2 support levels
/// * `output_l3` - Output array for L3 support levels
/// * `output_l4` - Output array for L4 support levels
/// * `output_l5` - Output array for L5 support levels
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - Input array is empty or too short
/// * `KandError::LengthMismatch` - Input/output arrays have different lengths
/// * `KandError::InsufficientData` - Input length <= lookback period
/// * `KandError::NaNDetected` - Input contains NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::ecl;
/// let high = vec![24.20, 24.07, 24.04];
/// let low = vec![23.85, 23.72, 23.64];
/// let close = vec![23.89, 23.95, 23.67];
/// let mut h5 = vec![0.0; 3];
/// let mut h4 = vec![0.0; 3];
/// let mut h3 = vec![0.0; 3];
/// let mut h2 = vec![0.0; 3];
/// let mut h1 = vec![0.0; 3];
/// let mut l1 = vec![0.0; 3];
/// let mut l2 = vec![0.0; 3];
/// let mut l3 = vec![0.0; 3];
/// let mut l4 = vec![0.0; 3];
/// let mut l5 = vec![0.0; 3];
///
/// ecl::ecl(
///     &high, &low, &close, &mut h5, &mut h4, &mut h3, &mut h2, &mut h1, &mut l1, &mut l2,
///     &mut l3, &mut l4, &mut l5,
/// )
/// .unwrap();
/// ```
#[allow(clippy::similar_names)]
pub fn ecl(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    output_h5: &mut [TAFloat],
    output_h4: &mut [TAFloat],
    output_h3: &mut [TAFloat],
    output_h2: &mut [TAFloat],
    output_h1: &mut [TAFloat],
    output_l1: &mut [TAFloat],
    output_l2: &mut [TAFloat],
    output_l3: &mut [TAFloat],
    output_l4: &mut [TAFloat],
    output_l5: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback()?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        // ECL requires at least 2 periods:
        // - One for initial range calculation (needs previous prices)
        // - One for the current period
        if len < 2 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len != input_low.len()
            || len != input_close.len()
            || len != output_h5.len()
            || len != output_h4.len()
            || len != output_h3.len()
            || len != output_h2.len()
            || len != output_h1.len()
            || len != output_l1.len()
            || len != output_l2.len()
            || len != output_l3.len()
            || len != output_l4.len()
            || len != output_l5.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Data sufficiency check
        if len <= lookback + 1 {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            // NaN check
            if input_high[i].is_nan() || input_low[i].is_nan() || input_close[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let opt_factor = 1.1;

    for i in lookback..len {
        let range = input_high[i - 1] - input_low[i - 1];
        let h5_val = (input_high[i - 1] / input_low[i - 1]) * input_close[i - 1];

        output_h5[i] = h5_val;
        output_h4[i] = input_close[i - 1] + range * opt_factor / 2.0;
        output_h3[i] = input_close[i - 1] + range * opt_factor / 4.0;
        output_h2[i] = input_close[i - 1] + range * opt_factor / 6.0;
        output_h1[i] = input_close[i - 1] + range * opt_factor / 12.0;
        output_l1[i] = input_close[i - 1] - range * opt_factor / 12.0;
        output_l2[i] = input_close[i - 1] - range * opt_factor / 6.0;
        output_l3[i] = input_close[i - 1] - range * opt_factor / 4.0;
        output_l4[i] = input_close[i - 1] - range * opt_factor / 2.0;
        output_l5[i] = input_close[i - 1] - (h5_val - input_close[i - 1]);
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_h5[i] = TAFloat::NAN;
        output_h4[i] = TAFloat::NAN;
        output_h3[i] = TAFloat::NAN;
        output_h2[i] = TAFloat::NAN;
        output_h1[i] = TAFloat::NAN;
        output_l1[i] = TAFloat::NAN;
        output_l2[i] = TAFloat::NAN;
        output_l3[i] = TAFloat::NAN;
        output_l4[i] = TAFloat::NAN;
        output_l5[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Incrementally calculates Expanded Camarilla Levels (ECL) for a single period.
///
/// # Description
/// Provides an efficient way to calculate ECL values for new data without reprocessing
/// the entire dataset. Uses only the previous period's prices to generate new levels.
///
/// # Mathematical Formula
/// ```text
/// Range = prev_high - prev_low
/// H5 = (prev_high/prev_low) * prev_close
/// H4 = prev_close + Range * 1.1/2
/// H3 = prev_close + Range * 1.1/4
/// H2 = prev_close + Range * 1.1/6
/// H1 = prev_close + Range * 1.1/12
/// L1 = prev_close - Range * 1.1/12
/// L2 = prev_close - Range * 1.1/6
/// L3 = prev_close - Range * 1.1/4
/// L4 = prev_close - Range * 1.1/2
/// L5 = prev_close - (H5 - prev_close)
/// ```
///
/// # Arguments
/// * `prev_high` - Previous period's high price
/// * `prev_low` - Previous period's low price
/// * `prev_close` - Previous period's close price
///
/// # Returns
/// * `Result<(TAFloat,TAFloat,TAFloat,TAFloat,TAFloat,TAFloat,TAFloat,TAFloat,TAFloat,TAFloat), KandError>` - Tuple containing (H5,H4,H3,H2,H1,L1,L2,L3,L4,L5)
///
/// # Errors
/// * `KandError::NaNDetected` - Input contains NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::ecl;
/// let (h5, h4, h3, h2, h1, l1, l2, l3, l4, l5) = ecl::ecl_inc(
///     24.20, // prev_high
///     23.85, // prev_low
///     23.89, // prev_close
/// )
/// .unwrap();
/// ```
#[allow(clippy::similar_names)]
pub fn ecl_inc(
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
) -> Result<
    (
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
        TAFloat,
    ),
    KandError,
> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if prev_high.is_nan() || prev_low.is_nan() || prev_close.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let opt_factor = 1.1;
    let range = prev_high - prev_low;
    let h5_val = (prev_high / prev_low) * prev_close;

    let h4 = prev_close + range * opt_factor / 2.0;
    let h3 = prev_close + range * opt_factor / 4.0;
    let h2 = prev_close + range * opt_factor / 6.0;
    let h1 = prev_close + range * opt_factor / 12.0;
    let l1 = prev_close - range * opt_factor / 12.0;
    let l2 = prev_close - range * opt_factor / 6.0;
    let l3 = prev_close - range * opt_factor / 4.0;
    let l4 = prev_close - range * opt_factor / 2.0;
    let l5 = prev_close - (h5_val - prev_close);

    Ok((h5_val, h4, h3, h2, h1, l1, l2, l3, l4, l5))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    #[allow(clippy::similar_names)]
    fn test_ecl_calculation() {
        let input_high = vec![24.20, 24.07, 24.04, 23.87, 23.67];
        let input_low = vec![23.85, 23.72, 23.64, 23.37, 23.46];
        let input_close = vec![23.89, 23.95, 23.67, 23.78, 23.50];
        let mut output_h5 = vec![0.0; 5];
        let mut output_h4 = vec![0.0; 5];
        let mut output_h3 = vec![0.0; 5];
        let mut output_h2 = vec![0.0; 5];
        let mut output_h1 = vec![0.0; 5];
        let mut output_l1 = vec![0.0; 5];
        let mut output_l2 = vec![0.0; 5];
        let mut output_l3 = vec![0.0; 5];
        let mut output_l4 = vec![0.0; 5];
        let mut output_l5 = vec![0.0; 5];

        ecl(
            &input_high,
            &input_low,
            &input_close,
            &mut output_h5,
            &mut output_h4,
            &mut output_h3,
            &mut output_h2,
            &mut output_h1,
            &mut output_l1,
            &mut output_l2,
            &mut output_l3,
            &mut output_l4,
            &mut output_l5,
        )
        .unwrap();

        // First value should be NaN
        let outputs = [
            &output_h5, &output_h4, &output_h3, &output_h2, &output_h1, &output_l1, &output_l2,
            &output_l3, &output_l4, &output_l5,
        ];
        for output in outputs {
            assert!(output[0].is_nan());
        }

        // Verify remaining values are calculated
        let outputs = [
            &output_h5, &output_h4, &output_h3, &output_h2, &output_h1, &output_l1, &output_l2,
            &output_l3, &output_l4, &output_l5,
        ];
        for i in 1..5 {
            for output in &outputs {
                assert!(output[i].is_finite());
            }
        }

        // Test incremental calculation matches
        let i = input_high.len() - 1;
        let (h5_inc, h4_inc, h3_inc, h2_inc, h1_inc, l1_inc, l2_inc, l3_inc, l4_inc, l5_inc) =
            ecl_inc(input_high[i - 1], input_low[i - 1], input_close[i - 1]).unwrap();

        assert_relative_eq!(h5_inc, output_h5[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(h4_inc, output_h4[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(h3_inc, output_h3[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(h2_inc, output_h2[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(h1_inc, output_h1[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(l1_inc, output_l1[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(l2_inc, output_l2[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(l3_inc, output_l3[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(l4_inc, output_l4[i], epsilon = TAFloat::EPSILON);
        assert_relative_eq!(l5_inc, output_l5[i], epsilon = TAFloat::EPSILON);
    }
}
