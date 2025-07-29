use kand::ta::ohlcv::ema;
use wasm_bindgen::prelude::*;

/**
 * Calculates Exponential Moving Average (EMA) for a price series.
 * @param inputPrices - Array of price values to calculate EMA from
 * @param paramPeriod - The time period for EMA calculation (must be >= 2)
 * @param paramK - Optional custom smoothing factor. If null, uses 2/(period+1)
 * @returns Promise<number[]> - Array of EMA values with the same length as input
 * @throws Error if input is invalid or calculation fails
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
 * @param inputPrice - The current period's price value
 * @param prevEma - The previous period's EMA value
 * @param paramPeriod - The time period for EMA calculation (must be >= 2)
 * @param paramK - Optional custom smoothing factor. If null, uses 2/(period+1)
 * @returns Promise<number> - The new EMA value
 * @throws Error if parameters are invalid or calculation fails
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
