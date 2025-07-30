use crate::{KandError, TAFloat};

/// Calculates the lookback period required for Variance calculation.
///
/// # Description
/// The lookback period represents the number of data points needed before the first valid output
/// can be calculated. For Variance calculation, this equals the specified period minus one.
///
/// # Arguments
/// * `param_period` - The time period used for Variance calculation (must be >= 2)
///
/// # Returns
/// * `Result<usize, KandError>` - The lookback period (period - 1) on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if `param_period` is less than 2
///
/// # Example
/// ```
/// use kand::stats::var;
/// let period = 14;
/// let lookback = var::lookback(period).unwrap();
/// assert_eq!(lookback, 13); // lookback is period - 1
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

/// Calculates Variance (VAR) for an entire price series.
///
/// # Description
/// Variance measures the average squared deviation of data points from their mean over a specified period.
/// It helps quantify the spread or dispersion of values in a dataset.
///
/// # Mathematical Formula
/// ```text
/// VAR = sum((x - mean)^2) / n
/// ```
/// Where:
/// - x represents each value in the dataset
/// - mean is the average of all values in the period (sum(x) / n)
/// - n is the time period
///
/// # Calculation Steps
/// 1. Calculate sum and sum of squares for initial period
/// 2. Calculate mean for the period
/// 3. Apply variance formula using sum and sum of squares
/// 4. Update sums incrementally for subsequent periods
///
/// # Arguments
/// * `input_prices` - Array of input price values
/// * `param_period` - The time period for Variance calculation (must be >= 2)
/// * `output_var` - Array to store calculated Variance values
/// * `output_sum` - Array to store running sum values
/// * `output_sum_sq` - Array to store running sum of squares values
///
/// # Returns
/// * `Result<(), KandError>` - Ok(()) on success
///
/// # Errors
/// * Returns `KandError::InvalidData` if input array is empty
/// * Returns `KandError::LengthMismatch` if output arrays don't match input length
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::InsufficientData` if input length is less than period
/// * Returns `KandError::NaNDetected` if any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::stats::var;
/// let input_prices = vec![2.0, 4.0, 6.0, 8.0, 10.0];
/// let period = 3;
/// let mut output_var = vec![0.0; 5];
/// let mut output_sum = vec![0.0; 5];
/// let mut output_sum_sq = vec![0.0; 5];
///
/// var::var(
///     &input_prices,
///     period,
///     &mut output_var,
///     &mut output_sum,
///     &mut output_sum_sq,
/// )
/// .unwrap();
/// // First (period-1) values are NaN, followed by calculated Variance values
/// ```
pub fn var(
    input_prices: &[TAFloat],
    param_period: usize,
    output_var: &mut [TAFloat],
    output_sum: &mut [TAFloat],
    output_sum_sq: &mut [TAFloat],
) -> Result<(), KandError> {
    let len = input_prices.len();
    let lookback = lookback(param_period)?;

    #[cfg(feature = "check")]
    {
        // Empty data check
        if len == 0 {
            return Err(KandError::InvalidData);
        }

        // Length consistency check
        if output_var.len() != len || output_sum.len() != len || output_sum_sq.len() != len {
            return Err(KandError::LengthMismatch);
        }

        // Parameter range check
        if param_period < 2 {
            return Err(KandError::InvalidParameter);
        }

        // Data sufficiency check
        if len <= lookback {
            return Err(KandError::InsufficientData);
        }
    }

    #[cfg(feature = "check-nan")]
    {
        // NaN check
        for price in input_prices {
            if price.is_nan() {
                return Err(KandError::NaNDetected);
            }
        }
    }

    // Calculate initial values
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    for val in input_prices.iter().take(param_period) {
        sum += *val;
        sum_sq += *val * *val;
    }

    let period_t = param_period as TAFloat;
    let mean = sum / period_t;
    output_var[lookback] = sum.mul_add(-mean, sum_sq) / period_t;
    output_sum[lookback] = sum;
    output_sum_sq[lookback] = sum_sq;

    // Calculate remaining VAR values incrementally
    for i in param_period..len {
        let old_val = input_prices[i - param_period];
        let new_val = input_prices[i];

        sum = sum - old_val + new_val;
        sum_sq = new_val.mul_add(new_val, old_val.mul_add(-old_val, sum_sq));

        let mean = sum / period_t;
        output_var[i] = sum.mul_add(-mean, sum_sq) / period_t;
        output_sum[i] = sum;
        output_sum_sq[i] = sum_sq;
    }

    // Fill initial values with NAN
    for i in 0..lookback {
        output_var[i] = TAFloat::NAN;
        output_sum[i] = TAFloat::NAN;
        output_sum_sq[i] = TAFloat::NAN;
    }

    Ok(())
}

