use crate::{
    TAFloat,
    error::KandError,
    helper::{highest_bars, lowest_bars},
};

/// Returns the lookback period required for Aroon Oscillator calculation.
///
/// # Description
/// The lookback period represents the minimum number of data points needed before the first valid output
/// can be calculated. For the Aroon Oscillator, this equals the specified period parameter.
///
/// # Arguments
/// * `param_period` - The time period for Aroon calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2 (when "check" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::aroonosc::lookback;
/// let period = 14;
/// let lookback_period = lookback(period).unwrap();
/// assert_eq!(lookback_period, 14);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period)
}

/// Calculates Aroon Oscillator values for an entire price series.
///
/// # Description
/// The Aroon Oscillator is a trend-following indicator that measures the strength of a trend and the likelihood
/// that the trend will continue. It oscillates between -100 and +100.
///
/// # Mathematical Formula
/// ```text
/// Aroon Up = ((Period - Days Since Period High) / Period) * 100
/// Aroon Down = ((Period - Days Since Period Low) / Period) * 100
/// Aroon Oscillator = Aroon Up - Aroon Down
/// ```
///
/// # Calculation Principle
/// 1. Calculate days since highest price within period
/// 2. Calculate days since lowest price within period
/// 3. Calculate Aroon Up and Aroon Down using above values
/// 4. Subtract Aroon Down from Aroon Up to get oscillator value
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_period` - The time period for Aroon calculation (must be >= 2)
/// * `output_aroonosc` - Array to store the calculated Aroon Oscillator values
/// * `output_prev_high` - Array to store highest prices within the period
/// * `output_prev_low` - Array to store lowest prices within the period
/// * `output_days_since_high` - Array to store number of days since highest price
/// * `output_days_since_low` - Array to store number of days since lowest price
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input contains NaN (when "`check-nan`" enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::aroonosc;
///
/// let high = vec![10.0, 11.0, 12.0, 11.0, 10.0];
/// let low = vec![9.0, 10.0, 11.0, 10.0, 9.0];
/// let period = 2;
/// let mut aroon_osc = vec![0.0; 5];
/// let mut prev_high = vec![0.0; 5];
/// let mut prev_low = vec![0.0; 5];
/// let mut days_high = vec![0; 5];
/// let mut days_low = vec![0; 5];
///
/// aroonosc::aroonosc(
///     &high,
///     &low,
///     period,
///     &mut aroon_osc,
///     &mut prev_high,
///     &mut prev_low,
///     &mut days_high,
///     &mut days_low,
/// )
/// .unwrap();
/// ```
pub fn aroonosc(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    param_period: usize,
    output_aroonosc: &mut [TAFloat],
    output_prev_high: &mut [TAFloat],
    output_prev_low: &mut [TAFloat],
    output_days_since_high: &mut [usize],
    output_days_since_low: &mut [usize],
) -> Result<(), KandError> {
    let len = input_high.len();
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
        if len != input_low.len()
            || len != output_aroonosc.len()
            || len != output_prev_high.len()
            || len != output_prev_low.len()
            || len != output_days_since_high.len()
            || len != output_days_since_low.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let param_period_t = param_period as TAFloat;
    let hundred_t = 100.0;

    for i in lookback..len {
        let days_since_high = highest_bars(input_high, i, param_period + 1)?;
        let days_since_low = lowest_bars(input_low, i, param_period + 1)?;

        output_days_since_high[i] = days_since_high;
        output_days_since_low[i] = days_since_low;

        output_prev_high[i] = input_high[i - days_since_high];
        output_prev_low[i] = input_low[i - days_since_low];

        // Calculate Aroon Up and Down values
        let days_since_high_t = days_since_high as TAFloat;
        let days_since_low_t = days_since_low as TAFloat;

        let aroon_up = hundred_t - (hundred_t * days_since_high_t / param_period_t);
        let aroon_down = hundred_t - (hundred_t * days_since_low_t / param_period_t);

        output_aroonosc[i] = aroon_up - aroon_down;
    }

    for i in 0..lookback {
        output_aroonosc[i] = TAFloat::NAN;
        output_prev_high[i] = TAFloat::NAN;
        output_prev_low[i] = TAFloat::NAN;
        output_days_since_high[i] = 0;
        output_days_since_low[i] = 0;
    }

    Ok(())
}

