use super::{ad, ema};
use crate::{KandError, TAFloat};

/// Returns the lookback period required for A/D Oscillator calculation.
///
/// The A/D Oscillator requires a lookback equal to the slow EMA period minus one.
///
/// # Errors
///
/// - [`KandError::InvalidParameter`] if fast or slow period is less than 2, or if fast period is not less than slow period (enabled by "check" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::adosc;
/// let lookback = adosc::lookback(3, 10).unwrap();
/// assert_eq!(lookback, 9);
/// ```
#[must_use]
pub const fn lookback(
    param_fast_period: usize,
    param_slow_period: usize,
) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_fast_period < 2 || param_slow_period < 2 || param_fast_period >= param_slow_period
        {
            return Err(KandError::InvalidParameter);
        }
    }

    ema::lookback(param_slow_period)
}

/// Calculates the Accumulation/Distribution Oscillator (A/D Oscillator or ADOSC) for the entire price series.
///
/// The A/D Oscillator is the difference between fast and slow EMAs of the Accumulation/Distribution (A/D) line.
/// It helps identify trend strength and potential reversals by measuring momentum in money flow.
///
/// # Formula
///
/// ```text
/// Money Flow Multiplier (MFM) = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume (MFV) = MFM * Volume
/// A/D = Previous A/D + MFV
/// ADOSC = EMA(A/D, fast_period) - EMA(A/D, slow_period)
/// ```
///
/// # Calculation
///
/// 1. Compute the A/D line as the cumulative sum of MFV (see A/D documentation for details).
/// 2. Calculate the fast EMA of the A/D line.
/// 3. Calculate the slow EMA of the A/D line.
/// 4. Subtract the slow EMA from the fast EMA to get ADOSC.
///
/// If High - Low is zero, MFM is set to 0 to avoid division by zero.
/// Outputs for the first `lookback` periods are set to NaN.
///
/// # Errors
///
/// - [`KandError::InvalidData`] if input arrays are empty (enabled by "check" feature).
/// - [`KandError::InsufficientData`] if input length is less than or equal to lookback (enabled by "check" feature).
/// - [`KandError::LengthMismatch`] if input or output arrays have different lengths (enabled by "check" feature).
/// - [`KandError::InvalidParameter`] if periods are invalid (propagated from lookback).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::adosc;
/// let high = vec![10.0, 11.0, 12.0, 11.5, 10.5];
/// let low = vec![8.0, 9.0, 10.0, 9.5, 8.5];
/// let close = vec![9.0, 10.0, 11.0, 10.0, 9.0];
/// let volume = vec![100.0, 150.0, 200.0, 150.0, 100.0];
/// let mut adosc_out = vec![0.0; 5];
/// let mut ad_out = vec![0.0; 5];
/// let mut ad_fast_ema = vec![0.0; 5];
/// let mut ad_slow_ema = vec![0.0; 5];
///
/// adosc::adosc(
///     &high,
///     &low,
///     &close,
///     &volume,
///     3,
///     5,
///     &mut adosc_out,
///     &mut ad_out,
///     &mut ad_fast_ema,
///     &mut ad_slow_ema,
/// )
/// .unwrap();
/// ```
pub fn adosc(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    input_close: &[TAFloat],
    input_volume: &[TAFloat],
    param_fast_period: usize,
    param_slow_period: usize,
    output_adosc: &mut [TAFloat],
    output_ad: &mut [TAFloat],
    output_ad_fast_ema: &mut [TAFloat],
    output_ad_slow_ema: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(param_fast_period, param_slow_period)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        if len != input_low.len()
            || len != input_close.len()
            || len != input_volume.len()
            || len != output_adosc.len()
            || len != output_ad.len()
            || len != output_ad_fast_ema.len()
            || len != output_ad_slow_ema.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "check-nan")]
    {
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

    ad::ad(input_high, input_low, input_close, input_volume, output_ad)?;

    ema::ema(output_ad, param_fast_period, None, output_ad_fast_ema)?;
    ema::ema(output_ad, param_slow_period, None, output_ad_slow_ema)?;

    for i in lookback..len {
        output_adosc[i] = output_ad_fast_ema[i] - output_ad_slow_ema[i];
    }

    Ok(())
}

