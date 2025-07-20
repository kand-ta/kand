use pyo3::prelude::*;

pub mod ta;

/// A Python module implemented in Rust.
#[rustfmt::skip]
#[pymodule]
#[pyo3(name = "_kand")]
fn kand(m: &Bound<'_, PyModule>) -> PyResult<()> {

    // Add all OHLCV functions
    m.add_function(wrap_pyfunction!(ta::ohlcv::ad::ad_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ad::ad_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adosc::adosc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adosc::adosc_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adx::adx_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adx::adx_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adxr::adxr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::adxr::adxr_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroon::aroon_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroon::aroon_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroonosc::aroonosc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::aroonosc::aroonosc_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::atr::atr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::atr::atr_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bbands::bbands_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bbands::bbands_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bop::bop_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::bop::bop_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cci::cci_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cci::cci_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_doji::cdl_doji_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_doji::cdl_doji_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_dragonfly_doji::cdl_dragonfly_doji_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_dragonfly_doji::cdl_dragonfly_doji_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_gravestone_doji::cdl_gravestone_doji_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_gravestone_doji::cdl_gravestone_doji_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_hammer::cdl_hammer_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_hammer::cdl_hammer_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_inverted_hammer::cdl_inverted_hammer_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_inverted_hammer::cdl_inverted_hammer_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_long_shadow::cdl_long_shadow_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_long_shadow::cdl_long_shadow_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_marubozu::cdl_marubozu_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::cdl_marubozu::cdl_marubozu_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dema::dema_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dema::dema_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dx::dx_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::dx::dx_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ecl::ecl_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ecl::ecl_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ema::ema_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::ema::ema_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::macd::macd_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::macd::macd_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::medprice::medprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::medprice::medprice_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::mfi::mfi_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midpoint::midpoint_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midpoint::midpoint_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midprice::midprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::midprice::midprice_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::minus_di::minus_di_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::minus_dm::minus_dm_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::mom::mom_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::mom::mom_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::natr::natr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::natr::natr_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::obv::obv_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::obv::obv_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_di::plus_di_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_di::plus_di_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_dm::plus_dm_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::plus_dm::plus_dm_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rma::rma_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rma::rma_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::roc::roc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::roc::roc_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocp::rocp_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocp::rocp_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr::rocr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr::rocr_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr100::rocr100_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rocr100::rocr100_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rsi::rsi_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::rsi::rsi_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sar::sar_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sar::sar_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sma::sma_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::sma::sma_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::stoch::stoch_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::supertrend::supertrend_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::supertrend::supertrend_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::t3::t3_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::t3::t3_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::tema::tema_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::tema::tema_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trange::trange_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trange::trange_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trima::trima_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trima::trima_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trix::trix_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::trix::trix_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::typprice::typprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::typprice::typprice_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vegas::vegas_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vegas::vegas_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wclprice::wclprice_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wclprice::wclprice_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::willr::willr_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::willr::willr_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wma::wma_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::wma::wma_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vwap::vwap_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::ohlcv::vwap::vwap_inc_py, m)?)?;

    // Add all stats functions
    m.add_function(wrap_pyfunction!(ta::stats::correl::correl_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::correl::correl_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::max::max_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::max::max_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::min::min_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::min::min_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::stddev::stddev_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::stddev::stddev_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::sum::sum_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::sum::sum_inc_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::var::var_py, m)?)?;
    m.add_function(wrap_pyfunction!(ta::stats::var::var_inc_py, m)?)?;

    // Add all helper functions

    Ok(())
}
