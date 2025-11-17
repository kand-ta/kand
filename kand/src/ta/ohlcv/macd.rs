use super::ema;
use crate::{KandError, TAFloat};

/// Calculate the lookback period required for MACD calculation
///
/// Returns the minimum number of data points needed before the first valid MACD output can be generated.
///
/// # Arguments
/// * `opt_fast_period` - Fast EMA period, must be > 0 and < `slow_period`
/// * `opt_slow_period` - Slow EMA period, must be > 0 and > `fast_period`
/// * `opt_signal_period` - Signal line period, must be > 0
///
/// # Returns
/// * `Result<usize, KandError>` - Lookback period if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If any period is 0 or `fast_period` >= `slow_period`
///
/// # Example
/// ```
/// use kand::ohlcv::macd;
/// let lookback = macd::lookback(12, 26, 9).unwrap();
/// assert_eq!(lookback, 33); // 25 (slow EMA) + 8 (signal)
/// ```
pub fn lookback(
    opt_fast_period: usize,
    opt_slow_period: usize,
    opt_signal_period: usize,
) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if opt_fast_period < 2 || opt_slow_period < 2 || opt_signal_period < 2 {
            return Err(KandError::InvalidParameter);
        }

        if opt_fast_period >= opt_slow_period {
            return Err(KandError::InvalidParameter);
        }
    }
    let slow_lookback = ema::lookback(opt_slow_period)?;
    let signal_lookback = ema::lookback(opt_signal_period)?;
    Ok(slow_lookback + signal_lookback)
}

