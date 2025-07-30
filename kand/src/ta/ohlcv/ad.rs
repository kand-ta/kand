use crate::{KandError, TAFloat};

/// Returns the lookback period required for A/D calculation.
///
/// The A/D indicator requires no lookback period, as it can be calculated starting from the first data point.
///
/// # Errors
///
/// This function always returns `Ok(0)`.
///
/// # Examples
///
/// ```
/// use kand::ohlcv::ad;
/// let lookback = ad::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
#[must_use]
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates the Accumulation/Distribution (A/D) indicator for the entire price series.
///
/// The A/D indicator measures the cumulative flow of money into and out of a security by analyzing
/// price and volume data. It starts from 0 and accumulates the money flow volume over time.
///
/// # Formula
///
/// ```text
/// Money Flow Multiplier (MFM) = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume (MFV) = MFM * Volume
/// A/D = Previous A/D + MFV
/// ```
///
/// # Calculation
///
/// 1. Compute the Money Flow Multiplier (MFM), ranging from -1 to +1:
///    - Positive values indicate buying pressure (close near high).
///    - Negative values indicate selling pressure (close near low).
/// 2. Multiply MFM by volume to get Money Flow Volume (MFV).
/// 3. Accumulate MFV to form the A/D line, starting from 0.
///
/// If High - Low is zero, MFM is set to 0 to avoid division by zero.
///
/// # Errors
///
/// - [`KandError::InvalidData`] if input arrays are empty (enabled by "check" feature).
/// - [`KandError::LengthMismatch`] if input arrays have different lengths (enabled by "check" feature).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::ad;
/// let input_high = vec![10.0, 12.0, 15.0];
/// let input_low = vec![8.0, 9.0, 11.0];
/// let input_close = vec![9.0, 11.0, 13.0];
/// let input_volume = vec![100.0, 150.0, 200.0];
/// let mut output_ad = vec![0.0; 3];
///
/// ad::ad(
///     &input_high,
///     &input_low,
///     &input_close,
///     &input_volume,
///     &mut output_ad,
/// )
/// .unwrap();
/// ```
pub fn ad(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    input_volume: &[TAFloat],
    output_ad: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback()?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        if len != input_low.len()
            || len != input_close.len()
            || len != input_volume.len()
            || len != output_ad.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in lookback..len {
            if input_high[i].is_nan()
                || input_low[i].is_nan()
                || input_close[i].is_nan()
                || input_volume[i].is_nan()
            {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut ad = 0.0;
    for i in lookback..len {
        let high_low_diff = input_high[i] - input_low[i];
        let mfm = if high_low_diff == 0.0 {
            0.0
        } else {
            ((input_close[i] - input_low[i]) - (input_high[i] - input_close[i])) / high_low_diff
        };
        ad = mfm.mul_add(input_volume[i], ad);
        output_ad[i] = ad;
    }

    Ok(())
}

/// Calculates the latest A/D value incrementally using the previous A/D value.
///
/// This is an optimized version that computes only the latest A/D value, avoiding recalculation of the entire series.
///
/// # Formula
///
/// ```text
/// Money Flow Multiplier (MFM) = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume (MFV) = MFM * Volume
/// Latest A/D = Previous A/D + MFV
/// ```
///
/// If High - Low is zero, MFM is set to 0 to avoid division by zero.
///
/// # Errors
///
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::ad;
/// let input_high = 15.0;
/// let input_low = 11.0;
/// let input_close = 13.0;
/// let input_volume = 200.0;
/// let prev_ad = 25.0;
///
/// let output_ad = ad::ad_inc(input_high, input_low, input_close, input_volume, prev_ad).unwrap();
/// ```
#[must_use]
pub fn ad_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    input_volume: TAFloat,
    prev_ad: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan()
            || input_low.is_nan()
            || input_close.is_nan()
            || input_volume.is_nan()
            || prev_ad.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high_low_diff = input_high - input_low;
    let mfm = if high_low_diff == 0.0 {
        0.0
    } else {
        ((input_close - input_low) - (input_high - input_close)) / high_low_diff
    };
    Ok(mfm.mul_add(input_volume, prev_ad))
}

#[cfg(test)]
mod tests {
    use crate::EPSILON;

    use approx::assert_relative_eq;

    use super::*;

    /// Tests the calculation of A/D for a full series and verifies incremental calculations match.
    #[test]
    fn test_ad_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0,
        ];
        let input_volume = vec![
            1055.365, 756.488, 682.152, 1197.747, 425.97, 859.638, 741.925, 888.477, 1043.333,
            467.901, 387.47, 566.099, 672.296, 834.915, 1854.024, 3670.795, 3761.198, 1605.442,
            1726.574, 934.713, 2199.061, 2349.823, 837.218, 1000.638, 1218.202,
        ];
        let mut output_ad = vec![0.0; input_high.len()];

        ad(
            &input_high,
            &input_low,
            &input_close,
            &input_volume,
            &mut output_ad,
        )
        .unwrap();

        let expected_values = [
            -1_055.365,
            -1_262.015_380_487_751_1,
            -1_682.083_847_274_164,
            -1_313.393_007_007_976_8,
            -902.421_950_669_947_9,
            -112.958_481_282_220_36,
            -849.961_922_409_807_9,
            -635.816_183_948_236_4,
            -1_096.943_608_639_632,
            -1_167.128_758_639_693_8,
            -1_330.133_379_329_504_6,
            -765.526_076_299_179_7,
            -789.973_203_571_907,
            -1_246.813_486_590_858_2,
            -2_815.387_753_760_547,
            -5_117.296_306_636_379,
            -5_086.466_814_832_707,
            -3_847.125_607_355_984_4,
            -2_629.436_575_777_188,
            -3_492.706_543_930_014,
            -5_352.790_336_125_074,
            -5_039.053_750_294_157,
            -4_512.022_865_217_038,
            -3_664.661_784_292_062_7,
            -2_976.517_143_070_741_4,
        ];

        for (i, &expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_ad[i], expected, epsilon = EPSILON);
        }

        // Verify incremental calculation matches full series
        let mut prev_ad = output_ad[0];
        for i in 1..input_high.len() {
            let result = ad_inc(
                input_high[i],
                input_low[i],
                input_close[i],
                input_volume[i],
                prev_ad,
            )
            .unwrap();
            assert_relative_eq!(result, output_ad[i], epsilon = EPSILON);
            prev_ad = result;
        }
    }
}
