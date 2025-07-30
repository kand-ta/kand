use super::trange;
use crate::{KandError, TAFloat};

/// Returns the lookback period needed for +DI calculation
///
/// # Arguments
/// * `param_period` - The period parameter for +DI calculation. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The number of data points needed before first valid output
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::plus_di;
/// let period = 14;
/// let lookback = plus_di::lookback(period).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period)
}

/// Calculates Plus Directional Indicator (+DI) values for the entire input array
///
/// The Plus Directional Indicator (+DI) measures the strength of upward price movement.
/// It is a key component of the Directional Movement System developed by J. Welles Wilder.
///
/// # Calculation Principles
/// 1. Identifies upward price movement by comparing consecutive highs and lows
/// 2. Uses Wilder's smoothing method to reduce noise
/// 3. Normalizes the indicator to a percentage scale (0-100)
///
/// # Mathematical Formula
/// ```text
/// 1. Plus Directional Movement (+DM):
///    +DM = high[i] - high[i-1] if:
///          - high[i] - high[i-1] > low[i-1] - low[i] AND
///          - high[i] - high[i-1] > 0
///          Otherwise, +DM = 0
///
/// 2. True Range (TR):
///    TR = max(high - low, |high - prev_close|, |low - prev_close|)
///
/// 3. Initial Values:
///    First +DI = 100 * SMA(+DM, period) / SMA(TR, period)
///
/// 4. Subsequent Values (Wilder's Smoothing):
///    Smoothed +DM = ((prev_smoothed_+DM * (period-1)) + current_+DM) / period
///    Smoothed TR = ((prev_smoothed_TR * (period-1)) + current_TR) / period
///    +DI = 100 * Smoothed_+DM / Smoothed_TR
/// ```
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - Smoothing period (>= 2)
/// * `output_plus_di` - Output array for +DI values
/// * `output_smoothed_plus_dm` - Output array for smoothed +DM values
/// * `output_smoothed_tr` - Output array for smoothed TR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If period < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input contains NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::plus_di;
///
/// let high = vec![10.0, 12.0, 11.5, 11.0];
/// let low = vec![9.0, 10.0, 10.0, 9.5];
/// let close = vec![9.5, 11.0, 10.5, 10.0];
/// let period = 2;
/// let mut plus_di = vec![0.0; 4];
/// let mut smoothed_plus_dm = vec![0.0; 4];
/// let mut smoothed_tr = vec![0.0; 4];
///
/// plus_di::plus_di(
///     &high,
///     &low,
///     &close,
///     period,
///     &mut plus_di,
///     &mut smoothed_plus_dm,
///     &mut smoothed_tr,
/// )
/// .unwrap();
/// ```
pub fn plus_di(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    output_plus_di: &mut [TAFloat],
    output_smoothed_plus_dm: &mut [TAFloat],
    output_smoothed_tr: &mut [TAFloat],
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
            || len != input_close.len()
            || len != output_plus_di.len()
            || len != output_smoothed_plus_dm.len()
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

    // Calculate initial +DM and TR sums
    let mut plus_dm_sum = 0.0;
    let mut tr_sum = 0.0;
    let mut prev_high = input_high[0];
    let mut prev_low = input_low[0];
    let mut prev_close = input_close[0];

    // Calculate first period-1 +DM1 and TR1 values
    for i in 1..param_period {
        let high_diff = input_high[i] - prev_high;
        let low_diff = prev_low - input_low[i];

        let plus_dm1 = if high_diff > low_diff && high_diff > 0.0 {
            high_diff
        } else {
            0.0
        };

        plus_dm_sum += plus_dm1;

        let tr1 = trange::trange_inc(input_high[i], input_low[i], prev_close)?;
        tr_sum += tr1;

        prev_high = input_high[i];
        prev_low = input_low[i];
        prev_close = input_close[i];
    }

    // Calculate first +DI value
    let hundred = 100.0;
    let period_t = param_period as TAFloat;

    // Initialize smoothed values
    let mut curr_smoothed_plus_dm = plus_dm_sum;
    let mut curr_smoothed_tr = tr_sum;

    // Calculate remaining +DI values using Wilder's smoothing
    for i in param_period..len {
        let high_diff = input_high[i] - input_high[i - 1];
        let low_diff = input_low[i - 1] - input_low[i];

        let plus_dm1 = if high_diff > low_diff && high_diff > 0.0 {
            high_diff
        } else {
            0.0
        };

        let tr1 = trange::trange_inc(input_high[i], input_low[i], input_close[i - 1])?;

        // Apply Wilder's smoothing
        curr_smoothed_plus_dm =
            curr_smoothed_plus_dm - (curr_smoothed_plus_dm / period_t) + plus_dm1;
        curr_smoothed_tr = curr_smoothed_tr - (curr_smoothed_tr / period_t) + tr1;

        output_smoothed_plus_dm[i] = curr_smoothed_plus_dm;
        output_smoothed_tr[i] = curr_smoothed_tr;

        output_plus_di[i] = if curr_smoothed_tr == 0.0 {
            0.0
        } else {
            hundred * curr_smoothed_plus_dm / curr_smoothed_tr
        };
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_plus_di[i] = TAFloat::NAN;
        output_smoothed_plus_dm[i] = TAFloat::NAN;
        output_smoothed_tr[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest +DI value incrementally using previous smoothed values
///
/// This function enables real-time calculation of +DI by using the previous smoothed values
/// and current price data, avoiding the need to recalculate the entire series.
///
/// # Mathematical Formula
/// ```text
/// 1. Current +DM:
///    +DM = high - prev_high if:
///          - (high - prev_high) > (prev_low - low) AND
///          - (high - prev_high) > 0
///          Otherwise, +DM = 0
///
/// 2. Current TR:
///    TR = max(high - low, |high - prev_close|, |low - prev_close|)
///
/// 3. Wilder's Smoothing:
///    smoothed_+DM = ((prev_smoothed_+DM * (period-1)) + current_+DM) / period
///    smoothed_TR = ((prev_smoothed_TR * (period-1)) + current_TR) / period
///
/// 4. +DI:
///    +DI = 100 * smoothed_+DM / smoothed_TR
/// ```
///
/// # Arguments
/// * `input_high` - Current high price
/// * `input_low` - Current low price
/// * `prev_high` - Previous high price
/// * `prev_low` - Previous low price
/// * `prev_close` - Previous close price
/// * `prev_smoothed_plus_dm` - Previous smoothed +DM value
/// * `prev_smoothed_tr` - Previous smoothed TR value
/// * `param_period` - Smoothing period (>= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple of (latest +DI, new smoothed +DM, new smoothed TR)
///
/// # Errors
/// * `KandError::InvalidParameter` - If period < 2
/// * `KandError::NaNDetected` - If any input contains NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::plus_di;
///
/// let (plus_di, smoothed_plus_dm, smoothed_tr) = plus_di::plus_di_inc(
///     10.5, // high
///     9.5,  // low
///     10.0, // prev_high
///     9.0,  // prev_low
///     9.5,  // prev_close
///     15.0, // prev_smoothed_plus_dm
///     20.0, // prev_smoothed_tr
///     14,   // period
/// )
/// .unwrap();
/// ```
pub fn plus_di_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
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
            || prev_smoothed_plus_dm.is_nan()
            || prev_smoothed_tr.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high_diff = input_high - prev_high;
    let low_diff = prev_low - input_low;

    let plus_dm = if high_diff > low_diff && high_diff > 0.0 {
        high_diff
    } else {
        0.0
    };

    let tr = trange::trange_inc(input_high, input_low, prev_close)?;
    let period_t = param_period as TAFloat;

    let output_smoothed_plus_dm =
        prev_smoothed_plus_dm - (prev_smoothed_plus_dm / period_t) + plus_dm;
    let output_smoothed_tr = prev_smoothed_tr - (prev_smoothed_tr / period_t) + tr;

    let output_plus_di = if output_smoothed_tr == 0.0 {
        0.0
    } else {
        100.0 * output_smoothed_plus_dm / output_smoothed_tr
    };

    Ok((output_plus_di, output_smoothed_plus_dm, output_smoothed_tr))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_plus_di_calculation() {
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

        let param_period = 14;
        let mut output_plus_di = vec![0.0; input_high.len()];
        let mut output_smoothed_plus_dm = vec![0.0; input_high.len()];
        let mut output_smoothed_tr = vec![0.0; input_high.len()];

        plus_di(
            &input_high,
            &input_low,
            &input_close,
            param_period,
            &mut output_plus_di,
            &mut output_smoothed_plus_dm,
            &mut output_smoothed_tr,
        )
        .unwrap();

        // Check first valid value
        assert_relative_eq!(
            output_plus_di[14],
            17.333_631_354_074_154,
            epsilon = 0.00001
        );
        assert_relative_eq!(output_plus_di[15], 15.208_643_871_406_93, epsilon = 0.00001);
        assert_relative_eq!(
            output_plus_di[16],
            13.584_828_258_444_833,
            epsilon = 0.00001
        );
        assert_relative_eq!(
            output_plus_di[17],
            12.779_367_774_255_537,
            epsilon = 0.00001
        );
        assert_relative_eq!(output_plus_di[18], 18.396_699_555_422_47, epsilon = 0.00001);

        // Now test incremental calculation matches regular calculation
        let mut prev_smoothed_plus_dm = output_smoothed_plus_dm[14]; // First valid smoothed +DM value
        let mut prev_smoothed_tr = output_smoothed_tr[14]; // First valid smoothed TR value

        // Test each incremental step
        for i in 15..19 {
            let (plus_di, new_smoothed_plus_dm, new_smoothed_tr) = plus_di_inc(
                input_high[i],
                input_low[i],
                input_high[i - 1],
                input_low[i - 1],
                input_close[i - 1],
                prev_smoothed_plus_dm,
                prev_smoothed_tr,
                param_period,
            )
            .unwrap();
            assert_relative_eq!(plus_di, output_plus_di[i], epsilon = 0.00001);
            prev_smoothed_plus_dm = new_smoothed_plus_dm;
            prev_smoothed_tr = new_smoothed_tr;
        }
    }
}
