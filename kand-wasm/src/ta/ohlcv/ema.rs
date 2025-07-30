use kand::ta::ohlcv::ema;
use wasm_bindgen::prelude::*;

/**
 * Returns the lookback period for EMA calculation.
 * @param {number} param_period - The time period for EMA calculation (must be >= 2).
 * @returns {number} The lookback period.
 * @throws {Error} If the period is invalid.
 */
#[wasm_bindgen(js_name = emaLookback)]
pub fn ema_lookback_wasm(param_period: usize) -> Result<usize, JsValue> {
    ema::lookback(param_period).map_err(|e| JsValue::from_str(&e.to_string()))
}

/**
 * Calculates Exponential Moving Average (EMA) for a price series.
 * @param {Float64Array} input_prices - Array of price values to calculate EMA from.
 * @param {number} param_period - The time period for EMA calculation (must be >= 2).
 * @param {number | null | undefined} param_k - Optional custom smoothing factor. If not provided, it defaults to `2 / (period + 1)`.
 * @returns {Float64Array} An array of EMA values with the same length as the input.
 * @throws {Error} If the input is invalid or the calculation fails.
 */
#[wasm_bindgen(js_name = ema)]
pub fn ema_wasm(
    input_prices: Vec<f64>,
    param_period: usize,
    param_k: Option<f64>,
) -> Result<Vec<f64>, JsValue> {
    let mut output_ema = vec![0.0; input_prices.len()];

    ema::ema(&input_prices, param_period, param_k, &mut output_ema)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output_ema)
}

/**
 * Calculates a single EMA value incrementally using the previous EMA.
 * @param {number} input_price - The current period's price value.
 * @param {number} prev_ema - The previous period's EMA value.
 * @param {number} param_period - The time period for EMA calculation (must be >= 2).
 * @param {number | null | undefined} param_k - Optional custom smoothing factor. If not provided, it defaults to `2 / (period + 1)`.
 * @returns {number} The new EMA value.
 * @throws {Error} If the parameters are invalid or the calculation fails.
 */
#[wasm_bindgen(js_name = emaInc)]
pub fn ema_inc_wasm(
    input_price: f64,
    prev_ema: f64,
    param_period: usize,
    param_k: Option<f64>,
) -> Result<f64, JsValue> {
    ema::ema_inc(input_price, prev_ema, param_period, param_k)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}
