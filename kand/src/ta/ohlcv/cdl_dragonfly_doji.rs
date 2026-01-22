use crate::{
    KandError, TAFloat, TAInt,
    helper::{real_body_length, upper_shadow_length},
    types::Signal,
};

/// Returns the lookback period required for Dragonfly Doji pattern detection.
///
/// # Description
/// The lookback period is the minimum number of data points needed before the indicator
/// can produce valid output values. For Dragonfly Doji pattern, only a single candlestick
/// is required.
///
/// # Returns
/// * `Result<usize, KandError>` - Always returns `Ok(0)` as the pattern requires a single candlestick
///
/// # Errors
/// This function does not return any errors. It always returns `Ok(0)`.
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_dragonfly_doji;
/// let lookback = cdl_dragonfly_doji::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Detects Dragonfly Doji candlestick patterns in price data.
///
/// # Description
/// A Dragonfly Doji is a candlestick pattern that suggests a potential bullish reversal.
/// The pattern is characterized by:
/// - Opening and closing prices that are very close or equal and near the high
/// - Little to no upper shadow (the part above the body)
/// - Long lower shadow (the part below the body)
///
/// # Calculation
/// 1. Calculate the real body length (absolute difference between open and close)
/// 2. Calculate the total range (high - low)
/// 3. Calculate the upper shadow length
/// 4. Check if:
///    - Body is small relative to range (using `opt_body_percent`)
///    - Upper shadow is minimal (less than or equal to body length)
///
/// # Arguments
/// * `input_open` - Array of opening prices for each period
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `input_close` - Array of closing prices for each period
/// * `opt_body_percent` - Maximum body size as percentage of total range (typically 5%)
/// * `output_signals` - Output array that will contain the pattern signals:
///   - 100: Bullish Dragonfly Doji pattern detected
///   - 0: No pattern detected
///
/// # Returns
/// * `Ok(())` - Calculation completed successfully
///
/// # Errors
/// * [`KandError::LengthMismatch`] - If input arrays have different lengths
/// * [`KandError::InvalidParameter`] - If `opt_body_percent` is less than or equal to zero
/// * [`KandError::NaNDetected`] - If any input contains NaN values (when `check-nan` feature enabled)
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_dragonfly_doji;
///
/// let input_open = vec![100.0, 101.0, 102.0];
/// let input_high = vec![102.0, 103.0, 104.0];
/// let input_low = vec![98.0, 99.0, 100.0];
/// let input_close = vec![101.0, 102.0, 103.0];
/// let mut output_signals = vec![0i64; 3];
///
/// cdl_dragonfly_doji::cdl_dragonfly_doji(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     5.0,
///     &mut output_signals,
/// )
/// .unwrap();
/// ```
pub fn cdl_dragonfly_doji(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    opt_body_percent: TAFloat,
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

        // Check parameters
        if opt_body_percent <= 0.0 {
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

    // Process each candle
    for i in 0..len {
        output_signals[i] = cdl_dragonfly_doji_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            opt_body_percent,
        )?;
    }

    Ok(())
}

