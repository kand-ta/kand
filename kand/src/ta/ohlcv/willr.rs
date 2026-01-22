use crate::{
    EPSILON, KandError, TAFloat,
    helper::{highest_bars, lowest_bars},
};

/// Returns the lookback period required for Williams %R calculation
///
/// # Description
/// The lookback period is the number of data points needed before the first valid output can be calculated.
/// For Williams %R, this is one less than the period parameter.
///
/// # Arguments
/// * `opt_period` - The period used for Williams %R calculation. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 2
///
/// # Example
/// ```
/// use kand::ohlcv::willr;
/// let lookback = willr::lookback(14).unwrap();
/// assert_eq!(lookback, 13);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period - 1)
}

/// Calculates Williams %R (Williams Percent Range) for the entire price series
///
/// # Description
/// Williams %R is a momentum indicator that measures overbought and oversold levels by comparing
/// the closing price to the high-low range over a specified period. The indicator oscillates
/// between 0 and -100.
///
/// # Mathematical Formula
/// ```text
/// %R = (Highest High - Close) / (Highest High - Lowest Low) × -100
/// ```
///
/// # Calculation Principles
/// 1. Find the highest high and lowest low over the lookback period
/// 2. Calculate the high-low range (denominator)
/// 3. Compare current close to the highest high (numerator)
/// 4. Normalize to -100 to 0 range
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `opt_period` - Lookback period for calculations. Must be >= 2.
/// * `output` - Array to store calculated Williams %R values
/// * `output_highest_high` - Array to store highest high values for each period
/// * `output_lowest_low` - Array to store lowest low values for each period
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok value if successful
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If `opt_period` < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::willr;
///
/// let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let input_close = vec![9.0, 11.0, 14.0, 12.0, 11.0];
/// let opt_period = 3;
/// let mut output = vec![0.0; 5];
/// let mut output_highest_high = vec![0.0; 5];
/// let mut output_lowest_low = vec![0.0; 5];
///
/// willr::willr(
///     &input_high,
///     &input_low,
///     &input_close,
///     opt_period,
///     &mut output,
///     &mut output_highest_high,
///     &mut output_lowest_low,
/// )
/// .unwrap();
/// ```
pub fn willr(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    opt_period: usize,
    output: &mut [TAFloat],
    output_highest_high: &mut [TAFloat],
    output_lowest_low: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(opt_period)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }
        if len != input_low.len()
            || len != input_close.len()
            || len != output.len()
            || len != output_highest_high.len()
            || len != output_lowest_low.len()
        {
            return Err(KandError::LengthMismatch);
        }
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() || input_close[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    for i in lookback..len {
        let highest_idx = highest_bars(input_high, i, opt_period)?;
        let lowest_idx = lowest_bars(input_low, i, opt_period)?;

        let highest_high = input_high[i - highest_idx];
        let lowest_low = input_low[i - lowest_idx];

        output_highest_high[i] = highest_high;
        output_lowest_low[i] = lowest_low;

        let denom = highest_high - lowest_low;
        if denom == 0.0 {
            output[i] = 0.0;
        } else {
            output[i] = (highest_high - input_close[i]) / denom * -100.0;
        }
    }

    for i in 0..lookback {
        output[i] = TAFloat::NAN;
        output_highest_high[i] = TAFloat::NAN;
        output_lowest_low[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates Williams %R incrementally for the latest data point
///
/// # Description
/// This function provides an optimized way to calculate the latest Williams %R value
/// by using previously calculated highest high and lowest low values. This is useful
/// for real-time calculations where a complete recalculation is not necessary.
///
/// # Arguments
/// * `prev_highest_high` - Previous period's highest high value
/// * `prev_lowest_low` - Previous period's lowest low value
/// * `prev_high` - Previous period's high price
/// * `prev_low` - Previous period's low price
/// * `input_close` - Current period's closing price
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing:
///   - Current Williams %R value
///   - New highest high
///   - New lowest low
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::willr::willr_inc;
///
/// let prev_highest_high = 15.0;
/// let prev_lowest_low = 10.0;
/// let prev_high = 14.0;
/// let prev_low = 11.0;
/// let input_close = 12.0;
/// let input_high = 13.0;
/// let input_low = 11.0;
///
/// let (willr, new_highest_high, new_lowest_low) = willr_inc(
///     prev_highest_high,
///     prev_lowest_low,
///     prev_high,
///     prev_low,
///     input_close,
///     input_high,
///     input_low,
/// )
/// .unwrap();
/// ```
pub fn willr_inc(
    prev_highest_high: TAFloat,
    prev_lowest_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    input_close: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check-nan")]
    {
        if prev_highest_high.is_nan()
            || prev_lowest_low.is_nan()
            || prev_high.is_nan()
            || prev_low.is_nan()
            || input_close.is_nan()
            || input_high.is_nan()
            || input_low.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    // Update highest high by removing old high and considering new high
    let new_highest_high = if input_high > prev_highest_high {
        input_high
    } else if (prev_high - prev_highest_high).abs() < EPSILON {
        // If previous high was the highest, need to find new highest between current high and previous highest
        input_high.max(prev_highest_high)
    } else {
        prev_highest_high
    };

    // Update lowest low by removing old low and considering new low
    let new_lowest_low = if input_low < prev_lowest_low {
        input_low
    } else if (prev_low - prev_lowest_low).abs() < EPSILON {
        // If previous low was the lowest, need to find new lowest between current low and previous lowest
        input_low.min(prev_lowest_low)
    } else {
        prev_lowest_low
    };

    let denom = new_highest_high - new_lowest_low;
    let willr = if denom == 0.0 {
        0.0
    } else {
        (new_highest_high - input_close) / denom * -100.0
    };

    Ok((willr, new_highest_high, new_lowest_low))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_willr_calculation() {
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
        let opt_period = 14;
        let mut output = vec![0.0; input_high.len()];
        let mut output_highest_high = vec![0.0; input_high.len()];
        let mut output_lowest_low = vec![0.0; input_high.len()];

        willr(
            &input_high,
            &input_low,
            &input_close,
            opt_period,
            &mut output,
            &mut output_highest_high,
            &mut output_lowest_low,
        )
        .unwrap();

        // First 13 values should be NaN
        for i in 0..13 {
            assert!(output[i].is_nan());
            assert!(output_highest_high[i].is_nan());
            assert!(output_lowest_low[i].is_nan());
        }

        // Compare with known values
        let expected_values = [
            -80.106_100_795_756_35,
            -94.935_451_837_137_89,
            -92.281_105_990_784,
            -85.153_892_576_945_03,
            -80.899_215_449_606_93,
            -64.121_907_060_953_25,
            -77.519_613_759_806_97,
            -97.742_212_060_588_33,
            -87.942_028_985_507_66,
            -73.030_303_030_303_03,
            -60.363_636_363_635_486,
            -48.787_878_787_878_79,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output[i + 13], *expected, epsilon = 0.0001);
        }
        // Test incremental calculation matches regular calculation
        let mut prev_highest_high = output_highest_high[13];
        let mut prev_lowest_low = output_lowest_low[13];

        for i in 14..19 {
            let (result, highest_high, lowest_low) = willr_inc(
                prev_highest_high,
                prev_lowest_low,
                input_high[i - 1],
                input_low[i - 1],
                input_close[i],
                input_high[i],
                input_low[i],
            )
            .unwrap();

            assert_relative_eq!(result, output[i], epsilon = 0.0001);

            prev_highest_high = highest_high;
            prev_lowest_low = lowest_low;
        }
    }
}
