# Auto-generated stub file for kand
"""
Type hints and function signatures stub file for IDE autocompletion.
Auto-generated to avoid manual maintenance. Can be enhanced with more precise type annotations.
"""

def ad(high, low, close, volume):
    """
    Computes the Accumulation/Distribution (A/D) indicator over NumPy arrays.

    The A/D indicator measures the cumulative flow of money into and out of a security by
    combining price and volume data. It helps identify whether buying or selling pressure
    is dominant.

    Args:
        high: High prices as a 1-D NumPy array of type `TAFloat`.
        low: Low prices as a 1-D NumPy array of type `TAFloat`.
        close: Close prices as a 1-D NumPy array of type `TAFloat`.
        volume: Volume data as a 1-D NumPy array of type `TAFloat`.

    Returns:
        A new 1-D NumPy array containing the A/D values. The array has the same length as the inputs.
    Examples:
        ```python
        >>> import numpy as np
        >>> import kand
        >>> high = np.array([10.0, 12.0, 15.0])
        >>> low = np.array([8.0, 9.0, 11.0])
        >>> close = np.array([9.0, 11.0, 13.0])
        >>> volume = np.array([100.0, 150.0, 200.0])
        >>> result = kand.ad(high, low, close, volume)
        >>> print(result)
        [-50.0, 25.0, 125.0]
        ```
    """
    ...

def ad_inc(high, low, close, volume, prev_ad):
    """
    Computes the latest Accumulation/Distribution (A/D) value incrementally.

    This function calculates only the latest A/D value using the previous A/D value,
    avoiding recalculation of the entire series.

    Args:
        high: Latest high price.
        low: Latest low price.
        close: Latest closing price.
        volume: Latest volume.
        prev_ad: Previous A/D value.

    Returns:
        The latest A/D value.

    Examples:
        ```python
        >>> import kand
        >>> high = 15.0
        >>> low = 11.0
        >>> close = 13.0
        >>> volume = 200.0
        >>> prev_ad = 25.0
        >>> result = kand.ad_inc(high, low, close, volume, prev_ad)
        >>> print(result)
        125.0
        ```
    """
    ...

def adosc(high, low, close, volume, fast_period, slow_period):
    """
    Calculate Accumulation/Distribution Oscillator (A/D Oscillator or ADOSC)

    The A/D Oscillator is a momentum indicator that measures the difference between a fast and slow EMA of the
    Accumulation/Distribution Line. It helps identify trend strength and potential reversals.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      volume: Volume as a 1-D NumPy array of type `TAFloat`.
      fast_period: Fast period for A/D Oscillator calculation.
      slow_period: Slow period for A/D Oscillator calculation.

    Returns:
      A tuple of 4 1-D NumPy arrays containing:
      - ADOSC values
      - A/D Line values
      - Fast EMA values
      - Slow EMA values
      Each array has the same length as the input, with the first `slow_period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 11.0, 12.0, 11.5, 10.5])
      >>> low = np.array([8.0, 9.0, 10.0, 9.5, 8.5])
      >>> close = np.array([9.0, 10.0, 11.0, 10.0, 9.0])
      >>> volume = np.array([100.0, 150.0, 200.0, 150.0, 100.0])
      >>> adosc, ad, fast_ema, slow_ema = kand.adosc(high, low, close, volume, 3, 5)
      ```
    """
    ...

def adosc_inc(high, low, close, volume, prev_ad, prev_fast_ema, prev_slow_ema, fast_period, slow_period):
    """
    Calculate latest A/D Oscillator value incrementally

    Provides optimized calculation of the latest ADOSC value when new data arrives,
    without recalculating the entire series.

    Args:
        high: Latest high price.
        low: Latest low price.
        close: Latest closing price.
        volume: Latest volume.
        prev_ad: Previous A/D value.
        prev_fast_ema: Previous fast EMA value.
        prev_slow_ema: Previous slow EMA value.
        fast_period: Fast EMA period.
        slow_period: Slow EMA period.

    Returns:
        A tuple containing (ADOSC, AD, Fast EMA, Slow EMA) values.

    Examples:
        ```python
        >>> import kand
        >>> adosc, ad, fast_ema, slow_ema = kand.adosc_inc(
        ...     10.5,  # high
        ...     9.5,   # low
        ...     10.0,  # close
        ...     150.0, # volume
        ...     100.0, # prev_ad
        ...     95.0,  # prev_fast_ema
        ...     90.0,  # prev_slow_ema
        ...     3,     # fast_period
        ...     10,    # slow_period
        ... )
        ```
    """
    ...

def adx(high, low, close, period):
    """
    Calculate Average Directional Index (ADX) for a NumPy array

    The ADX (Average Directional Index) measures the strength of a trend, regardless of whether it's up or down.
    Values range from 0 to 100, with higher values indicating stronger trends.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for ADX calculation (typically 14). Must be positive.

    Returns:
      A tuple of four 1-D NumPy arrays containing:
      - ADX values
      - Smoothed +DM values
      - Smoothed -DM values
      - Smoothed TR values
      Each array has the same length as the input, with the first (2*period-1) elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
      >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
      >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
      >>> adx, plus_dm, minus_dm, tr = kand.adx(high, low, close, 2)
      ```
    """
    ...

def adx_inc(high, low, prev_high, prev_low, prev_close, prev_adx, prev_smoothed_plus_dm, prev_smoothed_minus_dm, prev_smoothed_tr, period):
    """
    Calculate the latest ADX value incrementally

    Args:
      py: Python interpreter token
      high: Current period's high price
      low: Current period's low price
      prev_high: Previous period's high price
      prev_low: Previous period's low price
      prev_close: Previous period's close price
      prev_adx: Previous period's ADX value
      prev_smoothed_plus_dm: Previous period's smoothed +DM
      prev_smoothed_minus_dm: Previous period's smoothed -DM
      prev_smoothed_tr: Previous period's smoothed TR
      period: Period for ADX calculation (typically 14)

    Returns:
      A tuple containing:
      - Latest ADX value
      - New smoothed +DM
      - New smoothed -DM
      - New smoothed TR

    Examples:
      ```python
      >>> import kand
      >>> adx, plus_dm, minus_dm, tr = kand.adx_inc(
      ...     24.20,  # current high
      ...     23.85,  # current low
      ...     24.07,  # previous high
      ...     23.72,  # previous low
      ...     23.95,  # previous close
      ...     25.0,   # previous ADX
      ...     0.5,    # previous smoothed +DM
      ...     0.3,    # previous smoothed -DM
      ...     1.2,    # previous smoothed TR
      ...     14      # period
      ... )
      ```
    """
    ...

def adxr(high, low, close, period):
    """
    Calculate Average Directional Index Rating (ADXR) for a NumPy array.

    ADXR is a momentum indicator that measures the strength of a trend by comparing
    the current ADX value with the ADX value from `period` days ago.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for ADX calculation (typically 14).

    Returns:
      A tuple of 5 1-D NumPy arrays containing:
      - ADXR values
      - ADX values
      - Smoothed +DM values
      - Smoothed -DM values
      - Smoothed TR values
      The first (3*period-2) elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
      >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
      >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
      >>> adxr, adx, plus_dm, minus_dm, tr = kand.adxr(high, low, close, 2)
      ```
    """
    ...

def adxr_inc(high, low, prev_high, prev_low, prev_close, prev_adx, prev_adx_period_ago, prev_smoothed_plus_dm, prev_smoothed_minus_dm, prev_smoothed_tr, period):
    """
    Calculate the latest ADXR value incrementally

    Args:

      high: Current high price as TAFloat.
      low: Current low price as TAFloat.
      prev_high: Previous high price as TAFloat.
      prev_low: Previous low price as TAFloat.
      prev_close: Previous close price as TAFloat.
      prev_adx: Previous ADX value as TAFloat.
      prev_adx_period_ago: ADX value from period days ago as TAFloat.
      prev_smoothed_plus_dm: Previous smoothed +DM value as TAFloat.
      prev_smoothed_minus_dm: Previous smoothed -DM value as TAFloat.
      prev_smoothed_tr: Previous smoothed TR value as TAFloat.
      period: Period for ADX calculation (typically 14).

    Returns:
      A tuple of 5 values:
      - Latest ADXR value
      - Latest ADX value
      - New smoothed +DM value
      - New smoothed -DM value
      - New smoothed TR value

    Examples:
      ```python
      >>> import kand
      >>> adxr, adx, plus_dm, minus_dm, tr = kand.adxr_inc(
      ...     24.20,  # high
      ...     23.85,  # low
      ...     24.07,  # prev_high
      ...     23.72,  # prev_low
      ...     23.95,  # prev_close
      ...     25.0,   # prev_adx
      ...     20.0,   # prev_adx_period_ago
      ...     0.5,    # prev_smoothed_plus_dm
      ...     0.3,    # prev_smoothed_minus_dm
      ...     1.2,    # prev_smoothed_tr
      ...     14      # period
      ... )
      ```
    """
    ...

def aroon(high, low, period):
    """
    Calculate Aroon indicator for a NumPy array.

    The Aroon indicator consists of two lines that measure the time since the last high/low
    relative to a lookback period. It helps identify the start of new trends and trend reversals.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      period: The lookback period for calculations (must be >= 2).

    Returns:
      A tuple of 6 1-D NumPy arrays containing:
      - Aroon Up values
      - Aroon Down values
      - Previous high values
      - Previous low values
      - Days since high values
      - Days since low values
      The first (period) elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> aroon_up, aroon_down, prev_high, prev_low, days_high, days_low = kand.aroon(high, low, 3)
      ```
    """
    ...

def aroon_inc(high, low, prev_high, prev_low, days_since_high, days_since_low, period):
    """
    Calculate the next Aroon values incrementally.

    Args:

      high: Current period's high price.
      low: Current period's low price.
      prev_high: Previous highest price in period.
      prev_low: Previous lowest price in period.
      days_since_high: Days since previous highest price.
      days_since_low: Days since previous lowest price.
      period: The lookback period (must be >= 2).

    Returns:
      A tuple containing:
      - Aroon Up value
      - Aroon Down value
      - New highest price
      - New lowest price
      - Updated days since high
      - Updated days since low

    Examples:
      ```python
      >>> import kand
      >>> aroon_up, aroon_down, new_high, new_low, days_high, days_low = kand.aroon_inc(
      ...     15.0,  # high
      ...     12.0,  # low
      ...     14.0,  # prev_high
      ...     11.0,  # prev_low
      ...     2,     # days_since_high
      ...     1,     # days_since_low
      ...     14     # period
      ... )
      ```
    """
    ...

def aroonosc(high, low, period):
    """
    Calculate Aroon Oscillator for a NumPy array.

    The Aroon Oscillator measures the strength of a trend by comparing the time since the last high and low.
    It oscillates between -100 and +100, with positive values indicating an uptrend and negative values a downtrend.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      period: The lookback period for calculations (must be >= 2).

    Returns:
      A tuple of 5 1-D NumPy arrays containing:
      - Aroon Oscillator values
      - Previous high values
      - Previous low values
      - Days since high values
      - Days since low values
      The first (period) elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> osc, prev_high, prev_low, days_high, days_low = kand.aroonosc(high, low, 3)
      ```
    """
    ...