/// Processes a single candlestick to detect a Dragonfly Doji pattern.
///
/// # Description
/// This function analyzes an individual candlestick to determine if it forms a Dragonfly Doji pattern.
/// It's useful for real-time analysis or when processing candlesticks one at a time.
///
/// # Calculation
/// 1. Calculate real body length and total range
/// 2. Check if body is small relative to range (using `opt_body_percent`)
/// 3. Verify upper shadow is minimal
/// 4. Return appropriate signal value
///
/// # Arguments
/// * `input_open` - Opening price of the candlestick
/// * `input_high` - High price of the candlestick
/// * `input_low` - Low price of the candlestick
/// * `input_close` - Closing price of the candlestick
/// * `opt_body_percent` - Maximum body size as percentage of total range
///
/// # Returns
/// * `Ok(TAInt)` - Signal value where:
///   - 100: Bullish Dragonfly Doji pattern detected
///   - 0: No pattern detected
///
/// # Errors
/// * [`KandError::InvalidParameter`] - If `opt_body_percent` is less than or equal to zero
/// * [`KandError::NaNDetected`] - If any input value is NaN (when `check-nan` feature enabled)
/// * [`KandError::ConversionError`] - If numeric conversion fails
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_dragonfly_doji;
///
/// let signal = cdl_dragonfly_doji::cdl_dragonfly_doji_inc(
///     100.0, // input_open
///     102.0, // input_high
///     98.0,  // input_low
///     100.1, // input_close
///     5.0,   // opt_body_percent
/// )
/// .unwrap();
/// ```
pub fn cdl_dragonfly_doji_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    opt_body_percent: TAFloat,
) -> Result<TAInt, KandError> {
    #[cfg(feature = "check")]
    {
        // Check parameters
        if opt_body_percent <= 0.0 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_open.is_nan() || input_high.is_nan() || input_low.is_nan() || input_close.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let body = real_body_length(input_open, input_close);
    let range = input_high - input_low;
    let up_shadow = upper_shadow_length(input_high, input_open, input_close);

    // Check for Dragonfly Doji pattern
    let is_doji_body = range > 0.0 && body <= range * opt_body_percent / 100.0;
    let has_minimal_upper_shadow = up_shadow <= body;

    let signal = if is_doji_body && has_minimal_upper_shadow {
        Signal::Bullish.into()
    } else {
        Signal::Neutral.into()
    };

    Ok(signal)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_cdl_dragonfly_doji() {
        let input_open = vec![
            97285.7, 97486.5, 97009.3, 96554.9, 96542.5, 96450.1, 96772.8, 96797.0, 96662.7,
            96252.2, 96131.1, 96364.3, 96274.1, 96448.0, 96408.3, 95960.1, 95946.0, 96238.8,
            96358.5, 96770.6, 96884.2, 96613.9, 96489.0, 96710.1, 96779.9, 96149.6, 96548.1,
            96560.0, 96923.0, 96567.1, 96571.7, 96341.5, 96515.0, 96720.2, 96746.1, 96461.1,
            96460.9, 96735.0, 96679.9, 96759.9, 97350.8, 97216.6, 97346.4, 97419.9, 97534.2,
            97521.6,
        ];
        let input_high = vec![
            97500.0, 97500.0, 97076.1, 96754.1, 96826.5, 96795.0, 97154.2, 96936.7, 96797.1,
            96415.7, 96430.0, 96539.7, 96530.5, 96883.1, 96412.7, 96161.9, 96327.2, 96408.3,
            96781.0, 97041.4, 96913.2, 96696.8, 96730.7, 96827.7, 96794.7, 96577.5, 96560.0,
            96923.0, 96923.0, 96638.4, 96634.5, 96576.4, 96896.7, 96896.5, 96788.3, 96563.4,
            96815.0, 96822.3, 96835.0, 97805.8, 97561.9, 97473.4, 97480.0, 97586.0, 97727.7,
            97639.8,
        ];
        let input_low = vec![
            97147.7, 96845.8, 96536.0, 96337.2, 96330.0, 96440.0, 96592.4, 96662.7, 96220.0,
            96111.0, 95811.1, 96161.5, 95880.1, 96390.5, 95860.0, 95613.5, 95736.0, 96093.4,
            96337.3, 96650.8, 96609.1, 96313.0, 96050.4, 96522.0, 96036.0, 96130.0, 96313.1,
            96410.4, 96548.1, 96439.6, 96161.1, 96311.8, 96488.5, 96611.9, 96446.1, 96358.3,
            96456.2, 96600.0, 96508.0, 96700.0, 97150.0, 97021.3, 97290.0, 97333.5, 97411.4,
            97355.0,
        ];
        let input_close = vec![
            97486.5, 97009.3, 96555.0, 96542.5, 96450.1, 96772.8, 96796.9, 96662.7, 96252.3,
            96131.1, 96364.4, 96274.1, 96447.8, 96408.3, 95960.1, 95946.1, 96238.8, 96359.0,
            96770.6, 96884.2, 96613.9, 96489.1, 96710.0, 96780.0, 96149.6, 96548.1, 96560.0,
            96923.0, 96567.2, 96571.7, 96341.5, 96515.1, 96720.2, 96746.1, 96461.0, 96460.9,
            96735.0, 96679.9, 96759.9, 97350.9, 97216.7, 97346.3, 97419.9, 97534.2, 97521.5,
            97384.1,
        ];

        let opt_body_percent = 5.0;
        let mut output_signals = vec![0i64; input_open.len()];

        cdl_dragonfly_doji(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            opt_body_percent,
            &mut output_signals,
        )
        .unwrap();

        println!("output_signals: {output_signals:?}");

        // Test specific signals
        assert_eq!(output_signals[26], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-07 06:30

        // Test incremental calculation matches regular calculation
        for i in 0..18 {
            let signal: i64 = cdl_dragonfly_doji_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                opt_body_percent,
            )
            .unwrap();
            assert_eq!(signal, output_signals[i]);
        }
    }
}
