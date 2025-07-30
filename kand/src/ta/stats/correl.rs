use crate::{KandError, TAFloat};

/// Calculates the lookback period required for Correlation calculation.
///
/// The lookback period represents the number of data points needed before the first valid output
/// can be calculated. For Correlation, this equals the period minus 1.
///
/// # Arguments
/// * `param_period` - The time period for Correlation calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success, or error on failure
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period is less than 2
///
/// # Example
/// ```
/// use kand::stats::correl;
/// let period = 30;
/// let lookback = correl::lookback(period).unwrap();
/// assert_eq!(lookback, 29); // lookback is period - 1
/// ```
pub const fn lookback(param_period: usize) -> Result<usize, KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }
    Ok(param_period - 1)
}

/// Calculates the Pearson Correlation Coefficient (CORREL) for two price series.
///
/// NOTE: We don't reuse stddev here for performance reasons.
/// Calling stddev twice + covariance would require 3 passes through the data.
///
/// The Pearson Correlation Coefficient measures the linear correlation between two variables,
/// returning a value between -1 and +1, where:
/// - +1 indicates perfect positive correlation
/// - -1 indicates perfect negative correlation
/// - 0 indicates no linear correlation
///
/// # Mathematical Formula
/// ```text
/// r = [n(Σxy) - (Σx)(Σy)] / sqrt([n(Σx²) - (Σx)²][n(Σy²) - (Σy)²])
/// ```
/// Where:
/// - n is the period (number of observations)
/// - Σx is the sum of values in series 0
/// - Σy is the sum of values in series 1
/// - Σxy is the sum of products (x[i] * y[i])
/// - Σx² is the sum of squares of series 0
/// - Σy² is the sum of squares of series 1
///
/// # Calculation Steps
/// 1. Calculate initial sums for the first period
/// 2. Apply Pearson correlation formula
/// 3. For subsequent periods, update sums incrementally
/// 4. Fill initial values before lookback period with NaN
///
/// # Arguments
/// * `input_0` - First input series
/// * `input_1` - Second input series
/// * `param_period` - The time period for correlation calculation (must be >= 2)
/// * `output_correl` - Array to store calculated correlation values
/// * `output_sum_0` - Array to store running sum of series 0
/// * `output_sum_1` - Array to store running sum of series 1
/// * `output_sum_0_sq` - Array to store running sum of squares of series 0
/// * `output_sum_1_sq` - Array to store running sum of squares of series 1
/// * `output_sum_01` - Array to store running sum of products
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success, or error on failure
///
/// # Errors
/// * Returns `KandError::InvalidData` if input data is empty
/// * Returns `KandError::LengthMismatch` if arrays have different lengths
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::InsufficientData` if input length is less than period
/// * Returns `KandError::NaNDetected` if input contains NaN values (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::stats::correl;
/// let series1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
/// let series2 = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// let period = 3;
/// let mut output_correl = vec![0.0; 5];
/// let mut output_sum_0 = vec![0.0; 5];
/// let mut output_sum_1 = vec![0.0; 5];
/// let mut output_sum_0_sq = vec![0.0; 5];
/// let mut output_sum_1_sq = vec![0.0; 5];
/// let mut output_sum_01 = vec![0.0; 5];
///
/// correl::correl(
///     &series1,
///     &series2,
///     period,
///     &mut output_correl,
///     &mut output_sum_0,
///     &mut output_sum_1,
///     &mut output_sum_0_sq,
///     &mut output_sum_1_sq,
///     &mut output_sum_01,
/// )
/// .unwrap();
/// // output_correl = [NaN, NaN, 1.0, 1.0, 1.0] (perfect positive correlation)
/// ```
pub fn correl(
    input_0: &[TAFloat],
    input_1: &[TAFloat],
    param_period: usize,
    output_correl: &mut [TAFloat],
    output_sum_0: &mut [TAFloat],
    output_sum_1: &mut [TAFloat],
    output_sum_0_sq: &mut [TAFloat],
    output_sum_1_sq: &mut [TAFloat],
    output_sum_01: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_0.len();
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

        // Length consistency check - all arrays must have same length
        if len != input_1.len()
            || len != output_correl.len()
            || len != output_sum_0.len()
            || len != output_sum_1.len()
            || len != output_sum_0_sq.len()
            || len != output_sum_1_sq.len()
            || len != output_sum_01.len()
        {
            return Err(KandError::LengthMismatch);
        }

        // Parameter validation
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check for both input series
        for i in 0..len {
            if input_0[i].is_nan() || input_1[i].is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate initial sums for the first period
    let mut sum_0 = 0.0;
    let mut sum_1 = 0.0;
    let mut sum_0_sq = 0.0;
    let mut sum_1_sq = 0.0;
    let mut sum_01 = 0.0;

    // Initialize sums for the first window
    for i in 0..param_period {
        let val_0 = input_0[i];
        let val_1 = input_1[i];

        sum_0 += val_0;
        sum_1 += val_1;
        sum_0_sq += val_0 * val_0;
        sum_1_sq += val_1 * val_1;
        sum_01 += val_0 * val_1;
    }

    // Store sums for first valid position
    output_sum_0[lookback] = sum_0;
    output_sum_1[lookback] = sum_1;
    output_sum_0_sq[lookback] = sum_0_sq;
    output_sum_1_sq[lookback] = sum_1_sq;
    output_sum_01[lookback] = sum_01;

    // Calculate first correlation value
    let n = param_period as TAFloat;
    let numerator = n.mul_add(sum_01, -(sum_0 * sum_1));
    let denominator_0 = n.mul_add(sum_0_sq, -(sum_0 * sum_0));
    let denominator_1 = n.mul_add(sum_1_sq, -(sum_1 * sum_1));
    let denominator = (denominator_0 * denominator_1).sqrt();

    if denominator > 0.0 {
        output_correl[lookback] = numerator / denominator;
    } else {
        output_correl[lookback] = TAFloat::NAN;
    }

    // Calculate subsequent correlations using sliding window
    for i in param_period..len {
        // Remove old values and add new values
        let old_0 = input_0[i - param_period];
        let old_1 = input_1[i - param_period];
        let new_0 = input_0[i];
        let new_1 = input_1[i];

        // Update sums
        sum_0 = sum_0 - old_0 + new_0;
        sum_1 = sum_1 - old_1 + new_1;
        sum_0_sq = new_0.mul_add(new_0, old_0.mul_add(-old_0, sum_0_sq));
        sum_1_sq = new_1.mul_add(new_1, old_1.mul_add(-old_1, sum_1_sq));
        sum_01 = new_0.mul_add(new_1, old_0.mul_add(-old_1, sum_01));

        // Store updated sums
        output_sum_0[i] = sum_0;
        output_sum_1[i] = sum_1;
        output_sum_0_sq[i] = sum_0_sq;
        output_sum_1_sq[i] = sum_1_sq;
        output_sum_01[i] = sum_01;

        // Calculate correlation for this window
        let numerator = n.mul_add(sum_01, -(sum_0 * sum_1));
        let denominator_0 = n.mul_add(sum_0_sq, -(sum_0 * sum_0));
        let denominator_1 = n.mul_add(sum_1_sq, -(sum_1 * sum_1));
        let denominator = (denominator_0 * denominator_1).sqrt();

        if denominator > 0.0 {
            output_correl[i] = numerator / denominator;
        } else {
            output_correl[i] = TAFloat::NAN;
        }
    }

    // Fill initial values with NaN
    for i in 0..lookback {
        output_correl[i] = TAFloat::NAN;
        output_sum_0[i] = TAFloat::NAN;
        output_sum_1[i] = TAFloat::NAN;
        output_sum_0_sq[i] = TAFloat::NAN;
        output_sum_1_sq[i] = TAFloat::NAN;
        output_sum_01[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest Correlation value using incremental calculation.
///
/// This function provides an optimized way to update the Correlation value when new data arrives,
/// avoiding full recalculation of the entire series.
///
/// # Mathematical Formula
/// The correlation is updated by:
/// 1. Updating all sums by removing old values and adding new values
/// 2. Recalculating correlation using the updated sums
///
/// # Arguments
/// * `input_new_0` - The newest value from series 0 to add
/// * `input_new_1` - The newest value from series 1 to add
/// * `input_old_0` - The oldest value from series 0 to remove
/// * `input_old_1` - The oldest value from series 1 to remove
/// * `prev_sum_0` - Previous sum of series 0
/// * `prev_sum_1` - Previous sum of series 1
/// * `prev_sum_0_sq` - Previous sum of squares of series 0
/// * `prev_sum_1_sq` - Previous sum of squares of series 1
/// * `prev_sum_01` - Previous sum of products
/// * `param_period` - The time period (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing:
///   - New correlation value
///   - New sum of series 0
///   - New sum of series 1
///   - New sum of squares of series 0
///   - New sum of squares of series 1
///   - New sum of products
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::NaNDetected` if any input contains NaN (with "`check-nan`" feature)
///
/// # Example
/// ```
/// use kand::stats::correl;
/// let prev_sum_0 = 6.0;      // sum of [1.0, 2.0, 3.0]
/// let prev_sum_1 = 12.0;     // sum of [2.0, 4.0, 6.0]
/// let prev_sum_0_sq = 14.0;  // sum of squares [1, 4, 9]
/// let prev_sum_1_sq = 56.0;  // sum of squares [4, 16, 36]
/// let prev_sum_01 = 28.0;    // sum of products
/// let new_0 = 4.0;
/// let new_1 = 8.0;
/// let old_0 = 1.0;
/// let old_1 = 2.0;
/// let period = 3;
///
/// let (new_correl, new_sum_0, new_sum_1, new_sum_0_sq, new_sum_1_sq, new_sum_01) =
///     correl::correl_inc(
///         new_0, new_1, old_0, old_1,
///         prev_sum_0, prev_sum_1, prev_sum_0_sq, prev_sum_1_sq, prev_sum_01,
///         period
///     ).unwrap();
/// ```
pub fn correl_inc(
    input_new_0: TAFloat,
    input_new_1: TAFloat,
    input_old_0: TAFloat,
    input_old_1: TAFloat,
    prev_sum_0: TAFloat,
    prev_sum_1: TAFloat,
    prev_sum_0_sq: TAFloat,
    prev_sum_1_sq: TAFloat,
    prev_sum_01: TAFloat,
    param_period: usize,
) -> Result<(TAFloat, TAFloat, TAFloat, TAFloat, TAFloat, TAFloat), KandError> {
    #[cfg(feature = "check")]
    {
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check for all inputs
        if input_new_0.is_nan()
            || input_new_1.is_nan()
            || input_old_0.is_nan()
            || input_old_1.is_nan()
            || prev_sum_0.is_nan()
            || prev_sum_1.is_nan()
            || prev_sum_0_sq.is_nan()
            || prev_sum_1_sq.is_nan()
            || prev_sum_01.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    // Update all sums incrementally
    let new_sum_0 = prev_sum_0 - input_old_0 + input_new_0;
    let new_sum_1 = prev_sum_1 - input_old_1 + input_new_1;
    let new_sum_0_sq = input_new_0.mul_add(
        input_new_0,
        input_old_0.mul_add(-input_old_0, prev_sum_0_sq),
    );
    let new_sum_1_sq = input_new_1.mul_add(
        input_new_1,
        input_old_1.mul_add(-input_old_1, prev_sum_1_sq),
    );
    let new_sum_01 =
        input_new_0.mul_add(input_new_1, input_old_0.mul_add(-input_old_1, prev_sum_01));

    // Calculate new correlation using Pearson formula
    let n = param_period as TAFloat;
    let numerator = n.mul_add(new_sum_01, -(new_sum_0 * new_sum_1));
    let denominator_0 = n.mul_add(new_sum_0_sq, -(new_sum_0 * new_sum_0));
    let denominator_1 = n.mul_add(new_sum_1_sq, -(new_sum_1 * new_sum_1));
    let denominator = (denominator_0 * denominator_1).sqrt();

    let new_correl = if denominator > 0.0 {
        numerator / denominator
    } else {
        TAFloat::NAN
    };

    Ok((
        new_correl,
        new_sum_0,
        new_sum_1,
        new_sum_0_sq,
        new_sum_1_sq,
        new_sum_01,
    ))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_correl_perfect_positive() {
        // Test with perfectly correlated data (y = 2x)
        let input_0 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let input_1 = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0];
        let param_period = 3;
        let mut output_correl = vec![0.0; 6];
        let mut output_sum_0 = vec![0.0; 6];
        let mut output_sum_1 = vec![0.0; 6];
        let mut output_sum_0_sq = vec![0.0; 6];
        let mut output_sum_1_sq = vec![0.0; 6];
        let mut output_sum_01 = vec![0.0; 6];

        correl(
            &input_0,
            &input_1,
            param_period,
            &mut output_correl,
            &mut output_sum_0,
            &mut output_sum_1,
            &mut output_sum_0_sq,
            &mut output_sum_1_sq,
            &mut output_sum_01,
        )
        .unwrap();

        // First 2 values should be NaN
        for i in 0..2 {
            assert!(output_correl[i].is_nan());
        }

        // Perfect positive correlation should be 1.0
        for i in 2..6 {
            assert_relative_eq!(output_correl[i], 1.0, epsilon = 0.0001);
        }

        // Test incremental calculation matches
        for i in 3..6 {
            let (inc_correl, _, _, _, _, _) = correl_inc(
                input_0[i],
                input_1[i],
                input_0[i - param_period],
                input_1[i - param_period],
                output_sum_0[i - 1],
                output_sum_1[i - 1],
                output_sum_0_sq[i - 1],
                output_sum_1_sq[i - 1],
                output_sum_01[i - 1],
                param_period,
            )
            .unwrap();
            assert_relative_eq!(inc_correl, output_correl[i], epsilon = 0.0001);
        }
    }

    #[test]
    fn test_correl_perfect_negative() {
        // Test with perfectly negative correlated data (y = -x + 10)
        let input_0 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let input_1 = vec![9.0, 8.0, 7.0, 6.0, 5.0];
        let param_period = 3;
        let mut output_correl = vec![0.0; 5];
        let mut output_sum_0 = vec![0.0; 5];
        let mut output_sum_1 = vec![0.0; 5];
        let mut output_sum_0_sq = vec![0.0; 5];
        let mut output_sum_1_sq = vec![0.0; 5];
        let mut output_sum_01 = vec![0.0; 5];

        correl(
            &input_0,
            &input_1,
            param_period,
            &mut output_correl,
            &mut output_sum_0,
            &mut output_sum_1,
            &mut output_sum_0_sq,
            &mut output_sum_1_sq,
            &mut output_sum_01,
        )
        .unwrap();

        // Perfect negative correlation should be -1.0
        for i in 2..5 {
            assert_relative_eq!(output_correl[i], -1.0, epsilon = 0.0001);
        }
    }

    #[test]
    fn test_correl_no_variance() {
        // Test when one series has no variance
        let input_0 = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let input_1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let param_period = 3;
        let mut output_correl = vec![0.0; 5];
        let mut output_sum_0 = vec![0.0; 5];
        let mut output_sum_1 = vec![0.0; 5];
        let mut output_sum_0_sq = vec![0.0; 5];
        let mut output_sum_1_sq = vec![0.0; 5];
        let mut output_sum_01 = vec![0.0; 5];

        correl(
            &input_0,
            &input_1,
            param_period,
            &mut output_correl,
            &mut output_sum_0,
            &mut output_sum_1,
            &mut output_sum_0_sq,
            &mut output_sum_1_sq,
            &mut output_sum_01,
        )
        .unwrap();

        // When one series has no variance, correlation is undefined (NaN)
        for i in 2..5 {
            assert!(output_correl[i].is_nan());
        }
    }

    #[test]
    fn test_correl_lookback() {
        let lookback_5 = lookback(5).unwrap();
        assert_eq!(lookback_5, 4);

        let lookback_30 = lookback(30).unwrap();
        assert_eq!(lookback_30, 29);

        // Test invalid period
        #[cfg(feature = "check")]
        {
            let err = lookback(1);
            assert!(err.is_err());
        }
    }

    #[test]
    fn test_talib_compatibility() {
        // Real BTC/ETH daily closing prices from Binance
        // Testing against TA-Lib CORREL output with period=14
        let input_0 = vec![
            103_297.99, 102_120.01, 100_963.87, 105_333.93, 106_083.0, 107_340.58, 106_947.06,
            107_047.59, 107_296.79, 108_356.93, 107_146.5, 105_681.14, 108_849.6, 109_584.78,
            107_984.24, 108_198.12, 109_203.84, 108_262.94, 108_922.98, 111_233.99, 116_010.0,
            117_527.66, 117_420.0, 119_086.64, 119_841.18, 117_758.09, 118_630.43, 119_177.56,
            117_924.84, 117_893.24,
        ];
        let input_1 = vec![
            2406.49, 2295.73, 2227.7, 2411.66, 2448.45, 2418.49, 2415.96, 2423.17, 2435.62,
            2500.09, 2485.47, 2405.01, 2570.41, 2591.25, 2508.04, 2516.41, 2570.35, 2542.29,
            2615.25, 2768.74, 2951.29, 2958.22, 2943.28, 2972.03, 3013.62, 3137.89, 3371.35,
            3476.87, 3546.92, 3552.85,
        ];

        let param_period = 14;
        let len = input_0.len();
        let mut output_correl = vec![0.0; len];
        let mut output_sum_0 = vec![0.0; len];
        let mut output_sum_1 = vec![0.0; len];
        let mut output_sum_0_sq = vec![0.0; len];
        let mut output_sum_1_sq = vec![0.0; len];
        let mut output_sum_01 = vec![0.0; len];

        correl(
            &input_0,
            &input_1,
            param_period,
            &mut output_correl,
            &mut output_sum_0,
            &mut output_sum_1,
            &mut output_sum_0_sq,
            &mut output_sum_1_sq,
            &mut output_sum_01,
        )
        .unwrap();

        // First 13 values should be NaN (lookback = period - 1)
        for i in 0..13 {
            assert!(output_correl[i].is_nan());
        }

        // Expected correlation values from TA-Lib (full precision)
        let expected_values = [
            0.916_557_037_193_843_8,
            0.948_314_436_127_766,
            0.942_234_310_162_226_4,
            0.893_845_548_132_203_3,
            0.897_952_381_223_419_8,
            0.911_695_501_638_574,
            0.952_458_493_129_686_4,
            0.975_493_184_636_118_3,
            0.978_325_130_734_52,
            0.982_342_459_884_886_4,
            0.981_909_093_602_057_7,
            0.982_283_404_493_009_1,
            0.963_186_856_345_845_4,
            0.918_423_909_392_030_3,
            0.890_700_952_050_942_8,
            0.837_192_365_846_861_5,
            0.786_620_100_233_665_4,
        ];

        // Verify correlation values match TA-Lib output
        for (i, &expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_correl[i + 13], expected, epsilon = 0.000_000_000_1);
        }
    }
}
