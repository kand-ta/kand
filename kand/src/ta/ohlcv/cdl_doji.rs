use crate::{
    KandError, TAFloat, TAInt,
    helper::{lower_shadow_length, real_body_length, upper_shadow_length},
    types::Signal,
};

/// Returns the lookback period for Doji pattern detection.
///
/// # Description
/// The lookback period represents the minimum number of data points needed before
/// valid pattern detection can begin. For Doji patterns, no lookback is required
/// since each candle is evaluated independently.
///
/// # Returns
/// * `Result<usize, KandError>` - Returns 0 as no lookback is needed
///
/// # Errors
/// This function does not return any errors. It always returns `Ok(0)`.
///
/// # Example
/// ```
/// use kand::ohlcv::cdl_doji;
/// let lookback = cdl_doji::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Detects Doji candlestick patterns in price data.
///
/// # Description
/// A Doji pattern indicates market indecision, occurring when opening and closing prices
/// are nearly equal with upper and lower shadows of similar length.
///
/// # Mathematical Formula
/// ```text
/// Body = |Close - Open|
/// Range = High - Low
/// UpperShadow = High - max(Open, Close)
/// LowerShadow = min(Open, Close) - Low
///
/// IsDoji = Body <= Range * BodyPercent/100
/// ShadowDiff = min(|Upper-Lower|/Lower, |Lower-Upper|/Upper) * 100
/// ShadowsEqual = ShadowDiff < ShadowEqualPercent
/// ```
///
/// # Calculation Steps
/// 1. Calculate real body length and total range
/// 2. Check if body is small relative to range
/// 3. Calculate upper and lower shadow lengths
/// 4. Compare shadow lengths for equality
/// 5. Generate signal if both conditions met
///
/// # Arguments
/// * `input_open` - Array of opening prices
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of closing prices
/// * `param_body_percent` - Maximum body size as percentage of range (e.g. 5.0 for 5%)
/// * `param_shadow_equal_percent` - Maximum shadow length difference percentage (e.g. 100.0)
/// * `output_signals` - Output array for pattern signals
///
/// # Returns
/// * `Result<(), KandError>` - Empty result on success, or error on failure
///
/// # Errors
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InvalidParameter` - If any parameter is invalid (e.g. <= 0)
/// * `KandError::NaNDetected` - If any input contains NaN values (when `check-nan` enabled)
///
/// # Example
/// ```
/// use kand::ta::ohlcv::cdl_doji;
///
/// let input_open = vec![10.0, 10.5, 10.2];
/// let input_high = vec![11.0, 11.2, 10.8];
/// let input_low = vec![9.8, 10.1, 9.9];
/// let input_close = vec![10.3, 10.4, 10.25];
/// let mut output_signals = vec![0i64; 3];
///
/// cdl_doji::cdl_doji(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     5.0,   // param_body_percent
///     100.0, // param_shadow_equal_percent
///     &mut output_signals,
/// )
/// .unwrap();
/// ```
pub fn cdl_doji(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    param_body_percent: TAFloat,
    param_shadow_equal_percent: TAFloat,
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
        if param_body_percent <= 0.0 || param_shadow_equal_percent <= 0.0 {
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
        output_signals[i] = cdl_doji_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            param_body_percent,
            param_shadow_equal_percent,
        )?;
    }

    Ok(())
}

