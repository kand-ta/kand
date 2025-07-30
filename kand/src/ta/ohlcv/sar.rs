use crate::{KandError, TAFloat};

/// Returns the lookback period required by the Parabolic SAR indicator.
///
/// # Description
/// Calculates the minimum number of data points needed before the first valid SAR value can be computed.
/// For the Parabolic SAR indicator, this is always 1 period.
///
/// # Parameters
/// * `param_acceleration` - The acceleration factor used in SAR calculation. Type: `TAFloat`
/// * `param_maximum` - The maximum allowed acceleration factor. Type: `TAFloat`
///
/// # Returns
/// * `Ok(usize)` - The lookback period (1)
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_acceleration` or `param_maximum` are invalid
///
/// # Example
/// ```
/// use kand::ohlcv::sar;
///
/// let acceleration = 0.02;
/// let maximum = 0.2;
/// let lookback = sar::lookback(acceleration, maximum).unwrap();
/// assert_eq!(lookback, 1);
/// ```
pub const fn lookback(
    _param_acceleration: TAFloat,
    _param_maximum: TAFloat,
) -> Result<usize, KandError> {
    Ok(1)
}

/// Calculates the Parabolic SAR (Stop And Reverse) indicator.
///
/// # Description
/// The Parabolic SAR is a trend-following indicator that helps identify potential reversal points
/// in price movements. It plots points below prices in an uptrend and above prices in a downtrend.
///
/// # Mathematical Formula
/// For each period t:
/// ```text
/// Uptrend (Long):
/// SAR(t) = SAR(t-1) + AF * (EP - SAR(t-1))
/// where:
/// - AF (Acceleration Factor) increases by param_acceleration when new EP is reached
/// - EP (Extreme Point) is the highest high in current uptrend
///
/// Downtrend (Short):
/// SAR(t) = SAR(t-1) + AF * (EP - SAR(t-1))
/// where:
/// - AF increases by param_acceleration when new EP is reached
/// - EP is the lowest low in current downtrend
/// ```
///
/// # Parameters
/// * `input_high` - Array of high prices. Type: `&[TAFloat]`
/// * `input_low` - Array of low prices. Type: `&[TAFloat]`
/// * `param_acceleration` - Initial acceleration factor (e.g. 0.02). Type: `TAFloat`
/// * `param_maximum` - Maximum acceleration factor (e.g. 0.2). Type: `TAFloat`
/// * `output_sar` - Buffer to store SAR values. Type: `&mut [TAFloat]`
/// * `output_is_long` - Buffer to store trend direction (true=long, false=short). Type: `&mut [bool]`
/// * `output_af` - Buffer to store acceleration factors. Type: `&mut [TAFloat]`
/// * `output_ep` - Buffer to store extreme points. Type: `&mut [TAFloat]`
///
/// # Returns
/// * `Ok(())` - Calculation successful
///
/// # Errors
/// * `KandError::InvalidData` - Input arrays are empty
/// * `KandError::LengthMismatch` - Input/output array lengths don't match
/// * `KandError::InvalidParameter` - Invalid acceleration/maximum values
/// * `KandError::InsufficientData` - Not enough data points
/// * `KandError::NaNDetected` - Input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::sar;
///
/// let high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let mut sar = vec![0.0; 5];
/// let mut is_long = vec![false; 5];
/// let mut af = vec![0.0; 5];
/// let mut ep = vec![0.0; 5];
///
/// sar::sar(
///     &high,
///     &low,
///     0.02, // acceleration
///     0.2,  // maximum
///     &mut sar,
///     &mut is_long,
///     &mut af,
///     &mut ep,
/// )
/// .unwrap();
/// ```
pub fn sar(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    param_acceleration: TAFloat,
    param_maximum: TAFloat,
    output_sar: &mut [TAFloat],
    output_is_long: &mut [bool],
    output_af: &mut [TAFloat],
    output_ep: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(param_acceleration, param_maximum)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }
        if len != input_low.len()
            || len != output_sar.len()
            || len != output_is_long.len()
            || len != output_af.len()
            || len != output_ep.len()
        {
            return Err(KandError::LengthMismatch);
        }
        if param_acceleration <= 0.0 || param_maximum <= param_acceleration {
            return Err(KandError::InvalidParameter);
        }
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        for i in 0..len {
            if input_high[i].is_nan() || input_low[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Determine the initial trend by comparing the positive and negative directional movements.
    let plus_dm = input_high[1] - input_high[0];
    let minus_dm = input_low[0] - input_low[1];
    let initial_trend = plus_dm >= minus_dm; // Default to long if equal

    let mut af = param_acceleration;
    let mut ep = if initial_trend {
        input_high[1]
    } else {
        input_low[1]
    };

    // Initialize the SAR output and auxiliary arrays at the first index.
    output_sar[0] = TAFloat::NAN;
    output_is_long[0] = initial_trend;
    output_af[0] = 0.0;
    output_ep[0] = TAFloat::NAN;

    // Derive the second SAR value from the first bar's price data.
    output_sar[1] = if initial_trend {
        input_low[0]
    } else {
        input_high[0]
    };
    output_is_long[1] = initial_trend;
    output_af[1] = af;
    output_ep[1] = ep;

    let mut is_long = initial_trend;
    // Record the starting index of the current trend; a new trend begins at index 1.
    let mut trend_start = 1;

    // Iterate over remaining data points, computing the SAR values and updating state arrays.
    for i in 2..len {
        let prev_sar = output_sar[i - 1];
        let high = input_high[i];
        let low = input_low[i];

        let mut sar_val = af.mul_add(ep - prev_sar, prev_sar);

        if is_long {
            if i - trend_start < 2 {
                sar_val = sar_val.min(input_low[i - 1]);
            } else {
                sar_val = sar_val.min(input_low[i - 1]).min(input_low[i - 2]);
            }
            if high > ep {
                ep = high;
                af = (af + param_acceleration).min(param_maximum);
            }
            if low < sar_val {
                is_long = false;
                sar_val = ep;
                ep = low;
                af = param_acceleration;
                trend_start = i - 1;
            }
        } else {
            if i - trend_start < 2 {
                sar_val = sar_val.max(input_high[i - 1]);
            } else {
                sar_val = sar_val.max(input_high[i - 1]).max(input_high[i - 2]);
            }
            if low < ep {
                ep = low;
                af = (af + param_acceleration).min(param_maximum);
            }
            if high > sar_val {
                is_long = true;
                sar_val = ep;
                ep = high;
                af = param_acceleration;
                trend_start = i - 1;
            }
        }

        output_sar[i] = sar_val;
        output_is_long[i] = is_long;
        output_af[i] = af;
        output_ep[i] = ep;
    }

    Ok(())
}

/// Incrementally updates the Parabolic SAR with new price data.
///
/// # Description
/// Calculates the next SAR value based on the latest price data without reprocessing historical data.
/// Useful for real-time calculations where new data arrives sequentially.
///
/// # Parameters
/// * `input_high` - Current period's high price. Type: `TAFloat`
/// * `input_low` - Current period's low price. Type: `TAFloat`
/// * `prev_high` - Previous period's high price. Type: `TAFloat`
/// * `prev_low` - Previous period's low price. Type: `TAFloat`
/// * `prev_sar` - Previous period's SAR value. Type: `TAFloat`
/// * `input_is_long` - Current trend direction (true=long, false=short). Type: `bool`
/// * `input_af` - Current acceleration factor. Type: `TAFloat`
/// * `input_ep` - Current extreme point. Type: `TAFloat`
/// * `param_acceleration` - Acceleration factor increment. Type: `TAFloat`
/// * `param_maximum` - Maximum acceleration factor. Type: `TAFloat`
///
/// # Returns
/// A `Result` containing:
/// * `Ok((TAFloat, bool, TAFloat, TAFloat))` - Tuple containing:
///   * Updated SAR value
///   * New trend direction
///   * Updated acceleration factor
///   * Updated extreme point
///
/// # Errors
/// * `KandError::InvalidParameter` - Invalid acceleration/maximum values
/// * `KandError::NaNDetected` - Input contains NaN values
///
/// # Example
/// ```
/// use kand::ohlcv::sar;
///
/// let (new_sar, new_is_long, new_af, new_ep) = sar::sar_inc(
///     15.0, // current high
///     14.0, // current low
///     14.5, // previous high
///     13.5, // previous low
///     13.0, // previous SAR
///     true, // is long trend
///     0.02, // current AF
///     14.5, // current EP
///     0.02, // acceleration
///     0.2,  // maximum
/// )
/// .unwrap();
/// ```
pub fn sar_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    prev_high: TAFloat,
    prev_low: TAFloat,
    prev_sar: TAFloat,
    input_is_long: bool,
    input_af: TAFloat,
    input_ep: TAFloat,
    param_acceleration: TAFloat,
    param_maximum: TAFloat,
) -> Result<(TAFloat, bool, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_acceleration <= 0.0 || param_maximum <= param_acceleration {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan()
            || input_low.is_nan()
            || prev_high.is_nan()
            || prev_low.is_nan()
            || prev_sar.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let high = input_high;
    let low = input_low;
    let mut is_long = input_is_long;
    let mut af = input_af;
    let mut ep = input_ep;

    let mut sar = af.mul_add(ep - prev_sar, prev_sar);

    if is_long {
        sar = sar.min(prev_low);
        if high > ep {
            ep = high;
            af = (af + param_acceleration).min(param_maximum);
        }
        if low < sar {
            is_long = false;
            sar = ep;
            ep = low;
            af = param_acceleration;
        }
    } else {
        sar = sar.max(prev_high);
        if low < ep {
            ep = low;
            af = (af + param_acceleration).min(param_maximum);
        }
        if high > sar {
            is_long = true;
            sar = ep;
            ep = high;
            af = param_acceleration;
        }
    }

    Ok((sar, is_long, af, ep))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_sar_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2,
        ];

        let param_acceleration = 0.02;
        let param_maximum = 0.2;
        let mut output_sar = vec![0.0; input_high.len()];
        let mut output_is_long = vec![false; input_high.len()];
        let mut output_af = vec![0.0; input_high.len()];
        let mut output_ep = vec![0.0; input_high.len()];

        sar(
            &input_high,
            &input_low,
            param_acceleration,
            param_maximum,
            &mut output_sar,
            &mut output_is_long,
            &mut output_af,
            &mut output_ep,
        )
        .unwrap();

        // First value should be NaN
        assert!(output_sar[0].is_nan());

        // Compare with known values (offset by one due to initial NaN)
        let expected_values = [
            35266.0,
            35264.81,
            35261.4176,
            35_253.574_544,
            35130.7,
            35133.246,
            35138.43216,
            35_147.016_230_4,
            35_155.085_256_576,
            35_162.670_141_181_44,
            35281.5,
            35278.952,
            35_276.454_959_999_995,
            35_271.152_761_599_995,
            35_259.689_595_904,
            35_240.602_428_231_68,
            35_211.552_185_408_51,
            35_185.406_966_867_66,
            35_161.876_270_180_896,
        ];

        for i in 1..expected_values.len() {
            assert_relative_eq!(output_sar[i], expected_values[i - 1], epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_sar = output_sar[1];
        let mut is_long = output_is_long[1];
        let mut af = param_acceleration;
        let mut ep = output_ep[1]; // Use the previously calculated EP instead of input_high[1]

        for i in 2..6 {
            let (sar_value, new_is_long, new_af, new_ep) = sar_inc(
                input_high[i],
                input_low[i],
                input_high[i - 1],
                input_low[i - 1],
                prev_sar,
                is_long,
                af,
                ep,
                param_acceleration,
                param_maximum,
            )
            .unwrap();

            assert_relative_eq!(sar_value, output_sar[i], epsilon = 0.0001);

            prev_sar = sar_value;
            is_long = new_is_long;
            af = new_af;
            ep = new_ep;
        }
    }
}