def aroonosc_inc(high, low, prev_high, prev_low, days_since_high, days_since_low, period):
    """
    Calculate the next Aroon Oscillator value incrementally.

    Args:

      high: Current period's high price.
      low: Current period's low price.
      prev_high: Previous highest price within the period.
      prev_low: Previous lowest price within the period.
      days_since_high: Days since previous highest price.
      days_since_low: Days since previous lowest price.
      period: The lookback period for calculations (must be >= 2).

    Returns:
      A tuple containing:
      - Aroon Oscillator value
      - New highest price
      - New lowest price
      - Updated days since high
      - Updated days since low

    Examples:
      ```python
      >>> import kand
      >>> osc, high, low, days_high, days_low = kand.aroonosc_inc(
      ...     15.0,  # high
      ...     12.0,  # low
      ...     14.0,  # prev_high
      ...     11.0,  # prev_low
      ...     2,     # days_since_high
      ...     1,     # days_since_low
      ...     14     # period
      ... )
      ```
    """
    ...

def atr(high, low, close, period):
    """
    Computes the Average True Range (ATR) over NumPy arrays.

    The Average True Range (ATR) is a technical analysis indicator that measures market volatility
    by decomposing the entire range of an asset price for a given period.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for ATR calculation. Must be greater than 1.

    Returns:
      A new 1-D NumPy array containing the ATR values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
      >>> result = kand.atr(high, low, close, 3)
      ```
    """
    ...

def atr_inc(high, low, prev_close, prev_atr, period):
    """
    Calculate the next ATR value incrementally.

    Args:

      high: Current period's high price.
      low: Current period's low price.
      prev_close: Previous period's close price.
      prev_atr: Previous period's ATR value.
      period: The time period for ATR calculation (must be >= 2).

    Returns:
      The calculated ATR value.

    Examples:
      ```python
      >>> import kand
      >>> atr = kand.atr_inc(
      ...     15.0,  # high
      ...     11.0,  # low
      ...     12.0,  # prev_close
      ...     3.0,   # prev_atr
      ...     14     # period
      ... )
      ```
    """
    ...

def bbands(price, period, dev_up, dev_down):
    """
    Calculate Bollinger Bands for a NumPy array.

    Bollinger Bands consist of:
    - A middle band (N-period simple moving average)
    - An upper band (K standard deviations above middle band)
    - A lower band (K standard deviations below middle band)

    Args:
      price: Input price values as a 1-D NumPy array of type `TAFloat`.
      period: The time period for calculations (must be >= 2).
      dev_up: Number of standard deviations for upper band.
      dev_down: Number of standard deviations for lower band.

    Returns:
      A tuple of 7 1-D NumPy arrays containing:
      - Upper band values
      - Middle band values
      - Lower band values
      - SMA values
      - Variance values
      - Sum values
      - Sum of squares values
      The first (period-1) elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> price = np.array([10.0, 11.0, 12.0, 13.0, 14.0])
      >>> upper, middle, lower, sma, var, sum, sum_sq = kand.bbands(price, 3, 2.0, 2.0)
      ```
    """
    ...

def bbands_inc(price, prev_sma, prev_sum, prev_sum_sq, old_price, period, dev_up, dev_down):
    """
    Calculate the next Bollinger Bands values incrementally.

    Args:

      price: The current price value.
      prev_sma: The previous SMA value.
      prev_sum: The previous sum for variance calculation.
      prev_sum_sq: The previous sum of squares for variance calculation.
      old_price: The oldest price value to be removed from the period.
      period: The time period for calculations (must be >= 2).
      dev_up: Number of standard deviations for upper band.
      dev_down: Number of standard deviations for lower band.

    Returns:
      A tuple containing:
      - Upper Band value
      - Middle Band value
      - Lower Band value
      - New SMA value
      - New Sum value
      - New Sum of Squares value

    Examples:
      ```python
      >>> import kand
      >>> upper, middle, lower, sma, sum, sum_sq = kand.bbands_inc(
      ...     10.0,   # price
      ...     9.5,    # prev_sma
      ...     28.5,   # prev_sum
      ...     272.25, # prev_sum_sq
      ...     9.0,    # old_price
      ...     3,      # period
      ...     2.0,    # dev_up
      ...     2.0     # dev_down
      ... )
      ```
    """
    ...

def bop(open, high, low, close):
    """
    Calculate Balance of Power (BOP) indicator for NumPy arrays.

    The Balance of Power (BOP) is a momentum oscillator that measures the relative strength
    between buyers and sellers by comparing the closing price to the opening price and
    normalizing it by the trading range (high - low).

    Args:
      open: Input opening prices as a 1-D NumPy array of type `TAFloat`.
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      close: Input closing prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A 1-D NumPy array containing the BOP values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([10.0, 11.0, 12.0, 13.0])
      >>> high = np.array([12.0, 13.0, 14.0, 15.0])
      >>> low = np.array([8.0, 9.0, 10.0, 11.0])
      >>> close = np.array([11.0, 12.0, 13.0, 14.0])
      >>> bop = kand.bop(open, high, low, close)
      ```
    """
    ...

def bop_inc(open, high, low, close):
    """
    Calculate a single Balance of Power (BOP) value for the latest price data.

    Args:
      open: Current period's opening price
      high: Current period's high price
      low: Current period's low price
      close: Current period's closing price

    Returns:
      The calculated BOP value

    Examples:
      ```python
      >>> import kand
      >>> bop = kand.bop_inc(10.0, 12.0, 8.0, 11.0)
      ```
    """
    ...

def cci(high, low, close, period):
    """
    Computes the Commodity Channel Index (CCI) over NumPy arrays.

    The CCI is a momentum-based oscillator used to help determine when an investment vehicle is reaching
    a condition of being overbought or oversold.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for CCI calculation. Must be positive and less than input length.

    Returns:
      A tuple of 1-D NumPy arrays containing:
      - CCI values
      - Typical prices
      - SMA of typical prices
      - Mean deviation values
      Each array has the same length as the input, with the first `period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
      >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
      >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
      >>> cci, tp, sma_tp, mean_dev = kand.cci(high, low, close, 3)
      >>> print(cci)
      [nan, nan, -100.0, 66.67, -133.33]
      ```
    """
    ...

def cci_inc(prev_sma_tp, new_high, new_low, new_close, old_high, old_low, old_close, period, tp_buffer):
    """
    Calculates the next CCI value incrementally.

    Args:

      prev_sma_tp: Previous SMA value of typical prices.
      new_high: New high price.
      new_low: New low price.
      new_close: New close price.
      old_high: Old high price to be removed.
      old_low: Old low price to be removed.
      old_close: Old close price to be removed.
      period: Window size for CCI calculation.
      tp_buffer: List containing the last `period` typical prices.

    Returns:
      The next CCI value.

    Examples:
      ```python
      >>> import kand
      >>> prev_sma_tp = 100.0
      >>> new_high = 105.0
      >>> new_low = 95.0
      >>> new_close = 100.0
      >>> old_high = 102.0
      >>> old_low = 98.0
      >>> old_close = 100.0
      >>> period = 14
      >>> tp_buffer = [100.0] * period
      >>> next_cci = kand.cci_inc(prev_sma_tp, new_high, new_low, new_close,
      ...                                  old_high, old_low, old_close, period, tp_buffer)
      ```
    """
    ...

def cdl_doji(open, high, low, close, body_percent, shadow_equal_percent):
    """
    Detects Doji candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      body_percent: Maximum body size as percentage of range (e.g. 5.0 for 5%).
      shadow_equal_percent: Maximum shadow length difference percentage (e.g. 100.0).

    Returns:
      A 1-D NumPy array containing pattern signals (1.0 = pattern, 0.0 = no pattern).

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([10.0, 10.5, 10.2])
      >>> high = np.array([11.0, 11.2, 10.8])
      >>> low = np.array([9.8, 10.1, 9.9])
      >>> close = np.array([10.3, 10.4, 10.25])
      >>> signals = kand.cdl_doji(open, high, low, close, 5.0, 100.0)
      ```
    """
    ...

def cdl_doji_inc(open, high, low, close, body_percent, shadow_equal_percent):
    """
    Detects a Doji pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      body_percent: Maximum body size as percentage of range.
      shadow_equal_percent: Maximum shadow length difference percentage.

    Returns:
      Signal value (1.0 for Doji pattern, 0.0 for no pattern).

    Examples:
      ```python
      >>> import kand
      >>> signal = kand.cdl_doji_inc(10.0, 11.0, 9.8, 10.3, 5.0, 100.0)
      ```
    """
    ...

def cdl_dragonfly_doji(open, high, low, close, body_percent):
    """
    Detects Dragonfly Doji candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      body_percent: Maximum body size as percentage of total range (typically 5%).

    Returns:
      A 1-D NumPy array containing pattern signals:
      - 100: Bullish Dragonfly Doji pattern detected
      - 0: No pattern detected

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([100.0, 101.0, 102.0])
      >>> high = np.array([102.0, 103.0, 104.0])
      >>> low = np.array([98.0, 99.0, 100.0])
      >>> close = np.array([101.0, 102.0, 103.0])
      >>> signals = kand.cdl_dragonfly_doji(open, high, low, close, 5.0)
      ```
    """
    ...

def cdl_dragonfly_doji_inc(open, high, low, close, body_percent):
    """
    Detects a Dragonfly Doji pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      body_percent: Maximum body size as percentage of total range.

    Returns:
      Signal value:
      - 100: Bullish Dragonfly Doji pattern detected
      - 0: No pattern detected

    Examples:
      ```python
      >>> import kand
      >>> signal = kand.cdl_dragonfly_doji_inc(100.0, 102.0, 98.0, 100.1, 5.0)
      ```
    """
    ...

def cdl_gravestone_doji(open, high, low, close, body_percent):
    """
    Detects Gravestone Doji candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      body_percent: Maximum body size as percentage of total range (typically 5%).

    Returns:
      A 1-D NumPy array containing pattern signals:
      - -100: Bearish Gravestone Doji pattern detected
      - 0: No pattern detected

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([100.0, 101.0, 102.0])
      >>> high = np.array([102.0, 103.0, 104.0])
      >>> low = np.array([98.0, 99.0, 100.0])
      >>> close = np.array([101.0, 102.0, 103.0])
      >>> signals = kand.cdl_gravestone_doji(open, high, low, close, 5.0)
      ```
    """
    ...

def cdl_gravestone_doji_inc(open, high, low, close, body_percent):
    """
    Detects a Gravestone Doji pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      body_percent: Maximum body size as percentage of total range.

    Returns:
      Signal value:
      - -100: Bearish Gravestone Doji pattern detected
      - 0: No pattern detected

    Examples:
      ```python
      >>> import kand
      >>> signal = kand.cdl_gravestone_doji_inc(100.0, 102.0, 98.0, 100.1, 5.0)
      ```
    """
    ...

