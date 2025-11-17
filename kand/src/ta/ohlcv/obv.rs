use crate::{KandError, TAFloat};

/// Returns the lookback period required for On Balance Volume (OBV) calculation
///
/// # Description
/// OBV has no lookback period as it can be calculated from the first data point.
///
/// # Returns
/// * `Ok(usize)` - Returns 0 as OBV can be calculated from first data point
///
/// # Errors
/// This function does not return any errors.
///
/// # Example
/// ```
/// use kand::ohlcv::obv;
/// let lookback = obv::lookback().unwrap();
/// assert_eq!(lookback, 0);
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculate On Balance Volume (OBV) for the entire input array
///
/// # Description
/// On Balance Volume (OBV) is a momentum indicator that uses volume flow to predict changes in stock price.
/// The indicator assumes that volume precedes price movements.
///
/// # Mathematical Formula
/// ```text
/// If Close[i] > Close[i-1]:
///     OBV[i] = OBV[i-1] + Volume[i]
/// If Close[i] < Close[i-1]:
///     OBV[i] = OBV[i-1] - Volume[i]
/// If Close[i] = Close[i-1]:
///     OBV[i] = OBV[i-1]
/// ```
///
/// # Calculation Principles
/// 1. When volume increases without significant price change:
///    - Price is expected to jump upward
/// 2. When volume decreases without significant price change:
///    - Price is expected to jump downward
/// 3. First OBV value equals the first volume value
///
/// # Arguments
/// * `input_close` - Array of closing prices
/// * `input_volume` - Array of volume values
/// * `output_obv` - Array to store calculated OBV values
///
/// # Returns
/// * `Result<(), KandError>` - Ok if calculation succeeds, Err otherwise
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input arrays have different lengths
/// * `KandError::InsufficientData` - If input length <= lookback period
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::obv;
///
/// let input_close = vec![10.0, 12.0, 11.0, 13.0];
/// let input_volume = vec![100.0, 150.0, 120.0, 200.0];
/// let mut output_obv = vec![0.0; 4];
///
/// obv::obv(&input_close, &input_volume, &mut output_obv).unwrap();
/// // output_obv = [100.0, 250.0, 130.0, 330.0]
/// ```
pub fn obv(
    input_close: &[TAFloat],
    input_volume: &[TAFloat],
    output_obv: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_close.len();
    let lookback = lookback()?;

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
        if len != input_volume.len() || len != output_obv.len() {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            // NaN check
            if input_close[i].is_nan() || input_volume[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut obv = input_volume[lookback];
    output_obv[lookback] = obv;

    for i in (lookback + 1)..len {
        obv = if input_close[i] > input_close[i - 1] {
            obv + input_volume[i]
        } else if input_close[i] < input_close[i - 1] {
            obv - input_volume[i]
        } else {
            obv
        };
        output_obv[i] = obv;
    }

    Ok(())
}

/// Calculate latest On Balance Volume (OBV) value incrementally
///
/// # Description
/// Calculates a single OBV value using the previous OBV value and current price/volume data.
///
/// # Arguments
/// * `input_curr_close` - Current closing price
/// * `prev_close` - Previous closing price
/// * `input_volume` - Current volume
/// * `prev_obv` - Previous OBV value
///
/// # Returns
/// * `Result<TAFloat, KandError>` - Latest OBV value if calculation succeeds, Err otherwise
///
/// # Errors
/// * `KandError::NaNDetected` - If any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::ohlcv::obv;
///
/// let curr_close = 12.0;
/// let prev_close = 10.0;
/// let volume = 150.0;
/// let prev_obv = 100.0;
///
/// let output_obv = obv::obv_inc(curr_close, prev_close, volume, prev_obv).unwrap();
/// // output_obv = 250.0 (prev_obv + volume since price increased)
/// ```
pub fn obv_inc(
    input_curr_close: TAFloat,
    prev_close: TAFloat,
    input_volume: TAFloat,
    prev_obv: TAFloat,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check-nan")]
    {
        // NaN check
        if input_curr_close.is_nan()
            || prev_close.is_nan()
            || input_volume.is_nan()
            || prev_obv.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    Ok(if input_curr_close > prev_close {
        prev_obv + input_volume
    } else if input_curr_close < prev_close {
        prev_obv - input_volume
    } else {
        prev_obv
    })
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_obv_calculation() {
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6,
        ];
        let input_volume = vec![
            1055.365, 756.488, 682.152, 1197.747, 425.97, 859.638, 741.925, 888.477, 1043.333,
            467.901, 387.47, 566.099, 672.296, 834.915, 1854.024, 3670.795, 3761.198, 1605.442,
            1726.574, 934.713,
        ];
        let mut output_obv = vec![0.0; input_close.len()];

        obv(&input_close, &input_volume, &mut output_obv).unwrap();

        let expected_values = [
            1055.365, 1811.853, 1129.701, -68.046, 357.924, 1217.562, 475.637, 1364.114, 320.781,
            -147.12, -534.59, 31.509, -640.787, -1475.702, -3329.726, -7000.521, -10761.719,
            -9156.277, -7429.703, -8364.416,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_obv[i], *expected, epsilon = 0.00001);
        }

        let mut prev_obv = output_obv[0];

        for i in 1..input_close.len() {
            let result = obv_inc(
                input_close[i],
                input_close[i - 1],
                input_volume[i],
                prev_obv,
            )
            .unwrap();
            assert_relative_eq!(result, output_obv[i], epsilon = 0.00001);
            prev_obv = result;
        }
    }
}
