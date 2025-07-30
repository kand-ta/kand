use crate::{KandError, TAFloat};

/// Calculates the lookback period required for RSI (Relative Strength Index) calculation.
///
/// The lookback period equals the input parameter period since RSI needs historical data points
/// to establish the initial average gain and loss values.
///
/// # Arguments
/// * `param_period` - The number of periods to look back for RSI calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The required lookback period on success
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::ohlcv::rsi;
///
/// let param_period = 14;
/// let lookback = rsi::lookback(param_period).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period)
}

/// Calculates Relative Strength Index (RSI) for a price series.
///
/// RSI is a momentum oscillator that measures the speed and magnitude of recent price changes
/// to evaluate overbought or oversold conditions. It oscillates between 0 and 100, with
/// values above 70 generally indicating overbought conditions and values below 30 indicating
/// oversold conditions.
///
/// # Mathematical Formula
/// ```text
/// RSI = 100 - (100 / (1 + RS))
/// where:
/// RS = Average Gain / Average Loss
///
/// Initial Average Gain = Sum of Gains over past n periods / n
/// Initial Average Loss = Sum of Losses over past n periods / n
///
/// Subsequent values:
/// Average Gain = ((Previous Average Gain) × (n-1) + Current Gain) / n
/// Average Loss = ((Previous Average Loss) × (n-1) + Current Loss) / n
/// ```
///
/// # Calculation Principle
/// 1. Calculate price changes between consecutive periods
/// 2. Separate gains (positive changes) from losses (negative changes)
/// 3. Calculate initial average gain and loss over first n periods
/// 4. Apply Wilder's smoothing formula for subsequent periods
/// 5. Calculate RS ratio and convert to RSI value
///
/// # Arguments
/// * `input_prices` - Array of price values (typically closing prices)
/// * `param_period` - The time period for RSI calculation (typical values: 14, 9, or 25)
/// * `output_rsi` - Array to store calculated RSI values
/// * `output_avg_gain` - Array to store average gain values for each period
/// * `output_avg_loss` - Array to store average loss values for each period
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on successful calculation
///
/// # Errors
/// * `KandError::InvalidData` - If input array is empty
/// * `KandError::LengthMismatch` - If input and output arrays have different lengths
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
/// * `KandError::InsufficientData` - If input length is less than or equal to lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::rsi;
///
/// let input_prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42];
/// let param_period = 5;
/// let mut output_rsi = vec![0.0; input_prices.len()];
/// let mut output_avg_gain = vec![0.0; input_prices.len()];
/// let mut output_avg_loss = vec![0.0; input_prices.len()];
///
/// rsi::rsi(
///     &input_prices,
///     param_period,
///     &mut output_rsi,
///     &mut output_avg_gain,
///     &mut output_avg_loss,
/// )
/// .unwrap();
/// ```
pub fn rsi(
    input_prices: &[TAFloat],
    param_period: usize,
    output_rsi: &mut [TAFloat],
    output_avg_gain: &mut [TAFloat],
    output_avg_loss: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_prices.len();
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
        if output_rsi.len() != len || output_avg_gain.len() != len || output_avg_loss.len() != len {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for price in input_prices {
            // NaN check
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut gains = 0.0;
    let mut losses = 0.0;

    // Calculate initial gains/losses sum
    for i in 1..=lookback {
        let diff = input_prices[i] - input_prices[i - 1];
        if diff > 0.0 {
            gains += diff;
        } else {
            losses += diff.abs();
        }
    }

    // Calculate first RSI value
    let first_avg_gain = gains / param_period as TAFloat;
    let first_avg_loss = losses / param_period as TAFloat;

    output_avg_gain[lookback] = first_avg_gain;
    output_avg_loss[lookback] = first_avg_loss;

    if first_avg_loss == 0.0 {
        output_rsi[lookback] = 100.0;
    } else {
        let rs = first_avg_gain / first_avg_loss;
        output_rsi[lookback] = 100.0 - (100.0 / (1.0 + rs));
    }

    // Calculate remaining RSI values using smoothed averages
    let mut prev_avg_gain = first_avg_gain;
    let mut prev_avg_loss = first_avg_loss;
    let smoothing = param_period as TAFloat;

    for i in lookback + 1..len {
        let diff = input_prices[i] - input_prices[i - 1];
        let (curr_gain, curr_loss) = if diff > 0.0 {
            (diff, 0.0)
        } else {
            (0.0, diff.abs())
        };

        let curr_avg_gain = prev_avg_gain.mul_add(smoothing - 1.0, curr_gain) / smoothing;
        let curr_avg_loss = prev_avg_loss.mul_add(smoothing - 1.0, curr_loss) / smoothing;

        output_avg_gain[i] = curr_avg_gain;
        output_avg_loss[i] = curr_avg_loss;

        if curr_avg_loss == 0.0 {
            output_rsi[i] = 100.0;
        } else {
            let rs = curr_avg_gain / curr_avg_loss;
            output_rsi[i] = 100.0 - (100.0 / (1.0 + rs));
        }

        prev_avg_gain = curr_avg_gain;
        prev_avg_loss = curr_avg_loss;
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_rsi[i] = TAFloat::NAN;
        output_avg_gain[i] = TAFloat::NAN;
        output_avg_loss[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest RSI value incrementally using previous average gain and loss values.
///
/// This function optimizes RSI calculation for real-time data by using the previous period's
/// average gain and loss values to calculate the current RSI value, without needing the entire
/// price history.
///
/// # Mathematical Formula
/// ```text
/// Average Gain = ((Previous Average Gain) × (n-1) + Current Gain) / n
/// Average Loss = ((Previous Average Loss) × (n-1) + Current Loss) / n
/// RS = Average Gain / Average Loss
/// RSI = 100 - (100 / (1 + RS))
/// ```
///
/// # Arguments
/// * `input_curr_price` - Current period's price value
/// * `prev_price` - Previous period's price value
/// * `prev_avg_gain` - Previous period's average gain
/// * `prev_avg_loss` - Previous period's average loss
/// * `param_period` - The time period for RSI calculation
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing (RSI value, new average gain, new average loss)
///
/// # Errors
/// * `KandError::InvalidParameter` - If `param_period` is less than 2
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::rsi;
///
/// let (rsi_value, avg_gain, avg_loss) = rsi::rsi_inc(
///     45.42, // current price
///     45.10, // previous price
///     0.24,  // previous average gain
///     0.14,  // previous average loss
///     14,    // period
/// )
/// .unwrap();
/// ```
pub fn rsi_inc(
    input_curr_price: TAFloat,
    prev_price: TAFloat,
    prev_avg_gain: TAFloat,
    prev_avg_loss: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_curr_price.is_nan()
            || prev_price.is_nan()
            || prev_avg_gain.is_nan()
            || prev_avg_loss.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let diff = input_curr_price - prev_price;
    let (curr_gain, curr_loss) = if diff > 0.0 {
        (diff, 0.0)
    } else {
        (0.0, diff.abs())
    };

    let smoothing = param_period as TAFloat;
    let output_avg_gain = prev_avg_gain.mul_add(smoothing - 1.0, curr_gain) / smoothing;
    let output_avg_loss = prev_avg_loss.mul_add(smoothing - 1.0, curr_loss) / smoothing;

    let output_rsi = if output_avg_loss == 0.0 {
        100.0
    } else {
        let rs = output_avg_gain / output_avg_loss;
        100.0 - (100.0 / (1.0 + rs))
    };

    Ok((output_rsi, output_avg_gain, output_avg_loss))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    // Basic functionality tests
    #[test]
    fn test_rsi_calculation() {
        let input_prices = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0,
        ];
        let param_period = 14;
        let mut output_rsi = vec![0.0; input_prices.len()];
        let mut output_avg_gain = vec![0.0; input_prices.len()];
        let mut output_avg_loss = vec![0.0; input_prices.len()];

        rsi(
            &input_prices,
            param_period,
            &mut output_rsi,
            &mut output_avg_gain,
            &mut output_avg_loss,
        )
        .unwrap();

        // Verify first 14 values are NaN
        for value in output_rsi.iter().take(param_period) {
            assert!(value.is_nan());
        }

        // Verify against known values
        assert_relative_eq!(output_rsi[14], 37.748_344_370_861_39, epsilon = 0.00001);
        assert_relative_eq!(output_rsi[15], 34.223_538_361_225_86, epsilon = 0.00001);
        assert_relative_eq!(output_rsi[16], 31.518_806_080_459_882, epsilon = 0.00001);
        assert_relative_eq!(output_rsi[17], 33.425_568_632_418_2, epsilon = 0.00001);
        assert_relative_eq!(output_rsi[18], 40.465_006_259_629_995, epsilon = 0.00001);

        // Now test incremental calculation matches regular calculation
        let mut prev_avg_gain = output_avg_gain[param_period];
        let mut prev_avg_loss = output_avg_loss[param_period];
        let mut prev_price = input_prices[param_period];

        // Test each incremental step
        for i in param_period + 1..input_prices.len() {
            let (result, new_avg_gain, new_avg_loss) = rsi_inc(
                input_prices[i],
                prev_price,
                prev_avg_gain,
                prev_avg_loss,
                param_period,
            )
            .unwrap();

            assert_relative_eq!(result, output_rsi[i], epsilon = 0.00001);
            assert_relative_eq!(new_avg_gain, output_avg_gain[i], epsilon = 0.00001);
            assert_relative_eq!(new_avg_loss, output_avg_loss[i], epsilon = 0.00001);

            prev_avg_gain = new_avg_gain;
            prev_avg_loss = new_avg_loss;
            prev_price = input_prices[i];
        }
    }
}
