use crate::{
    KandError, TAFloat,
    ta::{ohlcv::sma, stats::var},
};

/// Returns the lookback period required for Bollinger Bands calculation.
///
/// # Description
/// The lookback period represents the minimum number of data points needed before
/// the first valid output can be calculated. For Bollinger Bands, this equals
/// the specified period parameter.
///
/// # Arguments
/// * `param_period` - The time period used for calculations (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period on success, or error on failure
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
///
/// # Example
/// ```
/// use kand::ta::ohlcv::bbands;
/// let period = 20;
/// let lookback = bbands::lookback(period).unwrap();
/// assert_eq!(lookback, 19);
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    sma::lookback(param_period)
}

/// Calculates Bollinger Bands for a price series.
///
/// # Description
/// Bollinger Bands are volatility bands placed above and below a moving average.
/// They consist of:
/// - A middle band (N-period simple moving average)
/// - An upper band (K standard deviations above middle band)
/// - A lower band (K standard deviations below middle band)
///
/// # Mathematical Formula
/// ```text
/// Middle Band = SMA(price, N)
/// Standard Deviation = sqrt(sum((price - SMA)^2) / N)
/// Upper Band = Middle Band + (K × Standard Deviation)
/// Lower Band = Middle Band - (K × Standard Deviation)
/// ```
/// where:
/// - N is the period
/// - K is the number of standard deviations
///
/// # Calculation Steps
/// 1. Calculate N-period SMA as middle band
/// 2. Calculate N-period standard deviation
/// 3. Add/subtract K standard deviations to get upper/lower bands
///
/// # Arguments
/// * `input_price` - Slice of input price values
/// * `param_period` - The time period for calculations (must be >= 2)
/// * `param_dev_up` - Number of standard deviations for upper band
/// * `param_dev_down` - Number of standard deviations for lower band
/// * `output_upper` - Buffer to store upper band values
/// * `output_middle` - Buffer to store middle band values
/// * `output_lower` - Buffer to store lower band values
/// * `output_sma` - Buffer to store SMA values
/// * `output_var` - Buffer to store variance values
/// * `output_sum` - Buffer to store running sum values
/// * `output_sum_sq` - Buffer to store running sum of squares values
///
/// # Returns
/// * `Result<(), KandError>` - Empty result on success, or error on failure
///
/// # Errors
/// * `KandError::InvalidData` - If input slice is empty
/// * `KandError::LengthMismatch` - If input and output slices have different lengths
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::InsufficientData` - If input length is less than required period
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ta::ohlcv::bbands;
/// let prices = vec![10.0, 11.0, 12.0, 13.0, 14.0];
/// let period = 3;
/// let mut upper = vec![0.0; 5];
/// let mut middle = vec![0.0; 5];
/// let mut lower = vec![0.0; 5];
/// let mut sma = vec![0.0; 5];
/// let mut var = vec![0.0; 5];
/// let mut sum = vec![0.0; 5];
/// let mut sum_sq = vec![0.0; 5];
///
/// bbands::bbands(
///     &prices,
///     period,
///     2.0,
///     2.0,
///     &mut upper,
///     &mut middle,
///     &mut lower,
///     &mut sma,
///     &mut var,
///     &mut sum,
///     &mut sum_sq,
/// )
/// .unwrap();
/// ```
pub fn bbands(
    input_price: &[TAFloat],
    param_period: usize,
    param_dev_up: TAFloat,
    param_dev_down: TAFloat,
    output_upper: &mut [TAFloat],
    output_middle: &mut [TAFloat],
    output_lower: &mut [TAFloat],
    output_sma: &mut [TAFloat],
    output_var: &mut [TAFloat],
    output_sum: &mut [TAFloat],
    output_sum_sq: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_price.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Data sufficiency check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }

        // Length check
        if len != output_upper.len()
            || len != output_middle.len()
            || len != output_lower.len()
            || len != output_sma.len()
            || len != output_var.len()
            || len != output_sum.len()
            || len != output_sum_sq.len()
        {
            return Err(KandError::LengthMismatch);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        for price in input_price {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate SMA first
    sma::sma(input_price, param_period, output_sma)?;

    // Calculate variance
    var::var(
        input_price,
        param_period,
        output_var,
        output_sum,
        output_sum_sq,
    )?;

    for i in lookback..len {
        output_middle[i] = output_sma[i];
        let std_dev = output_var[i].sqrt();

        // Calculate upper and lower bands using standard deviations
        output_upper[i] = param_dev_up.mul_add(std_dev, output_sma[i]);
        output_lower[i] = param_dev_down.mul_add(-std_dev, output_sma[i]);
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_upper[i] = TAFloat::NAN;
        output_middle[i] = TAFloat::NAN;
        output_lower[i] = TAFloat::NAN;
        output_sma[i] = TAFloat::NAN;
        output_var[i] = TAFloat::NAN;
        output_sum[i] = TAFloat::NAN;
        output_sum_sq[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the next Bollinger Bands values using an incremental approach.
///
/// # Description
/// This function provides an optimized way to calculate the next set of Bollinger Bands values
/// when new data arrives, without recalculating the entire series. It uses the previous values
/// to compute the new bands efficiently.
///
/// # Calculation Steps
/// 1. Calculate new SMA using incremental approach
/// 2. Calculate new variance using incremental approach
/// 3. Compute standard deviation and bands
///
/// # Arguments
/// * `input_price` - The current price value
/// * `prev_sma` - The previous SMA value
/// * `prev_sum` - The previous sum for variance calculation
/// * `prev_sum_sq` - The previous sum of squares for variance calculation
/// * `input_old_price` - The oldest price value to be removed from the period
/// * `param_period` - The time period for calculations (must be >= 2)
/// * `param_dev_up` - Number of standard deviations for upper band
/// * `param_dev_down` - Number of standard deviations for lower band
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - A tuple containing:
///   - Upper Band value
///   - Middle Band value
///   - Lower Band value
///   - New SMA value
///   - New Sum value
///   - New Sum of Squares value
///
/// # Errors
/// * `KandError::InvalidParameter` - If period is less than 2
/// * `KandError::NaNDetected` - If any input contains NaN values
///
/// # Example
/// ```
/// use kand::ta::ohlcv::bbands;
/// let (upper, middle, lower, sma, sum, sum_sq) = bbands::bbands_inc(
///     10.0,   // new price
///     9.5,    // previous SMA
///     28.5,   // previous sum
///     272.25, // previous sum of squares
///     9.0,    // oldest price
///     3,      // period
///     2.0,    // upper deviation
///     2.0,    // lower deviation
/// )
/// .unwrap();
/// ```
pub fn bbands_inc(
    input_price: TAFloat,
    prev_sma: TAFloat,
    prev_sum: TAFloat,
    prev_sum_sq: TAFloat,
    input_old_price: TAFloat,
    param_period: usize,
    param_dev_up: TAFloat,
    param_dev_down: TAFloat,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "deep-check")]
    {
        if input_price.is_nan()
            || prev_sma.is_nan()
            || prev_sum.is_nan()
            || prev_sum_sq.is_nan()
            || input_old_price.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
        if param_dev_up.is_nan() || param_dev_down.is_nan() {
            return Err(KandError::NaNDetected);
        }
    }

    // Calculate new SMA using incremental SMA
    let new_sma = sma::sma_inc(prev_sma, input_price, input_old_price, param_period)?;

    // Calculate new variance using incremental variance
    let (new_variance, new_sum, new_sum_sq) = var::var_inc(
        input_price,
        prev_sum,
        prev_sum_sq,
        input_old_price,
        param_period,
    )?;

    let std_dev = new_variance.sqrt();
    let upper = param_dev_up.mul_add(std_dev, new_sma);
    let lower = param_dev_down.mul_add(-std_dev, new_sma);

    Ok((upper, new_sma, lower, new_sma, new_sum, new_sum_sq))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_bbands_calculation() {
        let input_price = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0, 35114.5, 35097.2,
            35092.0, 35073.2, 35139.3, 35092.0, 35126.7, 35106.3, 35124.8, 35170.1, 35215.3,
            35154.0, 35216.3, 35211.8, 35158.4, 35172.0, 35176.7, 35113.3, 35114.7, 35129.3,
        ];

        let param_period = 20;
        let param_dev_up = 2.0;
        let param_dev_down = 2.0;
        let mut output_upper = vec![0.0; input_price.len()];
        let mut output_middle = vec![0.0; input_price.len()];
        let mut output_lower = vec![0.0; input_price.len()];
        let mut output_sma = vec![0.0; input_price.len()];
        let mut output_var = vec![0.0; input_price.len()];
        let mut output_sum = vec![0.0; input_price.len()];
        let mut output_sum_sq = vec![0.0; input_price.len()];

        bbands(
            &input_price,
            param_period,
            param_dev_up,
            param_dev_down,
            &mut output_upper,
            &mut output_middle,
            &mut output_lower,
            &mut output_sma,
            &mut output_var,
            &mut output_sum,
            &mut output_sum_sq,
        )
        .unwrap();

        // First 19 values should be NaN
        for i in 0..19 {
            assert!(output_upper[i].is_nan());
            assert!(output_middle[i].is_nan());
            assert!(output_lower[i].is_nan());
            assert!(output_sma[i].is_nan());
            assert!(output_var[i].is_nan());
            assert!(output_sum[i].is_nan());
            assert!(output_sum_sq[i].is_nan());
        }

        // Compare with known values
        let expected_upper = vec![
            35_315.492_158_169_03,
            35_324.023_520_348_93,
            35_323.822_186_479_93,
            35_319.449_647_081_79,
            35_314.110_592_229_015,
            35_306.809_201_120_76,
            35_288.014_966_586_7,
            35_276.648_971_890_07,
            35_253.671_769_987_47,
            35_239.448_423_376_95,
            35_232.360_028_616_255,
            35_221.822_196_455_76,
            35_200.867_114_660_425,
            35_179.557_912_368_81,
            35_172.678_349_978_625,
            35_185.898_169_265_74,
            35_210.535_957_882_84,
            35_218.090_674_365_98,
            35_236.434_486_030_11,
            35_252.252_647_217_1,
            35_257.112_658_379_57,
            35_250.714_615_459_73,
            35_240.881_227_372_27,
            35_230.468_530_636_03,
            35_225.037_992_782_84,
            35_223.587_496_067_295,
        ];
        let expected_middle = vec![
            35_154.365_000_000_005,
            35140.535,
            35127.095,
            35_117.560_000_000_005,
            35_111.150_000_000_01,
            35_106.075_000_000_004,
            35_099.070_000_000_01,
            35093.79,
            35085.795,
            35079.575,
            35_077.305_000_000_01,
            35_073.150_000_000_01,
            35_067.990_000_000_005,
            35_062.680_000_000_01,
            35_060.885_000_000_01,
            35_064.875_000_000_01,
            35_073.580_000_000_01,
            35_081.315_000_000_01,
            35_091.460_000_000_01,
            35_098.600_000_000_01,
            35_105.290_000_000_015,
            35_116.915_000_000_015,
            35_128.120_000_000_01,
            35_133.785_000_000_02,
            35_137.430_000_000_01,
            35_139.895_000_000_01,
        ];
        let expected_lower = vec![
            34_993.237_841_830_98,
            34_957.046_479_651_08,
            34_930.367_813_520_075,
            34_915.670_352_918_22,
            34_908.189_407_771,
            34_905.340_798_879_246,
            34_910.125_033_413_315,
            34_910.931_028_109_93,
            34_917.918_230_012_525,
            34_919.701_576_623_05,
            34_922.249_971_383_76,
            34_924.477_803_544_26,
            34_935.112_885_339_586,
            34_945.802_087_631_21,
            34_949.091_650_021_39,
            34_943.851_830_734_275,
            34_936.624_042_117_175,
            34_944.539_325_634_04,
            34_946.485_513_969_9,
            34_944.947_352_782_925,
            34_953.467_341_620_46,
            34_983.115_384_540_3,
            35_015.358_772_627_75,
            35_037.101_469_364,
            35_049.822_007_217_175,
            35_056.202_503_932_73,
        ];

        for i in 0..expected_upper.len() {
            assert_relative_eq!(output_upper[i + 19], expected_upper[i], epsilon = 0.0001);
            assert_relative_eq!(output_middle[i + 19], expected_middle[i], epsilon = 0.0001);
            assert_relative_eq!(output_lower[i + 19], expected_lower[i], epsilon = 0.0001);
        }

        // Test incremental calculation
        let mut prev_sma = output_sma[19];
        let mut prev_sum = output_sum[19];
        let mut prev_sum_sq = output_sum_sq[19];

        for i in 20..45 {
            let (upper, middle, lower, new_sma, new_sum, new_sum_sq) = bbands_inc(
                input_price[i],
                prev_sma,
                prev_sum,
                prev_sum_sq,
                input_price[i - param_period],
                param_period,
                param_dev_up,
                param_dev_down,
            )
            .unwrap();

            assert_relative_eq!(upper, output_upper[i], epsilon = 0.0001);
            assert_relative_eq!(middle, output_middle[i], epsilon = 0.0001);
            assert_relative_eq!(lower, output_lower[i], epsilon = 0.0001);

            prev_sma = new_sma;
            prev_sum = new_sum;
            prev_sum_sq = new_sum_sq;
        }
    }
}
