use crate::{
    KandError, TAFloat, TAInt,
    helper::{lower_shadow_length, period_to_k, real_body_length, upper_shadow_length},
    types::Signal,
};

/// Returns the lookback period for Hammer pattern detection.
///
/// # Description
/// Calculates the minimum number of historical data points needed to generate valid signals.
/// For Hammer pattern, this equals `param_period - 1` to ensure proper EMA calculation of candle body sizes.
///
/// # Arguments
/// * `param_period` - The period used for EMA calculation of candle body sizes
///
/// # Returns
/// * `Ok(usize)` - The required lookback period
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_period` is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_hammer;
/// let lookback = cdl_hammer::lookback(14).unwrap();
/// assert_eq!(lookback, 13);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Detects Hammer candlestick patterns in price data.
///
/// # Description
/// A Hammer is a bullish reversal pattern that forms during downtrends, characterized by a small real body
/// near the high and a long lower shadow, suggesting buyers regained control after initial selling pressure.
///
/// # Mathematical Formula
/// ```text
/// Body = |Close - Open|
/// UpperShadow = High - max(Open, Close)
/// LowerShadow = min(Open, Close) - Low
/// BodyAvg[i] = BodyAvg[i-1] + k * (Body[i] - BodyAvg[i-1])
/// where k = 2/(period + 1)
///
/// Conditions for Hammer:
/// 1. Body <= BodyAvg && Body > 0
/// 2. LowerShadow >= param_factor * Body
/// 3. UpperShadow <= Body
/// 4. min(Open, Close) > (High + Low)/2
/// ```
///
/// # Arguments
/// * `input_open` - Array of opening prices
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_period` - Period for EMA calculation of body sizes
/// * `param_factor` - Minimum ratio of lower shadow to body length
/// * `output_signals` - Output array for pattern signals:
///   - 1: Bullish Hammer detected
///   - 0: No pattern detected
/// * `output_body_avg` - Output array storing EMA values of candle body sizes
///
/// # Returns
/// * `Ok(())` - Calculation completed successfully
///
/// # Errors
/// * [`KandError::LengthMismatch`] - If input arrays have different lengths
/// * [`KandError::InvalidParameter`] - If parameter values are invalid
/// * [`KandError::InsufficientData`] - If input length is less than required lookback
/// * [`KandError::NaNDetected`] - If input contains NaN values (when `deep-check` enabled)
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_hammer;
/// let input_open = vec![10.0, 11.0, 10.5];
/// let input_high = vec![12.0, 11.5, 11.0];
/// let input_low = vec![9.0, 10.0, 9.5];
/// let input_close = vec![11.0, 10.5, 10.8];
/// let mut output = vec![0i64; 3];
/// let mut body_avg = vec![0.0; 3];
/// cdl_hammer::cdl_hammer(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     2,
///     2.0,
///     &mut output,
///     &mut body_avg,
/// )
/// .unwrap();
/// ```
pub fn cdl_hammer(
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
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Data sufficiency check
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
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if param_factor <= 0.0 {
            return Err(KandError::InvalidParameter);
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

    // Calculate initial SMA
    let mut sum = 0.0;
    for i in 0..param_period {
        sum += real_body_length(input_open[i], input_close[i]);
    }
    let mut body_avg = sum / param_period as TAFloat;
    output_body_avg[lookback] = body_avg;

    // Process remaining candles
    for i in lookback..len {
        let (signal, new_body_avg) = cdl_hammer_inc(
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

/// Incrementally processes a single candlestick for Hammer pattern detection.
///
/// # Description
/// Calculates the pattern signal and updated EMA value for a single candlestick,
/// using the previous EMA value of body sizes for comparison.
///
/// # Arguments
/// * `input_open` - Opening price of current candlestick
/// * `input_high` - High price of current candlestick
/// * `input_low` - Low price of current candlestick
/// * `input_close` - Closing price of current candlestick
/// * `prev_body_avg` - Previous EMA value of body sizes
/// * `param_period` - Period for EMA calculation
/// * `param_factor` - Minimum ratio of lower shadow to body length
///
/// # Returns
/// * `Ok((TAInt, TAFloat))` - Returns tuple containing:
///   - First element: Pattern signal (100 for bullish hammer, 0 for no pattern)
///   - Second element: Updated EMA value of body sizes
///
/// # Errors
/// * [`KandError::InvalidParameter`] - If parameters are invalid:
///   - `param_period` is less than 2
///   - `param_factor` is less than or equal to zero
/// * [`KandError::NaNDetected`] - If any input value is NaN (when `deep-check` enabled)
/// * [`KandError::ConversionError`] - If numeric conversion fails
///
/// # Examples
/// ```
/// use kand::ohlcv::cdl_hammer;
/// let (signal, body_avg) = cdl_hammer::cdl_hammer_inc(
///     10.0, // open
///     11.0, // high
///     9.0,  // low
///     10.5, // close
///     0.5,  // prev_body_avg
///     14,   // period
///     2.0,  // factor
/// )
/// .unwrap();
/// ```
pub fn cdl_hammer_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    prev_body_avg: TAFloat,
    param_period: usize,
    param_factor: TAFloat,
) -> Result<(TAInt, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if param_factor <= 0.0 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
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

    // Check for Hammer pattern
    let is_small_body = body <= body_avg && body > 0.0;
    let has_long_lower_shadow = down_shadow >= param_factor * body;
    let has_minimal_upper_shadow = up_shadow <= body;
    let body_in_upper_half =
        TAFloat::min(input_open, input_close) > f64::midpoint(input_high, input_low);

    let signal =
        if is_small_body && has_long_lower_shadow && has_minimal_upper_shadow && body_in_upper_half
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
    fn test_cdl_hammer() {
        let input_open = vec![
            97798.1, 96982.9, 97050.5, 97281.3, 97480.7, 98310.4, 98232.0, 98473.2, 98136.9,
            97912.7, 97759.0, 97516.4, 96913.4, 96738.1, 96999.0, 97472.5, 97368.3, 97140.0,
            97971.6, 97684.9, 96985.2, 97298.6, 97664.5, 97286.7, 97041.2, 95591.8, 96464.5,
            95750.1, 95132.6, 94132.8, 93408.4, 94009.0, 93876.7, 93847.5, 93539.2, 94308.4,
            94403.9, 93820.9, 94001.6, 93880.0, 93317.1, 92969.9, 92765.2, 92874.4, 93562.1,
            93583.3, 94171.9, 93910.0, 94387.7, 93965.3, 93872.7, 93974.7, 94162.4, 94518.5,
            95271.1, 95354.0, 95340.9, 94978.8, 95281.6, 95742.5, 95829.2, 95680.3, 95227.2,
        ];
        let input_high = vec![
            97833.9, 97420.1, 97562.1, 97550.0, 98371.8, 98667.2, 98594.9, 98523.7, 98216.5,
            97912.7, 97947.4, 97582.8, 97294.2, 97051.5, 97683.0, 97700.0, 97368.3, 97999.0,
            97985.8, 97897.9, 97608.8, 97755.5, 97748.0, 97570.0, 97167.0, 96869.7, 96875.7,
            95825.8, 95430.9, 94150.6, 94282.2, 94983.2, 94159.7, 94052.2, 94577.2, 94645.4,
            94473.2, 94217.7, 94461.0, 94102.9, 93525.7, 93480.0, 93168.0, 93650.0, 93931.3,
            94249.5, 94204.0, 94421.0, 94578.5, 94237.2, 94162.8, 94303.4, 94662.1, 95373.1,
            95354.1, 95525.7, 95582.9, 95380.6, 95830.0, 95891.9, 95877.8, 95713.6, 95380.0,
        ];
        let input_low = vec![
            96750.1, 96760.0, 96759.1, 96985.1, 97469.9, 97982.8, 98161.2, 98043.2, 97780.9,
            97618.2, 97481.4, 96880.4, 96520.0, 96576.3, 96948.1, 97131.8, 96029.6, 97023.7,
            97130.0, 96500.0, 96716.2, 97273.0, 97226.8, 96006.0, 95325.6, 95539.0, 95740.3,
            95006.0, 93750.0, 91800.1, 91130.3, 93547.0, 93170.9, 93290.4, 93400.0, 94239.3,
            93732.0, 93748.6, 93725.6, 93118.8, 92777.9, 92595.7, 92739.1, 92843.5, 93430.5,
            93520.1, 93700.9, 93781.1, 93924.6, 93605.1, 93805.7, 93974.7, 93964.4, 94404.9,
            94852.4, 95011.0, 94793.8, 94914.3, 95211.9, 95225.3, 95476.5, 95180.0, 95081.1,
        ];
        let input_close = vec![
            96977.5, 97050.5, 97281.2, 97480.7, 98310.3, 98232.0, 98473.2, 98136.8, 97912.7,
            97759.1, 97516.4, 96913.4, 96738.0, 96998.8, 97472.6, 97368.3, 97140.0, 97971.6,
            97684.9, 96985.3, 97298.6, 97664.5, 97287.8, 97041.2, 95591.8, 96464.6, 95750.2,
            95132.6, 94132.8, 93408.4, 94009.0, 93876.7, 93847.5, 93539.1, 94308.4, 94403.8,
            93820.9, 94001.6, 93879.9, 93317.1, 92969.8, 92765.2, 92874.4, 93562.1, 93583.4,
            94171.9, 93909.9, 94387.6, 93965.3, 93872.7, 93974.6, 94162.3, 94518.6, 95271.1,
            95354.1, 95340.9, 94978.9, 95281.5, 95742.6, 95829.2, 95680.2, 95227.2, 95223.8,
        ];

        let param_period = 14;
        let param_factor = 2.0;
        let mut output_signals = vec![0; input_open.len()];
        let mut output_body_avg = vec![0.0; input_open.len()];

        cdl_hammer(
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
            assert_eq!(output_signals[i], Signal::Invalid.into());
            assert!(output_body_avg[i].is_nan());
        }

        // Test specific signals
        assert_eq!(output_signals[16], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-03 06:30
        assert_eq!(output_signals[54], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-03 16:00
        assert_eq!(output_signals[59], Signal::Bullish.into()); // TV BTCUSDT.P 5m 2025-02-03 17:15

        // Test incremental calculation matches regular calculation
        let mut prev_body_avg = output_body_avg[13]; // First valid body average

        // Test each incremental step
        for i in 14..18 {
            let (signal, new_body_avg) = cdl_hammer_inc(
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