/// Calculates the latest A/D Oscillator value incrementally using previous values.
///
/// This is an optimized version that computes only the latest ADOSC value, avoiding recalculation of the entire series.
///
/// # Formula
///
/// ```text
/// Money Flow Multiplier (MFM) = ((Close - Low) - (High - Close)) / (High - Low)
/// Money Flow Volume (MFV) = MFM * Volume
/// Latest A/D = Previous A/D + MFV
/// Latest Fast EMA = (Latest A/D - Previous Fast EMA) * (2 / (fast_period + 1)) + Previous Fast EMA
/// Latest Slow EMA = (Latest A/D - Previous Slow EMA) * (2 / (slow_period + 1)) + Previous Slow EMA
/// Latest ADOSC = Latest Fast EMA - Latest Slow EMA
/// ```
///
/// If High - Low is zero, MFM is set to 0 to avoid division by zero.
///
/// # Errors
///
/// - [`KandError::InvalidParameter`] if fast or slow period is 0, or if fast period is not less than slow period (enabled by "check" feature).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::adosc;
/// let (adosc, ad, ad_fast_ema, ad_slow_ema) = adosc::adosc_inc(
///     10.5,
///     9.5,
///     10.0,
///     150.0,
///     100.0,
///     95.0,
///     90.0,
///     3,
///     10,
/// )
/// .unwrap();
/// ```
#[must_use]
pub fn adosc_inc(
    input_high: TAFloat,
    input_low: TAFloat,
    input_close: TAFloat,
    input_volume: TAFloat,
    prev_ad: TAFloat,
    prev_ad_fast_ema: TAFloat,
    prev_ad_slow_ema: TAFloat,
    param_fast_period: usize,
    param_slow_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_fast_period == 0
            || param_slow_period == 0
            || param_fast_period >= param_slow_period
        {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_high.is_nan()
            || input_low.is_nan()
            || input_close.is_nan()
            || input_volume.is_nan()
            || prev_ad.is_nan()
            || prev_ad_fast_ema.is_nan()
            || prev_ad_slow_ema.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let output_ad = ad::ad_inc(input_high, input_low, input_close, input_volume, prev_ad)?;
    let output_ad_fast_ema = ema::ema_inc(output_ad, prev_ad_fast_ema, param_fast_period, None)?;
    let output_ad_slow_ema = ema::ema_inc(output_ad, prev_ad_slow_ema, param_slow_period, None)?;
    let output_adosc = output_ad_fast_ema - output_ad_slow_ema;

    Ok((
        output_adosc,
        output_ad,
        output_ad_fast_ema,
        output_ad_slow_ema,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::EPSILON;

    use super::*;

    /// Tests the calculation of A/D Oscillator for a full series and verifies incremental calculations match.
    #[test]
    fn test_adosc_calculation() {
        let input_high = vec![
            35266.0, 35247.5, 35235.7, 35190.8, 35182.0, 35258.0, 35262.9, 35281.5, 35256.0,
            35210.0, 35185.4, 35230.0, 35241.0, 35218.1, 35212.6, 35128.9, 35047.7, 35019.5,
            35078.8, 35085.0, 35034.1, 34984.4, 35010.8, 35047.1, 35091.4, 35150.4, 35123.9,
            35110.0, 35092.1, 35179.2, 35244.9, 35150.2, 35136.0, 35133.6, 35188.0, 35215.3,
            35221.9, 35219.2, 35234.0, 35216.7, 35197.9, 35178.4, 35183.4, 35129.7, 35149.1,
            35129.3, 35125.5, 35114.5, 35120.1, 35129.4, 35105.4, 35054.1, 35034.6, 35032.9,
            35070.8, 35086.0, 35086.9, 35048.9, 34988.6, 35004.3, 34985.0, 35004.2, 35010.0,
            35041.8, 35024.7, 34982.0, 35018.0, 34978.2, 34959.5, 34965.0, 34985.3, 35002.4,
            35018.0, 34989.0, 34943.0, 34900.0, 34932.1, 34930.0, 34920.3, 34929.9, 34940.0,
            35019.7, 35009.1, 34980.2, 34977.3, 34976.1, 34969.4, 35000.0, 35010.0, 35015.9,
            35062.9, 35084.8, 35085.1, 35077.9, 35118.0, 35104.0, 35086.2, 35041.7, 35009.2,
            34994.2,
        ];
        let input_low = vec![
            35216.1, 35206.5, 35180.0, 35130.7, 35153.6, 35174.7, 35202.6, 35203.5, 35175.0,
            35166.0, 35170.9, 35154.1, 35186.0, 35143.9, 35080.1, 35021.1, 34950.1, 34966.0,
            35012.3, 35022.2, 34931.6, 34911.0, 34952.5, 34977.9, 35039.0, 35073.0, 35055.0,
            35084.0, 35060.0, 35073.1, 35090.0, 35072.0, 35078.0, 35088.0, 35124.8, 35169.4,
            35138.0, 35141.0, 35182.0, 35151.1, 35158.4, 35140.0, 35087.0, 35085.8, 35114.7,
            35086.0, 35090.6, 35074.1, 35078.4, 35100.0, 35030.2, 34986.3, 34988.1, 34973.1,
            35012.3, 35048.3, 35038.9, 34937.3, 34937.0, 34958.7, 34925.0, 34910.0, 34981.6,
            34980.2, 34982.0, 34940.9, 34970.0, 34924.7, 34922.1, 34914.0, 34955.8, 34975.0,
            34975.0, 34926.0, 34865.1, 34821.0, 34830.4, 34883.5, 34888.5, 34904.6, 34880.6,
            34934.0, 34978.5, 34965.9, 34936.4, 34942.5, 34945.0, 34969.3, 34983.8, 35003.9,
            35001.1, 35032.1, 35027.3, 35062.3, 35067.8, 35070.7, 35030.2, 34981.0, 34970.5,
            34974.5,
        ];
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
            35154.0, 35216.3, 35211.8, 35158.4, 35172.0, 35176.7, 35113.3, 35114.7, 35129.3,
            35094.6, 35114.4, 35094.5, 35116.0, 35105.4, 35050.7, 35031.3, 35008.1, 35021.4,
            35048.4, 35080.1, 35043.6, 34962.7, 34970.1, 34980.1, 34930.6, 35000.0, 34998.0,
            35024.7, 34982.1, 34972.3, 34971.6, 34953.0, 34937.0, 34964.3, 34975.1, 34995.1,
            34989.0, 34942.9, 34895.2, 34830.4, 34925.1, 34888.6, 34910.3, 34917.6, 34940.0,
            35005.4, 34980.1, 34966.8, 34976.1, 34948.6, 34969.3, 34996.5, 35004.0, 35011.0,
            35059.2, 35036.1, 35062.3, 35067.7, 35087.9, 35076.7, 35041.6, 34993.3, 34974.5,
            34990.2,
        ];
        let input_volume = vec![
            1055.365, 756.488, 682.152, 1197.747, 425.97, 859.638, 741.925, 888.477, 1043.333,
            467.901, 387.47, 566.099, 672.296, 834.915, 1854.024, 3670.795, 3761.198, 1605.442,
            1726.574, 934.713, 2199.061, 2349.823, 837.218, 1000.638, 1218.202, 2573.668, 1098.409,
            609.582, 670.489, 1637.998, 2682.922, 923.588, 554.766, 510.261, 882.672, 1087.53,
            1164.362, 991.265, 1042.659, 748.721, 469.772, 419.244, 896.583, 736.185, 510.968,
            503.042, 376.2, 592.877, 580.915, 333.615, 1106.869, 1761.343, 506.403, 1181.917,
            817.219, 727.725, 723.652, 1702.198, 769.212, 414.213, 702.499, 1083.179, 411.098,
            971.148, 774.147, 376.625, 333.361, 666.541, 418.598, 836.645, 506.807, 418.69,
            606.013, 658.819, 1776.331, 1757.305, 985.24, 607.588, 350.444, 402.724, 476.235,
            1899.96, 546.185, 233.707, 612.487, 313.292, 167.004, 298.175, 397.43, 194.525,
            685.384, 737.572, 576.129, 264.406, 577.913, 314.803, 694.229, 1253.468, 466.235,
            248.839,
        ];
        let param_fast_period = 3;
        let param_slow_period = 10;
        let mut output_adosc = vec![0.0; input_high.len()];
        let mut output_ad = vec![0.0; input_high.len()];
        let mut output_ad_fast_ema = vec![0.0; input_high.len()];
        let mut output_ad_slow_ema = vec![0.0; input_high.len()];

        adosc(
            &input_high,
            &input_low,
            &input_close,
            &input_volume,
            param_fast_period,
            param_slow_period,
            &mut output_adosc,
            &mut output_ad,
            &mut output_ad_fast_ema,
            &mut output_ad_slow_ema,
        )
        .unwrap();

        let expected_values = [
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
            -20.897_560_400_954_944,
            -113.006_596_430_246_87,
            39.233_539_470_168_466,
            90.168_278_254_455_34,
            -42.550_171_226_589_67,
            -592.067_697_274_955_4,
            -1495.471_434_093_873_8,
            -1719.284_357_755_546_8,
            -1260.209_773_442_224_6,
            -570.395_499_498_773_5,
            -511.021_422_797_917_7,
            -1032.120_377_773_458_1,
            -1051.642_820_428_754_5,
            -796.333_736_224_223_7,
            -349.880_245_462_221_4,
            83.522_227_098_310_85,
            312.478_479_876_569_96,
            456.358_684_600_642_4,
            399.132_419_401_690_64,
            301.554_565_477_041_25,
            363.412_358_422_480_8,
            -475.932_587_904_782_4,
            -658.787_445_656_951_6,
            -677.962_679_865_731_4,
            -524.481_981_814_993_1,
            -292.253_246_808_773_84,
            175.350_167_221_840_37,
            121.525_736_229_688_85,
            380.468_332_324_480_8,
            500.298_653_312_819_17,
            318.629_613_122_204_8,
            168.799_403_197_662_74,
            213.744_556_233_260_03,
            83.082_596_846_209_07,
            96.244_336_691_993_8,
            68.303_043_208_752_11,
            -45.815_725_437_386_39,
            -44.777_157_576_240_825,
            -38.413_888_892_083_83,
            116.171_265_425_306_05,
            101.693_356_373_634_74,
            -73.643_785_940_669_2,
            44.825_022_944_714_42,
            66.691_232_998_159_65,
            300.997_969_076_142_45,
            430.381_835_636_462_7,
            603.260_661_313_950_1,
            433.980_224_577_686_84,
            30.206_550_097_492_254,
            -68.468_876_257_924_42,
            -110.704_331_064_092_46,
            -299.716_667_630_496_9,
            -35.877_993_320_214_046,
            95.583_039_929_305_87,
            278.118_719_322_890_7,
            82.343_107_720_553_23,
            58.038_015_167_665_89,
            -56.179_275_588_915_516,
            -85.508_505_250_133_11,
            -116.798_729_011_984_05,
            139.916_423_976_043_46,
            281.960_199_020_270_9,
            376.670_218_959_854_54,
            313.908_880_573_037_55,
            162.537_450_978_871_22,
            -42.584_065_563_915_83,
            -548.706_875_059_190_2,
            -435.543_557_433_369_64,
            -500.571_623_447_350_7,
            -440.291_467_418_113_37,
            -372.059_546_147_532_3,
            -158.793_574_159_791_66,
            345.674_058_599_960_06,
            365.009_843_917_410_76,
            274.736_072_570_696_25,
            396.276_482_140_549_9,
            346.483_325_557_857_37,
            347.316_995_690_533_53,
            389.325_156_813_214_1,
            439.653_796_951_506_8,
            431.621_550_770_050_36,
            581.061_289_074_445_3,
            390.315_324_963_115_7,
            315.492_200_162_527_07,
            230.315_728_813_188_12,
            137.903_252_420_144_1,
            23.492_246_296_503_254,
            -156.404_749_053_747_76,
            -452.976_229_976_808_8,
            -650.802_619_576_638_8,
            -625.544_385_992_726_3,
        ];
        for (i, &expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_adosc[i], expected, epsilon = EPSILON);
        }

        let mut prev_ad = output_ad[9];
        let mut prev_ad_fast_ema = output_ad_fast_ema[9];
        let mut prev_ad_slow_ema = output_ad_slow_ema[9];

        for i in 10..input_high.len() {
            let (output_adosc_inc, output_ad_inc, output_ad_fast_ema_inc, output_ad_slow_ema_inc) =
                adosc_inc(
                    input_high[i],
                    input_low[i],
                    input_close[i],
                    input_volume[i],
                    prev_ad,
                    prev_ad_fast_ema,
                    prev_ad_slow_ema,
                    param_fast_period,
                    param_slow_period,
                )
                .unwrap();
            assert_relative_eq!(output_adosc_inc, output_adosc[i], epsilon = EPSILON);
            assert_relative_eq!(output_ad_inc, output_ad[i], epsilon = EPSILON);
            assert_relative_eq!(
                output_ad_fast_ema_inc,
                output_ad_fast_ema[i],
                epsilon = EPSILON
            );
            assert_relative_eq!(
                output_ad_slow_ema_inc,
                output_ad_slow_ema[i],
                epsilon = EPSILON
            );
            prev_ad = output_ad_inc;
            prev_ad_fast_ema = output_ad_fast_ema_inc;
            prev_ad_slow_ema = output_ad_slow_ema_inc;
        }
    }
}
