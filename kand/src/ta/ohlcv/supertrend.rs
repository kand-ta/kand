use super::atr;
use crate::{KandError, TAFloat, TAInt, types::Signal};

/// Returns the lookback period required for Supertrend calculation
///
/// # Description
/// Calculates the number of data points needed before the first valid Supertrend value can be generated.
/// The lookback period is equal to the ATR period.
///
/// # Arguments
/// * `opt_period` - The period used for ATR calculation, must be >= 2
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 2
///
/// # Example
/// ```
/// use kand::ohlcv::supertrend::lookback;
///
/// let period = 14;
/// let lookback_period = lookback(period).unwrap();
/// assert_eq!(lookback_period, 14);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    atr::lookback(opt_period)
}

/// Calculates Supertrend values for the entire price series
///
/// # Description
/// The Supertrend indicator is a trend-following indicator that combines Average True Range (ATR)
/// with dynamic support and resistance bands. It helps:
/// - Identify trend direction and potential reversals
/// - Provide adaptive stop-loss levels
/// - Generate trend-based trading signals
///
/// # Calculation Details
/// 1. Calculate ATR for volatility measurement
/// 2. Compute basic upper and lower bands:
///    ```text
///    Basic Upper = (High + Low) / 2 + Multiplier * ATR
///    Basic Lower = (High + Low) / 2 - Multiplier * ATR
///    ```
/// 3. Determine final bands using price action:
///    ```text
///    Final Upper = if Close[i-1] <= Upper[i-1]
///                  then min(Basic Upper[i], Upper[i-1])
///                  else Basic Upper[i]
///
///    Final Lower = if Close[i-1] >= Lower[i-1]
///                  then max(Basic Lower[i], Lower[i-1])
///                  else Basic Lower[i]
///    ```
/// 4. Identify trend direction:
///    ```text
///    Trend = if Close > Final Upper then 1 (Uptrend)
///            else if Close < Final Lower then -1 (Downtrend)
///            else Previous Trend
///    ```
/// 5. Generate Supertrend values:
///    ```text
///    Supertrend = if Trend == 1 then Final Lower (Support)
///                 else Final Upper (Resistance)
///    ```
///
/// # Parameters
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `opt_period` - ATR calculation period (typically 7-14)
/// * `opt_multiplier` - ATR multiplier (typically 2-4)
/// * `output_trend` - Output array for trend signals:
///   - 1: Uptrend
///   - 0: Initial/undefined
///   - -1: Downtrend
/// * `output_supertrend` - Output array for Supertrend values (support/resistance levels)
/// * `output_atr` - Output array for ATR values
/// * `output_upper` - Output array for upper band values
/// * `output_lower` - Output array for lower band values
///
/// # Returns
/// * `Ok(())` - Calculation successful
///
/// # Errors
/// * `KandError::InvalidData` - Empty input arrays
/// * `KandError::LengthMismatch` - Input/output arrays have different lengths
/// * `KandError::InvalidParameter` - Invalid `opt_period` (<2)
/// * `KandError::InsufficientData` - Input length less than required lookback
/// * `KandError::NaNDetected` - NaN values in input (with `check-nan` feature)
/// * `KandError::ConversionError` - Numeric conversion error
pub fn supertrend(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    opt_period: usize,
    opt_multiplier: TAFloat,
    output_trend: &mut [TAInt],
    output_supertrend: &mut [TAFloat],
    output_atr: &mut [TAFloat],
    output_upper: &mut [TAFloat],
    output_lower: &mut [TAFloat],
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
            || len != output_trend.len()
            || len != output_supertrend.len()
            || len != output_atr.len()
            || len != output_upper.len()
            || len != output_lower.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() || input_close[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate ATR
    atr::atr(input_high, input_low, input_close, opt_period, output_atr)?;

    let mut basic_upper = vec![0.0; len];
    let mut basic_lower = vec![0.0; len];

    // Convert trend direction values
    let up_trend = Signal::Bullish.into();
    let down_trend = Signal::Bearish.into();
    let no_trend = Signal::Neutral.into();

    // Calculate basic bands
    for i in lookback..len {
        let hl2 = f64::midpoint(input_high[i], input_low[i]);
        basic_upper[i] = opt_multiplier.mul_add(output_atr[i], hl2);
        basic_lower[i] = opt_multiplier.mul_add(-output_atr[i], hl2);
    }

    // Initialize the first valid point (at lookback)
    output_trend[lookback] = up_trend;
    output_supertrend[lookback] = basic_lower[lookback];
    output_upper[lookback] = basic_upper[lookback];
    output_lower[lookback] = basic_lower[lookback];

    // Calculate final bands and trend for remaining points
    for i in (lookback + 1)..len {
        // Calculate final upper band using min/max logic
        if input_close[i - 1] <= output_upper[i - 1] {
            output_upper[i] = basic_upper[i].min(output_upper[i - 1]);
        } else {
            output_upper[i] = basic_upper[i];
        }

        // Calculate final lower band using min/max logic
        if input_close[i - 1] >= output_lower[i - 1] {
            output_lower[i] = basic_lower[i].max(output_lower[i - 1]);
        } else {
            output_lower[i] = basic_lower[i];
        }

        // Determine trend direction based on previous trend
        if output_trend[i - 1] == up_trend {
            if input_close[i] < output_lower[i] {
                output_trend[i] = down_trend;
                output_supertrend[i] = output_upper[i];
            } else {
                output_trend[i] = up_trend;
                output_supertrend[i] = output_lower[i];
            }
        } else if input_close[i] > output_upper[i] {
            output_trend[i] = up_trend;
            output_supertrend[i] = output_lower[i];
        } else {
            output_trend[i] = down_trend;
            output_supertrend[i] = output_upper[i];
        }
    }

    // Fill initial values with 0 (matching Python's initialization)
    for i in 0..lookback {
        output_trend[i] = no_trend;
        output_supertrend[i] = TAFloat::NAN;
        output_atr[i] = TAFloat::NAN;
        output_upper[i] = TAFloat::NAN;
        output_lower[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates a single Supertrend value incrementally
///
/// # Description
/// Provides an optimized method for calculating the latest Supertrend value using previous state.
/// Ideal for real-time processing as it:
/// - Utilizes existing ATR, trend, and band values
/// - Only processes the most recent data point
/// - Avoids recomputing the entire series
///
/// # Calculation Process
/// 1. Update ATR using previous value
/// 2. Calculate new basic bands:
///    ```text
///    Basic Upper = (High + Low) / 2 + Multiplier * ATR
///    Basic Lower = (High + Low) / 2 - Multiplier * ATR
///    ```
/// 3. Determine final bands based on price position:
///    ```text
///    Final Upper = min(Basic Upper, Prev Upper) if Prev Close <= Prev Upper
///    Final Lower = max(Basic Lower, Prev Lower) if Prev Close >= Prev Lower
///    ```
/// 4. Update trend and Supertrend:
///    - Switch to downtrend if uptrend and price < lower band
///    - Switch to uptrend if downtrend and price > upper band
///    - Maintain trend otherwise
///
/// # Parameters
/// * `input_high` - Current period's high price
/// * `input_low` - Current period's low price
/// * `input_close` - Current period's close price
/// * `prev_close` - Previous period's close price
/// * `prev_atr` - Previous period's ATR value
/// * `prev_trend` - Previous period's trend (1: up, -1: down)
/// * `prev_upper` - Previous period's upper band
/// * `prev_lower` - Previous period's lower band
/// * `opt_period` - ATR calculation period (typically 7-14)
/// * `opt_multiplier` - ATR multiplier (typically 2-4)
///
/// # Returns
/// * `Ok((TAInt, TAFloat, TAFloat, TAFloat, TAFloat))` - Tuple containing:
///   - Current trend signal (1: uptrend, -1: downtrend)
///   - Current Supertrend value (support/resistance level)
///   - Current ATR value
///   - Current upper band
///   - Current lower band
///
/// # Errors
/// * `KandError::InvalidParameter` - Invalid `opt_period` (<2)
/// * `KandError::NaNDetected` - NaN values in input (with `check-nan` feature)
/// * `KandError::ConversionError` - Numeric conversion error
pub fn supertrend_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    prev_close: TAFloat,
    prev_atr: TAFloat,
    prev_trend: TAInt,
    prev_upper: TAFloat,
    prev_lower: TAFloat,
    opt_period: usize,
    opt_multiplier: TAFloat,
) -> Result<(TAInt, TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan()
            || input_low.is_nan()
            || input_close.is_nan()
            || prev_close.is_nan()
            || prev_atr.is_nan()
            || prev_upper.is_nan()
            || prev_lower.is_nan()
            || opt_multiplier.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let output_atr = atr::atr_inc(input_high, input_low, prev_close, prev_atr, opt_period)?;

    let hl2 = f64::midpoint(input_high, input_low);
    let basic_upper = opt_multiplier.mul_add(output_atr, hl2);
    let basic_lower = opt_multiplier.mul_add(-output_atr, hl2);

    let output_upper = if prev_close <= prev_upper {
        basic_upper.min(prev_upper)
    } else {
        basic_upper
    };

    let output_lower = if prev_close >= prev_lower {
        basic_lower.max(prev_lower)
    } else {
        basic_lower
    };

    let up_trend = Signal::Bullish.into();
    let down_trend = Signal::Bearish.into();

    let (output_trend, output_supertrend) = if prev_trend == up_trend {
        if input_close < output_lower {
            (down_trend, output_upper)
        } else {
            (up_trend, output_lower)
        }
    } else if input_close > output_upper {
        (up_trend, output_lower)
    } else {
        (down_trend, output_upper)
    };

    Ok((
        output_trend,
        output_supertrend,
        output_atr,
        output_upper,
        output_lower,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    /// Test the calculation of Supertrend
    #[test]
    fn test_supertrend_calculation() {
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

        let len = input_high.len();
        let opt_period = 10;
        let opt_multiplier = 3.0;

        let mut output_trend = vec![0; len];
        let mut output_supertrend = vec![0.0; len];
        let mut output_atr = vec![0.0; len];
        let mut output_upper = vec![0.0; len];
        let mut output_lower = vec![0.0; len];

        supertrend(
            &input_high,
            &input_low,
            &input_close,
            opt_period,
            opt_multiplier,
            &mut output_trend,
            &mut output_supertrend,
            &mut output_atr,
            &mut output_upper,
            &mut output_lower,
        )
        .unwrap();

        // Test Supertrend values
        let expected_supertrend = [
            35014.05,
            35_021.590_000_000_004,
            35_043.585_999_999_996,
            35_043.585_999_999_996,
            35_043.585_999_999_996,
            35_285.012_906,
            35_217.191_615_399_99,
            35_205.262_453_86,
            35_205.262_453_86,
            35_205.262_453_86,
            35_201.637_078_863_94,
            35_166.628_370_977_545,
            35_166.628_370_977_545,
            35_166.628_370_977_545,
            35_166.628_370_977_545,
        ];

        // Test Trend values
        let expected_trend = [
            100, 100, 100, 100, 100, -100, -100, -100, -100, -100, -100, -100, -100, -100, -100,
        ];

        // Check if the first 10 values are NaN
        for i in 0..10 {
            assert!(
                output_supertrend[i].is_nan(),
                "Expected NaN for output_supertrend[{i}]"
            );
            assert!(output_trend[i] == 0, "Expected NaN for output_trend[{i}]");
        }

        // Check the remaining values against expected values
        for i in 10..expected_supertrend.len() {
            let expected_st = expected_supertrend[i - 10];
            let expected_tr = expected_trend[i - 10];
            assert_relative_eq!(output_supertrend[i], expected_st, epsilon = 0.00001);
            assert!(output_trend[i] == expected_tr);
        }

        // Test incremental calculation matches regular calculation
        let prev_close = input_close[len - 2];
        let prev_atr = output_atr[len - 2];
        let prev_trend = output_trend[len - 2];
        let prev_upper = output_upper[len - 2];
        let prev_lower = output_lower[len - 2];

        let (trend, supertrend, _, _, _) = supertrend_inc(
            input_high[len - 1],
            input_low[len - 1],
            input_close[len - 1],
            prev_close,
            prev_atr,
            prev_trend,
            prev_upper,
            prev_lower,
            opt_period,
            opt_multiplier,
        )
        .unwrap();

        assert!(trend == output_trend[len - 1]);
        assert_relative_eq!(supertrend, output_supertrend[len - 1], epsilon = 0.00001);
    }
}
