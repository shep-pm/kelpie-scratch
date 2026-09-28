//! A public scratch repository for kelpie's live checks, archived once they are done.

/// Clamps a percentage to the range `0..=100`.
///
/// Values below 0 become 0, values above 100 become 100, and values in
/// range pass through unchanged.
///
/// ```
/// assert_eq!(kelpie_scratch::clamp_percent(-5), 0);
/// assert_eq!(kelpie_scratch::clamp_percent(42), 42);
/// assert_eq!(kelpie_scratch::clamp_percent(250), 100);
/// ```
#[must_use]
pub fn clamp_percent(value: i64) -> u8 {
    // The clamp bounds the value to 0..=100, so the conversion always fits.
    u8::try_from(value.clamp(0, 100)).unwrap_or(100)
}

#[cfg(test)]
mod tests {
    use super::clamp_percent;

    #[test]
    fn below_zero_becomes_zero() {
        assert_eq!(clamp_percent(-1), 0);
        assert_eq!(clamp_percent(i64::MIN), 0);
    }

    #[test]
    fn zero_passes_through() {
        assert_eq!(clamp_percent(0), 0);
    }

    #[test]
    fn in_range_passes_through() {
        assert_eq!(clamp_percent(50), 50);
    }

    #[test]
    fn hundred_passes_through() {
        assert_eq!(clamp_percent(100), 100);
    }

    #[test]
    fn above_hundred_becomes_hundred() {
        assert_eq!(clamp_percent(101), 100);
        assert_eq!(clamp_percent(i64::MAX), 100);
    }
}
