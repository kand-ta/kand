use crate::{KandError, TAFloat};

/// Returns the lookback period required for Plus DM calculation
///
/// # Description
/// Calculates the number of data points needed before the first valid output value can be generated.
///
/// # Arguments
/// * `param_period` - The period parameter for Plus DM calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period value if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
///
/// # Example
/// ```
/// use kand::ohlcv::plus_dm;
/// let period = 14;
/// let lookback = plus_dm::lookback(period).unwrap();
/// assert_eq!(lookback, 13);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates Plus Directional Movement (+DM) for the entire input array
///
/// # Description
/// Plus Directional Movement (+DM) is a component of the Directional Movement System developed by J. Welles Wilder.
/// It measures the strength of upward price movement by comparing consecutive highs and lows.
///
/// # Mathematical Formula
/// ```text
/// 1. Calculate +DM1 (one period directional movement):
///    If (High[i] - High[i-1]) > (Low[i-1] - Low[i]) AND (High[i] - High[i-1]) > 0:
///        +DM1 = High[i] - High[i-1]
///    Else:
///        +DM1 = 0
///
/// 2. Initial +DM:
///    First +DM = Sum(+DM1, period)
///
/// 3. Subsequent +DM using Wilder's smoothing:
///    +DM[i] = +DM[i-1] - (+DM[i-1]/period) + +DM1
/// ```
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_period` - The smoothing period (must be >= 2)
/// * `output_dm` - Array to store calculated +DM values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok value if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::InsufficientData` - If input length < lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::plus_dm;
///
/// let high = vec![10.0, 11.0, 12.0, 11.5, 10.9];
/// let low = vec![9.0, 9.5, 10.0, 9.8, 9.2];
/// let period = 3;
/// let mut output = vec![0.0; 5];
///
/// plus_dm::plus_dm(&high, &low, period, &mut output).unwrap();
/// ```
pub fn plus_dm(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    param_period: usize,
    output_dm: &mut [TAFloat],
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
        if len != input_low.len() || len != output_dm.len() {
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

    // Calculate first +DM values and initial +DM (sum of +DM1)
    let mut dm_sum = 0.0;

    for i in 1..param_period {
        let high_diff = input_high[i] - input_high[i - 1];
        let low_diff = input_low[i - 1] - input_low[i];

        let dm = if high_diff > low_diff && high_diff > 0.0 {
            high_diff
        } else {
            0.0
        };
        dm_sum += dm;
    }
    output_dm[lookback] = dm_sum;

    // Calculate remaining +DM values using Wilder's smoothing
    for i in param_period..len {
        let high_diff = input_high[i] - input_high[i - 1];
        let low_diff = input_low[i - 1] - input_low[i];

        let dm = if high_diff > low_diff && high_diff > 0.0 {
            high_diff
        } else {
            0.0
        };

        output_dm[i] = output_dm[i - 1] - (output_dm[i - 1] / param_period as TAFloat) + dm;
    }

    // Fill initial values with NAN
    for value in output_dm.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest Plus DM value incrementally
///
/// # Description
/// This function provides an optimized way to calculate the latest Plus DM value using the previous
/// Plus DM value and current price data. It implements Wilder's smoothing formula for real-time calculation.
///
/// # Mathematical Formula
/// ```text
/// +DM[today] = +DM[yesterday] - (+DM[yesterday]/period) + +DM1
///
/// where +DM1 is:
/// If (High[today] - High[yesterday]) > (Low[yesterday] - Low[today]) AND (High[today] - High[yesterday]) > 0:
///     +DM1 = High[today] - High[yesterday]
/// Else:
///     +DM1 = 0
/// ```
///
/// # Arguments
/// * `input_high` - Current high price
/// * `prev_high` - Previous high price
/// * `input_low` - Current low price
/// * `prev_low` - Previous low price
/// * `prev_plus_dm` - Previous Plus DM value
/// * `param_period` - The smoothing period (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The latest Plus DM value if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::plus_dm;
///
/// let high = 10.5;
/// let prev_high = 10.0;
/// let low = 9.8;
/// let prev_low = 9.5;
/// let prev_plus_dm = 0.45;
/// let period = 14;
///
/// let new_plus_dm =
///     plus_dm::plus_dm_inc(high, prev_high, low, prev_low, prev_plus_dm, period).unwrap();
/// ```
pub fn plus_dm_inc(
    input_high: TAFloat,
    prev_high: TAFloat,
    input_low: TAFloat,
    prev_low: TAFloat,
    prev_plus_dm: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
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
            || prev_high.is_nan()
            || input_low.is_nan()
            || prev_low.is_nan()
            || prev_plus_dm.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high_diff = input_high - prev_high;
    let low_diff = prev_low - input_low;

    let dm = if high_diff > low_diff && high_diff > 0.0 {
        high_diff
    } else {
        0.0
    };

    Ok(prev_plus_dm - (prev_plus_dm / param_period as TAFloat) + dm)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_plus_dm_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1, 35179.2,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0, 35073.1,
        ];
        let param_period = 14;
        let mut output_dm = vec![0.0; input_high.len()];

        plus_dm(&input_high, &input_low, param_period, &mut output_dm).unwrap();

        // First period-1 values should be NaN
        for value in output_dm.iter().take(param_period - 1) {
            assert!(value.is_nan());
        }

        // Test subsequent values
        let expected = [
            155.099_999_999_998_54,
            144.021_428_571_427_2,
            133.734_183_673_468_12,
            124.181_741_982_506_11,
            115.311_617_555_184_24,
            166.375_073_444_102_55,
            160.691_139_626_663_76,
            149.213_201_081_902_07,
            138.555_115_290_337_62,
            155.058_321_341_029_26,
            180.282_726_959_522_8,
            211.705_389_319_559_8,
            255.583_575_796_734_1,
            237.327_606_096_967_37,
            220.375_634_232_898_28,
            204.634_517_501_976_97,
            277.117_766_251_834_27,
        ];

        for (i, expected_value) in expected.iter().enumerate() {
            assert_relative_eq!(output_dm[i + 13], *expected_value, epsilon = 0.00001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_dm = output_dm[14]; // First valid +DM value

        // Test each incremental step
        for i in 15..19 {
            let result = plus_dm_inc(
                input_high[i],
                input_high[i - 1],
                input_low[i],
                input_low[i - 1],
                prev_dm,
                param_period,
            )
            .unwrap();
            assert_relative_eq!(result, output_dm[i], epsilon = 0.0001);
            prev_dm = result;
        }
    }
}
