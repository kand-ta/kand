use crate::{KandError, TAFloat};

/// Returns the lookback period required for Heikin-Ashi calculation
///
/// # Description
/// Calculates the minimum number of data points needed before the first valid Heikin-Ashi value can be computed.
/// The lookback period is 1 since we need the previous candle to calculate the first value.
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (always 1)
///
/// # Errors
/// This function does not return any errors.
pub const fn lookback() -> Result<usize, KandError> {
    Ok(1)
}

/// Calculates Heikin-Ashi candlestick values from OHLC price data
///
/// # Description
/// Heikin-Ashi is a Japanese candlestick charting technique that uses modified formulas
/// for open, high, low and close values to filter out market noise and better identify trends.
/// The resulting candles provide a clearer view of price action compared to traditional candlesticks.
///
/// # Mathematical Formula
/// ```text
/// HA_Close = (Open + High + Low + Close) / 4
/// HA_Open = (Previous HA_Open + Previous HA_Close) / 2
/// HA_High = max(High, HA_Open, HA_Close)
/// HA_Low = min(Low, HA_Open, HA_Close)
/// ```
///
/// # Parameters
/// * `input_open` - Array of opening prices (slice of type `TAFloat`)
/// * `input_high` - Array of high prices (slice of type `TAFloat`)
/// * `input_low` - Array of low prices (slice of type `TAFloat`)
/// * `input_close` - Array of closing prices (slice of type `TAFloat`)
/// * `output_open` - Array to store calculated HA open values (mutable slice of type `TAFloat`)
/// * `output_high` - Array to store calculated HA high values (mutable slice of type `TAFloat`)
/// * `output_low` - Array to store calculated HA low values (mutable slice of type `TAFloat`)
/// * `output_close` - Array to store calculated HA close values (mutable slice of type `TAFloat`)
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) if calculation succeeds
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths
/// * `KandError::NaNDetected` - If input contains NaN values (with "`check-nan`")
///
/// # Example
/// ```
/// use kand::ohlcv::ha;
///
/// let input_open = vec![10.0, 10.5, 11.2];
/// let input_high = vec![11.0, 11.5, 11.8];
/// let input_low = vec![9.5, 10.2, 10.8];
/// let input_close = vec![10.8, 11.3, 11.5];
///
/// let mut output_open = vec![0.0; 3];
/// let mut output_high = vec![0.0; 3];
/// let mut output_low = vec![0.0; 3];
/// let mut output_close = vec![0.0; 3];
///
/// ha::ha(
///     &input_open,
///     &input_high,
///     &input_low,
///     &input_close,
///     &mut output_open,
///     &mut output_high,
///     &mut output_low,
///     &mut output_close,
/// )
/// .unwrap();
/// ```
pub fn ha(
    input_open: &[TAFloat],
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    output_open: &mut [TAFloat],
    output_high: &mut [TAFloat],
    output_low: &mut [TAFloat],
    output_close: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_open.len();
    let lookback = lookback()?;

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
            || len != output_open.len()
            || len != output_high.len()
            || len != output_low.len()
            || len != output_close.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
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

    // Calculate first candle
    output_close[0] = (input_open[0] + input_high[0] + input_low[0] + input_close[0]) / 4.0;
    output_open[0] = f64::midpoint(input_open[0], input_close[0]);
    output_high[0] = input_high[0];
    output_low[0] = input_low[0];

    // Calculate remaining candles
    for i in lookback..len {
        let (o, h, l, c) = ha_inc(
            input_open[i],
            input_high[i],
            input_low[i],
            input_close[i],
            output_open[i - 1],
            output_close[i - 1],
        )?;
        output_open[i] = o;
        output_high[i] = h;
        output_low[i] = l;
        output_close[i] = c;
    }

    Ok(())
}

