use crate::{KandError, TAFloat};

/// Calculates the lookback period required for Minus Directional Movement (-DM) calculation.
///
/// # Description
/// Returns the minimum number of data points needed before the first valid -DM value can be calculated.
/// For the -DM indicator, this equals `period - 1`.
///
/// # Arguments
/// * `param_period` - The time period used for calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::minus_dm;
/// let period = 14;
/// let lookback = minus_dm::lookback(period).unwrap();
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

/// Calculates Minus Directional Movement (-DM) for a price series.
///
/// # Description
/// Minus Directional Movement (-DM) is a component of the Directional Movement System developed by J. Welles Wilder.
/// It measures the strength of downward price movement by comparing consecutive lows.
///
/// # Mathematical Formula
/// ```text
/// For each period:
/// 1. Calculate -DM1 (one-period directional movement):
///    If (Low[t-1] - Low[t]) > (High[t] - High[t-1]) AND (Low[t-1] - Low[t]) > 0:
///        -DM1 = Low[t-1] - Low[t]
///    Else:
///        -DM1 = 0
///
/// 2. Initial -DM:
///    First -DM = Sum(-DM1, period)
///
/// 3. Subsequent -DM using Wilder's smoothing:
///    -DM[t] = -DM[t-1] - (-DM[t-1]/period) + -DM1[t]
/// ```
///
/// # Calculation Steps
/// 1. Calculate initial -DM1 values for the first period
/// 2. Sum these values to get the first -DM
/// 3. Apply Wilder's smoothing formula for subsequent periods
/// 4. Fill initial values before lookback period with NaN
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `param_period` - Time period for calculation (must be >= 2)
/// * `output_dm` - Array to store calculated -DM values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok on success
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output array lengths differ
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input contains NaN (with `check-nan`)
///
/// # Examples
/// ```
/// use kand::ohlcv::minus_dm;
///
/// let high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let mut minus_dm = vec![0.0; 5];
///
/// minus_dm::minus_dm(&high, &low, 3, &mut minus_dm).unwrap();
/// ```
pub fn minus_dm(
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

    // Calculate first -DM values and initial -DM (sum of -DM1)
    let mut dm_sum = 0.0;

    for i in 1..param_period {
        let high_diff = input_high[i] - input_high[i - 1];
        let low_diff = input_low[i - 1] - input_low[i];

        let dm = if low_diff > high_diff && low_diff > 0.0 {
            low_diff
        } else {
            0.0
        };
        dm_sum += dm;
    }
    output_dm[lookback] = dm_sum;

    // Calculate remaining -DM values using Wilder's smoothing
    for i in param_period..len {
        let high_diff = input_high[i] - input_high[i - 1];
        let low_diff = input_low[i - 1] - input_low[i];

        let dm = if low_diff > high_diff && low_diff > 0.0 {
            low_diff
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

/// Calculates the next Minus DM value incrementally.
///
/// # Description
/// Provides an optimized way to calculate the next -DM value when new data arrives,
/// without recalculating the entire series. Uses Wilder's smoothing formula.
///
/// # Mathematical Formula
/// ```text
/// -DM[t] = -DM[t-1] - (-DM[t-1]/period) + -DM1[t]
///
/// where -DM1[t] is:
/// If (Low[t-1] - Low[t]) > (High[t] - High[t-1]) AND (Low[t-1] - Low[t]) > 0:
///     -DM1[t] = Low[t-1] - Low[t]
/// Else:
///     -DM1[t] = 0
/// ```
///
/// # Arguments
/// * `input_high` - Current high price
/// * `prev_high` - Previous high price
/// * `input_low` - Current low price
/// * `prev_low` - Previous low price
/// * `prev_minus_dm` - Previous -DM value
/// * `param_period` - Time period for calculation (must be between 2 and 100)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The next -DM value on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is not between 2 and 100
/// * `KandError::NaNDetected` - If any input contains NaN (with `check-nan`)
///
/// # Examples
/// ```
/// use kand::ohlcv::minus_dm;
///
/// let current_high = 15.0;
/// let prev_high = 14.0;
/// let current_low = 13.0;
/// let prev_low = 12.0;
/// let prev_minus_dm = 2.5;
/// let period = 14;
///
/// let next_minus_dm = minus_dm::minus_dm_inc(
///     current_high,
///     prev_high,
///     current_low,
///     prev_low,
///     prev_minus_dm,
///     period,
/// )
/// .unwrap();
/// ```
pub fn minus_dm_inc(
    input_high: TAFloat,
    prev_high: TAFloat,
    input_low: TAFloat,
    prev_low: TAFloat,
    prev_minus_dm: TAFloat,
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
            || prev_minus_dm.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high_diff = input_high - prev_high;
    let low_diff = prev_low - input_low;

    let dm = if low_diff > high_diff && low_diff > 0.0 {
        low_diff
    } else {
        0.0
    };

    Ok(prev_minus_dm - (prev_minus_dm / param_period as TAFloat) + dm)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_minus_dm_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0,
        ];
        let param_period = 14;
        let mut output_dm = vec![0.0; input_high.len()];

        minus_dm(&input_high, &input_low, param_period, &mut output_dm).unwrap();

        // First period-1 values should be NaN
        for value in output_dm.iter().take(param_period - 1) {
            assert!(value.is_nan());
        }

        // Test subsequent values
        let expected_values = [
            165.0,
            217.014_285_714_288_63,
            260.513_265_306_125_16,
            312.905_174_927_116_26,
            290.554_805_289_465_1,
            269.800_890_625_931_86,
            250.529_398_438_365_3,
            323.234_441_407_052_03,
            320.746_267_020_832_6,
            297.835_819_376_487_4,
            276.561_832_278_166_9,
            256.807_415_686_869_26,
            238.464_028_852_092_9,
            239.430_883_934_086_27,
            222.328_677_938_794_39,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_dm[i + 13], *expected, epsilon = 0.00001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_dm = output_dm[14]; // First valid -DM value

        // Test each incremental step
        for i in 15..19 {
            let result = minus_dm_inc(
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
