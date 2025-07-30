use super::trange;
use crate::{KandError, TAFloat};

/// Calculates the lookback period required for -DI (Minus Directional Indicator) calculation.
///
/// # Arguments
/// * `opt_period` - The time period used for calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - Returns `opt_period` on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::minus_di;
/// let lookback = minus_di::lookback(14).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period)
}

/// Calculates the Minus Directional Indicator (-DI) for the entire input array.
///
/// The -DI is a component of the Directional Movement System that measures the strength of downward price movement.
/// It is commonly used with +DI and ADX to analyze trend direction and strength.
///
/// # Mathematical Formula
/// ```text
/// 1. Minus Directional Movement (-DM):
///    -DM = prev_low - curr_low, if:
///          - (prev_low - curr_low) > (curr_high - prev_high) AND
///          - (prev_low - curr_low) > 0
///    Otherwise, -DM = 0
///
/// 2. True Range (TR):
///    TR = max(high - low, |high - prev_close|, |low - prev_close|)
///
/// 3. Initial Values:
///    First -DI = 100 * SMA(-DM, period) / SMA(TR, period)
///
/// 4. Subsequent Values (Wilder's Smoothing):
///    Smoothed -DM = ((prev_smoothed_-DM * (period-1)) + curr_-DM) / period
///    Smoothed TR = ((prev_smoothed_TR * (period-1)) + curr_TR) / period
///    -DI = 100 * Smoothed_-DM / Smoothed_TR
/// ```
///
/// # Calculation Principles
/// 1. Measures downward price movement strength (0 to 100)
/// 2. Uses Wilder's smoothing for more weight on recent data
/// 3. Higher values indicate stronger downward trends
/// 4. Part of the complete Directional Movement System
/// 5. First valid value appears after `opt_period` bars
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `opt_period` - Calculation period (>= 2)
/// * `output_minus_di` - Output array for -DI values
/// * `output_smoothed_minus_dm` - Output array for smoothed -DM values
/// * `output_smoothed_tr` - Output array for smoothed TR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok on success, Err otherwise
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 2
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InsufficientData` - If input length <= `opt_period`
/// * `KandError::NaNDetected` - If any input contains NaN
///
/// # Example
/// ```
/// use kand::ohlcv::minus_di;
///
/// let high = vec![35.0, 36.0, 35.5, 35.8, 36.2];
/// let low = vec![34.0, 35.0, 34.5, 34.8, 35.2];
/// let close = vec![34.5, 35.5, 35.0, 35.3, 35.7];
/// let period = 3;
///
/// let mut minus_di = vec![0.0; high.len()];
/// let mut smoothed_minus_dm = vec![0.0; high.len()];
/// let mut smoothed_tr = vec![0.0; high.len()];
///
/// minus_di::minus_di(
///     &high,
///     &low,
///     &close,
///     period,
///     &mut minus_di,
///     &mut smoothed_minus_dm,
///     &mut smoothed_tr,
/// )
/// .unwrap();
/// ```
pub fn minus_di(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    opt_period: usize,
    output_minus_di: &mut [TAFloat],
    output_smoothed_minus_dm: &mut [TAFloat],
    output_smoothed_tr: &mut [TAFloat],
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
            || len != input_close.len()
            || len != output_minus_di.len()
            || len != output_smoothed_minus_dm.len()
            || len != output_smoothed_tr.len()
        {
            return Err(KandError::LengthMismatch);
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

    // Calculate initial -DM and TR sums
    let mut minus_dm_sum = 0.0;
    let mut tr_sum = 0.0;
    let mut prev_high = input_high[0];
    let mut prev_low = input_low[0];
    let mut prev_close = input_close[0];

    // Calculate first period-1 -DM1 and TR1 values
    for i in 1..opt_period {
        let high_diff = input_high[i] - prev_high;
        let low_diff = prev_low - input_low[i];

        let minus_dm1 = if low_diff > high_diff && low_diff > 0.0 {
            low_diff
        } else {
            0.0
        };

        minus_dm_sum += minus_dm1;

        let tr1 = trange::trange_inc(input_high[i], input_low[i], prev_close)?;
        tr_sum += tr1;

        prev_high = input_high[i];
        prev_low = input_low[i];
        prev_close = input_close[i];
    }

    // Calculate first -DI value
    let hundred = 100.0;
    let period_t = opt_period as TAFloat;

    // Initialize smoothed values
    let mut curr_smoothed_minus_dm = minus_dm_sum;
    let mut curr_smoothed_tr = tr_sum;

    // Calculate remaining -DI values using Wilder's smoothing
    for i in lookback..len {
        let high_diff = input_high[i] - input_high[i - 1];
        let low_diff = input_low[i - 1] - input_low[i];

        let minus_dm1 = if low_diff > high_diff && low_diff > 0.0 {
            low_diff
        } else {
            0.0
        };

        let tr1 = trange::trange_inc(input_high[i], input_low[i], input_close[i - 1])?;

        // Apply Wilder's smoothing
        curr_smoothed_minus_dm =
            curr_smoothed_minus_dm - (curr_smoothed_minus_dm / period_t) + minus_dm1;
        curr_smoothed_tr = curr_smoothed_tr - (curr_smoothed_tr / period_t) + tr1;

        output_smoothed_minus_dm[i] = curr_smoothed_minus_dm;
        output_smoothed_tr[i] = curr_smoothed_tr;

        output_minus_di[i] = if curr_smoothed_tr == 0.0 {
            0.0
        } else {
            hundred * curr_smoothed_minus_dm / curr_smoothed_tr
        };
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_minus_di[i] = TAFloat::NAN;
        output_smoothed_minus_dm[i] = TAFloat::NAN;
        output_smoothed_tr[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest -DI value incrementally using previous smoothed values.
///
/// This function provides an efficient way to update -DI with new price data without recalculating the entire series.
/// It maintains the same mathematical properties as the full calculation.
///
/// # Mathematical Formula
/// ```text
/// 1. Current -DM:
///    -DM = prev_low - curr_low, if:
///          - (prev_low - curr_low) > (curr_high - prev_high) AND
///          - (prev_low - curr_low) > 0
///    Otherwise, -DM = 0
///
/// 2. Current TR:
///    TR = max(high - low, |high - prev_close|, |low - prev_close|)
///
/// 3. Wilder's Smoothing:
///    smoothed_-DM = ((prev_smoothed_-DM * (period-1)) + curr_-DM) / period
///    smoothed_TR = ((prev_smoothed_TR * (period-1)) + curr_TR) / period
///
/// 4. -DI Value:
///    -DI = 100 * smoothed_-DM / smoothed_TR
/// ```
///
/// # Arguments
/// * `input_high` - Current high price
/// * `input_low` - Current low price
/// * `prev_high` - Previous high price
/// * `prev_low` - Previous low price
/// * `prev_close` - Previous close price
/// * `prev_smoothed_minus_dm` - Previous smoothed -DM
/// * `prev_smoothed_tr` - Previous smoothed TR
/// * `opt_period` - Calculation period (>= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple of (new -DI, new smoothed -DM, new smoothed TR)
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 2
/// * `KandError::NaNDetected` - If any input contains NaN
///
/// # Example
/// ```
/// use kand::ohlcv::minus_di;
///
/// let (minus_di, smoothed_minus_dm, smoothed_tr) = minus_di::minus_di_inc(
///     36.2, // high
///     35.2, // low
///     35.8, // prev_high
///     34.8, // prev_low
///     35.3, // prev_close
///     0.5,  // prev_smoothed_minus_dm
///     1.5,  // prev_smoothed_tr
///     14,   // period
/// )
/// .unwrap();
/// ```
pub fn minus_di_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    opt_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        // -DI requires at least 2 periods:
        // - One for initial DM and TR calculations (needs previous prices)
        // - One for the current period
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_high.is_nan()
            || input_low.is_nan()
            || prev_high.is_nan()
            || prev_low.is_nan()
            || prev_close.is_nan()
            || prev_smoothed_minus_dm.is_nan()
            || prev_smoothed_tr.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high_diff = input_high - prev_high;
    let low_diff = prev_low - input_low;

    let minus_dm = if low_diff > high_diff && low_diff > 0.0 {
        low_diff
    } else {
        0.0
    };

    let tr = trange::trange_inc(input_high, input_low, prev_close)?;
    let period_t = opt_period as TAFloat;

    let output_smoothed_minus_dm =
        prev_smoothed_minus_dm - (prev_smoothed_minus_dm / period_t) + minus_dm;
    let output_smoothed_tr = prev_smoothed_tr - (prev_smoothed_tr / period_t) + tr;

    let output_minus_di = if output_smoothed_tr == 0.0 {
        0.0
    } else {
        100.0 * output_smoothed_minus_dm / output_smoothed_tr
    };

    Ok((
        output_minus_di,
        output_smoothed_minus_dm,
        output_smoothed_tr,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_minus_di_calculation() {
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
        let mut output_minus_di = vec![0.0; input_high.len()];
        let mut output_smoothed_minus_dm = vec![0.0; input_high.len()];
        let mut output_smoothed_tr = vec![0.0; input_high.len()];

        minus_di(
            &input_high,
            &input_low,
            &input_close,
            opt_period,
            &mut output_minus_di,
            &mut output_smoothed_minus_dm,
            &mut output_smoothed_tr,
        )
        .unwrap();

        // First opt_period values should be NaN
        for value in output_minus_di.iter().take(opt_period) {
            assert!(value.is_nan());
        }

        // Check first valid value
        assert_relative_eq!(
            output_minus_di[14],
            26.118_652_373_133_61,
            epsilon = 0.00001
        );
        assert_relative_eq!(
            output_minus_di[15],
            29.626_333_125_808_358,
            epsilon = 0.00001
        );
        assert_relative_eq!(
            output_minus_di[16],
            34.230_177_437_536_13,
            epsilon = 0.00001
        );
        assert_relative_eq!(
            output_minus_di[17],
            32.200_629_859_296_83,
            epsilon = 0.00001
        );
        assert_relative_eq!(
            output_minus_di[18],
            29.832_869_923_860_61,
            epsilon = 0.00001
        );

        // Now test incremental calculation matches regular calculation
        let mut prev_smoothed_minus_dm = output_smoothed_minus_dm[14];
        let mut prev_smoothed_tr = output_smoothed_tr[14];

        // Test each incremental step
        for i in 15..input_high.len() {
            let (minus_di, new_smoothed_minus_dm, new_smoothed_tr) = minus_di_inc(
                input_high[i],
                input_low[i],
                input_high[i - 1],
                input_low[i - 1],
                input_close[i - 1],
                prev_smoothed_minus_dm,
                prev_smoothed_tr,
                opt_period,
            )
            .unwrap();

            assert_relative_eq!(minus_di, output_minus_di[i], epsilon = 0.00001);
            prev_smoothed_minus_dm = new_smoothed_minus_dm;
            prev_smoothed_tr = new_smoothed_tr;
        }
    }
}