/// Calculates the latest Variance value using incremental computation.
///
/// # Description
/// This function efficiently updates the Variance by using the previous sum and sum of squares values,
/// removing the oldest value and adding the newest value to the calculation window.
///
/// # Arguments
/// * `input_price` - The newest price value to include in calculation
/// * `prev_sum` - Previous sum of values in the period
/// * `prev_sum_sq` - Previous sum of squared values in the period
/// * `input_old_price` - Oldest price value to remove from calculation
/// * `param_period` - The time period for Variance calculation (must be >= 2)
///
/// # Returns
/// * `Result<(TAFloat, TAFloat, TAFloat), KandError>` - Tuple containing (variance, `new_sum`, `new_sum_sq`) on success
///
/// # Errors
/// * Returns `KandError::InvalidParameter` if period is less than 2
/// * Returns `KandError::NaNDetected` if any input value is NaN (when "`check-nan`" feature is enabled)
///
/// # Example
/// ```
/// use kand::stats::var;
/// let (var_value, new_sum, new_sum_sq) = var::var_inc(
///     10.0,  // new price
///     25.0,  // previous sum
///     220.0, // previous sum of squares
///     5.0,   // price to remove
///     3,     // period
/// )
/// .unwrap();
/// ```
pub fn var_inc(
    input_price: TAFloat,
    prev_sum: TAFloat,
    prev_sum_sq: TAFloat,
    input_old_price: TAFloat,
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
        if input_price.is_nan()
            || prev_sum.is_nan()
            || prev_sum_sq.is_nan()
            || input_old_price.is_nan()
        {
            return Err(KandError::NaNDetected);
        }
    }

    let new_sum = prev_sum - input_old_price + input_price;
    let new_sum_sq = input_price.mul_add(
        input_price,
        input_old_price.mul_add(-input_old_price, prev_sum_sq),
    );

    let period_t = param_period as TAFloat;
    let mean = new_sum / period_t;
    let var = new_sum.mul_add(-mean, new_sum_sq) / period_t;

    Ok((var, new_sum, new_sum_sq))
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_var_calculation() {
        let input_close = vec![
            35216.1, 35221.4, 35190.7, 35170.0, 35181.5, 35254.6, 35202.8, 35251.9, 35197.6,
            35184.7, 35175.1, 35229.9, 35212.5, 35160.7, 35090.3, 35041.2, 34999.3, 35013.4,
            35069.0, 35024.6, 34939.5, 34952.6, 35000.0, 35041.8, 35080.0,
        ];
        let param_period = 14;
        let mut output_var = vec![0.0; input_close.len()];
        let mut output_sum = vec![0.0; input_close.len()];
        let mut output_sum_sq = vec![0.0; input_close.len()];

        var(
            &input_close,
            param_period,
            &mut output_var,
            &mut output_sum,
            &mut output_sum_sq,
        )
        .unwrap();

        // First 13 values should be NaN
        for i in 0..13 {
            assert!(output_var[i].is_nan());
            assert!(output_sum[i].is_nan());
            assert!(output_sum_sq[i].is_nan());
        }

        // Compare with known values
        let expected_values = [
            786.293_724_536_895_8,
            1_610.155_357_122_421_3,
            3_072.717_398_166_656_5,
            5_255.849_234_580_994,
            6_837.828_826_189_041,
            7_280.654_081_583_023,
            7_312.572_448_968_887,
            9_261.126_785_755_157,
            9_287.239_183_425_903,
            8_900.911_019_802_094,
            8_078.288_214_445_114,
        ];

        for (i, expected) in expected_values.iter().enumerate() {
            assert_relative_eq!(output_var[i + 13], *expected, epsilon = 0.0001);
        }

        // Now test incremental calculation matches regular calculation
        let mut prev_sum = output_sum[13];
        let mut prev_sum_sq = output_sum_sq[13];

        // Test each incremental step
        for i in 14..19 {
            let (var, new_sum, new_sum_sq) = var_inc(
                input_close[i],
                prev_sum,
                prev_sum_sq,
                input_close[i - param_period],
                param_period,
            )
            .unwrap();
            assert_relative_eq!(var, output_var[i], epsilon = 0.0001);
            assert_relative_eq!(new_sum, output_sum[i], epsilon = 0.0001);
            assert_relative_eq!(new_sum_sq, output_sum_sq[i], epsilon = 0.0001);
            prev_sum = new_sum;
            prev_sum_sq = new_sum_sq;
        }
    }
}