def cdl_hammer(open, high, low, close, period, factor):
    """
    Detects Hammer candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for EMA calculation of body sizes.
      factor: Minimum ratio of lower shadow to body length.

    Returns:
      A tuple of two 1-D NumPy arrays containing:
      - Pattern signals:
        - 100: Bullish Hammer pattern detected
        - 0: No pattern detected
      - EMA values of candle body sizes

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([100.0, 101.0, 102.0])
      >>> high = np.array([102.0, 103.0, 104.0])
      >>> low = np.array([98.0, 99.0, 100.0])
      >>> close = np.array([101.0, 102.0, 103.0])
      >>> signals, body_avg = kand.cdl_hammer(open, high, low, close, 14, 2.0)
      ```
    """
    ...

def cdl_hammer_inc(open, high, low, close, prev_body_avg, period, factor):
    """
    Detects a Hammer pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      prev_body_avg: Previous EMA value of body sizes.
      period: Period for EMA calculation.
      factor: Minimum ratio of lower shadow to body length.

    Returns:
      A tuple containing:
      - Signal value:
        - 100: Bullish Hammer pattern detected
        - 0: No pattern detected
      - Updated EMA value of body sizes

    Examples:
      ```python
      >>> import kand
      >>> signal, body_avg = kand.cdl_hammer_inc(100.0, 102.0, 98.0, 100.1, 0.5, 14, 2.0)
      ```
    """
    ...

def cdl_inverted_hammer(open, high, low, close, period, factor):
    """
    Detects Inverted Hammer candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for EMA calculation of body sizes.
      factor: Minimum ratio of upper shadow to body length.

    Returns:
      A tuple of two 1-D NumPy arrays containing:
      - Pattern signals:
        - 100: Bullish Inverted Hammer pattern detected
        - 0: No pattern detected
      - EMA values of candle body sizes

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([100.0, 101.0, 102.0])
      >>> high = np.array([102.0, 103.0, 104.0])
      >>> low = np.array([98.0, 99.0, 100.0])
      >>> close = np.array([101.0, 102.0, 103.0])
      >>> signals, body_avg = kand.cdl_inverted_hammer(open, high, low, close, 14, 2.0)
      ```
    """
    ...

def cdl_inverted_hammer_inc(open, high, low, close, prev_body_avg, period, factor):
    """
    Detects an Inverted Hammer pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      prev_body_avg: Previous EMA value of body sizes.
      period: Period for EMA calculation.
      factor: Minimum ratio of upper shadow to body length.

    Returns:
      A tuple containing:
      - Signal value:
        - 100: Bullish Inverted Hammer pattern detected
        - 0: No pattern detected
      - Updated EMA value of body sizes

    Examples:
      ```python
      >>> import kand
      >>> signal, body_avg = kand.cdl_inverted_hammer_inc(100.0, 102.0, 98.0, 100.1, 0.5, 14, 2.0)
      ```
    """
    ...

def cdl_long_shadow(open, high, low, close, period, shadow_factor):
    """
    Detects Long Shadow candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for EMA calculation of body sizes.
      shadow_factor: Minimum percentage of total range that shadow must be.

    Returns:
      A tuple of two 1-D NumPy arrays containing:
      - Pattern signals:
        - 100: Bullish Long Lower Shadow pattern detected
        - -100: Bearish Long Upper Shadow pattern detected
        - 0: No pattern detected
      - EMA values of candle body sizes

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([100.0, 101.0, 102.0])
      >>> high = np.array([102.0, 103.0, 104.0])
      >>> low = np.array([98.0, 99.0, 100.0])
      >>> close = np.array([101.0, 102.0, 103.0])
      >>> signals, body_avg = kand.cdl_long_shadow(open, high, low, close, 14, 75.0)
      ```
    """
    ...

def cdl_long_shadow_inc(open, high, low, close, prev_body_avg, period, shadow_factor):
    """
    Detects a Long Shadow pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      prev_body_avg: Previous EMA value of body sizes.
      period: Period for EMA calculation.
      shadow_factor: Minimum percentage of total range that shadow must be.

    Returns:
      A tuple containing:
      - Signal value:
        - 100: Bullish Long Lower Shadow pattern detected
        - -100: Bearish Long Upper Shadow pattern detected
        - 0: No pattern detected
      - Updated EMA value of body sizes

    Examples:
      ```python
      >>> import kand
      >>> signal, body_avg = kand.cdl_long_shadow_inc(100.0, 102.0, 98.0, 100.1, 0.5, 14, 75.0)
      ```
    """
    ...

def cdl_marubozu(open, high, low, close, period, shadow_percent):
    """
    Detects Marubozu candlestick patterns in price data.

    Args:
      open: Opening prices as a 1-D NumPy array of type `TAFloat`.
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for EMA calculation of body sizes.
      shadow_percent: Maximum shadow size as percentage of body.

    Returns:
      A tuple of two 1-D NumPy arrays containing:
      - Pattern signals:
        - 1.0: Bullish Marubozu pattern detected
        - -1.0: Bearish Marubozu pattern detected
        - 0.0: No pattern detected
      - EMA values of candle body sizes

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> open = np.array([100.0, 101.0, 102.0])
      >>> high = np.array([102.0, 103.0, 104.0])
      >>> low = np.array([98.0, 99.0, 100.0])
      >>> close = np.array([101.0, 102.0, 103.0])
      >>> signals, body_avg = kand.cdl_marubozu(open, high, low, close, 14, 5.0)
      ```
    """
    ...

def cdl_marubozu_inc(open, high, low, close, prev_body_avg, period, shadow_percent):
    """
    Detects a Marubozu pattern in a single candlestick.

    Args:

      open: Opening price.
      high: High price.
      low: Low price.
      close: Close price.
      prev_body_avg: Previous EMA value of body sizes.
      period: Period for EMA calculation.
      shadow_percent: Maximum shadow size as percentage of body.

    Returns:
      A tuple containing:
      - Signal value:
        - 1.0: Bullish Marubozu pattern detected
        - -1.0: Bearish Marubozu pattern detected
        - 0.0: No pattern detected
      - Updated EMA value of body sizes

    Examples:
      ```python
      >>> import kand
      >>> signal, body_avg = kand.cdl_marubozu_inc(100.0, 102.0, 98.0, 100.1, 0.5, 14, 5.0)
      ```
    """
    ...

def dema(input_price, period):
    """
    Calculates Double Exponential Moving Average (DEMA) over NumPy arrays.

    Args:
      input_price: Price values as a 1-D NumPy array of type `TAFloat`.
      period: Smoothing period for EMA calculations. Must be >= 2.

    Returns:
      A tuple of 1-D NumPy arrays containing:
      - DEMA values
      - First EMA values
      - Second EMA values
      Each array has the same length as the input, with the first `2*(period-1)` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0])
      >>> dema, ema1, ema2 = kand.dema(prices, 3)
      ```
    """
    ...

def dema_inc(price, prev_ema1, prev_ema2, period):
    """
    Calculates the next DEMA value incrementally.

    Args:

      price: Current price value.
      prev_ema1: Previous value of first EMA.
      prev_ema2: Previous value of second EMA.
      period: Smoothing period. Must be >= 2.

    Returns:
      A tuple containing (DEMA, new_ema1, new_ema2).

    Examples:
      ```python
      >>> import kand
      >>> dema, ema1, ema2 = kand.dema_inc(10.0, 9.5, 9.0, 3)
      ```
    """
    ...

def dx(high, low, close, period):
    """
    Computes the Directional Movement Index (DX) over NumPy arrays.

    The DX indicator measures the strength of a trend by comparing positive and negative directional movements.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for DX calculation. Must be positive and less than input length.

    Returns:
      A tuple of four 1-D NumPy arrays containing:
      - DX values
      - Smoothed +DM values
      - Smoothed -DM values
      - Smoothed TR values
      Each array has the same length as the input, with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
      >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
      >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
      >>> dx, plus_dm, minus_dm, tr = kand.dx(high, low, close, 3)
      ```
    """
    ...

def dx_inc(input_high, input_low, prev_high, prev_low, prev_close, prev_smoothed_plus_dm, prev_smoothed_minus_dm, prev_smoothed_tr, param_period):
    """
    Calculates the latest DX value incrementally.

    Computes only the most recent DX value using previous smoothed values.
    Optimized for real-time calculations where only the latest value is needed.

    For the formula, refer to the [`dx`] function documentation.

    Args:
        input_high (float): Current high price.
        input_low (float): Current low price.
        prev_high (float): Previous period's high price.
        prev_low (float): Previous period's low price.
        prev_close (float): Previous period's close price.
        prev_smoothed_plus_dm (float): Previous smoothed +DM value.
        prev_smoothed_minus_dm (float): Previous smoothed -DM value.
        prev_smoothed_tr (float): Previous smoothed TR value.
        param_period (int): Period for DX calculation (typically 14).

    Returns:
        tuple: A tuple containing:
            - Latest DX value (float)
            - New smoothed +DM (float)
            - New smoothed -DM (float)
            - New smoothed TR (float)

    Example:
        >>> import kand
        >>> high, low = 24.20, 23.85
        >>> prev_high, prev_low, prev_close = 24.07, 23.72, 23.95
        >>> prev_smoothed_plus_dm = 0.5
        >>> prev_smoothed_minus_dm = 0.3
        >>> prev_smoothed_tr = 1.2
        >>> period = 14
        >>> dx, plus_dm, minus_dm, tr = kand.dx_inc(
        ...     high, low, prev_high, prev_low, prev_close,
        ...     prev_smoothed_plus_dm, prev_smoothed_minus_dm,
        ...     prev_smoothed_tr, period)
    """
    ...

def ecl(high, low, close):
    """
    Computes the Expanded Camarilla Levels (ECL) over NumPy arrays.

    The ECL indicator calculates multiple support and resistance levels based on the previous period's
    high, low and close prices.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      close: Input close prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A tuple of ten 1-D NumPy arrays containing the ECL values (H5,H4,H3,H2,H1,L1,L2,L3,L4,L5).
      Each array has the same length as the input, with the first element containing NaN value.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([24.20, 24.07, 24.04, 23.87, 23.67])
      >>> low = np.array([23.85, 23.72, 23.64, 23.37, 23.46])
      >>> close = np.array([23.89, 23.95, 23.67, 23.78, 23.50])
      >>> h5,h4,h3,h2,h1,l1,l2,l3,l4,l5 = kand.ecl(high, low, close)
      ```
    """
    ...

