use crate::{KandError, TAFloat, ta::ohlcv::typprice};

/// Calculates the lookback period required for Money Flow Index (MFI) calculation.
///
/// # Description
/// The lookback period is equal to the input period parameter since MFI requires
/// previous data points to calculate the money flow ratio.
///
/// # Arguments
/// * `opt_period` - The time period for MFI calculation (e.g. 14 for a 14-period MFI)
///
/// # Returns
/// * `Result<usize, KandError>` - The required lookback period on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `opt_period` is less than 2
///
/// # Examples
/// ```
/// use kand::ohlcv::mfi;
/// let period = 14;
/// let lookback = mfi::lookback(period).unwrap();
/// assert_eq!(lookback, 14);
/// ```
pub const fn lookback(opt_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(opt_period)
}

/// Calculates Money Flow Index (MFI) for a price series.
///
/// # Description
/// Money Flow Index (MFI) is a technical oscillator that uses price and volume data to identify
/// overbought or oversold conditions in an asset. It can also be used to spot divergences which
/// may lead to price reversals.
///
/// # Mathematical Formula
/// ```text
/// Typical Price = (High + Low + Close) / 3
/// Money Flow = Typical Price × Volume
/// Positive Money Flow = Sum of Money Flow where Typical Price increases
/// Negative Money Flow = Sum of Money Flow where Typical Price decreases
/// Money Flow Ratio = Positive Money Flow / Negative Money Flow
/// MFI = 100 - (100 / (1 + Money Flow Ratio))
/// ```
///
/// # Arguments
/// * `input_high` - Array of high prices
/// * `input_low` - Array of low prices
/// * `input_close` - Array of close prices
/// * `input_volume` - Array of volume data
/// * `opt_period` - The time period for MFI calculation (typically 14)
/// * `output_mfi` - Array to store the calculated MFI values (0-100)
/// * `output_typ_prices` - Array to store the calculated typical prices
/// * `output_money_flows` - Array to store the calculated money flows
/// * `output_pos_flows` - Array to store the positive money flows
/// * `output_neg_flows` - Array to store the negative money flows
///
/// # Returns
/// * `Result<(), KandError>` - Empty result on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `opt_period` is zero
/// * Returns `KandError::InvalidData` if input arrays are empty
/// * Returns `KandError::LengthMismatch` if input arrays have different lengths
///
/// # Examples
/// ```
/// use kand::ta::ohlcv::mfi;
/// let high = vec![10.0, 11.0, 12.0, 11.0];
/// let low = vec![8.0, 9.0, 10.0, 9.0];
/// let close = vec![9.0, 10.0, 11.0, 10.0];
/// let volume = vec![100.0, 150.0, 200.0, 150.0];
/// let period = 2;
/// let mut mfi = vec![0.0; 4];
/// let mut typ_prices = vec![0.0; 4];
/// let mut money_flows = vec![0.0; 4];
/// let mut pos_flows = vec![0.0; 4];
/// let mut neg_flows = vec![0.0; 4];
///
/// mfi::mfi(
///     &high,
///     &low,
///     &close,
///     &volume,
///     period,
///     &mut mfi,
///     &mut typ_prices,
///     &mut money_flows,
///     &mut pos_flows,
///     &mut neg_flows,
/// )
/// .unwrap();
/// ```
pub fn mfi(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    input_volume: &[TAFloat],
    opt_period: usize,
    output_mfi: &mut [TAFloat],
    output_typ_prices: &mut [TAFloat],
    output_money_flows: &mut [TAFloat],
    output_pos_flows: &mut [TAFloat],
    output_neg_flows: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(opt_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Data sufficiency check
        if len < lookback {
            return Err(KandError::InsufficientData);
        }

        // Length consistency check
        if len != input_low.len()
            || len != input_close.len()
            || len != input_volume.len()
            || len != output_mfi.len()
            || len != output_typ_prices.len()
            || len != output_money_flows.len()
            || len != output_pos_flows.len()
            || len != output_neg_flows.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Parameter validation
        if opt_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    // Calculate typical prices
    typprice::typprice(input_high, input_low, input_close, output_typ_prices)?;

    // Initialize money flows
    for i in 0..len {
        output_money_flows[i] = output_typ_prices[i] * input_volume[i];
    }

    // Calculate MFI for each period
    for i in opt_period..len {
        let mut pos_flow = 0.0;
        let mut neg_flow = 0.0;

        // Calculate positive and negative money flows over the period
        for j in (i - opt_period + 1)..=i {
            if output_typ_prices[j] > output_typ_prices[j - 1] {
                pos_flow += output_money_flows[j];
            } else if output_typ_prices[j] < output_typ_prices[j - 1] {
                neg_flow += output_money_flows[j];
            }
        }

        output_pos_flows[i] = pos_flow;
        output_neg_flows[i] = neg_flow;

        // Calculate MFI using the optimized formula:
        // MFI = 100 * (posSumMF/(posSumMF+negSumMF))
        let total_flow = pos_flow + neg_flow;
        if total_flow < 1.0 {
            output_mfi[i] = 0.0;
        } else {
            output_mfi[i] = 100.0 * (pos_flow / total_flow);
        }
    }

    // Set initial values to NaN
    for i in 0..opt_period {
        output_mfi[i] = TAFloat::NAN;
        output_pos_flows[i] = TAFloat::NAN;
        output_neg_flows[i] = TAFloat::NAN;
        output_typ_prices[i] = TAFloat::NAN;
        output_money_flows[i] = TAFloat::NAN;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_mfi_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1, 35179.2,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0, 35073.1,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3,
        ];
        let input_volume = vec![
            1055.365, 756.488, 682.152, 1197.747, 425.97, 859.638, 741.925, 888.477, 1043.333,
            467.901, 387.47, 566.099, 672.296, 834.915, 1854.024, 3670.795, 3761.198, 1605.442,
            1726.574, 934.713, 2199.061, 2349.823, 837.218, 1000.638, 1218.202, 2573.668, 1098.409,
            609.582, 670.489, 1637.998,
        ];
        let opt_period = 14;
        let mut output_mfi = vec![0.0; input_high.len()];
        let mut output_typ_prices = vec![0.0; input_high.len()];
        let mut output_money_flows = vec![0.0; input_high.len()];
        let mut output_pos_flows = vec![0.0; input_high.len()];
        let mut output_neg_flows = vec![0.0; input_high.len()];

        mfi(
            &input_high,
            &input_low,
            &input_close,
            &input_volume,
            opt_period,
            &mut output_mfi,
            &mut output_typ_prices,
            &mut output_money_flows,
            &mut output_pos_flows,
            &mut output_neg_flows,
        )
        .unwrap();

        // First 13 values should be NaN
        for value in output_mfi.iter().take(14) {
            assert!(value.is_nan());
        }

        // Compare with known values
        let expected_values = [
            30.014_661_355_061_06,
            23.918_092_608_653_822,
            19.698_868_438_822_11,
            28.256_916_470_899_76,
            33.135_317_878_529_85,
            28.508_181_473_309_836,
            26.506_381_443_527_35,
            20.718_757_833_025_176,
            24.742_436_684_915_482,
            28.621_541_030_827_412,
            32.847_079_830_044_35,
            38.195_318_060_163_59,
            34.931_258_232_645_05,
            37.643_920_900_904_62,
            39.487_301_568_579_9,
            50.487_923_737_718_95,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_mfi[i + 14], *expected, epsilon = 0.0001);
        }
    }
}
