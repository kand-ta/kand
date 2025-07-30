use super::sma;
use crate::{KandError, TAFloat};

/// Returns the lookback period required for Average Daily Range (ADR) calculation.
///
/// The lookback period is the number of data points needed before the first valid ADR value, equal to the period minus one.
///
/// # Errors
///
/// - [`KandError::InvalidParameter`] if period is less than 2 (enabled by "check" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::adr;
/// let lookback = adr::lookback(14).unwrap();
/// assert_eq!(lookback, 13);
/// ```
#[must_use]
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    sma::lookback(param_period)
}

/// Calculates the Average Daily Range (ADR) for the entire price series.
///
/// The ADR measures the average price range (High - Low) over a specified period, indicating market volatility.
///
/// # Formula
///
/// ```text
/// Daily Range = High - Low
/// ADR = SMA(Daily Range, period)
/// ```
///
/// # Calculation
///
/// 1. Compute the daily range (High - Low) for each period.
/// 2. Apply a Simple Moving Average (SMA) to the daily ranges.
/// 3. Set the first `period - 1` values to NaN, as they lack sufficient data.
///
/// # Errors
///
/// - [`KandError::InvalidData`] if input arrays are empty (enabled by "check" feature).
/// - [`KandError::InsufficientData`] if input length is less than or equal to lookback (enabled by "check" feature).
/// - [`KandError::LengthMismatch`] if input or output arrays have different lengths (enabled by "check" feature).
/// - [`KandError::InvalidParameter`] if period is less than 2 (propagated from SMA).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::adr;
/// let input_high = vec![10.0, 12.0, 15.0, 14.0, 13.0];
/// let input_low = vec![8.0, 9.0, 11.0, 10.0, 9.0];
/// let param_period = 3;
/// let mut output_adr = vec![0.0; 5];
///
/// adr::adr(&input_high, &input_low, param_period, &mut output_adr).unwrap();
/// ```
pub fn adr(
    input_high: &[TAFloat],
    input_low: &[TAFloat],
    param_period: usize,
    output_adr: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_high.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        if len != input_low.len() || len != output_adr.len() {
            return Err(KandError::LengthMismatch);
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

    let mut ranges = Vec::with_capacity(len);
    ranges.extend(
        input_high
            .iter()
            .zip(input_low.iter())
            .map(|(&h, &l)| h - l),
    );

    sma::sma(&ranges, param_period, output_adr)
}

/// Calculates the latest Average Daily Range (ADR) value incrementally using the previous ADR value.
///
/// This optimized version computes only the latest ADR value, avoiding recalculation of the entire series.
///
/// # Formula
///
/// ```text
/// New Range = New High - New Low
/// Old Range = Old High - Old Low
/// Latest ADR = SMA_inc(Previous ADR, New Range, Old Range, period)
/// ```
///
/// # Errors
///
/// - [`KandError::InvalidParameter`] if period is less than 2 (enabled by "check" feature).
/// - [`KandError::NaNDetected`] if any input contains NaN values (enabled by "check-nan" feature).
///
/// # Examples
///
/// ```
/// use kand::ohlcv::adr;
/// let prev_adr = 3.0;
/// let new_high = 15.0;
/// let new_low = 12.0;
/// let old_high = 10.0;
/// let old_low = 8.0;
/// let period = 14;
///
/// let next_adr = adr::adr_inc(prev_adr, new_high, new_low, old_high, old_low, period).unwrap();
/// ```
#[must_use]
pub fn adr_inc(
    prev_adr: TAFloat,
    input_new_high: TAFloat,
    input_new_low: TAFloat,
    input_old_high: TAFloat,
    input_old_low: TAFloat,
    param_period: usize,
) -> Result<TAFloat, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        if input_new_high.is_nan()
            || input_new_low.is_nan()
            || input_old_high.is_nan()
            || input_old_low.is_nan()
            || prev_adr.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let new_range = input_new_high - input_new_low;
    let old_range = input_old_high - input_old_low;

    sma::sma_inc(prev_adr, new_range, old_range, param_period)
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    /// Tests the ADR calculation for a full series and verifies incremental calculations match.
    #[test]
    fn test_adr_calculation() {
        const EPSILON: f64 = 1e-9; // Local epsilon for this test

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
        let mut output_adr = vec![0.0; input_high.len()];
        let period = 3;

        adr(&input_high, &input_low, period, &mut output_adr).unwrap();

        let expected_values = [
            std::f64::NAN,
            std::f64::NAN,
            48.866_666_666_666_183_971_30,
            52.266_666_666_667_639_162_82,
            48.066_666_666_668_119_489_71,
            57.266_666_666_670_062_113_55,
            57.333_333_333_335_758_652_54,
            73.866_666_666_668_606_922_03,
            73.100_000_000_000_974_864_63,
            67.666_666_666_666_671_403_62,
            46.500_000_000_000_000_000_00,
            44.800_000_000_000_487_432_32,
            48.466_666_666_667_151_730_51,
            68.366_666_666_666_176_865_87,
            87.233_333_333_332_367_942_60,
            104.833_333_333_333_328_596_38,
            112.633_333_333_333_823_134_13,
            86.300_000_000_000_480_326_89,
            72.533_333_333_332_848_269_49,
            60.933_333_333_334_303_461_02,
            77.266_666_666_667_632_057_40,
            79.566_666_666_668_126_595_14,
            78.066_666_666_668_126_595_14,
            66.966_666_666_667_151_730_51,
            59.966_666_666_667_151_730_51,
            66.333_333_333_333_328_596_38,
            66.233_333_333_334_783_787_90,
            57.433_333_333_334_303_461_02,
            42.333_333_333_333_335_701_81,
            54.733_333_333_332_360_837_18,
            97.699_999_999_999_519_673_11,
            113.066_666_666_665_696_538_98,
            97.033_333_333_332_848_269_49,
            60.599_999_999_998_544_808_48,
            55.599_999_999_998_544_808_48,
            51.566_666_666_665_696_538_98,
            64.333_333_333_333_328_596_38,
            69.333_333_333_333_328_596_38,
            71.366_666_666_666_176_865_87,
            65.266_666_666_665_216_212_10,
            52.366_666_666_666_183_971_30,
            47.833_333_333_333_335_701_81,
            58.100_000_000_000_967_759_21,
            59.566_666_666_665_696_538_98,
            58.233_333_333_332_360_837_18,
            40.533_333_333_332_848_269_49,
            37.533_333_333_335_271_220_22,
            39.533_333_333_335_271_220_22,
            39.000_000_000_000_000_000_00,
            37.166_666_666_666_664_298_19,
            48.766_666_666_667_639_162_82,
            57.466_666_666_667_151_730_51,
            63.166_666_666_666_664_298_19,
            58.033_333_333_332_848_269_49,
            54.933_333_333_334_303_461_02,
            52.000_000_000_000_000_000_00,
            48.066_666_666_665_696_538_98,
            65.766_666_666_665_216_212_10,
            70.399_999_999_999_025_135_37,
            69.600_000_000_000_974_864_63,
            52.400_000_000_001_455_191_52,
            66.600_000_000_000_974_864_63,
            60.866_666_666_666_183_971_30,
            61.400_000_000_001_455_191_52,
            44.233_333_333_334_790_893_33,
            48.466_666_666_667_151_730_51,
            43.933_333_333_331_880_510_29,
            47.533_333_333_332_848_269_49,
            46.300_000_000_000_487_432_32,
            47.300_000_000_000_487_432_32,
            39.300_000_000_000_487_432_32,
            35.966_666_666_667_151_730_51,
            33.300_000_000_000_487_432_32,
            44.466_666_666_667_151_730_51,
            61.300_000_000_000_487_432_32,
            73.300_000_000_000_480_326_89,
            86.199_999_999_999_519_673_11,
            75.733_333_333_332_367_942_60,
            60.000_000_000_000_000_000_00,
            34.533_333_333_335_271_220_22,
            38.833_333_333_335_758_652_54,
            56.800_000_000_000_487_432_32,
            58.566_666_666_665_696_538_98,
            43.533_333_333_330_425_318_76,
            28.599_999_999_998_544_808_48,
            29.599_999_999_998_544_808_48,
            32.966_666_666_667_151_730_51,
            29.566_666_666_665_696_538_98,
            27.099_999_999_998_544_808_48,
            22.966_666_666_664_725_227_06,
            33.333_333_333_333_335_701_81,
            42.166_666_666_669_094_354_35,
            57.433_333_333_334_303_461_02,
            42.033_333_333_332_848_269_49,
            41.199_999_999_997_089_616_95,
            33.033_333_333_332_848_269_49,
            46.500_000_000_000_000_000_00,
            50.000_000_000_000_000_000_00,
            51.799_999_999_998_057_376_16,
            39.699_999_999_997_089_616_95,
        ];

        for (i, &expected) in expected_values.iter().enumerate() {
            if expected.is_nan() {
                assert!(output_adr[i].is_nan());
            } else {
                assert_relative_eq!(output_adr[i], expected, epsilon = EPSILON);
            }
        }

        let lookback = lookback(period).unwrap();
        let mut prev_adr = output_adr[lookback];

        for i in (lookback + 1)..input_high.len() {
            let next_adr = adr_inc(
                prev_adr,
                input_high[i],
                input_low[i],
                input_high[i - period],
                input_low[i - period],
                period,
            )
            .unwrap();

            assert_relative_eq!(next_adr, output_adr[i], epsilon = EPSILON);

            prev_adr = next_adr;
        }
    }
}
