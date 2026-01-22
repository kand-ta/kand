use super::typprice;
use crate::{KandError, TAFloat};

/// Returns the lookback period required for VWAP calculation.
///
/// # Function Overview
/// Determines the minimum number of data points needed before VWAP can be calculated.
///
/// # Mathematical Formula
/// No formula required - VWAP has no lookback period.
///
/// # Calculation Principle
/// VWAP can be calculated from the first data point, so lookback is always 0.
///
/// # Parameters
/// None
///
/// # Returns
/// * `Result<usize, KandError>` - Returns 0 as the lookback period
///
/// # Errors
/// None - this function cannot fail
///
/// # Example
/// ```
/// use kand::ohlcv::vwap;
/// let lookback = vwap::lookback().unwrap();
/// assert_eq!(lookback, 0); // VWAP has no lookback period
/// ```
pub const fn lookback() -> Result<usize, KandError> {
    Ok(0)
}

/// Calculates Volume Weighted Average Price (VWAP).
///
/// # Function Overview
/// VWAP is a trading benchmark that calculates the average price weighted by volume over a period.
/// It helps traders understand both price trends and trading activity.
///
/// # Mathematical Formula
/// ```text
/// Typical Price = (High + Low + Close) / 3
/// VWAP = Σ(Typical Price * Volume) / Σ(Volume)
/// ```
///
/// # Calculation Principle
/// 1. Calculate typical price for each period
/// 2. Multiply typical price by volume
/// 3. Keep running sum of price-volume products and volumes
/// 4. Divide cumulative price-volume by cumulative volume
///
/// # Parameters
/// * `input_high` - Array of high prices for each period
/// * `input_low` - Array of low prices for each period
/// * `input_close` - Array of closing prices for each period
/// * `input_volume` - Array of trading volumes for each period
/// * `output_vwap` - Mutable slice to store calculated VWAP values
/// * `output_cum_pv` - Mutable slice to store cumulative price-volume products
/// * `output_cum_vol` - Mutable slice to store cumulative volumes
///
/// # Returns
/// * `Result<(), KandError>` - Unit type on successful calculation
///
/// # Errors
/// * `KandError::InvalidData` - If input arrays are empty
/// * `KandError::LengthMismatch` - If input/output arrays have different lengths
/// * `KandError::NaNDetected` - If any input contains NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::ohlcv::vwap;
///
/// let high = vec![10.0, 12.0, 15.0];
/// let low = vec![8.0, 9.0, 11.0];
/// let close = vec![9.0, 10.0, 12.0];
/// let volume = vec![100.0, 150.0, 200.0];
/// let mut vwap_values = vec![0.0; 3];
/// let mut cum_pv = vec![0.0; 3];
/// let mut cum_vol = vec![0.0; 3];
///
/// vwap::vwap(
///     &high,
///     &low,
///     &close,
///     &volume,
///     &mut vwap_values,
///     &mut cum_pv,
///     &mut cum_vol,
/// )
/// .unwrap();
/// ```
pub fn vwap(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    input_volume: &[TAFloat],
    output_vwap: &mut [TAFloat],
    output_cum_pv: &mut [TAFloat],
    output_cum_vol: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if input_low.len() != len
            || input_close.len() != len
            || input_volume.len() != len
            || output_vwap.len() != len
            || output_cum_pv.len() != len
            || output_cum_vol.len() != len
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        for i in 0..len {
            if input_high[i].is_nan()
                || input_low[i].is_nan()
                || input_close[i].is_nan()
                || input_volume[i].is_nan()
            {
                return Err(KandError::NaNDetected);
            }
        }
    }

    let mut cum_pv = 0.0;
    let mut cum_vol = 0.0;

    for i in 0..len {
        let (new_cum_pv, new_cum_vol, vwap) = vwap_inc(
            input_high[i],
            input_low[i],
            input_close[i],
            input_volume[i],
            cum_pv,
            cum_vol,
        )?;

        cum_pv = new_cum_pv;
        cum_vol = new_cum_vol;

        output_cum_pv[i] = cum_pv;
        output_cum_vol[i] = cum_vol;
        output_vwap[i] = vwap;
    }

    Ok(())
}