def ecl_inc(prev_high, prev_low, prev_close):
    """
    Computes the latest Expanded Camarilla Levels (ECL) values incrementally.

    This function provides an efficient way to calculate ECL values for new data without
    reprocessing the entire dataset.

    Args:

      prev_high: Previous period's high price as `TAFloat`.
      prev_low: Previous period's low price as `TAFloat`.
      prev_close: Previous period's close price as `TAFloat`.

    Returns:
      A tuple of ten values (H5,H4,H3,H2,H1,L1,L2,L3,L4,L5) containing the latest ECL levels.

    Examples:
      ```python
      >>> import kand
      >>> h5,h4,h3,h2,h1,l1,l2,l3,l4,l5 = kand.ecl_inc(24.20, 23.85, 23.89)
      ```
    """
    ...

def ema(data, period, k=None):
    """
    Computes the Exponential Moving Average (EMA) over a NumPy array.

    The Exponential Moving Average is calculated by applying more weight to recent prices
    via a smoothing factor k. Each value is calculated as:
    EMA = Price * k + EMA(previous) * (1 - k)
    where k is typically 2/(period+1).

    Args:
      data: Input data as a 1-D NumPy array of type `TAFloat`.
      period: Window size for EMA calculation. Must be positive and less than input length.
      k: Optional custom smoothing factor. If None, uses default k = 2/(period+1).

    Returns:
      A new 1-D NumPy array containing the EMA values. The array has the same length as the input,
      with the first `period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> result = kand.ema(data, 3)
      >>> print(result)
      [nan, nan, 2.0, 3.0, 4.2]
      ```
    """
    ...

def ema_inc(price, prev_ema, period, k=None):
    """
    Computes the latest EMA value incrementally.

    This function provides an efficient way to calculate EMA values for new data without
    reprocessing the entire dataset.

    Args:

      price: Current period's price value as `TAFloat`.
      prev_ema: Previous period's EMA value as `TAFloat`.
      period: Window size for EMA calculation. Must be >= 2.
      k: Optional custom smoothing factor. If None, uses default k = 2/(period+1).

    Returns:
      The new EMA value as `TAFloat`.

    Examples:
      ```python
      >>> import kand
      >>> current_price = 15.0
      >>> prev_ema = 14.5
      >>> period = 14
      >>> new_ema = kand.ema_inc(current_price, prev_ema, period)
      ```
    """
    ...

def macd(data, fast_period, slow_period, signal_period):
    """
    Computes the Moving Average Convergence Divergence (MACD) over a NumPy array.

    MACD is a trend-following momentum indicator that shows the relationship between two moving averages
    of an asset's price. It consists of three components:
    - MACD Line: Difference between fast and slow EMAs
    - Signal Line: EMA of the MACD line
    - Histogram: Difference between MACD line and signal line

    Args:
      data: Input price data as a 1-D NumPy array of type `TAFloat`.
      fast_period: Period for fast EMA calculation (typically 12).
      slow_period: Period for slow EMA calculation (typically 26).
      signal_period: Period for signal line calculation (typically 9).

    Returns:
      A tuple of five 1-D NumPy arrays containing:
      - MACD line values
      - Signal line values
      - MACD histogram values
      - Fast EMA values
      - Slow EMA values
      Each array has the same length as the input, with initial elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> macd_line, signal_line, histogram, fast_ema, slow_ema = kand.macd(data, 2, 3, 2)
      ```
    """
    ...

def macd_inc(price, prev_fast_ema, prev_slow_ema, prev_signal, fast_period, slow_period, signal_period):
    """
    Computes the latest MACD values incrementally from previous state.

    This function provides an efficient way to calculate MACD for streaming data by using
    previous EMA values instead of recalculating the entire series.

    Args:

      price: Current price value as `TAFloat`.
      prev_fast_ema: Previous fast EMA value as `TAFloat`.
      prev_slow_ema: Previous slow EMA value as `TAFloat`.
      prev_signal: Previous signal line value as `TAFloat`.
      fast_period: Period for fast EMA calculation (typically 12).
      slow_period: Period for slow EMA calculation (typically 26).
      signal_period: Period for signal line calculation (typically 9).

    Returns:
      A tuple of three values:
      - MACD line value
      - Signal line value
      - MACD histogram value

    Examples:
      ```python
      >>> import kand
      >>> macd_line, signal_line, histogram = kand.macd_inc(
      ...     100.0,  # current price
      ...     95.0,   # previous fast EMA
      ...     98.0,   # previous slow EMA
      ...     -2.5,   # previous signal
      ...     12,     # fast period
      ...     26,     # slow period
      ...     9       # signal period
      ... )
      ```
    """
    ...

def max(prices, period):
    """
    Calculate Maximum Value for a NumPy array

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for MAX calculation (must be >= 2).

    Returns:
      A 1-D NumPy array containing MAX values. The first (period-1) elements contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([1.0, 2.0, 3.0, 2.5, 4.0])
      >>> max_values = kand.max(prices, 3)
      ```
    """
    ...

def max_inc(price, prev_max, old_price, period):
    """
    Calculate the latest Maximum Value incrementally

    Args:
      py: Python interpreter token
      price: Current period's price
      prev_max: Previous period's MAX value
      old_price: Price being removed from the period
      period: Period for MAX calculation (must be >= 2)

    Returns:
      The new MAX value

    Examples:
      ```python
      >>> import kand
      >>> new_max = kand.max_inc(10.5, 11.0, 9.0, 14)
      ```
    """
    ...

def medprice(high, low):
    """
    Calculates the Median Price (MEDPRICE) for a NumPy array.

    The Median Price is a technical analysis indicator that represents the middle point between
    high and low prices for each period.

    Args:
      high: Array of high prices as a 1-D NumPy array of type `TAFloat`.
      low: Array of low prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A 1-D NumPy array containing the median price values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 11.0, 12.0])
      >>> low = np.array([8.0, 9.0, 10.0])
      >>> result = kand.medprice(high, low)
      >>> print(result)
      [9.0, 10.0, 11.0]
      ```
    """
    ...

def medprice_inc(high, low):
    """
    Calculates a single Median Price value incrementally.

    Args:

      high: Current period's high price as `TAFloat`.
      low: Current period's low price as `TAFloat`.

    Returns:
      The calculated median price value.

    Examples:
      ```python
      >>> import kand
      >>> result = kand.medprice_inc(10.0, 8.0)
      >>> print(result)
      9.0
      ```
    """
    ...

def mfi(high, low, close, volume, period):
    """
    Calculates the Money Flow Index (MFI) for a NumPy array.

    The Money Flow Index (MFI) is a technical oscillator that uses price and volume data to identify
    overbought or oversold conditions in an asset.

    Args:
      high: Array of high prices as a 1-D NumPy array of type `TAFloat`.
      low: Array of low prices as a 1-D NumPy array of type `TAFloat`.
      close: Array of close prices as a 1-D NumPy array of type `TAFloat`.
      volume: Array of volume data as a 1-D NumPy array of type `TAFloat`.
      period: The time period for MFI calculation (typically 14).

    Returns:
      A tuple of five 1-D NumPy arrays containing:
      - MFI values (0-100)
      - Typical prices
      - Money flows
      - Positive money flows
      - Negative money flows

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 11.0, 12.0, 11.0])
      >>> low = np.array([8.0, 9.0, 10.0, 9.0])
      >>> close = np.array([9.0, 10.0, 11.0, 10.0])
      >>> volume = np.array([100.0, 150.0, 200.0, 150.0])
      >>> mfi, typ_prices, money_flows, pos_flows, neg_flows = kand.mfi(high, low, close, volume, 2)
      ```
    """
    ...

def midpoint(data, period):
    """
    Calculates Midpoint values for a NumPy array.

    The Midpoint is a technical indicator that represents the arithmetic mean of the highest and lowest
    prices over a specified period.

    Args:
      data: Input price data as a 1-D NumPy array of type `TAFloat`.
      period: Time period for calculation (must be >= 2).

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - Midpoint values
      - Highest values for each period
      - Lowest values for each period
      Each array has the same length as the input, with initial elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> midpoint, highest, lowest = kand.midpoint(data, 3)
      ```
    """
    ...

def midpoint_inc(price, prev_highest, prev_lowest, period):
    """
    Calculates the next Midpoint value incrementally.

    Provides an optimized way to calculate the next Midpoint value when new data arrives,
    without recalculating the entire series.

    Args:
      price: Current price value as `TAFloat`.
      prev_highest: Previous highest value as `TAFloat`.
      prev_lowest: Previous lowest value as `TAFloat`.
      period: Time period for calculation (must be >= 2).

    Returns:
      A tuple containing:
      - Midpoint value
      - New highest value
      - New lowest value

    Examples:
      ```python
      >>> import kand
      >>> midpoint, new_highest, new_lowest = kand.midpoint_inc(
      ...     15.0,  # current price
      ...     16.0,  # previous highest
      ...     14.0,  # previous lowest
      ...     14     # period
      ... )
      ```
    """
    ...

def midprice(high, low, period):
    """
    Calculates Midpoint Price values for a NumPy array.

    The Midpoint Price is a technical indicator that represents the mean value between the highest high
    and lowest low prices over a specified period.

    Args:
      high: Input high price data as a 1-D NumPy array of type `TAFloat`.
      low: Input low price data as a 1-D NumPy array of type `TAFloat`.
      period: Time period for calculation (must be >= 2).

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - Midpoint Price values
      - Highest high values for each period
      - Lowest low values for each period
      Each array has the same length as the input, with initial elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> midprice, highest, lowest = kand.midprice(high, low, 3)
      ```
    """
    ...

def midprice_inc(high, low, prev_highest, prev_lowest, period):
    """
    Calculates the next Midpoint Price value incrementally.

    Provides an optimized way to calculate the next Midpoint Price value when new data arrives,
    without recalculating the entire series.

    Args:

      high: Current high price value as `TAFloat`.
      low: Current low price value as `TAFloat`.
      prev_highest: Previous highest high value as `TAFloat`.
      prev_lowest: Previous lowest low value as `TAFloat`.
      period: Time period for calculation (must be >= 2).

    Returns:
      A tuple containing:
      - Midpoint Price value
      - New highest high value
      - New lowest low value

    Examples:
      ```python
      >>> import kand
      >>> midprice, new_highest, new_lowest = kand.midprice_inc(
      ...     10.5,  # current high
      ...     9.8,   # current low
      ...     10.2,  # previous highest high
      ...     9.5,   # previous lowest low
      ...     14     # period
      ... )
      ```
    """
    ...

def min(prices, period):
    """
    Calculate Minimum Value (MIN) for a NumPy array

    The MIN indicator finds the lowest price value within a given time period.

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for MIN calculation (must be >= 2).

    Returns:
      A 1-D NumPy array containing MIN values. First (period-1) elements contain NaN.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([10.0, 8.0, 6.0, 7.0, 9.0])
      >>> min_values = kand.min(prices, 3)
      ```
    """
    ...

def min_inc(price, prev_min, prev_price, period):
    """
    Calculate the latest MIN value incrementally

    Args:
      py: Python interpreter token
      price: Current period's price
      prev_min: Previous period's MIN value
      prev_price: Price value being removed from the period
      period: Period for MIN calculation (must be >= 2)

    Returns:
      The new MIN value

    Examples:
      ```python
      >>> import kand
      >>> new_min = kand.min_inc(15.0, 12.0, 14.0, 14)
      ```
    """
    ...

