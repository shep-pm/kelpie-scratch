//! A public scratch repository for kelpie's live checks, archived once they are done.

/// Clamps `value` to the `0..=100` percentage range.
///
/// Values below `0` become `0`, values above `100` become `100`, and values
/// already within range pass through unchanged.
pub fn clamp_percent(value: i64) -> u8 {
    value.clamp(0, 100) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_range_clamps_to_zero() {
        assert_eq!(clamp_percent(-1), 0);
    }

    #[test]
    fn zero_passes_through() {
        assert_eq!(clamp_percent(0), 0);
    }

    #[test]
    fn hundred_passes_through() {
        assert_eq!(clamp_percent(100), 100);
    }

    #[test]
    fn above_range_clamps_to_hundred() {
        assert_eq!(clamp_percent(101), 100);
    }
}
