use crate::{KandError, TAFloat};

/// Returns the lookback period required for Triangular Moving Average (TRIMA) calculation.
///
/// # Description
/// The lookback period represents the minimum number of data points needed before the first valid TRIMA value
/// can be calculated. For TRIMA, this equals one less than the specified period.
///
/// # Arguments
/// * `opt_period` - The smoothing period used for TRIMA calculation. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The required lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - Returned if period < 2
///
/// # Example
/// ```
/// use kand::ohlcv::trima;
/// let period = 14;
/// let lookback = trima::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // period - 1
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

/// Calculates Triangular Moving Average (TRIMA) for a price series.
///
/// # Description
/// TRIMA is a double-smoothed moving average that places more weight on the middle portion of the price series
/// and less weight on the first and last portions. This results in a smoother moving average compared to a
/// Simple Moving Average (SMA).
///
/// # Mathematical Formula
/// ```text
/// For odd period:
///   n = (period + 1) / 2
///   TRIMA = SMA(SMA(price, n), n)
///
/// For even period:
///   n = (period / 2) + 1
///   m = period / 2
///   TRIMA = SMA(SMA(price, n), m)
/// ```
///
/// # Calculation Steps
/// 1. Determine window sizes n and m based on period
/// 2. Calculate first SMA using window size n
/// 3. Calculate second SMA (TRIMA) using window size m
/// 4. Fill initial values with NaN
///
/// # Arguments
/// * `input` - Slice of input price values
/// * `opt_period` - Smoothing period for calculations (must be >= 2)
/// * `output_sma1` - Mutable slice to store intermediate SMA values
/// * `output_sma2` - Mutable slice to store final TRIMA values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok value on success
///
/// # Errors
/// * `KandError::InvalidData` - Input slice is empty
/// * `KandError::LengthMismatch` - Output arrays don't match input length
/// * `KandError::InvalidParameter` - Period is less than 2
/// * `KandError::InsufficientData` - Input length is less than required lookback period
/// * `KandError::NaNDetected` - Input contains NaN values (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::trima;
///
/// let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let period = 3;
/// let mut output_sma1 = vec![0.0; input.len()];
/// let mut output_sma2 = vec![0.0; input.len()];
///
/// trima::trima(&input, period, &mut output_sma1, &mut output_sma2).unwrap();
/// ```
pub fn trima(
    input: &[TAFloat],
    opt_period: usize,
    output_sma1: &mut [TAFloat],
    output_sma2: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input.len();
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
        if len != output_sma1.len() || len != output_sma2.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for value in input.iter().take(len) {
            if value.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let (n, m) = if opt_period % 2 == 1 {
        let n = opt_period.div_ceil(2);
        (n, n)
    } else {
        let n = (opt_period / 2) + 1;
        let m = opt_period / 2;
        (n, m)
    };

    // First SMA calculation
    let mut sum = 0.0;
    for value in input.iter().take(n) {
        sum += *value;
    }
    output_sma1[n - 1] = sum / (n as TAFloat);

    for i in n..len {
        sum = sum + input[i] - input[i - n];
        output_sma1[i] = sum / (n as TAFloat);
    }

    // Second SMA calculation
    sum = 0.0;
    for value in output_sma1.iter().take(m) {
        sum += *value;
    }
    output_sma2[m - 1] = sum / (m as TAFloat);

    for i in m..len {
        sum = sum + output_sma1[i] - output_sma1[i - m];
        output_sma2[i] = sum / (m as TAFloat);
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_sma1[i] = TAFloat::NAN;
        output_sma2[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the next TRIMA value incrementally using previous SMA values.
///
/// # Description
/// This function allows for real-time TRIMA calculation without needing the entire price history.
/// It maintains the double-smoothing characteristic of TRIMA while only performing the minimal
/// required calculations.
///
/// # Mathematical Formula
/// ```text
/// First SMA update:
///   new_sma1 = prev_sma1 + (new_price - old_price) / n
///
/// Second SMA update:
///   new_sma2 = prev_sma2 + (new_sma1 - old_sma1) / m
///
/// Where:
/// - For odd period:  n = m = (period + 1)/2
/// - For even period: n = (period/2) + 1, m = period/2
/// ```
///
/// # Arguments
/// * `prev_sma1` - Previous first SMA value
/// * `prev_sma2` - Previous TRIMA value
/// * `input_new_price` - Latest price to include in calculation
/// * `input_old_price` - Price dropping out of first window
/// * `input_old_sma1` - SMA1 value dropping out of second window
/// * `opt_period` - The smoothing period for calculations (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat), KandError>` - Tuple of (`new_sma1`, `new_trima`) on success
///
/// # Errors
/// * `KandError::InvalidParameter` - Period is less than 2
/// * `KandError::NaNDetected` - Input contains NaN values (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::trima;
///
/// // Example with period = 5 (odd period)
/// let prev_sma1 = 35.5;
/// let prev_sma2 = 35.2;
/// let new_price = 36.0;
/// let old_price = 35.0;
/// let old_sma1 = 35.1;
/// let period = 5;
///
/// let (new_sma1, new_trima) =
///     trima::trima_inc(prev_sma1, prev_sma2, new_price, old_price, old_sma1, period).unwrap();
/// ```
pub fn trima_inc(
    prev_sma1: TAFloat,
    prev_sma2: TAFloat,
    input_new_price: TAFloat,
    input_old_price: TAFloat,
    input_old_sma1: TAFloat,
    opt_period: usize,
) -> Result<(TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if prev_sma1.is_nan()
            || prev_sma2.is_nan()
            || input_new_price.is_nan()
            || input_old_price.is_nan()
            || input_old_sma1.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let (n, m) = if opt_period % 2 == 1 {
        let n = opt_period.div_ceil(2);
        (n, n)
    } else {
        let n = (opt_period / 2) + 1;
        let m = opt_period / 2;
        (n, m)
    };

    let n_t = n as TAFloat;
    let m_t = m as TAFloat;

    // Incremental update for the first SMA using the correct window length (n)
    let new_sma1 = prev_sma1 + (input_new_price - input_old_price) / n_t;

    // Incremental update for the second SMA using the correct window length (m)
    let new_sma2 = prev_sma2 + (new_sma1 - input_old_sma1) / m_t;

    Ok((new_sma1, new_sma2))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_trima_calculation() {
        let input = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
            35154.0, 35216.3, 35211.8, 35158.4, 35172.0, 35176.7, 35113.3, 35114.7, 35129.3,
            35094.6, 35114.4, 35094.5, 35116.0, 35105.4, 35050.7, 35031.3, 35008.1, 35021.4,
            35048.4, 35080.1, 35043.6, 34962.7, 34970.1, 34980.1, 34930.6, 35000.0, 34998.0,
            35024.7, 34982.1, 34972.3, 34971.6, 34953.0, 34937.0, 34964.3, 34975.1, 34995.1,
            34989.0, 34942.9, 34895.2, 34830.4, 34925.1, 34888.6, 34910.3, 34917.6, 34940.0,
            35005.4, 34980.1, 34966.8, 34976.1, 34948.6, 34969.3, 34996.5, 35004.0, 35011.0,
            35059.2, 35036.1, 35062.3, 35067.7, 35087.9, 35076.7, 35041.6, 34993.3, 34974.5,
            34990.2,
        ];

        let expected_values = vec![
            35_106.679_166_666_67,
            35_097.465_000_000_004,
            35_089.510_416_666_664,
            35_082.868_333_333_33,
            35077.1975,
            35072.55375,
            35_069.712_916_666_67,
            35_069.024_166_666_67,
            35_070.279_166_666_674,
            35_073.292_083_333_34,
            35_077.280_833_333_345,
            35_081.945_416_666_68,
            35_087.193_750_000_006,
            35_093.083_750_000_01,
            35_099.648_750_000_015,
            35_106.536_666_666_68,
            35_113.231_250_000_026,
            35_119.662_916_666_69,
            35_125.514_583_333_36,
            35_130.942_500_000_02,
            35_135.868_333_333_36,
            35_139.502_083_333_355,
            35_141.475_416_666_69,
            35_141.742_083_333_35,
            35_140.314_166_666_69,
            35_137.719_583_333_36,
            35_134.415_416_666_69,
            35_130.317_083_333_364,
            35_125.260_000_000_024,
            35_119.511_666_666_695,
            35_112.968_750_000_02,
            35_105.784_166_666_686,
            35_098.112_083_333_35,
            35_090.089_166_666_69,
            35_081.735_000_000_015,
            35_072.903_750_000_02,
            35_064.015_416_666_68,
            35_055.564_166_666_685,
            35_047.394_583_333_34,
            35_039.740_833_333_344,
            35_032.530_000_000_01,
            35_025.340_000_000_01,
            35_018.330_833_333_35,
            35_011.985_833_333_35,
            35_006.155_000_000_01,
            35_000.572_916_666_67,
            34_995.195_000_000_01,
            34_990.188_333_333_346,
            34_985.202_500_000_01,
            34_980.142_083_333_34,
            34_975.193_333_333_34,
            34_970.623_750_000_01,
            34_966.521_666_666_68,
            34_962.781_250_000_01,
            34_959.394_583_333_34,
            34_956.408_750_000_02,
            34_953.662_916_666_68,
            34_951.247_083_333_35,
            34_949.064_583_333_35,
            34_947.027_083_333_35,
            34_945.585_416_666_676,
            34_945.450_833_333_34,
            34_946.196_250_000_01,
            34_947.977_500_000_01,
            34_950.870_416_666_67,
            34_954.949_583_333_335,
            34_959.867_083_333_33,
            34_965.069_999_999_99,
            34_970.187_083_333_32,
            34_975.223_333_333_32,
            34_980.194_166_666_65,
        ];

        let opt_period = 30;
        let mut output_sma1 = vec![0.0; input.len()];
        let mut output_sma2 = vec![0.0; input.len()];

        trima(&input, opt_period, &mut output_sma1, &mut output_sma2).unwrap();

        // First 29 values should be NaN
        for i in 0..29 {
            assert!(output_sma1[i].is_nan());
            assert!(output_sma2[i].is_nan());
        }

        // Compare with known values
        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_sma2[i + 29], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation
        let (n, m) = if opt_period % 2 == 1 {
            (opt_period.div_ceil(2), opt_period.div_ceil(2))
        } else {
            (((opt_period / 2) + 1), (opt_period / 2))
        };

        let mut prev_sma1 = output_sma1[43];
        let mut prev_sma2 = output_sma2[43];

        // Test incremental calculation for all remaining values
        for i in 44..input.len() {
            let old_price = input[i - n];
            let old_sma1 = output_sma1[i - m];
            let (new_sma1, new_sma2) = trima_inc(
                prev_sma1, prev_sma2, input[i], old_price, old_sma1, opt_period,
            )
            .unwrap();

            assert_relative_eq!(new_sma2, output_sma2[i], epsilon = 0.0001);
            prev_sma1 = new_sma1;
            prev_sma2 = new_sma2;
        }
    }
}