def minus_di(high, low, close, period):
    """
    Computes the Minus Directional Indicator (-DI) over NumPy arrays.

    The -DI measures the presence and strength of a downward price trend. It is one component used in calculating
    the Average Directional Index (ADX), which helps determine trend strength.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for -DI calculation. Must be positive and less than input length.

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - The -DI values
      - The smoothed -DM values
      - The smoothed TR values
      Each array has the same length as the input, with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([35.0, 36.0, 35.5, 35.8, 36.2])
      >>> low = np.array([34.0, 35.0, 34.5, 34.8, 35.2])
      >>> close = np.array([34.5, 35.5, 35.0, 35.3, 35.7])
      >>> minus_di, smoothed_minus_dm, smoothed_tr = kand.minus_di(high, low, close, 3)
      >>> print(minus_di)
      [nan, nan, nan, 25.3, 24.1]
      ```
    """
    ...

def minus_dm(high, low, period):
    """
    Computes the Minus Directional Movement (-DM) over NumPy arrays.

    Minus Directional Movement (-DM) measures downward price movement and is used as part of the
    Directional Movement System developed by J. Welles Wilder.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for -DM calculation. Must be positive and less than input length.

    Returns:
      A new 1-D NumPy array containing the -DM values. The array has the same length as the input,
      with the first `period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([35266.0, 35247.5, 35235.7, 35190.8, 35182.0])
      >>> low = np.array([35216.1, 35206.5, 35180.0, 35130.7, 35153.6])
      >>> result = kand.minus_dm(high, low, 3)
      ```
    """
    ...

def mom(data, period):
    """
    Computes the Momentum (MOM) over a NumPy array.

    Momentum measures the change in price between the current price and the price n periods ago.

    Args:
      data: Input data as a 1-D NumPy array of type `TAFloat`.
      period: Window size for momentum calculation. Must be positive and less than input length.

    Returns:
      A new 1-D NumPy array containing the momentum values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([2.0, 4.0, 6.0, 8.0, 10.0])
      >>> result = kand.mom(data, 2)
      >>> print(result)
      [nan, nan, 4.0, 4.0, 4.0]
      ```
    """
    ...

def mom_inc(current_price, old_price):
    """
    Calculates the next Momentum (MOM) value incrementally.

    This function provides an optimized way to calculate the latest momentum value
    when streaming data is available, without needing the full price history.

    Args:

      current_price: The current period's price value.
      old_price: The price value from n periods ago.

    Returns:
      The calculated momentum value.

    Examples:
      ```python
      >>> import kand
      >>> momentum = kand.mom_inc(10.0, 6.0)
      >>> print(momentum)
      4.0
      ```
    """
    ...

def natr(high, low, close, period):
    """
    Computes the Normalized Average True Range (NATR) over NumPy arrays.

    The NATR is a measure of volatility that accounts for the price level of the instrument.
    It expresses the ATR as a percentage of the closing price.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for NATR calculation. Must be positive and less than input length.

    Returns:
      A new 1-D NumPy array containing the NATR values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
      >>> result = kand.natr(high, low, close, 3)
      ```
    """
    ...

def natr_inc(high, low, close, prev_close, prev_atr, period):
    """
    Calculates the next NATR value incrementally.

    This function provides an optimized way to calculate a single new NATR value
    using the previous ATR value and current price data, without recalculating the entire series.

    Args:

      high: Current period's high price.
      low: Current period's low price.
      close: Current period's closing price.
      prev_close: Previous period's closing price.
      prev_atr: Previous period's ATR value.
      period: Period for NATR calculation (must be >= 2).

    Returns:
      The calculated NATR value.

    Examples:
      ```python
      >>> import kand
      >>> natr = kand.natr_inc(
      ...     15.0,  # high
      ...     11.0,  # low
      ...     14.0,  # close
      ...     12.0,  # prev_close
      ...     3.0,   # prev_atr
      ...     3      # period
      ... )
      ```
    """
    ...

def obv(close, volume):
    """
    Computes the On Balance Volume (OBV) over NumPy arrays.

    On Balance Volume (OBV) is a momentum indicator that uses volume flow to predict changes in stock price.
    When volume increases without a significant price change, the price will eventually jump upward.
    When volume decreases without a significant price change, the price will eventually jump downward.

    Args:
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      volume: Volume data as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A new 1-D NumPy array containing the OBV values. The array has the same length as the input.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> close = np.array([10.0, 12.0, 11.0, 13.0])
      >>> volume = np.array([100.0, 150.0, 120.0, 200.0])
      >>> result = kand.obv(close, volume)
      >>> print(result)
      [100.0, 250.0, 130.0, 330.0]
      ```
    """
    ...

def obv_inc(curr_close, prev_close, volume, prev_obv):
    """
    Calculates the next OBV value incrementally.

    This function provides an optimized way to calculate a single new OBV value
    using the previous OBV value and current price/volume data.

    Args:
      curr_close: Current closing price as `TAFloat`.
      prev_close: Previous closing price as `TAFloat`.
      volume: Current volume as `TAFloat`.
      prev_obv: Previous OBV value as `TAFloat`.

    Returns:
      The calculated OBV value.

    Examples:
      ```python
      >>> import kand
      >>> curr_close = 12.0
      >>> prev_close = 10.0
      >>> volume = 150.0
      >>> prev_obv = 100.0
      >>> result = kand.obv_inc(curr_close, prev_close, volume, prev_obv)
      >>> print(result)
      250.0
      ```
    """
    ...

def plus_di(high, low, close, period):
    """
    Computes the Plus Directional Indicator (+DI) over NumPy arrays.

    +DI measures the presence and strength of an upward price trend. It is one component used in calculating
    the Average Directional Index (ADX), which helps determine trend strength.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for +DI calculation. Must be positive and less than input length.

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - +DI values
      - Smoothed +DM values
      - Smoothed TR values
      Each array has the same length as the input, with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 11.5, 11.0])
      >>> low = np.array([9.0, 10.0, 10.0, 9.5])
      >>> close = np.array([9.5, 11.0, 10.5, 10.0])
      >>> plus_di, smoothed_plus_dm, smoothed_tr = kand.plus_di(high, low, close, 2)
      ```
    """
    ...

def plus_di_inc(high, low, prev_high, prev_low, prev_close, prev_smoothed_plus_dm, prev_smoothed_tr, period):
    """
    Calculates the next +DI value incrementally using previous smoothed values.

    This function enables real-time calculation of +DI by using the previous smoothed values
    and current price data, avoiding the need to recalculate the entire series.

    Args:
      high: Current high price as `TAFloat`.
      low: Current low price as `TAFloat`.
      prev_high: Previous high price as `TAFloat`.
      prev_low: Previous low price as `TAFloat`.
      prev_close: Previous close price as `TAFloat`.
      prev_smoothed_plus_dm: Previous smoothed +DM value as `TAFloat`.
      prev_smoothed_tr: Previous smoothed TR value as `TAFloat`.
      period: Smoothing period (>= 2).

    Returns:
      A tuple containing (latest +DI, new smoothed +DM, new smoothed TR).

    Examples:
      ```python
      >>> import kand
      >>> plus_di, smoothed_plus_dm, smoothed_tr = kand.plus_di_inc(
      ...     10.5,  # high
      ...     9.5,   # low
      ...     10.0,  # prev_high
      ...     9.0,   # prev_low
      ...     9.5,   # prev_close
      ...     15.0,  # prev_smoothed_plus_dm
      ...     20.0,  # prev_smoothed_tr
      ...     14     # period
      ... )
      ```
    """
    ...

def plus_dm(high, low, period):
    """
    Computes the Plus Directional Movement (+DM) over NumPy arrays.

    Plus Directional Movement (+DM) measures upward price movement and is used as part of the
    Directional Movement System developed by J. Welles Wilder.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for +DM calculation. Must be positive and less than input length.

    Returns:
      A new 1-D NumPy array containing the +DM values. The array has the same length as the input,
      with the first `period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([35266.0, 35247.5, 35235.7, 35190.8, 35182.0])
      >>> low = np.array([35216.1, 35206.5, 35180.0, 35130.7, 35153.6])
      >>> result = kand.plus_dm(high, low, 3)
      ```
    """
    ...

def plus_dm_inc(high, prev_high, low, prev_low, prev_plus_dm, period):
    """
    Calculates the next Plus DM value incrementally using previous values.

    This function enables real-time calculation of Plus DM by using the previous Plus DM value
    and current price data, avoiding the need to recalculate the entire series.

    Args:
      high: Current high price as `TAFloat`.
      prev_high: Previous high price as `TAFloat`.
      low: Current low price as `TAFloat`.
      prev_low: Previous low price as `TAFloat`.
      prev_plus_dm: Previous Plus DM value as `TAFloat`.
      period: Smoothing period (>= 2).

    Returns:
      The latest Plus DM value.

    Examples:
      ```python
      >>> import kand
      >>> new_plus_dm = kand.plus_dm_inc(
      ...     10.5,  # high
      ...     10.0,  # prev_high
      ...     9.8,   # low
      ...     9.5,   # prev_low
      ...     0.45,  # prev_plus_dm
      ...     14     # period
      ... )
      ```
    """
    ...

def rma(data, period):
    """
    Computes the Running Moving Average (RMA) over a NumPy array.

    The Running Moving Average is similar to an Exponential Moving Average (EMA) but uses a different
    smoothing factor. It is calculated using a weighted sum of the current value and previous RMA value,
    with weights determined by the period size.

    Args:
      data: Input data as a 1-D NumPy array of type `TAFloat`.
      period: Window size for RMA calculation. Must be positive and less than input length.

    Returns:
      A new 1-D NumPy array containing the RMA values. The array has the same length as the input,
      with the first `period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> result = kand.rma(data, 3)
      >>> print(result)
      [nan, nan, 2.0, 2.67, 3.44]
      ```
    """
    ...

def rma_inc(current_price, prev_rma, period):
    """
    Calculates the next RMA value incrementally.

    This function provides an optimized way to calculate the latest RMA value
    when streaming data is available, without needing the full price history.

    Args:
      current_price: The current period's price value.
      prev_rma: The previous period's RMA value.
      period: The smoothing period (must be >= 2).

    Returns:
      The calculated RMA value.

    Examples:
      ```python
      >>> import kand
      >>> new_rma = kand.rma_inc(10.0, 9.5, 14)
      ```
    """
    ...

def roc(data, period):
    """
    Computes the Rate of Change (ROC) over a NumPy array.

    The Rate of Change (ROC) is a momentum oscillator that measures the percentage change in price
    between the current price and the price n periods ago.

    Args:
      data: Input price data as a 1-D NumPy array of type `TAFloat`.
      period: Number of periods to look back. Must be positive.

    Returns:
      A new 1-D NumPy array containing the ROC values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
      >>> result = kand.roc(data, 2)
      >>> print(result)
      [nan, nan, 12.0, 2.86, 6.48]
      ```
    """
    ...

