use crate::{
    KandError, TAFloat, TAInt,
    helper::{lower_shadow_length, period_to_k, real_body_length, upper_shadow_length},
    types::Signal,
};

/// Returns the required lookback period for Long Shadow pattern detection.
///
/// # Description
/// Calculates the minimum number of historical data points needed to generate the first valid signal.
/// For Long Shadow pattern detection, this equals `opt_period - 1` to ensure proper EMA calculation
/// of candle body sizes.
///
/// # Arguments
/// * `opt_period` - The period used for EMA calculation of candle body sizes (must be >= 2)
///
/// # Returns
/// * `Ok(usize)` - The required lookback period
///
/// # Errors
/// * `KandError::InvalidParameter` - If `opt_period` < 2
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_long_shadow;
///
/// let period = 14;
/// let lookback = cdl_long_shadow::lookback(period).unwrap();
/// assert_eq!(lookback, 13);
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

/// Detects Long Shadow candlestick patterns in price data.
///
/// # Description
/// A Long Shadow candlestick pattern indicates potential trend reversals based on the relative lengths
/// of the candlestick's shadows and body. The pattern is characterized by a small real body and a
/// significantly long shadow on either the upper or lower side.
///
/// # Mathematical Formula
/// ```text
/// Body = |Close - Open|
/// UpperShadow = High - max(Open, Close)
/// LowerShadow = min(Open, Close) - Low
/// TotalRange = High - Low
/// BodyAvg = EMA(Body, Period)
///
/// Pattern detected when:
/// 1. Body <= BodyAvg
/// 2. UpperShadow or LowerShadow >= ShadowFactor * TotalRange
/// ```
///
/// # Arguments
/// * `input_open` - Array of opening prices
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `opt_period` - Period for EMA calculation of body sizes (typically 14)
/// * `opt_shadow_factor` - Minimum percentage of total range that shadow must be (typically 75.0)
/// * `output_signals` - Output array for pattern signals:
///   - 1: Bullish Long Lower Shadow
///   - -1: Bearish Long Upper Shadow
///   - 0: No pattern
///   - `i64::MIN`: Insufficient data
/// * `output_body_avg` - Output array storing the EMA values of candle body sizes
///
/// # Returns
/// * `Ok(())` - Calculation completed successfully
///
/// # Errors
/// * [`KandError::LengthMismatch`] - Input arrays have different lengths
/// * [`KandError::InvalidParameter`] - Parameter values are invalid
/// * [`KandError::InsufficientData`] - Input length is less than required lookback
/// * [`KandError::NaNDetected`] - Input contains NaN values (when `check-nan` enabled)
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_long_shadow;
///
/// let open = vec![10.0, 11.0, 10.5];
/// let high = vec![12.0, 11.5, 11.0];
/// let low = vec![9.0, 10.5, 9.5];
/// let close = vec![11.0, 10.5, 10.0];
/// let mut signals = vec![0; 3];
/// let mut body_avg = vec![0.0; 3];
///
/// cdl_long_shadow::cdl_long_shadow(
///     &open,
///     &high,
///     &low,
///     &close,
///     2,
///     75.0,
///     &mut signals,
///     &mut body_avg,
/// )
/// .unwrap();
/// ```
pub fn cdl_long_shadow(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    opt_period: usize,
    opt_shadow_factor: TAFloat,
    output_signals: &mut [TAInt],
    output_body_avg: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_open.len();
    let lookback = lookback(opt_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Length consistency check
        if len != input_high.len()
            || len != input_low.len()
            || len != input_close.len()
            || len != output_signals.len()
            || len != output_body_avg.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Parameter range check
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if opt_shadow_factor <= 0.0 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            if input_open[i].is_nan()
                || input_high[i].is_nan()
                || input_low[i].is_nan()
                || input_close[i].is_nan()
            {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate initial SMA
    let mut sum = 0.0;
    for i in 0..opt_period {
        sum += real_body_length(input_open[i], input_close[i]);
    }
    let mut body_avg = sum / opt_period as TAFloat;
    output_body_avg[lookback] = body_avg;

    // Process remaining candles
    for i in lookback..len {
        let (signal, new_body_avg) = cdl_long_shadow_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            body_avg,
            opt_period,
            opt_shadow_factor,
        )?;
        output_signals[i] = signal;
        output_body_avg[i] = new_body_avg;
        body_avg = new_body_avg;
    }

    // Fill initial values
    for i in 0..lookback {
        output_signals[i] = Signal::Invalid.into();
        output_body_avg[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Processes a single candlestick to detect a Long Shadow pattern.
///
/// # Description
/// Implements incremental calculation for Long Shadow pattern detection by analyzing
/// a single candlestick's components and comparing them against thresholds.
///
/// # Calculation
/// 1. Calculate real body, upper shadow, and lower shadow lengths
/// 2. Calculate total range = high - low
/// 3. Update EMA of body sizes using: EMA = (body - `prev_ema`) * k + `prev_ema`
/// 4. Check pattern conditions:
///    - Small body: body <= `body_avg`
///    - Shadow threshold = `shadow_factor` * `total_range` / 100
///    - Long upper/lower shadow: shadow >= `shadow_threshold`
///
/// # Arguments
/// * `input_open` - Opening price of the candlestick
/// * `input_high` - High price of the candlestick
/// * `input_low` - Low price of the candlestick
/// * `input_close` - Closing price of the candlestick
/// * `prev_body_avg` - Previous EMA value of body sizes
/// * `opt_period` - Period for EMA calculation
/// * `opt_shadow_factor` - Minimum percentage of total range that shadow must be
///
/// # Returns
/// * `Ok((TAInt, TAFloat))` - Tuple containing:
///   - First element: Pattern signal where:
///     * 100: Bullish Long Lower Shadow
///     * -100: Bearish Long Upper Shadow
///     * 0: No pattern detected
///   - Second element: Updated EMA value of body sizes
///
/// # Errors
/// * [`KandError::InvalidParameter`] - If parameters are invalid:
///   - `opt_period` is less than 2
///   - `opt_shadow_factor` is less than or equal to zero
/// * [`KandError::NaNDetected`] - If any input value is NaN (when `check-nan` enabled)
/// * [`KandError::ConversionError`] - If numeric conversion fails
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_long_shadow;
///
/// let (signal, body_avg) = cdl_long_shadow::cdl_long_shadow_inc(
///     10.0, // open
///     11.0, // high
///     9.0,  // low
///     10.5, // close
///     0.5,  // previous body average
///     14,   // period
///     75.0, // shadow factor
/// )
/// .unwrap();
/// ```
pub fn cdl_long_shadow_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    prev_body_avg: TAFloat,
    opt_period: usize,
    opt_shadow_factor: TAFloat,
) -> Result<(TAInt, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if opt_shadow_factor <= 0.0 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_open.is_nan()
            || input_high.is_nan()
            || input_low.is_nan()
            || input_close.is_nan()
            || prev_body_avg.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let body = real_body_length(input_open, input_close);
    let up_shadow = upper_shadow_length(input_high, input_open, input_close);
    let down_shadow = lower_shadow_length(input_low, input_open, input_close);
    let total_range = input_high - input_low;
    let k = period_to_k(opt_period)?;
    let body_avg = (body - prev_body_avg).mul_add(k, prev_body_avg);

    // Check for Long Shadow patterns
    let is_small_body = body <= body_avg;
    let shadow_threshold = (opt_shadow_factor / 100.0) * total_range;
    let has_long_upper_shadow = up_shadow >= shadow_threshold;
    let has_long_lower_shadow = down_shadow >= shadow_threshold;

    let signal = if is_small_body {
        if has_long_upper_shadow && !has_long_lower_shadow {
            Signal::Bearish.into()
        } else if has_long_lower_shadow && !has_long_upper_shadow {
            Signal::Bullish.into()
        } else {
            Signal::Neutral.into()
        }
    } else {
        Signal::Neutral.into()
    };

    Ok((signal, body_avg))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_cdl_long_shadow() {
        let input_open = vec![
            96674.3, 96814.9, 96667.3, 96747.9, 96743.4, 96712.4, 96677.7, 96556.3, 96500.0,
            96442.8, 96229.7, 96152.2, 96145.7, 96233.1, 96140.1, 95505.0, 95575.1, 95585.2,
            95544.0, 95450.0, 95592.4, 95456.0, 95664.2, 95674.1, 95546.9,
        ];
        let input_high = vec![
            96911.0, 96831.2, 96754.9, 96875.7, 96793.9, 96725.7, 96802.4, 96646.6, 96526.0,
            96470.9, 96229.8, 96245.3, 96341.5, 96233.1, 96140.1, 95582.6, 95856.1, 95585.2,
            95702.4, 95729.2, 95633.3, 95720.6, 95698.2, 95682.9, 95794.6,
        ];
        let input_low = vec![
            96567.2, 96646.7, 96648.0, 96730.0, 96694.7, 96660.4, 96556.2, 96500.1, 96400.0,
            96200.0, 96100.0, 96073.6, 96130.4, 96045.6, 95400.0, 95200.1, 95544.8, 95400.7,
            95427.6, 95359.5, 95442.3, 95427.0, 95545.4, 95473.1, 95475.8,
        ];
        let input_close = vec![
            96814.9, 96667.4, 96747.9, 96743.4, 96712.5, 96677.7, 96556.2, 96500.1, 96442.7,
            96229.7, 96152.2, 96145.7, 96233.1, 96140.0, 95505.2, 95575.1, 95585.3, 95544.0,
            95450.1, 95592.5, 95456.0, 95664.2, 95674.1, 95547.0, 95679.4,
        ];

        let opt_period = 14;
        let opt_shadow_factor = 75.0;
        let mut output_signals = vec![0; input_open.len()];
        let mut output_body_avg = vec![0.0; input_open.len()];

        cdl_long_shadow(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            opt_period,
            opt_shadow_factor,
            &mut output_signals,
            &mut output_body_avg,
        )
        .unwrap();

        // First 13 values should be i64::MIN
        for i in 0..13 {
            assert_eq!(output_signals[i], Signal::Invalid.into());
            assert!(output_body_avg[i].is_nan());
        }

        println!("output_signals: {output_signals:?}");

        // Test specific signals
        assert_eq!(output_signals[15], Signal::Bullish.into()); // Example bullish signal
        assert_eq!(output_signals[16], Signal::Bearish.into()); // Example bearish signal
        assert_eq!(output_signals[17], Signal::Bullish.into()); // Example bullish signal
        assert_eq!(output_signals[22], Signal::Bullish.into()); // Example bullish signal

        // Test incremental calculation matches regular calculation
        let mut prev_body_avg = output_body_avg[13]; // First valid body average

        // Test each incremental step
        for i in 14..18 {
            let (signal, new_body_avg) = cdl_long_shadow_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                prev_body_avg,
                opt_period,
                opt_shadow_factor,
            )
            .unwrap();
            assert_eq!(signal, output_signals[i]);
            assert_relative_eq!(new_body_avg, output_body_avg[i], epsilon = 0.00001);
            prev_body_avg = new_body_avg;
        }
    }
}
