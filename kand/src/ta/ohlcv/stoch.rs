use crate::{KandError, TAFloat, ta::ohlcv::sma};

/// Calculates the lookback period required for Stochastic Oscillator calculation.
///
/// # Description
/// The lookback period represents the minimum number of data points needed before
/// the indicator can generate valid values. It is calculated based on the input parameters.
///
/// # Arguments
/// * `param_k_period` - The period used for %K calculation, must be >= 2
/// * `param_k_slow_period` - The smoothing period for slow %K calculation, must be >= 2
/// * `param_d_period` - The period used for %D calculation, must be >= 2
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If any input parameter is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::stoch;
///
/// let k_period = 14;
/// let k_slow_period = 3;
/// let d_period = 3;
///
/// let lookback = stoch::lookback(k_period, k_slow_period, d_period).unwrap();
/// assert_eq!(lookback, 17); // 14 + 3 + 3 - 3 = 17
/// ```
pub const fn lookback(
    param_k_period: usize,
    param_k_slow_period: usize,
    param_d_period: usize,
) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_k_period < 2 || param_k_slow_period < 2 || param_d_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_k_period + param_k_slow_period + param_d_period - 3)
}

/// Calculates the Stochastic Oscillator indicator for the entire price series.
///
/// # Description
/// The Stochastic Oscillator is a momentum indicator that shows the location of the close
/// relative to the high-low range over a set number of periods. The indicator consists of
/// two lines: %K (the fast line) and %D (the slow line).
///
/// # Mathematical Formula
/// ```text
/// Fast %K = 100 * (Close - Lowest Low) / (Highest High - Lowest Low)
/// Slow %K = SMA(Fast %K, k_slow_period)
/// %D = SMA(Slow %K, d_period)
/// ```
///
/// # Calculation Steps
/// 1. Calculate the Fast %K by comparing current close to the high-low range
/// 2. Smooth the Fast %K using SMA to get Slow %K
/// 3. Calculate %D as the SMA of Slow %K
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_k_period` - Period for %K calculation, must be >= 2
/// * `param_k_slow_period` - Smoothing period for slow %K, must be >= 2
/// * `param_d_period` - Period for %D calculation, must be >= 2
/// * `output_fast_k` - Array to store Fast %K values
/// * `output_k` - Array to store Slow %K values
/// * `output_d` - Array to store %D values
///
/// # Returns
/// * `Result<(), KandError>` - Unit type if successful
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If any period parameter is less than 2
/// * `KandError::InsufficientData` - If input length is less than required lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::stoch;
///
/// let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let input_close = vec![9.0, 11.0, 14.0, 12.0, 11.0];
/// let param_k_period = 3;
/// let param_k_slow_period = 2;
/// let param_d_period = 2;
/// let mut output_fast_k = vec![0.0; 5];
/// let mut output_k = vec![0.0; 5];
/// let mut output_d = vec![0.0; 5];
///
/// stoch::stoch(
///     &input_high,
///     &input_low,
///     &input_close,
///     param_k_period,
///     param_k_slow_period,
///     param_d_period,
///     &mut output_fast_k,
///     &mut output_k,
///     &mut output_d,
/// )
/// .unwrap();
/// ```
#[allow(clippy::similar_names)]
pub fn stoch(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_k_period: usize,
    param_k_slow_period: usize,
    param_d_period: usize,
    output_fast_k: &mut [TAFloat],
    output_k: &mut [TAFloat],
    output_d: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(param_k_period, param_k_slow_period, param_d_period)?;

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
            || len != output_fast_k.len()
            || len != output_k.len()
            || len != output_d.len()
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

    let hundred = 100.0;

    // Calculate Fast %K first
    for i in (param_k_period - 1)..len {
        let mut highest_high = input_high[i];
        let mut lowest_low = input_low[i];

        for j in 0..param_k_period {
            let idx = i - j;
            highest_high = highest_high.max(input_high[idx]);
            lowest_low = lowest_low.min(input_low[idx]);
        }

        let range = highest_high - lowest_low;
        if range > 0.0 {
            output_fast_k[i] = hundred * (input_close[i] - lowest_low) / range;
        } else {
            output_fast_k[i] = 50.0; // Default to 50 when range is zero
        }
    }

    // Calculate Slow %K (SMA of Fast %K)
    sma::sma(output_fast_k, param_k_slow_period, output_k)?;

    // Calculate %D (SMA of Slow %K)
    sma::sma(
        &output_k[param_k_slow_period - 1..],
        param_d_period,
        &mut output_d[param_k_slow_period - 1..],
    )?;

    // Fill initial values with NAN
    for i in 0..lookback {
        output_fast_k[i] = TAFloat::NAN;
        output_k[i] = TAFloat::NAN;
        output_d[i] = TAFloat::NAN;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    #[allow(clippy::similar_names)]
    fn test_stoch_calculation() {
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

        let param_k_period = 14;
        let param_k_slow_period = 3;
        let param_d_period = 3;
        let mut output_fast_k = vec![0.0; input_high.len()];
        let mut output_k = vec![0.0; input_high.len()];
        let mut output_d = vec![0.0; input_high.len()];

        stoch(
            &input_high,
            &input_low,
            &input_close,
            param_k_period,
            param_k_slow_period,
            param_d_period,
            &mut output_fast_k,
            &mut output_k,
            &mut output_d,
        )
        .unwrap();

        // First 17 values should be NaN (lookback = 14 + 3 + 3 - 3 = 17)
        for i in 0..17 {
            assert!(output_k[i].is_nan());
            assert!(output_d[i].is_nan());
        }

        // Compare with known values
        let expected_k = [
            13.888_595_327_554_674,
            23.274_994_970_831_596,
            25.819_754_576_544_284,
            20.205_422_372_883_813,
            12.265_381_731_365_673,
            13.761_818_641_200_326,
            26.221_343_873_517_94,
            39.272_727_272_727_57,
        ];

        let expected_d = [
            11.330_297_439_346_538,
            15.457_813_387_810_21,
            20.994_448_291_643_52,
            23.100_057_306_753_232,
            19.430_186_226_931_26,
            15.410_874_248_483_273,
            17.416_181_415_361_315,
            26.418_629_929_148_62,
        ];

        for (i, (&exp_k, &exp_d)) in expected_k.iter().zip(expected_d.iter()).enumerate() {
            assert_relative_eq!(output_k[i + 17], exp_k, epsilon = 0.0001);
            assert_relative_eq!(output_d[i + 17], exp_d, epsilon = 0.0001);
        }
    }
}