def roc_inc(current_price, prev_price):
    """
    Calculates a single ROC value incrementally.

    This function provides an optimized way to calculate the latest ROC value
    when streaming data is available, without needing the full price history.

    Args:
      current_price: The current period's price value.
      prev_price: The price from n periods ago.

    Returns:
      The calculated ROC value.

    Examples:
      ```python
      >>> import kand
      >>> roc = kand.roc_inc(11.5, 10.0)
      >>> print(roc)
      15.0
      ```
    """
    ...

def rocp(data, period):
    """
    Computes the Rate of Change Percentage (ROCP) over a NumPy array.

    The Rate of Change Percentage (ROCP) is a momentum indicator that measures the percentage change
    between the current price and the price n periods ago.

    Args:
      data: Input price data as a 1-D NumPy array of type `TAFloat`.
      period: Number of periods to look back. Must be positive.

    Returns:
      A new 1-D NumPy array containing the ROCP values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
      >>> result = kand.rocp(data, 2)
      >>> print(result)
      [nan, nan, 0.12, 0.0286, 0.0648]
      ```
    """
    ...

def rocp_inc(current_price, prev_price):
    """
    Calculates a single ROCP value incrementally.

    This function provides an optimized way to calculate the latest ROCP value
    when streaming data is available, without needing the full price history.

    Args:
      current_price: The current period's price value.
      prev_price: The price from n periods ago.

    Returns:
      The calculated ROCP value.

    Examples:
      ```python
      >>> import kand
      >>> rocp = kand.rocp_inc(11.5, 10.0)
      >>> print(rocp)
      0.15
      ```
    """
    ...

def rocr(data, period):
    """
    Computes the Rate of Change Ratio (ROCR) over a NumPy array.

    The Rate of Change Ratio (ROCR) is a momentum indicator that measures the ratio between
    the current price and the price n periods ago.

    Args:
      data: Input price data as a 1-D NumPy array of type `TAFloat`.
      period: Number of periods to look back. Must be >= 2.

    Returns:
      A new 1-D NumPy array containing the ROCR values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
      >>> result = kand.rocr(data, 2)
      >>> print(result)
      [nan, nan, 1.12, 1.0286, 1.0648]
      ```
    """
    ...

def rocr100(data, period):
    """
    Computes the Rate of Change Ratio * 100 (ROCR100) over a NumPy array.

    ROCR100 is a momentum indicator that measures the percentage change in price over a specified period.
    It compares the current price to a past price and expresses the ratio as a percentage.
    Values above 100 indicate price increases, while values below 100 indicate price decreases.

    Args:
      data: Input price data as a 1-D NumPy array of type `TAFloat`.
      period: Number of periods to look back. Must be >= 2.

    Returns:
      A new 1-D NumPy array containing the ROCR100 values. The array has the same length as the input,
      with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([10.0, 10.5, 11.2, 10.8, 11.5])
      >>> result = kand.rocr100(data, 2)
      >>> print(result)
      [nan, nan, 106.67, 102.86, 106.48]
      ```
    """
    ...

def rocr100_inc(current_price, prev_price):
    """
    Calculates a single ROCR100 value incrementally.

    This function provides an optimized way to calculate the latest ROCR100 value
    when streaming data is available, without needing the full price history.

    Args:
      current_price: The current period's price value.
      prev_price: The price from n periods ago.

    Returns:
      The calculated ROCR100 value.

    Examples:
      ```python
      >>> import kand
      >>> rocr100 = kand.rocr100_inc(11.5, 10.0)
      >>> print(rocr100)
      115.0
      ```
    """
    ...

def rocr_inc(current_price, prev_price):
    """
    Calculates a single ROCR value incrementally.

    This function provides an optimized way to calculate the latest ROCR value
    when streaming data is available, without needing the full price history.

    Args:
      current_price: The current period's price value.
      prev_price: The price from n periods ago.

    Returns:
      The calculated ROCR value.

    Examples:
      ```python
      >>> import kand
      >>> rocr = kand.rocr_inc(11.5, 10.0)
      >>> print(rocr)
      1.15
      ```
    """
    ...

def rsi(prices, period):
    """
    Computes the Relative Strength Index (RSI) over NumPy arrays.

    The RSI is a momentum oscillator that measures the speed and magnitude of recent price changes
    to evaluate overbought or oversold conditions.

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Window size for RSI calculation. Must be positive and less than input length.

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - RSI values
      - Average gain values
      - Average loss values
      Each array has the same length as the input, with the first `period` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10, 45.42])
      >>> rsi, avg_gain, avg_loss = kand.rsi(prices, 5)
      ```
    """
    ...

def rsi_inc(current_price, prev_price, prev_avg_gain, prev_avg_loss, period):
    """
    Calculates a single RSI value incrementally.

    This function provides an optimized way to calculate the latest RSI value
    when streaming data is available, without needing the full price history.

    Args:
      current_price: The current period's price value.
      prev_price: The previous period's price value.
      prev_avg_gain: The previous period's average gain.
      prev_avg_loss: The previous period's average loss.
      period: The time period for RSI calculation.

    Returns:
      A tuple containing (RSI value, new average gain, new average loss).

    Examples:
      ```python
      >>> import kand
      >>> rsi, avg_gain, avg_loss = kand.rsi_inc(45.42, 45.10, 0.24, 0.14, 14)
      ```
    """
    ...

def sar(high, low, acceleration, maximum):
    """
    Calculates the Parabolic SAR (Stop And Reverse) indicator over NumPy arrays.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      acceleration: Initial acceleration factor (e.g. 0.02).
      maximum: Maximum acceleration factor (e.g. 0.2).

    Returns:
      A tuple of four 1-D NumPy arrays containing:
      - SAR values
      - Trend direction (true=long, false=short)
      - Acceleration factors
      - Extreme points
      Each array has the same length as the input, with the first element containing NaN.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> sar, is_long, af, ep = kand.sar(high, low, 0.02, 0.2)
      ```
    """
    ...

def sar_inc(high, low, prev_high, prev_low, prev_sar, is_long, af, ep, acceleration, maximum):
    """
    Incrementally updates the Parabolic SAR with new price data.

    Args:
      high: Current period's high price.
      low: Current period's low price.
      prev_high: Previous period's high price.
      prev_low: Previous period's low price.
      prev_sar: Previous period's SAR value.
      is_long: Current trend direction (true=long, false=short).
      af: Current acceleration factor.
      ep: Current extreme point.
      acceleration: Acceleration factor increment.
      maximum: Maximum acceleration factor.

    Returns:
      A tuple containing (SAR value, trend direction, acceleration factor, extreme point).

    Examples:
      ```python
      >>> import kand
      >>> sar, is_long, af, ep = kand.sar_inc(
      ...     15.0, 14.0, 14.5, 13.5, 13.0, True, 0.02, 14.5, 0.02, 0.2
      ... )
      ```
    """
    ...

def sma(data, period):
    """
    Computes the Simple Moving Average (SMA) over a NumPy array.

    The Simple Moving Average is calculated by taking the arithmetic mean of a window of values
    that moves across the input array. For each position, it sums the previous `period` values
    and divides by the period size.

    Args:
        data: Input data as a 1-D NumPy array of type `TAFloat`.
        period: Window size for SMA calculation. Must be positive and less than input length.

    Returns:
        A new 1-D NumPy array containing the SMA values. The array has the same length as the input,
        with the first `period-1` elements containing NaN values.

    Examples:
        ```python
        >>> import numpy as np
        >>> import kand
        >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
        >>> result = kand.sma(data, 3)
        >>> print(result)
        [nan, nan, 2.0, 3.0, 4.0]
        ```
    """
    ...

def sma_inc(prev_sma, new_price, old_price, period):
    """
    Incrementally calculates the next SMA value.

    This function provides an optimized way to update an existing SMA value
    when new data arrives, without recalculating the entire series.

    Args:
        prev_sma: Previous SMA value.
        new_price: New price to include in calculation.
        old_price: Oldest price to remove from calculation.
        period: The time period for SMA calculation (must be >= 2).

    Returns:
        The next SMA value.

    Examples:
        ```python
        >>> import kand
        >>> prev_sma = 4.0
        >>> new_price = 10.0
        >>> old_price = 2.0
        >>> period = 3
        >>> next_sma = kand.sma_inc(prev_sma, new_price, old_price, period)
        >>> print(next_sma)
        6.666666666666666
        ```
    """
    ...

def stddev(input, period):
    """
    Calculate Standard Deviation for a NumPy array

    Standard Deviation measures the dispersion of values from their mean over a specified period.
    It is calculated by taking the square root of the variance.

    Args:
      input: Input values as a 1-D NumPy array of type `TAFloat`.
      period: Period for calculation (must be >= 2).

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - Standard Deviation values
      - Running sum values
      - Running sum of squares values
      Each array has the same length as the input, with the first (period-1) elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> stddev, sum, sum_sq = kand.stddev(prices, 3)
      ```
    """
    ...

def stddev_inc(price, prev_sum, prev_sum_sq, old_price, period):
    """
    Calculate the latest Standard Deviation value incrementally

    Args:
      py: Python interpreter token
      price: Current period's price
      prev_sum: Previous period's sum
      prev_sum_sq: Previous period's sum of squares
      old_price: Price being removed from the period
      period: Period for calculation (must be >= 2)

    Returns:
      A tuple containing:
      - Latest Standard Deviation value
      - New sum
      - New sum of squares

    Examples:
      ```python
      >>> import kand
      >>> stddev, sum, sum_sq = kand.stddev_inc(
      ...     10.0,   # current price
      ...     100.0,  # previous sum
      ...     1050.0, # previous sum of squares
      ...     8.0,    # old price
      ...     14      # period
      ... )
      ```
    """
    ...

def stoch(high, low, close, k_period, k_slow_period, d_period):
    """
    Computes the Stochastic Oscillator indicator over NumPy arrays.

    The Stochastic Oscillator is a momentum indicator that shows the location of the close
    relative to the high-low range over a set number of periods. The indicator consists of
    two lines: %K (the fast line) and %D (the slow line).

    Args:
        high: High prices as a 1-D NumPy array of type `TAFloat`.
        low: Low prices as a 1-D NumPy array of type `TAFloat`.
        close: Close prices as a 1-D NumPy array of type `TAFloat`.
        k_period: Period for %K calculation. Must be >= 2.
        k_slow_period: Smoothing period for slow %K. Must be >= 2.
        d_period: Period for %D calculation. Must be >= 2.

    Returns:
        A tuple of three 1-D NumPy arrays containing:
        - Fast %K values
        - Slow %K values
        - %D values
        Each array has the same length as the input, with initial values being NaN.

    Examples:
        ```python
        >>> import numpy as np
        >>> import kand
        >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
        >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
        >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
        >>> fast_k, k, d = kand.stoch(high, low, close, 3, 2, 2)
        ```
    """
    ...

