use crate::{
    KandError, TAFloat, TAInt,
    helper::{lower_shadow_length, period_to_k, real_body_length, upper_shadow_length},
    types::Signal,
};

/// Calculates the lookback period for Marubozu pattern detection.
///
/// # Description
/// The lookback period represents the minimum number of historical data points needed
/// before generating the first valid signal. For Marubozu pattern, this equals
/// `param_period - 1` to allow for proper EMA calculation of average body sizes.
///
/// # Parameters
/// * `param_period` - The period used for calculating the exponential moving average (EMA)
///   of candle body sizes. Must be >= 2.
///
/// # Returns
/// * `Ok(usize)` - The required lookback period
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_marubozu;
///
/// let param_period = 14;
/// let lookback_period = cdl_marubozu::lookback(param_period).unwrap();
/// assert_eq!(lookback_period, 13);
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

/// Identifies Marubozu candlestick patterns in price data.
///
/// # Description
/// A Marubozu ("bald head" or "close-cropped" in Japanese) is a candlestick pattern with
/// a long real body and very small or no shadows/wicks. It indicates strong market momentum:
/// - Bullish Marubozu (green/white): Open is low, close is high, showing strong buying pressure
/// - Bearish Marubozu (red/black): Open is high, close is low, showing strong selling pressure
///
/// # Calculation
/// The pattern is identified by:
/// 1. Body length = |close - open|
/// 2. Upper shadow = high - max(open,close)
/// 3. Lower shadow = min(open,close) - low
/// 4. Compare body length to EMA of previous body lengths
/// 5. Check if both shadows are smaller than threshold (`param_shadow_percent`% of body)
///
/// # Parameters
/// * `input_open` - Array of opening prices
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - Period for EMA calculation (>= 2)
/// * `param_shadow_percent` - Maximum shadow size as percentage of body (e.g. 5.0)
/// * `output_signals` - Output array for pattern signals:
///   - 1: Bullish Marubozu (strong upward trend)
///   - 0: No pattern
///   - -1: Bearish Marubozu (strong downward trend)
/// * `output_body_avg` - Output array for EMA values of body sizes
///
/// # Returns
/// * `Ok(())` - Calculation successful
///
/// # Errors
/// * [`KandError::InvalidData`] - Empty input arrays
/// * [`KandError::LengthMismatch`] - Input/output arrays have different lengths
/// * [`KandError::InvalidParameter`] - Invalid `param_period` (<2) or `param_shadow_percent` (<=0)
/// * [`KandError::InsufficientData`] - Input length less than required lookback period
/// * [`KandError::NaNDetected`] - NaN values in input (when `check-nan` enabled)
/// * [`KandError::ConversionError`] - Numeric conversion error
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_marubozu;
///
/// let input_open = vec![10.0, 10.5, 10.3, 10.2];
/// let input_high = vec![11.0, 11.2, 10.8, 10.5];
/// let input_low = vec![9.8, 10.3, 10.1, 10.0];
/// let input_close = vec![10.5, 11.0, 10.5, 10.1];
/// let mut output_signals = vec![0i64; 4];
/// let mut output_body_avg = vec![0.0; 4];
///
/// cdl_marubozu::cdl_marubozu(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     2,
///     5.0,
///     &mut output_signals,
///     &mut output_body_avg,
/// )
/// .unwrap();
/// ```
pub fn cdl_marubozu(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    param_shadow_percent: TAFloat,
    output_signals: &mut [TAInt],
    output_body_avg: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_open.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Check data sufficiency
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Check array lengths
        if len != input_high.len()
            || len != input_low.len()
            || len != input_close.len()
            || len != output_signals.len()
            || len != output_body_avg.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Check param_period
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if param_shadow_percent <= 0.0 {
            return Err(KandError::InvalidParameter);
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

    // Calculate initial SMA
    let mut sum = 0.0;
    for i in 0..param_period {
        sum += real_body_length(input_open[i], input_close[i]);
    }
    let mut body_avg = sum / param_period as TAFloat;
    output_body_avg[lookback] = body_avg;

    // Process each candle
    for i in param_period..len {
        let (signal, new_body_avg) = cdl_marubozu_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            body_avg,
            param_period,
            param_shadow_percent,
        )?;
        output_signals[i] = signal;
        output_body_avg[i] = new_body_avg;
        body_avg = new_body_avg;
    }

    // Fill initial values
    for i in 0..lookback {
        output_signals[i] = Signal::Neutral.into();
        output_body_avg[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Incrementally processes a single candlestick for Marubozu pattern detection.
///
/// # Description
/// This function efficiently updates Marubozu calculations for new data points without
/// reprocessing the entire dataset. It:
/// 1. Updates the EMA of body sizes using the latest candle
/// 2. Checks if the current candle forms a Marubozu pattern
///
/// # Calculation
/// 1. Body = |close - open|
/// 2. Upper shadow = high - max(open,close)
/// 3. Lower shadow = min(open,close) - low
/// 4. Shadow threshold = body * `param_shadow_percent` / 100
/// 5. New EMA = (Current - Prev) * K + Prev, where K = 2/(period+1)
/// 6. Pattern identified if:
///    - Body > current EMA
///    - Both shadows <= shadow threshold
///    - Direction determined by close vs open
///
/// # Parameters
/// * `input_open` - Opening price of current candle
/// * `input_high` - High price of current candle
/// * `input_low` - Low price of current candle
/// * `input_close` - Closing price of current candle
/// * `prev_body_avg` - Previous EMA value of body sizes
/// * `param_period` - Period for EMA calculation (>= 2)
/// * `param_shadow_percent` - Maximum shadow size as percentage of body
///
/// # Returns
/// * `Ok((TAInt, T))` - Tuple containing:
///   - Signal value (100: Bullish, 0: None, -100: Bearish)
///   - Updated EMA of body sizes
///
/// # Errors
/// * [`KandError::InvalidParameter`] - If `param_period` < 2 or `param_shadow_percent` <= 0
/// * [`KandError::NaNDetected`] - If any input contains NaN (when `check-nan` feature is enabled)
/// * [`KandError::ConversionError`] - If numeric conversion fails
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_marubozu;
///
/// let (signal, new_avg) = cdl_marubozu::cdl_marubozu_inc(
///     10.0, // open
///     10.5, // high
///     9.8,  // low
///     10.4, // close
///     10.2, // prev_body_avg
///     14,   // period
///     5.0,  // shadow_percent
/// )
/// .unwrap();
/// ```
pub fn cdl_marubozu_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    prev_body_avg: TAFloat,
    param_period: usize,
    param_shadow_percent: TAFloat,
) -> Result<(TAInt, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if param_shadow_percent <= 0.0 {
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
    let dn_shadow = lower_shadow_length(input_low, input_open, input_close);
    let shadow_threshold = body * param_shadow_percent / 100.0;

    // Calculate new body average using EMA formula
    let multiplier = period_to_k(param_period)?;
    let new_body_avg = (body - prev_body_avg).mul_add(multiplier, prev_body_avg);

    // Check for Marubozu pattern
    let signal =
        if body > prev_body_avg && up_shadow <= shadow_threshold && dn_shadow <= shadow_threshold {
            // Bullish if close > open, Bearish if close < open
            if input_close > input_open {
                Signal::Bullish.into()
            } else {
                Signal::Bearish.into()
            }
        } else {
            Signal::Neutral.into()
        };

    Ok((signal, new_body_avg))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_cdl_marubozu() {
        let input_open = vec![
            96105.4, 96156.3, 96166.5, 96171.2, 96225.4, 96183.7, 96069.0, 95991.2, 96005.3,
            95930.5, 95902.0, 95931.7, 95979.0, 95950.1, 96045.6, 96139.0, 96146.5, 96139.2,
            96216.2, 96460.1, 96519.4, 96511.7, 96405.0, 96316.9, 96232.7, 96277.0, 96196.6,
            96300.0, 96200.0, 96007.3, 95958.1, 95909.6, 95800.1, 95437.2, 94963.6, 94820.8,
            94789.1, 95121.0, 94925.0, 95136.7, 95376.3, 95399.9, 95575.1, 95679.0, 95723.1,
            95779.9, 96350.1, 95906.8, 95747.8, 95779.8, 95869.8, 95984.8, 96040.7, 96040.1,
            96062.5, 96009.7, 96087.9, 96230.0, 96275.6, 96350.2, 96477.6, 96423.6,
        ];
        let input_high = vec![
            96205.1, 96180.8, 96182.7, 96240.2, 96230.6, 96183.8, 96069.0, 96020.0, 96084.6,
            95975.0, 95945.7, 95991.2, 95993.3, 96016.3, 96142.3, 96177.5, 96210.0, 96247.7,
            96640.7, 96591.7, 96750.0, 96530.7, 96413.9, 96319.9, 96305.2, 96291.8, 96300.0,
            96300.0, 96225.5, 96043.9, 95973.0, 95952.3, 95822.4, 95484.7, 95155.8, 94930.3,
            95237.6, 95121.0, 95136.7, 95463.1, 95471.3, 95671.7, 95679.0, 95742.6, 95788.0,
            96417.5, 96366.4, 96000.0, 95940.7, 95940.6, 96034.7, 96100.1, 96120.0, 96062.5,
            96062.5, 96115.1, 96236.6, 96333.2, 96481.8, 96547.0, 96522.1, 96763.3,
        ];
        let input_low = vec![
            96105.4, 96123.9, 96081.6, 96157.3, 96162.4, 96050.2, 95980.1, 95974.0, 95893.0,
            95828.0, 95829.0, 95849.9, 95932.8, 95950.0, 96041.0, 96097.7, 96082.0, 96123.0,
            96204.1, 96346.9, 96442.5, 96405.2, 96302.2, 96182.6, 96206.5, 96145.0, 96190.3,
            96200.0, 95972.1, 95921.7, 95888.2, 95777.3, 95180.0, 94848.0, 94727.4, 94650.1,
            94744.4, 94718.4, 94828.2, 95095.9, 95217.6, 95399.9, 95428.4, 95420.0, 95654.7,
            95747.1, 95902.0, 95462.4, 95615.5, 95674.7, 95780.0, 95984.8, 96000.0, 95948.2,
            95935.0, 96008.2, 96062.5, 96229.9, 96225.5, 96350.2, 96407.5, 96414.0,
        ];
        let input_close = vec![
            96156.3, 96166.4, 96171.1, 96225.4, 96183.7, 96069.0, 95991.3, 96005.3, 95930.5,
            95902.0, 95931.7, 95979.1, 95950.1, 96011.4, 96139.0, 96146.5, 96139.1, 96216.3,
            96460.2, 96519.3, 96511.8, 96405.2, 96316.9, 96232.7, 96277.0, 96196.6, 96300.0,
            96200.0, 96007.4, 95958.1, 95909.6, 95800.0, 95437.3, 94963.6, 94820.3, 94789.2,
            95120.9, 94925.0, 95136.7, 95376.3, 95399.9, 95575.0, 95678.9, 95723.1, 95779.9,
            96350.0, 95906.8, 95747.8, 95779.9, 95869.9, 95984.8, 96040.7, 96040.1, 96062.4,
            96009.7, 96087.9, 96230.0, 96275.6, 96350.2, 96477.6, 96423.6, 96749.1,
        ];

        let param_period = 14;
        let param_shadow_percent = 5.0;
        let mut output_signals = vec![0i64; input_open.len()];
        let mut output_body_avg = vec![0.0; input_open.len()];

        cdl_marubozu(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            param_period,
            param_shadow_percent,
            &mut output_signals,
            &mut output_body_avg,
        )
        .unwrap();

        // First 13 values should be 0
        for i in 0..13 {
            assert_eq!(output_signals[i], 0);
            assert!(output_body_avg[i].is_nan());
        }

        // Test specific signals
        assert_eq!(output_signals[14], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-10 14:05
        assert_eq!(output_signals[27], Signal::Bearish.into()); // TV BTCUSDT.P 5m 2025-02-10 05:10
        assert_eq!(output_signals[46], Signal::Bearish.into()); // TV BTCUSDT.P 5m 2025-02-10 06:45
        assert_eq!(output_signals[61], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-10 08:00

        // Now test incremental calculation matches regular calculation
        let mut prev_body_avg = output_body_avg[13]; // First valid body average

        // Test each incremental step
        for i in 14..18 {
            let (signal, new_body_avg) = cdl_marubozu_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                prev_body_avg,
                param_period,
                param_shadow_percent,
            )
            .unwrap();
            assert_eq!(signal, output_signals[i]);
            assert_relative_eq!(new_body_avg, output_body_avg[i], epsilon = 0.00001);
            prev_body_avg = new_body_avg;
        }
    }
}
