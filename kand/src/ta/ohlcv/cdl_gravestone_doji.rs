use crate::{
    KandError, TAFloat, TAInt,
    helper::{lower_shadow_length, real_body_length},
    types::Signal,
};

/// Returns the lookback period required for Gravestone Doji pattern detection.
///
/// # Description
/// The lookback period represents the minimum number of historical data points needed
/// for the indicator to generate valid signals. For Gravestone Doji pattern, only a single
/// candlestick is required.
///
/// # Returns
/// * `Result<usize, KandError>` - Returns `Ok(0)` as the Gravestone Doji pattern only requires a single candlestick
///
/// # Errors
/// This function does not return any errors.
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_gravestone_doji;
/// let lookback = cdl_gravestone_doji::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Detects Gravestone Doji candlestick patterns in price data.
///
/// # Description
/// A Gravestone Doji is a bearish reversal pattern that forms when the opening and closing prices
/// are at or near the low of the day, with a long upper shadow and minimal lower shadow.
///
/// # Calculation
/// 1. Calculate real body length = |close - open|
/// 2. Calculate total range = high - low
/// 3. Calculate lower shadow length = min(open,close) - low
/// 4. Check if body is small relative to range (doji condition)
/// 5. Check if lower shadow is minimal
///
/// # Arguments
/// * `input_open` - Array of opening prices for each period
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `input_close` - Array of closing prices for each period
/// * `param_body_percent` - Maximum body size as percentage of total range to qualify as doji (e.g. 5.0 for 5%)
/// * `output_signals` - Output array that will contain the pattern signals:
///   - -100: Bearish Gravestone Doji pattern detected
///   - 0: No pattern detected
/// # Returns
/// * `Ok(())` - Calculation completed successfully
///
/// # Errors
/// * [`KandError::LengthMismatch`] - If input arrays have different lengths
/// * [`KandError::NaNDetected`] - If any input contains NaN values (when `deep-check` feature enabled)
/// * [`KandError::InvalidParameter`] - If `param_body_percent` is less than or equal to zero
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_gravestone_doji;
///
/// let input_open = vec![100.0, 101.0];
/// let input_high = vec![102.0, 103.0];
/// let input_low = vec![99.0, 98.0];
/// let input_close = vec![99.5, 98.5];
/// let mut output_signals = vec![0i64; 2];
///
/// cdl_gravestone_doji::cdl_gravestone_doji(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     5.0,
///     &mut output_signals,
/// )
/// .unwrap();
/// ```
pub fn cdl_gravestone_doji(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_body_percent: TAFloat,
    output_signals: &mut [TAInt],
) -> Result<(), KandError> {
    let len = input_open.len();

    #[cfg(feature = "check")]
    {
        // Check array lengths
        if len != input_high.len()
            || len != input_low.len()
            || len != input_close.len()
            || len != output_signals.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
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

    // Process each candle
    for i in 0..len {
        output_signals[i] = cdl_gravestone_doji_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            param_body_percent,
        )?;
    }

    Ok(())
}