def sum(input, period):
    """
    Calculate Sum for a NumPy array

    Calculates the rolling sum of values over a specified period.

    Args:
      input: Input values as a 1-D NumPy array of type `TAFloat`.
      period: Period for sum calculation (must be >= 2).

    Returns:
      A 1-D NumPy array containing the sum values.
      The first (period-1) elements contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> sums = kand.sum(data, 3)
      ```
    """
    ...

def sum_inc(new_price, old_price, prev_sum):
    """
    Calculate the latest sum value incrementally

    Args:
      py: Python interpreter token
      new_price: The newest price value to add
      old_price: The oldest price value to remove
      prev_sum: The previous sum value

    Returns:
      The new sum value

    Examples:
      ```python
      >>> import kand
      >>> new_sum = kand.sum_inc(
      ...     5.0,    # new price
      ...     3.0,    # old price
      ...     10.0,   # previous sum
      ... )
      ```
    """
    ...

def supertrend(high, low, close, period, multiplier):
    """
    Computes the Supertrend indicator over NumPy arrays.

    The Supertrend indicator is a trend-following indicator that combines Average True Range (ATR)
    with basic upper and lower bands to identify trend direction and potential reversal points.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for ATR calculation (typically 7-14). Must be positive.
      multiplier: ATR multiplier (typically 2-4).

    Returns:
      A tuple of five 1-D NumPy arrays:
      - trend: Array containing trend direction (1.0 for uptrend, -1.0 for downtrend)
      - supertrend: Array containing Supertrend values
      - atr: Array containing ATR values
      - upper: Array containing upper band values
      - lower: Array containing lower band values
      All arrays have the same length as the input, with the first `period-1` elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
      >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
      >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
      >>> trend, supertrend, atr, upper, lower = kand.supertrend(high, low, close, 3, 3.0)
      ```
    """
    ...

def supertrend_inc(high, low, close, prev_close, prev_atr, prev_trend, prev_upper, prev_lower, period, multiplier):
    """
    Calculates a single Supertrend value incrementally.

    This function provides an optimized way to calculate the latest Supertrend value
    using previous values, making it ideal for real-time calculations.

    Args:
      high: Current period's high price
      low: Current period's low price
      close: Current period's close price
      prev_close: Previous period's close price
      prev_atr: Previous period's ATR value
      prev_trend: Previous period's trend direction (1 for uptrend, -1 for downtrend)
      prev_upper: Previous period's upper band
      prev_lower: Previous period's lower band
      period: ATR calculation period (typically 7-14)
      multiplier: ATR multiplier (typically 2-4)

    Returns:
      A tuple containing:
      - trend: Current trend direction (1 for uptrend, -1 for downtrend)
      - supertrend: Current Supertrend value
      - atr: Current ATR value
      - upper: Current upper band
      - lower: Current lower band

    Examples:
      ```python
      >>> import kand
      >>> trend, supertrend, atr, upper, lower = kand.supertrend_inc(
      ...     15.0,   # Current high
      ...     11.0,   # Current low
      ...     14.0,   # Current close
      ...     11.0,   # Previous close
      ...     2.0,    # Previous ATR
      ...     1,      # Previous trend
      ...     16.0,   # Previous upper band
      ...     10.0,   # Previous lower band
      ...     7,      # ATR period
      ...     3.0,    # Multiplier
      ... )
      ```
    """
    ...

def t3(data, period, vfactor):
    """
    Computes the T3 (Triple Exponential Moving Average) indicator over a NumPy array.

    T3 is a sophisticated moving average developed by Tim Tillson that reduces lag while maintaining smoothness.
    It combines six EMAs with optimized weightings to produce a responsive yet smooth indicator.

    Args:
        data: Input data as a 1-D NumPy array of type `TAFloat`.
        period: Smoothing period for EMAs (must be >= 2).
        vfactor: Volume factor controlling smoothing (typically 0-1).

    Returns:
        A tuple of seven 1-D NumPy arrays containing:
        - T3 values
        - EMA1 values
        - EMA2 values
        - EMA3 values
        - EMA4 values
        - EMA5 values
        - EMA6 values
        Each array has the same length as the input, with initial values being NaN.

    Examples:
        ```python
        >>> import numpy as np
        >>> import kand
        >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
        >>> t3, e1, e2, e3, e4, e5, e6 = kand.t3(data, 2, 0.7)
        ```
    """
    ...

def t3_inc(price, prev_ema1, prev_ema2, prev_ema3, prev_ema4, prev_ema5, prev_ema6, period, vfactor):
    """
    Incrementally calculates the next T3 value.

    This function provides an optimized way to update T3 values in real-time by using
    previously calculated EMA values.

    Args:
        price: Latest price value to calculate T3 from.
        prev_ema1: Previous EMA1 value.
        prev_ema2: Previous EMA2 value.
        prev_ema3: Previous EMA3 value.
        prev_ema4: Previous EMA4 value.
        prev_ema5: Previous EMA5 value.
        prev_ema6: Previous EMA6 value.
        period: Smoothing period for EMAs (must be >= 2).
        vfactor: Volume factor (typically 0-1).

    Returns:
        A tuple containing:
        - Latest T3 value
        - Updated EMA1 value
        - Updated EMA2 value
        - Updated EMA3 value
        - Updated EMA4 value
        - Updated EMA5 value
        - Updated EMA6 value

    Examples:
        ```python
        >>> import kand
        >>> t3, e1, e2, e3, e4, e5, e6 = kand.t3_inc(
        ...     100.0,  # New price
        ...     95.0,   # Previous EMA1
        ...     94.0,   # Previous EMA2
        ...     93.0,   # Previous EMA3
        ...     92.0,   # Previous EMA4
        ...     91.0,   # Previous EMA5
        ...     90.0,   # Previous EMA6
        ...     5,      # Period
        ...     0.7,    # Volume factor
        ... )
        ```
    """
    ...

def tema(prices, period):
    """
    Calculate Triple Exponential Moving Average (TEMA) for a NumPy array.

    TEMA is an enhanced moving average designed to reduce lag while maintaining smoothing properties.
    It applies triple exponential smoothing to put more weight on recent data and less on older data.

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Smoothing period for calculations (must be >= 2).

    Returns:
      A tuple of 4 1-D NumPy arrays containing:
      - TEMA values
      - First EMA values
      - Second EMA values
      - Third EMA values
      The first (3 * (period - 1)) elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0])
      >>> tema, ema1, ema2, ema3 = kand.tema(prices, 3)
      ```
    """
    ...

def tema_inc(new_price, prev_ema1, prev_ema2, prev_ema3, period):
    """
    Calculate the next TEMA value incrementally.

    Args:
      new_price: Latest price value to process.
      prev_ema1: Previous value of first EMA.
      prev_ema2: Previous value of second EMA.
      prev_ema3: Previous value of third EMA.
      period: Smoothing period for calculations (must be >= 2).

    Returns:
      A tuple containing:
      - Current TEMA value
      - Updated first EMA
      - Updated second EMA
      - Updated third EMA

    Examples:
      ```python
      >>> import kand
      >>> tema, ema1, ema2, ema3 = kand.tema_inc(
      ...     10.0,  # new_price
      ...     9.0,   # prev_ema1
      ...     8.0,   # prev_ema2
      ...     7.0,   # prev_ema3
      ...     3      # period
      ... )
      ```
    """
    ...

def trange(high, low, close):
    """
    Computes the True Range (TR) over NumPy arrays.

    True Range measures the market's volatility by considering the current high-low range
    and the previous close price.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A new 1-D NumPy array containing the TR values. The array has the same length as the input,
      with the first element containing NaN value.
    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0])
      >>> low = np.array([8.0, 9.0, 11.0])
      >>> close = np.array([9.0, 11.0, 14.0])
      >>> result = kand.trange(high, low, close)
      >>> print(result)
      [nan, 3.0, 4.0]
      ```
    """
    ...

def trange_inc(high, low, prev_close):
    """
    Calculates a single True Range value for the most recent period.

    Args:
      high: Current period's high price.
      low: Current period's low price.
      prev_close: Previous period's closing price.

    Returns:
      The calculated True Range value.

    Examples:
      ```python
      >>> import kand
      >>> tr = kand.trange_inc(12.0, 9.0, 11.0)
      >>> print(tr)
      3.0  # max(3, 1, 2)
      ```
    """
    ...

def trima(prices, period):
    """
    Calculate Triangular Moving Average (TRIMA) for a NumPy array.

    TRIMA is a double-smoothed moving average that places more weight on the middle portion of the price series
    and less weight on the first and last portions. This results in a smoother moving average compared to a
    Simple Moving Average (SMA).

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Smoothing period for calculations (must be >= 2).

    Returns:
      A tuple of 2 1-D NumPy arrays containing:
      - First SMA values
      - Final TRIMA values
      The first (period - 1) elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> sma1, trima = kand.trima(prices, 3)
      ```
    """
    ...

def trima_inc(prev_sma1, prev_sma2, new_price, old_price, old_sma1, period):
    """
    Calculate the next TRIMA value incrementally.

    Args:
      prev_sma1: Previous first SMA value.
      prev_sma2: Previous TRIMA value.
      new_price: Latest price to include in calculation.
      old_price: Price dropping out of first window.
      old_sma1: SMA1 value dropping out of second window.
      period: Smoothing period for calculations (must be >= 2).

    Returns:
      A tuple containing:
      - Updated first SMA value
      - Updated TRIMA value

    Examples:
      ```python
      >>> import kand
      >>> trima, sma1 = kand.trima_inc(
      ...     35.5,  # prev_sma1
      ...     35.2,  # prev_sma2
      ...     36.0,  # new_price
      ...     35.0,  # old_price
      ...     35.1,  # old_sma1
      ...     5      # period
      ... )
      ```
    """
    ...

def trix(prices, period):
    """
    Calculates the Triple Exponential Moving Average Oscillator (TRIX) over a NumPy array.

    TRIX is a momentum oscillator that measures the rate of change of a triple exponentially smoothed moving average.
    It helps identify oversold and overbought conditions and potential trend reversals through divergences.

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for EMA calculations (must be >= 2).

    Returns:
      A tuple of 4 1-D NumPy arrays containing:
      - TRIX values
      - First EMA values
      - Second EMA values
      - Third EMA values
      The first lookback elements of each array contain NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
      >>> trix, ema1, ema2, ema3 = kand.trix(prices, 2)
      ```
    """
    ...

def trix_inc(price, prev_ema1, prev_ema2, prev_ema3, period):
    """
    Calculates a single new TRIX value incrementally.

    Args:
      price: Current price value.
      prev_ema1: Previous first EMA value.
      prev_ema2: Previous second EMA value.
      prev_ema3: Previous third EMA value.
      period: Period for EMA calculations (must be >= 2).

    Returns:
      A tuple containing:
      - TRIX value
      - Updated first EMA value
      - Updated second EMA value
      - Updated third EMA value

    Examples:
      ```python
      >>> import kand
      >>> trix, ema1, ema2, ema3 = kand.trix_inc(
      ...     100.0,  # price
      ...     98.0,   # prev_ema1
      ...     97.0,   # prev_ema2
      ...     96.0,   # prev_ema3
      ...     14      # period
      ... )
      ```
    """
    ...

