use super::dx;
use crate::{KandError, TAFloat};

/// Calculate the lookback period required for ADX calculation
///
/// Returns the number of data points needed before the first valid ADX value can be calculated.
///
/// # Arguments
/// * `param_period` - The period parameter used for ADX calculation (typically 14)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful, or error if invalid parameters
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::adx::lookback;
///
/// let period = 14;
/// let lookback_period = lookback(period).unwrap();
/// assert_eq!(lookback_period, 27); // 2 * period - 1
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period * 2 - 1)
}

/// Calculate Average Directional Index (ADX) for the entire input array
///
/// ADX measures the strength of a trend, regardless of its direction. Values range from 0 to 100,
/// with higher values indicating stronger trends.
///
/// # Calculation Steps
/// 1. Calculate +DM, -DM and TR for each period
/// 2. Apply Wilder's smoothing to +DM, -DM and TR
/// 3. Calculate +DI and -DI
/// 4. Calculate DX using +DI and -DI
/// 5. Apply Wilder's smoothing to DX to get ADX
///
/// # Mathematical Formula
/// ```text
/// TR = max(high - low, |high - prev_close|, |low - prev_close|)
/// +DM = if(high - prev_high > prev_low - low) then max(high - prev_high, 0) else 0
/// -DM = if(prev_low - low > high - prev_high) then max(prev_low - low, 0) else 0
///
/// Smoothed TR = ((prev_TR * (period-1)) + TR) / period
/// Smoothed +DM = ((prev_+DM * (period-1)) + +DM) / period
/// Smoothed -DM = ((prev_-DM * (period-1)) + -DM) / period
///
/// +DI = 100 * Smoothed +DM / Smoothed TR
/// -DI = 100 * Smoothed -DM / Smoothed TR
/// DX = 100 * |+DI - -DI| / (+DI + -DI)
/// ADX = Wilder's Smoothing of DX
/// ```
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - The period parameter (typically 14)
/// * `output_adx` - Output array for ADX values
/// * `output_smoothed_plus_dm` - Output array for smoothed +DM values
/// * `output_smoothed_minus_dm` - Output array for smoothed -DM values
/// * `output_smoothed_tr` - Output array for smoothed TR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds, Err otherwise
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::adx::adx;
///
/// let high = vec![24.20, 24.07, 24.04, 23.87, 23.67];
/// let low = vec![23.85, 23.72, 23.64, 23.37, 23.46];
/// let close = vec![23.89, 23.95, 23.67, 23.78, 23.50];
/// let period = 2;
///
/// let mut output_adx = vec![0.0; 5];
/// let mut smoothed_plus_dm = vec![0.0; 5];
/// let mut smoothed_minus_dm = vec![0.0; 5];
/// let mut smoothed_tr = vec![0.0; 5];
///
/// adx(
///     &high,
///     &low,
///     &close,
///     period,
///     &mut output_adx,
///     &mut smoothed_plus_dm,
///     &mut smoothed_minus_dm,
///     &mut smoothed_tr,
/// )
/// .unwrap();
/// ```
pub fn adx(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    output_adx: &mut [TAFloat],
    output_smoothed_plus_dm: &mut [TAFloat],
    output_smoothed_minus_dm: &mut [TAFloat],
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
            || len != output_adx.len()
            || len != output_smoothed_plus_dm.len()
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

    let mut dx_values = vec![0.0; len];

    // Calculate DX values
    dx::dx(
        input_high,
        input_low,
        input_close,
        param_period,
        &mut dx_values,
        output_smoothed_plus_dm,
        output_smoothed_minus_dm,
        output_smoothed_tr,
    )?;

    // Calculate initial ADX as simple average of first period DX values
    let mut sum = 0.0;
    for value in dx_values
        .iter()
        .take(lookback + 1)
        .skip(lookback + 1 - param_period)
    {
        sum += *value;
    }
    output_adx[lookback] = sum / param_period as TAFloat;

    // Calculate remaining ADX values using Wilder's smoothing
    let period_t = param_period as TAFloat;
    for i in (lookback + 1)..len {
        output_adx[i] = output_adx[i - 1].mul_add(period_t - 1.0, dx_values[i]) / period_t;
    }

    // Fill initial values with NAN
    for item in output_adx.iter_mut().take(param_period * 2 - 1) {
        *item = TAFloat::NAN;
    }

    Ok(())
}