/// Processes a single candlestick to detect a Gravestone Doji pattern.
///
/// # Description
/// This function performs incremental Gravestone Doji pattern detection on individual candlesticks.
/// A Gravestone Doji forms when the open and close prices are at or near the low, with a long
/// upper shadow and minimal lower shadow.
///
/// # Calculation
/// 1. Calculate real body = |close - open|
/// 2. Calculate total range = high - low
/// 3. Calculate lower shadow = min(open,close) - low
/// 4. Check if:
///    - Body is small relative to range (doji condition)
///    - Lower shadow is minimal
///
/// # Arguments
/// * `input_open` - Opening price of the candlestick
/// * `input_high` - High price of the candlestick
/// * `input_low` - Low price of the candlestick
/// * `input_close` - Closing price of the candlestick
/// * `param_body_percent` - Maximum body size as percentage of total range to qualify as doji
///
/// # Returns
/// * `Ok(TAInt)` - Signal value where:
///   - -100: Bearish Gravestone Doji pattern detected
///   - 0: No pattern detected
///
/// # Errors
/// * [`KandError::InvalidParameter`] - If `param_body_percent` is less than or equal to zero
/// * [`KandError::NaNDetected`] - If any input value is NaN (when `deep-check` feature enabled)
/// * [`KandError::ConversionError`] - If numeric conversion fails
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_gravestone_doji;
///
/// let signal = cdl_gravestone_doji::cdl_gravestone_doji_inc(
///     100.0, // open
///     102.0, // high
///     99.8,  // low
///     99.9,  // close
///     5.0,   // body_percent
/// )
/// .unwrap();
/// ```
pub fn cdl_gravestone_doji_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    param_body_percent: TAFloat,
) -> Result<TAInt, KandError> {
    #[cfg(feature = "deep-check")]
    {
        if input_open.is_nan() || input_high.is_nan() || input_low.is_nan() || input_close.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let body = real_body_length(input_open, input_close);
    let range = input_high - input_low;
    let dn_shadow = lower_shadow_length(input_low, input_open, input_close);

    // Check for Gravestone Doji pattern
    let is_doji_body = range > 0.0 && body <= range * param_body_percent / 100.0;
    let has_minimal_lower_shadow = dn_shadow <= body;

    let output_signal = if is_doji_body && has_minimal_lower_shadow {
        Signal::Bearish.into()
    } else {
        Signal::Neutral.into()
    };

    Ok(output_signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdl_gravestone_doji() {
        let input_open = vec![
            102_730.6, 102_233.5, 102_003.4, 102_330.6, 102_211.8, 102_994.9, 102_817.5, 102_407.9,
            102_525.3, 103_002.3, 102_826.2, 102_499.1, 102_161.0, 102_033.9, 102_191.6, 102_358.0,
            102_368.7, 102_354.8, 101_928.3, 101_923.5, 101_226.3,
        ];
        let input_high = vec![
            102_870.3, 102_421.8, 102_374.7, 102_543.5, 103_065.9, 103_059.4, 102_899.3, 102_713.6,
            103_089.9, 103_083.5, 102_914.6, 102_510.8, 102_204.8, 102_366.8, 102_358.1, 102_624.0,
            102_495.0, 102_354.9, 102_115.4, 101_933.7, 101_449.1,
        ];
        let input_low = vec![
            102_205.0, 101_850.0, 101_984.1, 101_921.2, 102_170.3, 102_700.0, 102_301.0, 102_308.3,
            102_336.7, 102_733.2, 102_435.0, 102_123.9, 101_778.0, 101_929.2, 101_994.0, 102_357.9,
            102_241.4, 101_921.8, 101_852.7, 101_195.2, 101_056.1,
        ];
        let input_close = vec![
            102_233.4, 102_003.3, 102_330.6, 102_211.8, 102_994.8, 102_817.5, 102_407.8, 102_525.2,
            103_002.3, 102_826.3, 102_499.2, 102_161.1, 102_033.8, 102_191.6, 102_358.0, 102_368.7,
            102_354.8, 101_928.4, 101_923.6, 101_226.3, 101_260.0,
        ];

        let param_body_percent = 5.0;
        let mut output_signals = vec![0i64; input_open.len()];

        cdl_gravestone_doji(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            param_body_percent,
            &mut output_signals,
        )
        .unwrap();

        println!("output_signals: {output_signals:?}");

        // Test specific signals
        assert_eq!(output_signals[15], Signal::Bearish.into()); // TV BTCUSDT.P 5m 2025-01-29 03:45

        // Test incremental calculation matches regular calculation
        for i in 0..18 {
            let output_signal: i64 = cdl_gravestone_doji_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                param_body_percent,
            )
            .unwrap();
            assert_eq!(output_signal, output_signals[i]);
        }
    }
}