/// Calculates a single Heikin-Ashi candle incrementally for streaming data
///
/// # Description
/// Provides an optimized way to calculate the latest Heikin-Ashi candle when new data arrives,
/// without recalculating the entire series. This is particularly useful for real-time
/// data processing and streaming applications.
///
/// # Mathematical Formula
/// ```text
/// HA_Close = (Open + High + Low + Close) / 4
/// HA_Open = (Previous HA_Open + Previous HA_Close) / 2
/// HA_High = max(High, HA_Open, HA_Close)
/// HA_Low = min(Low, HA_Open, HA_Close)
/// ```
///
/// # Parameters
/// * `curr_open` - Current candle's open price (type `TAFloat`)
/// * `curr_high` - Current candle's high price (type `TAFloat`)
/// * `curr_low` - Current candle's low price (type `TAFloat`)
/// * `curr_close` - Current candle's close price (type `TAFloat`)
/// * `prev_ha_open` - Previous Heikin-Ashi candle's open price (type `TAFloat`)
/// * `prev_ha_close` - Previous Heikin-Ashi candle's close price (type `TAFloat`)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - Tuple of (`HA_Open`, `HA_High`, `HA_Low`, `HA_Close`) if successful
///
/// # Errors
/// * `KandError::NaNDetected` - If any input is NaN (with "`check-nan`")
///
/// # Example
/// ```
/// use kand::ohlcv::ha::ha_inc;
///
/// let (ha_open, ha_high, ha_low, ha_close) = ha_inc(
///     11.2,    // Current open
///     11.8,    // Current high
///     10.8,    // Current low
///     11.5,    // Current close
///     10.3625, // Previous HA open
///     10.875,  // Previous HA close
/// )
/// .unwrap();
/// ```
pub fn ha_inc(
    curr_open: TAFloat,
    curr_high: TAFloat,
    curr_low: TAFloat,
    curr_close: TAFloat,
    prev_ha_open: TAFloat,
    prev_ha_close: TAFloat,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if curr_open.is_nan()
            || curr_high.is_nan()
            || curr_low.is_nan()
            || curr_close.is_nan()
            || prev_ha_open.is_nan()
            || prev_ha_close.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let ha_close = (curr_open + curr_high + curr_low + curr_close) / 4.0;
    let ha_open = f64::midpoint(prev_ha_open, prev_ha_close);
    let ha_high = curr_high.max(ha_open).max(ha_close);
    let ha_low = curr_low.min(ha_open).min(ha_close);

    Ok((ha_open, ha_high, ha_low, ha_close))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_ha_calculation() {
        let input_open = vec![10.0, 10.5, 11.2, 10.8, 11.5];
        let input_high = vec![11.0, 11.5, 11.8, 11.3, 12.0];
        let input_low = vec![9.5, 10.2, 10.8, 10.5, 11.3];
        let input_close = vec![10.8, 11.3, 11.5, 11.0, 11.8];

        let mut output_open = vec![0.0; 5];
        let mut output_high = vec![0.0; 5];
        let mut output_low = vec![0.0; 5];
        let mut output_close = vec![0.0; 5];

        ha(
            &input_open,
            &input_high,
            &input_low,
            &input_close,
            &mut output_open,
            &mut output_high,
            &mut output_low,
            &mut output_close,
        )
        .unwrap();

        // Test first candle
        assert_relative_eq!(output_open[0], 10.4, epsilon = 0.0001);
        assert_relative_eq!(output_high[0], 11.0, epsilon = 0.0001);
        assert_relative_eq!(output_low[0], 9.5, epsilon = 0.0001);
        assert_relative_eq!(output_close[0], 10.325, epsilon = 0.0001);

        // Test subsequent candles
        assert_relative_eq!(output_open[1], 10.3625, epsilon = 0.0001);
        assert_relative_eq!(output_close[1], 10.875, epsilon = 0.0001);
        assert_relative_eq!(output_high[1], 11.5, epsilon = 0.0001);
        assert_relative_eq!(output_low[1], 10.2, epsilon = 0.0001);

        // Test incremental calculation matches regular calculation
        for i in 1..5 {
            let (ha_open, ha_high, ha_low, ha_close) = ha_inc(
                input_open[i],
                input_high[i],
                input_low[i],
                input_close[i],
                output_open[i - 1],
                output_close[i - 1],
            )
            .unwrap();

            assert_relative_eq!(ha_open, output_open[i], epsilon = 0.0001);
            assert_relative_eq!(ha_high, output_high[i], epsilon = 0.0001);
            assert_relative_eq!(ha_low, output_low[i], epsilon = 0.0001);
            assert_relative_eq!(ha_close, output_close[i], epsilon = 0.0001);
        }
    }
}
