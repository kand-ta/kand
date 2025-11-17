use crate::{KandError, TAFloat};

/// Returns the lookback period required for Balance of Power (BOP) calculation.
///
/// # Description
/// The BOP indicator has no lookback period as it is calculated point-by-point
/// using only current period's price data.
///
/// # Arguments
/// * None
///
/// # Returns
/// * `Result<usize, KandError>` - Always returns 0 as no lookback is needed
///
/// # Errors
/// This function does not return any errors. It always returns `Ok(0)`.
///
/// # Example
/// ```
/// use kand::ohlcv::bop;
/// let lookback = bop::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates the Balance of Power (BOP) indicator for a price series.
///
/// # Description
/// The Balance of Power (BOP) is a momentum oscillator that measures the relative strength
/// between buyers and sellers by comparing the closing price to the opening price and
/// normalizing it by the trading range (high - low).
///
/// # Mathematical Formula
/// ```text
/// BOP = (Close - Open) / (High - Low)
/// ```
///
/// # Calculation Steps
/// 1. Calculate the price range (High - Low) for each period
/// 2. Calculate the close-open difference (Close - Open) for each period
/// 3. Divide the close-open difference by the price range
/// 4. If price range is zero, return zero to avoid division by zero
///
/// # Arguments
/// * `input_open` - Array of opening prices
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `output_bop` - Array to store calculated BOP values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok value on success
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::NaNDetected` - If any input contains NaN values (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::bop;
///
/// let input_open = vec![10.0, 11.0, 12.0, 13.0];
/// let input_high = vec![12.0, 13.0, 14.0, 15.0];
/// let input_low = vec![8.0, 9.0, 10.0, 11.0];
/// let input_close = vec![11.0, 12.0, 13.0, 14.0];
/// let mut output_bop = vec![0.0; 4];
///
/// bop::bop(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     &mut output_bop,
/// )
/// .unwrap();
/// ```
pub fn bop(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    output_bop: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_open.len();

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len != input_high.len()
            || len != input_low.len()
            || len != input_close.len()
            || len != output_bop.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            // NaN check
            if input_open[i].is_nan()
                || input_high[i].is_nan()
                || input_low[i].is_nan()
                || input_close[i].is_nan()
            {
                return Err(KandError::NaNDetected);
            }
        }
    }

    for i in 0..len {
        let range = input_high[i] - input_low[i];
        if range == 0.0 {
            output_bop[i] = 0.0;
        } else {
            output_bop[i] = (input_close[i] - input_open[i]) / range;
        }
    }

    Ok(())
}

/// Calculates a single Balance of Power (BOP) value for the latest price data.
///
/// # Description
/// This function provides an efficient way to calculate the BOP value incrementally
/// for new price data, without recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// BOP = (Close - Open) / (High - Low)
/// ```
///
/// # Arguments
/// * `input_open` - Current period's opening price
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `input_close` - Current period's closing price
///
/// # Returns
/// * `Result<TAFloat, KandError>` - Calculated BOP value if successful
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::bop::bop_inc;
///
/// let input_open = 10.0;
/// let input_high = 12.0;
/// let input_low = 8.0;
/// let input_close = 11.0;
///
/// let output_bop = bop_inc(input_open, input_high, input_low, input_close).unwrap();
/// ```
pub fn bop_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_open.is_nan() || input_high.is_nan() || input_low.is_nan() || input_close.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let range = input_high - input_low;
    if range == 0.0 {
        Ok(0.0)
    } else {
        Ok((input_close - input_open) / range)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_bop_calculation() {
        let input_open = vec![
            35253.1, 35216.2, 35221.4, 35190.7, 35169.9, 35181.5, 35254.6, 35203.5, 35251.8,
            35198.0, 35184.6, 35175.0, 35229.8, 35212.6, 35160.7, 35090.3, 35041.2, 34999.2,
            35013.4, 35069.0,
        ];
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];

        let mut output_bop = vec![0.0; input_open.len()];
        bop(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            &mut output_bop,
        )
        .unwrap();

        let expected_values = [
            -0.741_482_965_931_842_2,
            0.126_829_268_292_789_4,
            -0.551_166_965_888_796_5,
            -0.344_425_956_738_686_9,
            0.408_450_704_225_279_96,
            0.877_551_020_408_115_2,
            -0.859_038_142_620_118_4,
            0.620_512_820_512_839_2,
            -0.669_135_802_469_189_7,
            -0.302_272_727_272_793_4,
            -0.655_172_413_793_103_4,
            0.723_320_158_102_772_1,
            -0.314_545_454_545_507_5,
            -0.699_460_916_442_095_5,
            -0.531_320_754_716_937_2,
            -0.455_473_098_330_282_9,
            -0.429_303_278_688_471_35,
            0.265_420_560_747_745_17,
            0.836_090_225_563_887_9,
            -0.707_006_369_426_742,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_bop[i], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation matches regular calculation
        for i in 0..input_open.len() {
            let result =
                bop_inc(input_open[i], input_high[i], input_low[i], input_close[i]).unwrap();
            assert_relative_eq!(result, output_bop[i], epsilon = 0.0001);
        }
    }
}