/// Calculate Moving Average Convergence Divergence (MACD) for a price series
///
/// MACD is a trend-following momentum indicator that shows the relationship between two moving averages.
/// It consists of the MACD line (difference between fast and slow EMAs), signal line (EMA of MACD line),
/// and histogram (difference between MACD and signal lines).
///
/// # Mathematical Formula
/// ```text
/// Fast EMA = EMA(price, fast_period)
/// Slow EMA = EMA(price, slow_period)
/// MACD Line = Fast EMA - Slow EMA
/// Signal Line = EMA(MACD Line, signal_period)
/// Histogram = MACD Line - Signal Line
/// ```
///
/// # Calculation Steps
/// 1. Calculate fast EMA of price using `fast_period`
/// 2. Calculate slow EMA of price using `slow_period`
/// 3. Calculate MACD line as difference between fast and slow EMAs
/// 4. Calculate signal line as EMA of MACD line
/// 5. Calculate histogram as difference between MACD and signal lines
///
/// # Arguments
/// * `input_price` - Array of price values
/// * `opt_fast_period` - Fast EMA period (typically 12)
/// * `opt_slow_period` - Slow EMA period (typically 26)
/// * `opt_signal_period` - Signal line period (typically 9)
/// * `output_macd_line` - Output buffer for MACD line values
/// * `output_signal_line` - Output buffer for signal line values
/// * `output_histogram` - Output buffer for histogram values
/// * `output_fast_ema` - Output buffer for fast EMA values
/// * `output_slow_ema` - Output buffer for slow EMA values
///
/// # Returns
/// * `Result<(), KandError>` - Empty Ok if successful
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::InvalidParameter` - If any period is 0 or `fast_period` >= `slow_period`
/// * `KandError::InsufficientData` - If input length < required lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::macd;
///
/// let prices = vec![10.0, 12.0, 15.0, 11.0, 9.0, 10.0, 12.0];
/// let mut macd_line = vec![0.0; prices.len()];
/// let mut signal_line = vec![0.0; prices.len()];
/// let mut histogram = vec![0.0; prices.len()];
/// let mut fast_ema = vec![0.0; prices.len()];
/// let mut slow_ema = vec![0.0; prices.len()];
///
/// macd::macd(
///     &prices,
///     2, // fast period
///     3, // slow period
///     4, // signal period
///     &mut macd_line,
///     &mut signal_line,
///     &mut histogram,
///     &mut fast_ema,
///     &mut slow_ema,
/// )
/// .unwrap();
/// ```
pub fn macd(
    input_price: &[TAFloat],
    opt_fast_period: usize,
    opt_slow_period: usize,
    opt_signal_period: usize,
    output_macd_line: &mut [TAFloat],
    output_signal_line: &mut [TAFloat],
    output_histogram: &mut [TAFloat],
    output_fast_ema: &mut [TAFloat],
    output_slow_ema: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_price.len();
    let lookback = lookback(opt_fast_period, opt_slow_period, opt_signal_period)?;

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
        if len != output_macd_line.len()
            || len != output_signal_line.len()
            || len != output_histogram.len()
            || len != output_fast_ema.len()
            || len != output_slow_ema.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Check if remaining data after slow period is sufficient for signal calculation
        if len.saturating_sub(opt_slow_period) < opt_signal_period {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for price in input_price {
            // NaN check
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    ema::ema(input_price, opt_fast_period, None, output_fast_ema)?;
    ema::ema(input_price, opt_slow_period, None, output_slow_ema)?;

    // Calculate MACD line
    for i in 0..len {
        output_macd_line[i] = output_fast_ema[i] - output_slow_ema[i];
    }

    // Calculate signal line using non-NaN MACD values
    ema::ema(
        &output_macd_line[opt_slow_period - 1..],
        opt_signal_period,
        None,
        &mut output_signal_line[opt_slow_period - 1..],
    )?;

    // Calculate histogram
    for i in lookback..len {
        output_histogram[i] = output_macd_line[i] - output_signal_line[i];
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_macd_line[i] = TAFloat::NAN;
        output_signal_line[i] = TAFloat::NAN;
        output_histogram[i] = TAFloat::NAN;
        output_fast_ema[i] = TAFloat::NAN;
        output_slow_ema[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculate latest MACD values incrementally from previous state
///
/// This function provides an efficient way to calculate MACD for streaming data by using
/// previous EMA values instead of recalculating the entire series.
///
/// # Mathematical Formula
/// ```text
/// Fast EMA = EMA(price, fast_period, prev_fast_ema)
/// Slow EMA = EMA(price, slow_period, prev_slow_ema)
/// MACD = Fast EMA - Slow EMA
/// Signal = EMA(MACD, signal_period, prev_signal)
/// Histogram = MACD - Signal
/// ```
///
/// # Arguments
/// * `input_price` - Current price value
/// * `prev_fast_ema` - Previous fast EMA value
/// * `prev_slow_ema` - Previous slow EMA value
/// * `prev_signal` - Previous signal line value
/// * `opt_fast_period` - Fast EMA period (typically 12)
/// * `opt_slow_period` - Slow EMA period (typically 26)
/// * `opt_signal_period` - Signal line period (typically 9)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple of (MACD, Signal, Histogram) if successful
///
/// # Errors
/// * `KandError::InvalidParameter` - If any period is 0 or `fast_period` >= `slow_period`
/// * `KandError::NaNDetected` - If any input value is NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::macd;
///
/// let (macd, signal, hist) = macd::macd_inc(
///     100.0, // current price
///     95.0,  // previous fast EMA
///     98.0,  // previous slow EMA
///     -2.5,  // previous signal
///     12,    // fast period
///     26,    // slow period
///     9,     // signal period
/// )
/// .unwrap();
/// ```
pub fn macd_inc(
    input_price: TAFloat,
    prev_fast_ema: TAFloat,
    prev_slow_ema: TAFloat,
    prev_signal: TAFloat,
    opt_fast_period: usize,
    opt_slow_period: usize,
    opt_signal_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if opt_fast_period < 2 || opt_slow_period < 2 || opt_signal_period < 2 {
            return Err(KandError::InvalidParameter);
        }
        if opt_fast_period >= opt_slow_period {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_price.is_nan()
            || prev_fast_ema.is_nan()
            || prev_slow_ema.is_nan()
            || prev_signal.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let fast_ema = ema::ema_inc(input_price, prev_fast_ema, opt_fast_period, None)?;
    let slow_ema = ema::ema_inc(input_price, prev_slow_ema, opt_slow_period, None)?;
    let macd = fast_ema - slow_ema;
    let signal = ema::ema_inc(macd, prev_signal, opt_signal_period, None)?;
    let histogram = macd - signal;

    Ok((macd, signal, histogram))
}