/// Processes a single candlestick to detect a Doji pattern.
///
/// # Description
/// This function provides an optimized way to detect Doji patterns by analyzing
/// individual candlesticks. It evaluates the body size relative to range and
/// compares upper/lower shadow lengths.
///
/// # Mathematical Formula
/// ```text
/// Body = |Close - Open|
/// Range = High - Low
/// UpperShadow = High - max(Open, Close)
/// LowerShadow = min(Open, Close) - Low
///
/// IsDoji = Body <= Range * BodyPercent/100
/// ShadowDiff = min(|Upper-Lower|/Lower, |Lower-Upper|/Upper) * 100
/// ShadowsEqual = ShadowDiff < ShadowEqualPercent
/// ```
///
/// # Arguments
/// * `input_open` - Opening price of the candlestick
/// * `input_high` - High price of the candlestick
/// * `input_low` - Low price of the candlestick
/// * `input_close` - Closing price of the candlestick
/// * `param_body_percent` - Maximum body size as percentage of range
/// * `param_shadow_equal_percent` - Maximum allowed difference between shadow lengths
///
/// # Returns
/// * `Result<TAInt, KandError>` - Signal value (Pattern for Doji, Neutral for no pattern)
///
/// # Errors
/// * `KandError::InvalidParameter` - If any parameter is invalid (e.g. <= 0)
/// * `KandError::NaNDetected` - If any input value is NaN (when `check-nan` enabled)
/// * `KandError::ConversionError` - If numeric conversion fails
pub fn cdl_doji_inc(
    input_open: TAFloat,
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    param_body_percent: TAFloat,
    param_shadow_equal_percent: TAFloat,
) -> Result<TAInt, KandError> {
    #[cfg(feature = "check")]
    {
        // Check parameters
        if param_body_percent <= 0.0 || param_shadow_equal_percent <= 0.0 {
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
    let dn_shadow = lower_shadow_length(input_low, input_open, input_close);

    // Check for Doji pattern
    let is_doji_body = range > 0.0 && body <= range * param_body_percent / 100.0;

    // Calculates the percentage difference between upper and lower shadows.
    // Returns the minimum relative difference to provide a more balanced comparison.
    //
    // The calculation is performed in two ways:
    // 1. (|upper - lower| / lower) * 100: difference relative to lower shadow
    // 2. (|lower - upper| / upper) * 100: difference relative to upper shadow
    //
    // The minimum of these two percentages is used because:
    // - It provides the most conservative estimate of shadow inequality
    // - It prevents bias from choice of baseline in relative calculations
    // - It better handles cases where shadows have significant size differences
    //
    // Returns 100% if either shadow length is zero, indicating maximum inequality.
    let shadow_diff_percent = if dn_shadow > 0.0 && up_shadow > 0.0 {
        let up_diff = (up_shadow - dn_shadow).abs() / dn_shadow * 100.0;
        let dn_diff = (dn_shadow - up_shadow).abs() / up_shadow * 100.0;
        up_diff.min(dn_diff)
    } else {
        100.0
    };

    let shadows_equal = shadow_diff_percent < param_shadow_equal_percent;

    Ok(if is_doji_body && shadows_equal {
        Signal::Pattern.into()
    } else {
        Signal::Neutral.into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdl_doji() {
        let input_open = vec![
            98826.3, 98554.6, 98610.4, 98508.0, 98314.1, 98245.0, 98214.2, 98486.1, 98563.5,
            98419.9, 98249.6, 98074.7, 97797.1, 97925.6, 97544.2, 97140.3, 97285.7, 97486.5,
            97009.3, 96554.9, 96542.5, 96450.1, 96772.8, 96797.0, 96662.7, 96252.2, 96131.1,
            96364.3, 96274.1, 96448.0, 96408.3, 95960.1, 95946.0, 96238.8, 96358.5, 96770.6,
            96884.2, 96613.9, 96489.0, 96710.1, 96779.9, 96149.6, 96548.1, 96560.0, 96923.0,
            96567.1, 96571.7, 96341.5, 96515.0, 96720.2, 96746.1, 96461.1, 96460.9, 96735.0,
            96679.9, 96759.9, 97350.8, 97216.6, 97346.4, 97419.9, 97534.2, 97521.6,
        ];
        let input_high = vec![
            99019.9, 98680.0, 98618.7, 98590.0, 98443.9, 98366.8, 98745.5, 98745.6, 98711.4,
            98454.3, 98922.1, 98356.2, 98025.4, 97952.6, 97554.0, 97285.7, 97500.0, 97500.0,
            97076.1, 96754.1, 96826.5, 96795.0, 97154.2, 96936.7, 96797.1, 96415.7, 96430.0,
            96539.7, 96530.5, 96883.1, 96412.7, 96161.9, 96327.2, 96408.3, 96781.0, 97041.4,
            96913.2, 96696.8, 96730.7, 96827.7, 96794.7, 96577.5, 96560.0, 96923.0, 96923.0,
            96638.4, 96634.5, 96576.4, 96896.7, 96896.5, 96788.3, 96563.4, 96815.0, 96822.3,
            96835.0, 97805.8, 97561.9, 97473.4, 97480.0, 97586.0, 97727.7, 97639.8,
        ];
        let input_low = vec![
            98550.0, 98465.0, 98300.0, 98252.3, 98135.6, 98122.0, 98214.2, 98350.0, 98223.3,
            98067.6, 97712.7, 97743.3, 97593.6, 97468.1, 96963.3, 96866.9, 97147.7, 96845.8,
            96536.0, 96337.2, 96330.0, 96440.0, 96592.4, 96662.7, 96220.0, 96111.0, 95811.1,
            96161.5, 95880.1, 96390.5, 95860.0, 95613.5, 95736.0, 96093.4, 96337.3, 96650.8,
            96609.1, 96313.0, 96050.4, 96522.0, 96036.0, 96130.0, 96313.1, 96410.4, 96548.1,
            96439.6, 96161.1, 96311.8, 96488.5, 96611.9, 96446.1, 96358.3, 96456.2, 96600.0,
            96508.0, 96700.0, 97150.0, 97021.3, 97290.0, 97333.5, 97411.4, 97355.0,
        ];
        let input_close = vec![
            98554.6, 98610.4, 98507.9, 98314.1, 98245.0, 98214.1, 98485.8, 98563.5, 98419.9,
            98249.5, 98074.7, 97797.1, 97925.6, 97546.1, 97140.3, 97285.6, 97486.5, 97009.3,
            96555.0, 96542.5, 96450.1, 96772.8, 96796.9, 96662.7, 96252.3, 96131.1, 96364.4,
            96274.1, 96447.8, 96408.3, 95960.1, 95946.1, 96238.8, 96359.0, 96770.6, 96884.2,
            96613.9, 96489.1, 96710.0, 96780.0, 96149.6, 96548.1, 96560.0, 96923.0, 96567.2,
            96571.7, 96341.5, 96515.1, 96720.2, 96746.1, 96461.0, 96460.9, 96735.0, 96679.9,
            96759.9, 97350.9, 97216.7, 97346.3, 97419.9, 97534.2, 97521.5, 97384.1,
        ];

        let param_body_percent = 5.0;
        let param_shadow_equal_percent = 100.0;
        let mut output_signals = vec![0i64; input_open.len()];

        cdl_doji(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            param_body_percent,
            param_shadow_equal_percent,
            &mut output_signals,
        )
        .unwrap();

        println!("output_signals: {output_signals:?}");

        // Verify specific doji signals
        let doji_indices = [19, 22, 31, 45, 51, 60]; // Indices for TV BTCUSDT.P 15m 2025-02-07 00:45, 01:30, 03:45, 07:15, 08:45, 11:00
        for &idx in &doji_indices {
            assert_eq!(
                output_signals[idx],
                Signal::Pattern.into(),
                "Expected doji signal at index {idx}"
            );
        }

        // Test incremental calculation matches regular calculation
        for i in 0..input_open.len() {
            let signal = cdl_doji_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                param_body_percent,
                param_shadow_equal_percent,
            )
            .unwrap();
            assert_eq!(signal, output_signals[i], "Mismatch at index {i}");
        }
    }
}
