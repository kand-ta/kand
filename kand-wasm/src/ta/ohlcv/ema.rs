use kand::ta::ohlcv::ema;
use wasm_bindgen::prelude::*;

/**
 * Calculates Exponential Moving Average (EMA) for a price series.
 *
 * EMA is a type of moving average that places greater weight and significance
 * on the most recent data points. The weighting given to each data point
 * decreases exponentially with time.
 *
 * Mathematical Formula:
 * - Initial EMA = SMA(first n prices)
 * - EMA = Price * k + EMA(previous) * (1 - k)
 * - where k = smoothing factor (default is 2/(period+1))
 *
 * @param inputPrices - Array of price values to calculate EMA from
 * @param paramPeriod - The time period for EMA calculation (must be >= 2)
 * @param paramK - Optional custom smoothing factor. If null, uses 2/(period+1)
 * @returns Promise<number[]> - Array of EMA values with the same length as input
 * @throws Error if input is invalid or calculation fails
 *
 * @example
 * ```typescript
 * const prices = [10.0, 11.0, 12.0, 13.0, 14.0];
 * const period = 3;
 * const emaValues = await ema_wasm(prices, period, null);
 * console.log(emaValues);
 * ```
 */
#[wasm_bindgen]
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
