use super::atr;
use crate::{KandError, TAFloat};

/// Returns the lookback period required for NATR calculation
///
/// # Description
/// The lookback period determines how many data points are needed before the first valid NATR value can be calculated.
///
/// # Arguments
/// * `param_period` - The period used for NATR calculation. Must be >= 2.
///
/// # Returns
/// * `Result<usize, KandError>` - The number of data points needed before first valid output
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
///
/// # Example
/// ```
/// use kand::ohlcv::natr;
/// let period = 14;
/// let lookback = natr::lookback(period).unwrap();
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

/// Calculates Normalized Average True Range (NATR) for the entire input array
///
/// # Description
/// NATR is a volatility indicator that expresses ATR as a percentage of closing price,
/// making it easier to compare volatility across different price levels.
///
/// # Mathematical Formula
/// ```text
/// TR = max(high - low, |high - prev_close|, |low - prev_close|)
/// ATR = EMA(TR, period)
/// NATR = (ATR / close_price) * 100
/// ```
///
/// # Calculation Steps
/// 1. Calculate True Range (TR) for each period
/// 2. Calculate ATR using exponential moving average of TR
/// 3. Normalize ATR by dividing by closing price and multiplying by 100
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - Period for NATR calculation (must be >= 2)
/// * `output_natr` - Array to store calculated NATR values
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::natr;
///
/// let high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let close = vec![9.0, 11.0, 14.0, 12.0, 11.0];
/// let period = 3;
/// let mut natr = vec![0.0; 5];
///
/// natr::natr(&high, &low, &close, period, &mut natr).unwrap();
/// ```
pub fn natr(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    output_natr: &mut [TAFloat],
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
        if len != input_low.len() || len != input_close.len() || len != output_natr.len() {
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

    // Calculate ATR first
    let mut atr_values = vec![0.0; len];
    atr::atr(
        input_high,
        input_low,
        input_close,
        param_period,
        &mut atr_values,
    )?;

    // Calculate NATR = (ATR / Close) * 100
    for i in lookback..len {
        output_natr[i] = (atr_values[i] / input_close[i]) * 100.0;
    }

    // Fill initial values with NAN up to lookback period
    for value in output_natr.iter_mut().take(lookback) {
        *value = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest NATR value incrementally
///
/// # Description
/// This function provides an optimized way to calculate a single new NATR value
/// using the previous ATR value and current price data, without recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// TR = max(high - low, |high - prev_close|, |low - prev_close|)
/// New ATR = ((prev_ATR * (period-1)) + TR) / period
/// NATR = (ATR / close_price) * 100
/// ```
///
/// # Arguments
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `input_close` - Current period's closing price
/// * `prev_close` - Previous period's closing price
/// * `prev_atr` - Previous period's ATR value
/// * `param_period` - Period for NATR calculation (must be >= 2)
///
/// # Returns
/// * `Result<TAFloat, KandError>` - The calculated NATR value
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` < 2
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::natr;
///
/// let high = 15.0;
/// let low = 11.0;
/// let close = 14.0;
/// let prev_close = 12.0;
/// let prev_atr = 3.0;
/// let period = 3;
///
/// let natr = natr::natr_inc(high, low, close, prev_close, prev_atr, period).unwrap();
/// ```
pub fn natr_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    prev_close: TAFloat,
    prev_atr: TAFloat,
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
            || input_low.is_nan()
            || input_close.is_nan()
            || prev_close.is_nan()
            || prev_atr.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let output_atr = atr::atr_inc(input_high, input_low, prev_close, prev_atr, param_period)?;
    Ok((output_atr / input_close) * 100.0)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_natr_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35013.4, 34950.1, 34900.0, 34920.0, 34952.6, 35000.0, 35041.8, 35080.0, 35080.0,
            35070.0, 35050.0, 35092.0,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3,
        ];
        let param_period = 14;
        let mut output_natr = vec![0.0; input_high.len()];

        natr(
            &input_high,
            &input_low,
            &input_close,
            param_period,
            &mut output_natr,
        )
        .unwrap();

        // First period values should be NaN
        for value in output_natr.iter().take(param_period) {
            assert!(value.is_nan());
        }

        // Test expected values
        let expected_values = [
            0.180_066_041_856_908_33,
            0.189_412_602_820_658_08,
            0.196_012_458_358_289_65,
            0.192_852_460_649_046_9,
            0.192_338_093_664_248_45,
            0.191_633_390_505_224_85,
            0.199_333_290_479_994_7,
            0.200_025_731_859_381_86,
            0.197_384_596_204_178_14,
            0.197_172_664_957_892_62,
            0.193_559_020_569_269_05,
            0.195_301_199_830_777_3,
            0.195_462_787_749_759_28,
            0.186_820_266_737_109_55,
            0.180_106_296_880_325_17,
            0.188_494_189_518_896_63,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_natr[i + param_period], *expected, epsilon = 0.1);
        }

        // Test incremental calculation matches
        for i in param_period + 1..input_high.len() {
            let output_natr_inc = natr_inc(
                input_high[i],
                input_low[i],
                input_close[i],
                input_close[i - 1],
                output_natr[i - 1] * input_close[i - 1] / 100.0,
                param_period,
            )
            .unwrap();
            assert_relative_eq!(output_natr_inc, output_natr[i], epsilon = 0.00001);
        }
    }
}
