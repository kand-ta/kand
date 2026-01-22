use crate::{
    TAFloat,
    error::KandError,
    helper::{highest_bars, lowest_bars},
};

/// Returns the lookback period required for Aroon indicator calculation.
///
/// # Description
/// The lookback period determines how many historical data points are needed
/// to start calculating valid Aroon values.
///
/// # Arguments
/// * `opt_period` - The time period used for Aroon calculations (e.g. 14, 25)
///
/// # Returns
/// * `Result<usize, KandError>` - Returns the lookback period equal to `opt_period` on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `opt_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::aroon;
/// let lookback = aroon::lookback(14).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period)
}

/// Calculates the Aroon indicator for a price series.
///
/// # Description
/// The Aroon indicator consists of two lines that measure the time since the last high/low
/// relative to a lookback period. It helps identify the start of new trends and trend reversals.
///
/// # Calculation Principle
/// 1. Find the number of periods since the highest high and lowest low
/// 2. Convert these periods into percentage values between 0-100
/// 3. Higher values (closer to 100) indicate more recent highs/lows
/// 4. Lower values (closer to 0) indicate older highs/lows
///
/// # Formula
/// ```text
/// Aroon Up = ((Period - Days Since High) / Period) × 100
/// Aroon Down = ((Period - Days Since Low) / Period) × 100
/// ```
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `opt_period` - The lookback period for calculations
/// * `output_aroon_up` - Buffer to store Aroon Up values
/// * `output_aroon_down` - Buffer to store Aroon Down values
/// * `output_prev_high` - Buffer to store highest prices in period
/// * `output_prev_low` - Buffer to store lowest prices in period
/// * `output_days_since_high` - Buffer to store days since highest price
/// * `output_days_since_low` - Buffer to store days since lowest price
///
/// # Returns
/// * `Result<(), KandError>` - `Ok(())` on success, or error on failure
///
/// # Errors
/// * Returns `KandError::InvalidData` if input arrays are empty
/// * Returns `KandError::LengthMismatch` if input/output array lengths don't match
/// * Returns `KandError::InvalidParameter` if period < 2
/// * Returns `KandError::InsufficientData` if input length <= lookback period
/// * Returns `KandError::NaNDetected` if any input contains NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::aroon;
///
/// let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let opt_period = 3;
/// let mut output_aroon_up = vec![0.0; 5];
/// let mut output_aroon_down = vec![0.0; 5];
/// let mut output_prev_high = vec![0.0; 5];
/// let mut output_prev_low = vec![0.0; 5];
/// let mut output_days_since_high = vec![0; 5];
/// let mut output_days_since_low = vec![0; 5];
///
/// aroon::aroon(
///     &input_high,
///     &input_low,
///     opt_period,
///     &mut output_aroon_up,
///     &mut output_aroon_down,
///     &mut output_prev_high,
///     &mut output_prev_low,
///     &mut output_days_since_high,
///     &mut output_days_since_low,
/// )
/// .unwrap();
/// ```
pub fn aroon(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    opt_period: usize,
    output_aroon_up: &mut [TAFloat],
    output_aroon_down: &mut [TAFloat],
    output_prev_high: &mut [TAFloat],
    output_prev_low: &mut [TAFloat],
    output_days_since_high: &mut [usize],
    output_days_since_low: &mut [usize],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(opt_period)?;

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
            || len != output_aroon_up.len()
            || len != output_aroon_down.len()
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
            // NaN check
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let opt_period_t = opt_period as TAFloat;
    let hundred_t = 100.0;

    // Calculate Aroon Up and Down values for each index starting from lookback
    // Note: We use opt_period + 1 in highest_bars/lowest_bars because:
    //
    // Visual example with opt_period = 3:
    // Array:     [1, 4, 2, 5, 3]
    //             0  1  2  3  4  <- indices
    //
    // For i = 4 (current index):
    // With opt_period:     [2, 5, 3]  <- only 3 values
    //                         2  3  4
    // With opt_period+1:   [4, 2, 5, 3]  <- 4 values (includes previous value)
    //                         1  2  3  4
    //
    // Using opt_period + 1 ensures we look at opt_period previous values PLUS
    // the current value, allowing days_since_high/low to range from 0 to opt_period:
    // - If current value is highest/lowest: days_since = 0
    // - If earliest value is highest/lowest: days_since = opt_period
    for i in lookback..len {
        let days_since_high = highest_bars(input_high, i, opt_period + 1)?;
        let days_since_low = lowest_bars(input_low, i, opt_period + 1)?;

        // Store intermediate values
        output_days_since_high[i] = days_since_high;
        output_days_since_low[i] = days_since_low;

        // Get highest high and lowest low from the indices we already calculated
        output_prev_high[i] = input_high[i - days_since_high];
        output_prev_low[i] = input_low[i - days_since_low];

        // Calculate Aroon Up and Down values
        let days_since_high_t = days_since_high as TAFloat;
        let days_since_low_t = days_since_low as TAFloat;

        output_aroon_up[i] = hundred_t - (hundred_t * days_since_high_t / opt_period_t);
        output_aroon_down[i] = hundred_t - (hundred_t * days_since_low_t / opt_period_t);
    }

    // Fill NaN values for lookback period
    for i in 0..lookback {
        output_aroon_up[i] = TAFloat::NAN;
        output_aroon_down[i] = TAFloat::NAN;
        output_prev_high[i] = TAFloat::NAN;
        output_prev_low[i] = TAFloat::NAN;
        output_days_since_high[i] = 0;
        output_days_since_low[i] = 0;
    }

    Ok(())
}

