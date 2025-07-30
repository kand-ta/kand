use kand::ta::ohlcv::adosc;
use wasm_bindgen::prelude::*;

/**
 * Returns the lookback period for ADOSC calculation.
 * @param {number} param_fast_period - The time period for the fast EMA (must be >= 2).
 * @param {number} param_slow_period - The time period for the slow EMA (must be >= fast_period).
 * @returns {number} The lookback period.
 * @throws {Error} If parameters are invalid.
 */
#[wasm_bindgen(js_name = adoscLookback)]
pub fn adosc_lookback_wasm(
    param_fast_period: usize,
    param_slow_period: usize,
) -> Result<usize, JsValue> {
    adosc::lookback(param_fast_period, param_slow_period)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

/**
 * Calculates the Chaikin A/D Oscillator (ADOSC).
 * @param {Float64Array} input_high - Array of high prices.
 * @param {Float64Array} input_low - Array of low prices.
 * @param {Float64Array} input_close - Array of close prices.
 * @param {Float64Array} input_volume - Array of volume values.
 * @param {number} param_fast_period - The time period for the fast EMA (must be >= 2).
 * @param {number} param_slow_period - The time period for the slow EMA (must be >= fast_period).
 * @returns {Float64Array} An array of ADOSC values.
 * @throws {Error} If inputs are invalid or calculation fails.
 */
#[wasm_bindgen(js_name = adosc)]
pub fn adosc_wasm(
    input_high: Vec<f64>,
    input_low: Vec<f64>,
    input_close: Vec<f64>,
    input_volume: Vec<f64>,
    param_fast_period: usize,
    param_slow_period: usize,
) -> Result<Vec<f64>, JsValue> {
    let len = input_high.len();
    let mut output_adosc = vec![0.0; len];
    let mut output_ad = vec![0.0; len];
    let mut output_ad_fast_ema = vec![0.0; len];
    let mut output_ad_slow_ema = vec![0.0; len];

    adosc::adosc(
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
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output_adosc)
}

/**
 * Calculates the next ADOSC value incrementally.
 * @param {number} input_high - The current high price.
 * @param {number} input_low - The current low price.
 * @param {number} input_close - The current close price.
 * @param {number} input_volume - The current volume.
 * @param {number} param_fast_period - The time period for the fast EMA.
 * @param {number} param_slow_period - The time period for the slow EMA.
 * @param {number} prev_ad - The previous A/D value.
 * @param {number} prev_fast_ema - The previous fast EMA of the A/D line.
 * @param {number} prev_slow_ema - The previous slow EMA of the A/D line.
 * @returns {number} The new ADOSC value.
 * @throws {Error} If inputs are invalid or calculation fails.
 */
#[wasm_bindgen(js_name = adoscInc)]
pub fn adosc_inc_wasm(
    input_high: f64,
    input_low: f64,
    input_close: f64,
    input_volume: f64,
    param_fast_period: usize,
    param_slow_period: usize,
    prev_ad: f64,
    prev_fast_ema: f64,
    prev_slow_ema: f64,
) -> Result<f64, JsValue> {
    // For incremental calculation, we only need the final ADOSC value.
    // The core `adosc_inc` function returns a tuple with intermediate values, which we can discard here.
    let (output_adosc, _, _, _) = adosc::adosc_inc(
        input_high,
        input_low,
        input_close,
        input_volume,
        prev_ad,
        prev_fast_ema,
        prev_slow_ema,
        param_fast_period,
        param_slow_period,
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output_adosc)
}
