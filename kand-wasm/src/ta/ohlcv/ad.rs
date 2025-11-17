use kand::ta::ohlcv::ad;
use wasm_bindgen::prelude::*;

/**
 * Returns the lookback period for A/D calculation.
 * @returns {number} The lookback period (always 0 for A/D).
 */
#[wasm_bindgen(js_name = adLookback)]
pub fn ad_lookback_wasm() -> Result<usize, JsValue> {
    ad::lookback().map_err(|e| JsValue::from_str(&e.to_string()))
}

/**
 * Calculates the Accumulation/Distribution (A/D) indicator.
 * @param {Float64Array} inputHigh - Array of high prices.
 * @param {Float64Array} inputLow - Array of low prices.
 * @param {Float64Array} inputClose - Array of close prices.
 * @param {Float64Array} inputVolume - Array of volume values.
 * @returns {Float64Array} An array of A/D values.
 * @throws {Error} If inputs are invalid or calculation fails.
 */
#[wasm_bindgen(js_name = ad)]
pub fn ad_wasm(
    input_high: Vec<f64>,
    input_low: Vec<f64>,
    input_close: Vec<f64>,
    input_volume: Vec<f64>,
) -> Result<Vec<f64>, JsValue> {
    let mut output_ad = vec![0.0; input_high.len()];
    ad::ad(
        &input_high,
        &input_low,
        &input_close,
        &input_volume,
        &mut output_ad,
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(output_ad)
}

/**
 * Calculates the next A/D value incrementally.
 * @param {number} inputHigh - The current high price.
 * @param {number} inputLow - The current low price.
 * @param {number} inputClose - The current close price.
 * @param {number} inputVolume - The current volume.
 * @param {number} prevAd - The previous A/D value.
 * @returns {number} The new A/D value.
 * @throws {Error} If inputs are invalid or calculation fails.
 */
#[wasm_bindgen(js_name = adInc)]
pub fn ad_inc_wasm(
    input_high: f64,
    input_low: f64,
    input_close: f64,
    input_volume: f64,
    prev_ad: f64,
) -> Result<f64, JsValue> {
    ad::ad_inc(input_high, input_low, input_close, input_volume, prev_ad)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