/// Calculates the next Aroon values incrementally.
///
/// # Description
/// This function provides an optimized way to calculate the next Aroon values
/// when processing streaming data, without recalculating the entire series.
///
/// # Calculation Principle
/// 1. Update days since last high/low by incrementing counters
/// 2. Check if current high/low prices create new extremes
/// 3. Calculate Aroon values using updated information
///
/// # Arguments
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `prev_high` - Previous highest price in period
/// * `prev_low` - Previous lowest price in period
/// * `input_days_since_high` - Days since previous highest price
/// * `input_days_since_low` - Days since previous lowest price
/// * `opt_period` - The lookback period
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat, usize, usize), KandError>` - Returns tuple containing:
///   - Aroon Up value
///   - Aroon Down value
///   - New highest price
///   - New lowest price
///   - Updated days since high
///   - Updated days since low
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period < 2
/// * Returns `KandError::NaNDetected` if any input is NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::aroon;
///
/// let (aroon_up, aroon_down, new_high, new_low, days_high, days_low) = aroon::aroon_inc(
///     15.0, // Current high
///     12.0, // Current low
///     14.0, // Previous high
///     11.0, // Previous low
///     2,    // Days since high
///     1,    // Days since low
///     14,   // Period
/// )
/// .unwrap();
/// ```
pub fn aroon_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    input_days_since_high: usize,
    input_days_since_low: usize,
    opt_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat, usize, usize), KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
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

    // Update days since high/low
    if days_since_high < opt_period {
        days_since_high += 1;
    }
    if days_since_low < opt_period {
        days_since_low += 1;
    }

    // Check if current values are new high/low
    if input_high >= prev_high {
        new_high = input_high;
        days_since_high = 0;
    }
    if input_low <= prev_low {
        new_low = input_low;
        days_since_low = 0;
    }

    let opt_period_t = opt_period as TAFloat;
    let hundred_t = 100.0;
    let days_since_high_t = days_since_high as TAFloat;
    let days_since_low_t = days_since_low as TAFloat;

    let aroon_up = hundred_t - (hundred_t * days_since_high_t / opt_period_t);
    let aroon_down = hundred_t - (hundred_t * days_since_low_t / opt_period_t);

    Ok((
        aroon_up,
        aroon_down,
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
    fn test_aroon_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1, 35179.2, 35244.9, 35150.2, 35136.0, 35133.6, 35188.0, 35215.3,
            35221.9, 35219.2, 35234.0, 35216.7, 35197.9, 35178.4, 35183.4, 35129.7, 35149.1,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0, 35073.1, 35090.0, 35072.0, 35078.0, 35088.0, 35124.8, 35169.4,
            35138.0, 35141.0, 35182.0, 35151.1, 35158.4, 35140.0, 35087.0, 35085.8, 35114.7,
        ];
        let opt_period = 14;
        let mut output_aroon_up = vec![0.0; input_high.len()];
        let mut output_aroon_down = vec![0.0; input_high.len()];
        let mut output_prev_high = vec![0.0; input_high.len()];
        let mut output_prev_low = vec![0.0; input_high.len()];
        let mut output_days_since_high = vec![0; input_high.len()];
        let mut output_days_since_low = vec![0; input_high.len()];

        aroon(
            &input_high,
            &input_low,
            opt_period,
            &mut output_aroon_up,
            &mut output_aroon_down,
            &mut output_prev_high,
            &mut output_prev_low,
            &mut output_days_since_high,
            &mut output_days_since_low,
        )
        .unwrap();

        // First 13 values should be NaN
        for i in 0..14 {
            assert!(output_aroon_up[i].is_nan());
            assert!(output_aroon_down[i].is_nan());
        }

        // Compare with known values
        let expected_up = [
            50.0,
            42.857_142_857_142_86,
            35.714_285_714_285_715,
            28.571_428_571_428_573,
            21.428_571_428_571_43,
            14.285_714_285_714_286,
            7.142_857_142_857_143,
            0.0,
            0.0,
            21.428_571_428_571_43,
            14.285_714_285_714_286,
            7.142_857_142_857_143,
            0.0,
            0.0,
            0.0,
            100.0,
            100.0,
            92.857_142_857_142_86,
            85.714_285_714_285_72,
            78.571_428_571_428_57,
            71.428_571_428_571_43,
            64.285_714_285_714_29,
            57.142_857_142_857_146,
            50.0,
            42.857_142_857_142_86,
            35.714_285_714_285_715,
            28.571_428_571_428_573,
            21.428_571_428_571_43,
            14.285_714_285_714_286,
            7.142_857_142_857_143,
            0.0,
        ];

        let expected_down = [
            100.0,
            100.0,
            100.0,
            92.857_142_857_142_86,
            85.714_285_714_285_72,
            78.571_428_571_428_57,
            100.0,
            100.0,
            92.857_142_857_142_86,
            85.714_285_714_285_72,
            78.571_428_571_428_57,
            71.428_571_428_571_43,
            64.285_714_285_714_29,
            57.142_857_142_857_146,
            50.0,
            42.857_142_857_142_86,
            35.714_285_714_285_715,
            28.571_428_571_428_573,
            21.428_571_428_571_43,
            14.285_714_285_714_286,
            7.142_857_142_857_143,
            0.0,
            0.0,
            0.0,
            0.0,
            7.142_857_142_857_143,
            0.0,
            7.142_857_142_857_143,
            0.0,
            14.285_714_285_714_286,
            7.142_857_142_857_143,
        ];

        for (i, (&exp_up, &exp_down)) in expected_up.iter().zip(expected_down.iter()).enumerate() {
            assert_relative_eq!(output_aroon_up[i + 14], exp_up, epsilon = 0.0001);
            assert_relative_eq!(output_aroon_down[i + 14], exp_down, epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_high = output_prev_high[14];
        let mut prev_low = output_prev_low[14];
        let mut days_since_high = output_days_since_high[14];
        let mut days_since_low = output_days_since_low[14];

        for i in 15..20 {
            let result = aroon_inc(
                input_high[i],
                input_low[i],
                prev_high,
                prev_low,
                days_since_high,
                days_since_low,
                opt_period,
            )
            .unwrap();

            assert_relative_eq!(result.0, output_aroon_up[i], epsilon = 0.0001);
            assert_relative_eq!(result.1, output_aroon_down[i], epsilon = 0.0001);

            prev_high = result.2;
            prev_low = result.3;
            days_since_high = result.4;
            days_since_low = result.5;
        }
    }
}
