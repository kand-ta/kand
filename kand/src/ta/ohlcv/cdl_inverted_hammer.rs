use crate::{
    KandError, TAFloat, TAInt,
    helper::{lower_shadow_length, period_to_k, real_body_length, upper_shadow_length},
    types::Signal,
};

/// Returns the required lookback period for Inverted Hammer pattern detection.
///
/// # Description
/// Calculates the minimum number of historical data points needed to generate valid signals.
/// For Inverted Hammer pattern with EMA of body sizes, this is `param_period` - 1.
///
/// # Arguments
/// * `param_period` - The period used for calculating the exponential moving average (EMA) of candle body sizes.
///   Must be >= 2.
///
/// # Returns
/// * `Ok(usize)` - The required lookback period (`param_period` - 1)
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_inverted_hammer;
///
/// let param_period = 14;
/// let lookback = cdl_inverted_hammer::lookback(param_period).unwrap();
/// assert_eq!(lookback, 13);
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

/// Detects Inverted Hammer candlestick patterns in price data.
///
/// # Description
/// An Inverted Hammer is a bullish reversal candlestick pattern that forms during a downtrend.
/// The pattern has a small real body near the bottom of the trading range with a long upper shadow
/// and little to no lower shadow.
///
/// # Calculation
/// For each candlestick:
/// 1. Calculate real body length = |close - open|
/// 2. Calculate upper shadow = high - max(open, close)
/// 3. Calculate lower shadow = min(open, close) - low
/// 4. Calculate EMA of body sizes using `param_period`
/// 5. Check pattern conditions:
///    - Small body: body <= `body_avg` && body > 0
///    - Long upper shadow: `upper_shadow` >= `param_factor` * body
///    - Minimal lower shadow: `lower_shadow` <= body
///    - Body in lower half: max(open, close) < (high + low)/2
///
/// # Arguments
/// * `input_open` - Array of opening prices
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - Period for EMA calculation of body sizes (typically 14)
/// * `param_factor` - Minimum ratio of upper shadow to body length (typically 2.0)
/// * `output_signals` - Output array for pattern signals:
///   - 1: Bullish Inverted Hammer detected
///   - 0: No pattern detected
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
/// use kand::ohlcv::cdl_inverted_hammer;
///
/// let input_open = vec![100.0, 101.0, 102.0];
/// let input_high = vec![105.0, 106.0, 107.0];
/// let input_low = vec![99.0, 100.0, 101.0];
/// let input_close = vec![101.0, 102.0, 103.0];
/// let mut output_signals = vec![0; 3];
/// let mut output_body_avg = vec![0.0; 3];
///
/// cdl_inverted_hammer::cdl_inverted_hammer(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     2,
///     2.0,
///     &mut output_signals,
///     &mut output_body_avg,
/// )
/// .unwrap();
/// ```
pub fn cdl_inverted_hammer(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_period: usize,
    param_factor: TAFloat,
    output_signals: &mut [TAInt],
    output_body_avg: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_open.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        if len != input_high.len() || len != input_low.len() || len != input_close.len() {
            return Err(KandError::LengthMismatch);
        }
        if len != output_signals.len() || len != output_body_avg.len() {
            return Err(KandError::LengthMismatch);
        }
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
        if param_factor <= 0.0 {
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
    for i in 0..param_period {
        sum += real_body_length(input_open[i], input_close[i]);
    }
    let mut body_avg = sum / param_period as TAFloat;
    output_body_avg[lookback] = body_avg;

    // Process remaining candles
    for i in lookback..len {
        let (signal, new_body_avg) = cdl_inverted_hammer_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            body_avg,
            param_period,
            param_factor,
        )?;
        output_signals[i] = signal;
        output_body_avg[i] = new_body_avg;
        body_avg = new_body_avg;
    }

    // Fill initial values with -1
    for i in 0..lookback {
        output_signals[i] = Signal::Invalid.into();
        output_body_avg[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Processes a single candlestick to detect an Inverted Hammer pattern.
///
/// # Description
/// Calculates the pattern signal and updated EMA of body sizes for a single candlestick.
/// This incremental version is useful for real-time processing of streaming data.
///
/// # Calculation
/// 1. Calculate real body, upper shadow and lower shadow lengths
/// 2. Update EMA of body sizes using: EMA = (body - `prev_ema`) * k + `prev_ema`
/// 3. Check pattern conditions:
///    - Small body: body <= `body_avg` && body > 0
///    - Long upper shadow: `upper_shadow` >= `param_factor` * body
///    - Minimal lower shadow: `lower_shadow` <= body
///    - Body in lower half: max(open, close) < (high + low)/2
///
/// # Arguments
/// * `input_open` - Opening price of the candlestick
/// * `input_high` - High price of the candlestick
/// * `input_low` - Low price of the candlestick
/// * `input_close` - Closing price of the candlestick
/// * `prev_body_avg` - Previous EMA value of body sizes
/// * `param_period` - Period for EMA calculation
/// * `param_factor` - Minimum ratio of upper shadow to body length
///
/// # Returns
/// * `Ok((TAInt, TAFloat))` - Tuple containing:
///   - First element: Pattern signal (100 for bullish inverted hammer, 0 for no pattern)
///   - Second element: Updated EMA value of body sizes
///
/// # Errors
/// * [`KandError::InvalidParameter`] - If parameters are invalid:
///   - `param_period` is less than 2
///   - `param_factor` is less than or equal to zero
/// * [`KandError::NaNDetected`] - If any input value is NaN (when `check-nan` enabled)
/// * [`KandError::ConversionError`] - If numeric conversion fails
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_inverted_hammer;
///
/// let (signal, new_body_avg) = cdl_inverted_hammer::cdl_inverted_hammer_inc(
///     100.0, // open
///     105.0, // high
///     99.5,  // low
///     100.5, // close
///     1.2,   // previous body average
///     14,    // period
///     2.0,   // factor
/// )
/// .unwrap();
/// ```
pub fn cdl_inverted_hammer_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    prev_body_avg: TAFloat,
    param_period: usize,
    param_factor: TAFloat,
) -> Result<(TAInt, TAFloat), KandError> {
    #[cfg(feature = "check-nan")]
    {
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
    let k = period_to_k(param_period)?;
    let body_avg = (body - prev_body_avg).mul_add(k, prev_body_avg);

    // Check for Inverted Hammer pattern
    let is_small_body = body <= body_avg && body > 0.0;
    let has_long_upper_shadow = up_shadow >= param_factor * body;
    let has_minimal_lower_shadow = down_shadow <= body;
    let body_in_lower_half =
        TAFloat::max(input_open, input_close) < f64::midpoint(input_high, input_low);

    let signal =
        if is_small_body && has_long_upper_shadow && has_minimal_lower_shadow && body_in_lower_half
        {
            Signal::Bullish.into()
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
    fn test_cdl_inverted_hammer() {
        let input_open = vec![
            96470.4, 96415.2, 96386.1, 96259.3, 96290.1, 96307.4, 96266.7, 96200.0, 96078.4,
            96175.2, 96123.1, 96242.9, 96149.3, 96104.0, 96191.9, 96236.3, 96278.1, 96200.6,
            96164.9, 96113.6, 96095.8, 96051.1, 96085.9, 96074.0, 96092.5, 96052.5, 96067.5,
            96100.0, 96067.1, 96054.1, 95951.3,
        ];
        let input_high = vec![
            96470.5, 96450.0, 96386.1, 96344.7, 96374.1, 96312.5, 96300.0, 96244.8, 96183.1,
            96198.5, 96242.9, 96275.6, 96156.9, 96211.7, 96240.5, 96323.0, 96300.0, 96200.7,
            96165.0, 96162.2, 96118.1, 96107.6, 96086.0, 96183.8, 96095.7, 96102.0, 96125.7,
            96120.5, 96110.5, 96054.1, 96043.7,
        ];
        let input_low = vec![
            96388.0, 96370.4, 96168.8, 96217.2, 96286.2, 96261.5, 96111.0, 96061.2, 96078.4,
            96070.4, 96079.1, 96138.3, 96008.7, 96052.1, 96169.4, 96236.2, 96200.6, 96137.0,
            96050.0, 96095.7, 96050.0, 96045.9, 96006.1, 96074.0, 96032.1, 96047.0, 96050.0,
            96060.3, 96054.0, 95835.8, 95900.0,
        ];
        let input_close = vec![
            96415.2, 96386.1, 96259.3, 96290.2, 96307.3, 96266.8, 96200.0, 96078.3, 96175.3,
            96123.2, 96242.9, 96149.3, 96104.1, 96192.0, 96236.3, 96278.1, 96200.6, 96165.0,
            96113.6, 96095.8, 96051.2, 96086.0, 96074.0, 96092.6, 96052.5, 96067.6, 96100.0,
            96067.0, 96054.0, 95951.4, 95951.5,
        ];

        let param_period = 14;
        let param_factor = 2.0;
        let mut output_signals = vec![0i64; input_open.len()];
        let mut output_body_avg = vec![0.0; input_open.len()];

        cdl_inverted_hammer(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            param_period,
            param_factor,
            &mut output_signals,
            &mut output_body_avg,
        )
        .unwrap();

        // First 13 values should be -1
        for i in 0..13 {
            assert_eq!(output_signals[i], -1);
            assert!(output_body_avg[i].is_nan());
        }

        println!("output_signals: {output_signals:?}");
        println!("output_body_avg: {output_body_avg:?}");

        // Test specific signals
        assert_eq!(output_signals[19], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-08 14:05
        assert_eq!(output_signals[23], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-08 14:25
        assert_eq!(output_signals[25], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-08 14:35
        assert_eq!(output_signals[28], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-08 14:50

        // Test incremental calculation matches regular calculation
        let mut prev_body_avg = output_body_avg[13]; // First valid body average

        // Test each incremental step
        for i in 14..18 {
            let (signal, new_body_avg) = cdl_inverted_hammer_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                prev_body_avg,
                param_period,
                param_factor,
            )
            .unwrap();
            assert_eq!(signal, output_signals[i]);
            assert_relative_eq!(new_body_avg, output_body_avg[i], epsilon = 0.00001);
            prev_body_avg = new_body_avg;
        }
    }
}