/// Calculates the next VWAP value incrementally.
///
/// # Function Overview
/// Updates cumulative price-volume product and volume for incremental VWAP calculation.
///
/// # Mathematical Formula
/// ```text
/// Cumulative PV = Previous Cumulative PV + (Typical Price * Volume)
/// Cumulative Volume = Previous Cumulative Volume + Volume
/// VWAP = Cumulative PV / Cumulative Volume
/// ```
///
/// # Calculation Principle
/// Maintains running sums and calculates VWAP by dividing cumulative values.
///
/// # Parameters
/// * `high` - High price for current period
/// * `low` - Low price for current period
/// * `close` - Close price for current period
/// * `volume` - Volume for current period
/// * `prev_cum_pv` - Previous cumulative price-volume product
/// * `prev_cum_vol` - Previous cumulative volume
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing (new cumulative PV, new cumulative volume, new VWAP)
///
/// # Errors
/// None - this function cannot fail
///
/// # Example
/// ```
/// use kand::ohlcv::vwap;
///
/// let high = 10.0;
/// let low = 8.0;
/// let close = 9.0;
/// let volume = 100.0;
/// let prev_cum_pv = 1000.0;
/// let prev_cum_vol = 150.0;
/// let (new_cum_pv, new_cum_vol, new_vwap) =
///     vwap::vwap_inc(high, low, close, volume, prev_cum_pv, prev_cum_vol).unwrap();
/// ```
pub fn vwap_inc(
    high: TAFloat,
    low: TAFloat,
    close: TAFloat,
    volume: TAFloat,
    prev_cum_pv: TAFloat,
    prev_cum_vol: TAFloat,
) -> Result<(TAFloat, TAFloat, TAFloat), KandError> {
    let typ_price = typprice::typprice_inc(high, low, close)?;
    let cum_pv = typ_price.mul_add(volume, prev_cum_pv);
    let cum_vol = prev_cum_vol + volume;
    let vwap = if cum_vol == 0.0 {
        0.0
    } else {
        cum_pv / cum_vol
    };
    Ok((cum_pv, cum_vol, vwap))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_vwap_calculation() {
        let input_high = vec![
            96955.7, 96850.0, 96787.8, 97163.0, 97212.0, 96870.7, 96824.2, 97041.9, 96979.8,
            97127.0, 97150.0, 97094.5, 96844.7, 96660.0,
        ];
        let input_low = vec![
            96490.7, 96309.5, 96407.1, 96492.8, 96707.0, 96505.0, 96556.2, 96765.8, 96743.4,
            96782.4, 96916.4, 96750.1, 96436.1, 96507.3,
        ];
        let input_close = vec![
            96708.6, 96497.4, 96495.2, 97094.9, 96715.4, 96635.9, 96786.6, 96889.9, 96828.0,
            97062.0, 96965.8, 96844.6, 96612.3, 96531.2,
        ];
        let input_volume = vec![
            3746.917, 3260.9, 2899.859, 4050.52, 4249.375, 2782.823, 2384.87, 3234.131, 2350.488,
            3032.885, 2050.853, 2505.323, 3741.102, 811.82,
        ];
        let mut output_vwap = vec![0.0; 14];
        let mut output_cum_pv = vec![0.0; 14];
        let mut output_cum_vol = vec![0.0; 14];

        vwap(
            &input_high,
            &input_low,
            &input_close,
            &input_volume,
            &mut output_vwap,
            &mut output_cum_pv,
            &mut output_cum_vol,
        )
        .unwrap();

        // Expected VWAP values from the sample data
        let expected_values = [
            96_718.333_333_333_33,
            96_641.074_167_366_7,
            96_618.330_105_563_28,
            96_704.971_912_915_3,
            96_745.385_200_930_96,
            96_735.461_637_859_99,
            96_734.122_217_709_84,
            96_754.185_927_279_93,
            96_761.995_006_596_99,
            96_783.653_909_970_1,
            96_797.333_609_807_04,
            96_804.124_320_528_47,
            96_788.052_086_747_02,
            96_783.669_535_800_8,
        ];

        // Compare calculated values with expected values
        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_vwap[i], *expected, epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_cum_pv = 0.0;
        let mut prev_cum_vol = 0.0;

        for i in 0..input_high.len() {
            let (new_cum_pv, new_cum_vol, vwap) = vwap_inc(
                input_high[i],
                input_low[i],
                input_close[i],
                input_volume[i],
                prev_cum_pv,
                prev_cum_vol,
            )
            .unwrap();

            assert_relative_eq!(vwap, expected_values[i], epsilon = 0.0001);
            prev_cum_pv = new_cum_pv;
            prev_cum_vol = new_cum_vol;
        }
    }
}