/// Calculate the latest ADX value incrementally
///
/// This function calculates only the most recent ADX value using the previous values and the latest prices.
/// It is optimized for real-time calculations where only the latest value needs to be updated.
///
/// # Arguments
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `prev_high` - Previous period's high price
/// * `prev_low` - Previous period's low price
/// * `prev_close` - Previous period's close price
/// * `prev_adx` - Previous period's ADX value
/// * `prev_smoothed_plus_dm` - Previous period's smoothed +DM
/// * `prev_smoothed_minus_dm` - Previous period's smoothed -DM
/// * `prev_smoothed_tr` - Previous period's smoothed TR
/// * `param_period` - The period parameter (typically 14)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing:
///   - Latest ADX value
///   - New smoothed +DM
///   - New smoothed -DM
///   - New smoothed TR
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::adx::adx_inc;
///
/// let (adx, plus_dm, minus_dm, tr) = adx_inc(
///     24.20, // current high
///     23.85, // current low
///     24.07, // previous high
///     23.72, // previous low
///     23.95, // previous close
///     25.0,  // previous ADX
///     0.5,   // previous smoothed +DM
///     0.3,   // previous smoothed -DM
///     1.2,   // previous smoothed TR
///     14,    // period
/// )
/// .unwrap();
/// ```
pub fn adx_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_close: TAFloat,
    prev_adx: TAFloat,
    prev_smoothed_plus_dm: TAFloat,
    prev_smoothed_minus_dm: TAFloat,
    prev_smoothed_tr: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
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
            || prev_adx.is_nan()
            || prev_smoothed_plus_dm.is_nan()
            || prev_smoothed_minus_dm.is_nan()
            || prev_smoothed_tr.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let (dx, output_smoothed_plus_dm, output_smoothed_minus_dm, output_smoothed_tr) = dx::dx_inc(
        input_high,
        input_low,
        prev_high,
        prev_low,
        prev_close,
        prev_smoothed_plus_dm,
        prev_smoothed_minus_dm,
        prev_smoothed_tr,
        param_period,
    )?;

    let period_t = param_period as TAFloat;
    let output_adx = prev_adx.mul_add(period_t - 1.0, dx) / period_t;

    Ok((
        output_adx,
        output_smoothed_plus_dm,
        output_smoothed_minus_dm,
        output_smoothed_tr,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    /// Basic functionality tests
    #[test]
    fn test_adx_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1, 35179.2, 35244.9, 35150.2, 35136.0, 35133.6, 35188.0, 35215.3,
            35221.9, 35219.2, 35234.0, 35216.7, 35197.9, 35178.4, 35183.4, 35129.7, 35149.1,
            35129.3, 35125.5, 35114.5, 35120.1, 35129.4,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0, 35073.1, 35090.0, 35072.0, 35078.0, 35088.0, 35124.8, 35169.4,
            35138.0, 35141.0, 35182.0, 35151.1, 35158.4, 35140.0, 35087.0, 35085.8, 35114.7,
            35086.0, 35090.6, 35074.1, 35078.4, 35100.0,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
            35154.0, 35216.3, 35211.8, 35158.4, 35172.0, 35176.7, 35113.3, 35114.7, 35129.3,
            35094.6, 35114.4, 35094.5, 35116.0, 35105.4,
        ];
        let param_period = 14;
        let mut output_adx = vec![0.0; input_high.len()];
        let mut output_smoothed_plus_dm = vec![0.0; input_high.len()];
        let mut output_smoothed_minus_dm = vec![0.0; input_high.len()];
        let mut output_smoothed_tr = vec![0.0; input_high.len()];

        adx(
            &input_high,
            &input_low,
            &input_close,
            param_period,
            &mut output_adx,
            &mut output_smoothed_plus_dm,
            &mut output_smoothed_minus_dm,
            &mut output_smoothed_tr,
        )
        .unwrap();

        // First (2*period-1) values should be NaN
        for value in output_adx.iter().take(2 * param_period - 1) {
            assert!(value.is_nan());
        }

        // Test against known values
        let expected_values = [
            23.383_153_393_338_485,
            22.136_715_365_358_942,
            21.473_716_847_015_616,
            21.641_935_163_874_564,
            21.481_280_068_978_72,
            21.332_100_338_004_008,
            21.193_576_302_098_915,
            21.750_776_176_581_077,
            22.574_928_816_977_09,
            22.676_673_819_631_873,
            22.771_151_322_097_033,
            23.058_109_702_865_06,
            22.634_231_810_989_206,
            22.240_630_911_390_202,
            21.456_760_075_706_324,
            20.186_735_391_919_566,
            19.029_883_367_053_312,
            17.784_948_566_988_245,
            16.972_889_325_368_367,
            16.218_834_315_292_767,
            15.852_748_058_319_879,
            15.370_446_543_964_162,
            14.680_323_641_503_575,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(
                output_adx[i + 27],
                *expected,
                epsilon = 0.00001,
                max_relative = 0.00001
            );
        }

        // Calculate and verify incremental values starting from index 28
        for i in 28..input_high.len() {
            let (result, new_smoothed_plus_dm, new_smoothed_minus_dm, new_smoothed_tr) = adx_inc(
                input_high[i],
                input_low[i],
                input_high[i - 1],
                input_low[i - 1],
                input_close[i - 1],
                output_adx[i - 1],
                output_smoothed_plus_dm[i - 1],
                output_smoothed_minus_dm[i - 1],
                output_smoothed_tr[i - 1],
                param_period,
            )
            .unwrap();

            // Compare with full calculation
            assert_relative_eq!(result, output_adx[i], epsilon = 0.00001);
            assert_relative_eq!(
                new_smoothed_plus_dm,
                output_smoothed_plus_dm[i],
                epsilon = 0.00001
            );
            assert_relative_eq!(
                new_smoothed_minus_dm,
                output_smoothed_minus_dm[i],
                epsilon = 0.00001
            );
            assert_relative_eq!(new_smoothed_tr, output_smoothed_tr[i], epsilon = 0.00001);
        }
    }
}