/// Calculates the next Aroon Oscillator value incrementally.
///
/// # Description
/// This function provides an efficient way to calculate the next Aroon Oscillator value
/// when new price data becomes available, without recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// Aroon Up = ((Period - Days Since Period High) / Period) * 100
/// Aroon Down = ((Period - Days Since Period Low) / Period) * 100
/// Aroon Oscillator = Aroon Up - Aroon Down
/// ```
///
/// # Calculation Principle
/// 1. Update days since high/low values
/// 2. Check if new high/low prices are set
/// 3. Calculate new Aroon Up and Down values
/// 4. Calculate oscillator as difference between Up and Down
///
/// # Arguments
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `prev_high` - Previous highest price within the period
/// * `prev_low` - Previous lowest price within the period
/// * `input_days_since_high` - Days since previous highest price
/// * `input_days_since_low` - Days since previous lowest price
/// * `param_period` - The time period for Aroon calculation (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, usize, usize), KandError>` - Returns tuple containing:
///   - Aroon Oscillator value
///   - New highest price
///   - New lowest price
///   - Updated days since highest price
///   - Updated days since lowest price
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2 (when "check" enabled)
/// * `KandError::NaNDetected` - If any input is NaN (when "`check-nan`" enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::aroonosc::aroonosc_inc;
///
/// let (aroonosc, high, low, days_high, days_low) = aroonosc_inc(
///     10.0, // current high
///     9.0,  // current low
///     11.0, // previous high
///     10.0, // previous low
///     1,    // days since high
///     2,    // days since low
///     14,   // period
/// )
/// .unwrap();
/// ```
pub fn aroonosc_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    input_days_since_high: usize,
    input_days_since_low: usize,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, usize, usize), KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan() || input_low.is_nan() || prev_high.is_nan() || prev_low.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    let mut new_high = prev_high;
    let mut new_low = prev_low;
    let mut days_since_high = input_days_since_high;
    let mut days_since_low = input_days_since_low;

    if days_since_high < param_period {
        days_since_high += 1;
    }
    if days_since_low < param_period {
        days_since_low += 1;
    }

    if input_high >= prev_high {
        new_high = input_high;
        days_since_high = 0;
    }
    if input_low <= prev_low {
        new_low = input_low;
        days_since_low = 0;
    }

    let param_period_t = param_period as TAFloat;
    let hundred_t = 100.0;
    let days_since_high_t = days_since_high as TAFloat;
    let days_since_low_t = days_since_low as TAFloat;

    let aroon_up = hundred_t - (hundred_t * days_since_high_t / param_period_t);
    let aroon_down = hundred_t - (hundred_t * days_since_low_t / param_period_t);
    let aroon_osc = aroon_up - aroon_down;

    Ok((
        aroon_osc,
        new_high,
        new_low,
        days_since_high,
        days_since_low,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_aroonosc_calculation() {
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
        let param_period = 14;
        let mut output_aroonosc = vec![0.0; input_high.len()];
        let mut output_prev_high = vec![0.0; input_high.len()];
        let mut output_prev_low = vec![0.0; input_high.len()];
        let mut output_days_since_high = vec![0; input_high.len()];
        let mut output_days_since_low = vec![0; input_high.len()];

        aroonosc(
            &input_high,
            &input_low,
            param_period,
            &mut output_aroonosc,
            &mut output_prev_high,
            &mut output_prev_low,
            &mut output_days_since_high,
            &mut output_days_since_low,
        )
        .unwrap();

        // First 13 values should be NaN
        for value in output_aroonosc.iter().take(14) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            -50.0,
            -57.142_857_142_857_146,
            -64.285_714_285_714_29,
            -64.285_714_285_714_29,
            -64.285_714_285_714_29,
            -64.285_714_285_714_29,
            -92.857_142_857_142_86,
            -100.0,
            -92.857_142_857_142_86,
            -64.285_714_285_714_29,
            -64.285_714_285_714_29,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_aroonosc[i + 14], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_high = output_prev_high[14];
        let mut prev_low = output_prev_low[14];
        let mut days_since_high = output_days_since_high[14];
        let mut days_since_low = output_days_since_low[14];

        for i in 15..20 {
            let (aroon_osc, new_high, new_low, new_days_high, new_days_low) = aroonosc_inc(
                input_high[i],
                input_low[i],
                prev_high,
                prev_low,
                days_since_high,
                days_since_low,
                param_period,
            )
            .unwrap();

            assert_relative_eq!(aroon_osc, output_aroonosc[i], epsilon = 0.0001);

            prev_high = new_high;
            prev_low = new_low;
            days_since_high = new_days_high;
            days_since_low = new_days_low;
        }
    }
}