def typprice(high, low, close):
    """
    Computes the Typical Price over NumPy arrays.

    The Typical Price is calculated by taking the arithmetic mean of the high, low and close prices
    for each period.

    Args:
      high: Input high prices as a 1-D NumPy array of type `TAFloat`.
      low: Input low prices as a 1-D NumPy array of type `TAFloat`.
      close: Input close prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A new 1-D NumPy array containing the Typical Price values. The array has the same length as the inputs.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([24.20, 24.07, 24.04])
      >>> low = np.array([23.85, 23.72, 23.64])
      >>> close = np.array([23.89, 23.95, 23.67])
      >>> result = kand.typprice(high, low, close)
      >>> print(result)
      [23.98, 23.91, 23.78]
      ```
    """
    ...

def typprice_inc(high, low, close):
    """
    Calculates a single Typical Price value incrementally.

    Args:
      high: Current period's high price.
      low: Current period's low price.
      close: Current period's close price.

    Returns:
      The calculated Typical Price value.

    Examples:
      ```python
      >>> import kand
      >>> typ_price = kand.typprice_inc(24.20, 23.85, 23.89)
      >>> print(typ_price)
      23.98  # (24.20 + 23.85 + 23.89) / 3
      ```
    """
    ...

def var(prices, period):
    """
    Calculate Variance (VAR) for a NumPy array

    Variance measures the average squared deviation of data points from their mean over a specified period.

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.
      period: Period for Variance calculation (must be >= 2).

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - Variance values
      - Running sum values
      - Running sum of squares values
      Each array has the same length as the input, with the first (period-1) elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([2.0, 4.0, 6.0, 8.0, 10.0])
      >>> var, sum, sum_sq = kand.var(prices, 3)
      ```
    """
    ...

def var_inc(price, prev_sum, prev_sum_sq, old_price, period):
    """
    Calculate the latest Variance value incrementally

    Args:
      py: Python interpreter token
      price: Current period's price
      prev_sum: Previous period's sum
      prev_sum_sq: Previous period's sum of squares
      old_price: Price being removed from the period
      period: Period for Variance calculation (must be >= 2)

    Returns:
      A tuple containing:
      - Latest Variance value
      - New sum
      - New sum of squares

    Examples:
      ```python
      >>> import kand
      >>> var, sum, sum_sq = kand.var_inc(
      ...     10.0,  # current price
      ...     25.0,  # previous sum
      ...     220.0, # previous sum of squares
      ...     5.0,   # price to remove
      ...     3      # period
      ... )
      ```
    """
    ...

def vegas(prices):
    """
    Computes the VEGAS (Volume and EMA Guided Adaptive Scaling) indicator over NumPy arrays.

    VEGAS is a trend following indicator that uses multiple EMAs to define channels and boundaries.

    Args:
      prices: Input prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A tuple of four 1-D NumPy arrays containing:
      - Channel Upper (EMA 144)
      - Channel Lower (EMA 169)
      - Boundary Upper (EMA 576)
      - Boundary Lower (EMA 676)
      Each array has the same length as the input, with the first 675 elements containing NaN values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> prices = np.array([44.34, 44.09, 44.15, 43.61, 44.33])
      >>> ch_upper, ch_lower, b_upper, b_lower = kand.vegas(prices)
      ```
    """
    ...

def vegas_inc(price, prev_channel_upper, prev_channel_lower, prev_boundary_upper, prev_boundary_lower):
    """
    Incrementally calculates the next VEGAS values.

    Args:
      price: Current price value.
      prev_channel_upper: Previous EMA(144) value.
      prev_channel_lower: Previous EMA(169) value.
      prev_boundary_upper: Previous EMA(576) value.
      prev_boundary_lower: Previous EMA(676) value.

    Returns:
      A tuple containing:
      - Updated Channel Upper value
      - Updated Channel Lower value
      - Updated Boundary Upper value
      - Updated Boundary Lower value

    Examples:
      ```python
      >>> import kand
      >>> price = 100.0
      >>> prev_values = (98.0, 97.5, 96.0, 95.5)
      >>> ch_upper, ch_lower, b_upper, b_lower = kand.vegas_inc(
      ...     price,
      ...     prev_values[0],
      ...     prev_values[1],
      ...     prev_values[2],
      ...     prev_values[3]
      ... )
      ```
    """
    ...

def vwap(high, low, close, volume):
    """
    Calculates Volume Weighted Average Price (VWAP) for a series of price data.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.
      volume: Volume data as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A tuple of three 1-D NumPy arrays containing:
      - VWAP values
      - Cumulative price-volume products
      - Cumulative volumes

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0])
      >>> low = np.array([8.0, 9.0, 11.0])
      >>> close = np.array([9.0, 10.0, 12.0])
      >>> volume = np.array([100.0, 150.0, 200.0])
      >>> vwap, cum_pv, cum_vol = kand.vwap(high, low, close, volume)
      ```
    """
    ...

def vwap_inc(high, low, close, volume, prev_cum_pv, prev_cum_vol):
    """
    Calculates a single VWAP value from the latest price and volume data.

    Args:
      high: Latest high price value as `TAFloat`.
      low: Latest low price value as `TAFloat`.
      close: Latest close price value as `TAFloat`.
      volume: Latest volume value as `TAFloat`.
      prev_cum_pv: Previous cumulative price-volume product as `TAFloat`.
      prev_cum_vol: Previous cumulative volume as `TAFloat`.

    Returns:
      A tuple containing (new cumulative PV, new cumulative volume, new VWAP).

    Examples:
      ```python
      >>> import kand
      >>> new_cum_pv, new_cum_vol, vwap = kand.vwap_inc(15.0, 11.0, 14.0, 200.0, 1000.0, 150.0)
      ```
    """
    ...

def wclprice(high, low, close):
    """
    Calculates the Weighted Close Price (WCLPRICE) for a series of price data.

    The Weighted Close Price is a price indicator that assigns more weight to the closing price
    compared to high and low prices. It provides a single value that reflects price action
    with emphasis on the closing price.

    Args:
      high: High prices as a 1-D NumPy array of type `TAFloat`.
      low: Low prices as a 1-D NumPy array of type `TAFloat`.
      close: Close prices as a 1-D NumPy array of type `TAFloat`.

    Returns:
      A 1-D NumPy array containing the WCLPRICE values.

    Examples:
      ```python
      >>> import numpy as np
      >>> import kand
      >>> high = np.array([10.0, 12.0, 15.0])
      >>> low = np.array([8.0, 9.0, 11.0])
      >>> close = np.array([9.0, 11.0, 14.0])
      >>> wclprice = kand.wclprice(high, low, close)
      ```
    """
    ...

def wclprice_inc(high, low, close):
    """
    Calculates a single Weighted Close Price (WCLPRICE) value from the latest price data.

    Args:
      high: Latest high price value as `TAFloat`.
      low: Latest low price value as `TAFloat`.
      close: Latest close price value as `TAFloat`.

    Returns:
      The calculated WCLPRICE value.

    Examples:
      ```python
      >>> import kand
      >>> wclprice = kand.wclprice_inc(15.0, 11.0, 14.0)
      ```
    """
    ...

def willr(high, low, close, period):
    """
    Calculates Williams %R (Williams Percent Range) for a series of prices.

    Williams %R is a momentum indicator that measures overbought and oversold levels by comparing
    the closing price to the high-low range over a specified period. The indicator oscillates
    between 0 and -100.

    Args:
        high: Input high prices as a 1-D NumPy array of type `TAFloat`.
        low: Input low prices as a 1-D NumPy array of type `TAFloat`.
        close: Input closing prices as a 1-D NumPy array of type `TAFloat`.
        period: Lookback period for calculations. Must be >= 2.

    Returns:
        A tuple of three 1-D NumPy arrays containing:
        - Williams %R values
        - Highest high values for each period
        - Lowest low values for each period
        Each array has the same length as the input, with the first `period-1` elements containing NaN values.

    Examples:
        ```python
        >>> import numpy as np
        >>> import kand
        >>> high = np.array([10.0, 12.0, 15.0, 14.0, 13.0])
        >>> low = np.array([8.0, 9.0, 11.0, 10.0, 9.0])
        >>> close = np.array([9.0, 11.0, 14.0, 12.0, 11.0])
        >>> willr, highest, lowest = kand.willr(high, low, close, 3)
        ```
    """
    ...

def willr_inc(prev_highest_high, prev_lowest_low, prev_high, prev_low, close, high, low):
    """
    Incrementally calculates Williams %R for the latest data point.

    This function provides an optimized way to calculate the latest Williams %R value
    by using previously calculated highest high and lowest low values.

    Args:
        prev_highest_high: Previous period's highest high value.
        prev_lowest_low: Previous period's lowest low value.
        prev_high: Previous period's high price.
        prev_low: Previous period's low price.
        close: Current period's closing price.
        high: Current period's high price.
        low: Current period's low price.

    Returns:
        A tuple containing:
        - Current Williams %R value
        - New highest high
        - New lowest low

    Examples:
        ```python
        >>> import kand
        >>> willr, high, low = kand.willr_inc(15.0, 10.0, 14.0, 11.0, 12.0, 13.0, 11.0)
        ```
    """
    ...

def wma(data, period):
    """
    Computes the Weighted Moving Average (WMA) over a NumPy array.

    The Weighted Moving Average assigns linearly decreasing weights to each price in the period,
    giving more importance to recent prices and less to older ones.

    Args:
        data: Input data as a 1-D NumPy array of type `TAFloat`.
        period: Window size for WMA calculation. Must be >= 2.

    Returns:
        A new 1-D NumPy array containing the WMA values. The array has the same length as the input,
        with the first `period-1` elements containing NaN values.

    Examples:
        ```python
        >>> import numpy as np
        >>> import kand
        >>> data = np.array([1.0, 2.0, 3.0, 4.0, 5.0])
        >>> result = kand.wma(data, 3)
        >>> print(result)
        [nan, nan, 2.0, 3.0, 4.0]
        ```
    """
    ...

def wma_inc(input_window, period):
    """
    Incrementally calculates the next WMA value.

    This function provides an optimized way to calculate the latest WMA value
    by using a window of the most recent prices.

    Args:
        input_window: Array of price values ordered from newest to oldest.
        period: The time period for WMA calculation (must be >= 2).

    Returns:
        The next WMA value.

    Examples:
        ```python
        >>> import kand
        >>> window = [5.0, 4.0, 3.0]  # newest to oldest
        >>> wma = kand.wma_inc(window, 3)
        >>> print(wma)
        4.333333333333333
        ```
    """
    ...
