//! A public scratch repository for kelpie's live checks, archived once they are done.

/// Clamps `value` to the `0..=100` percentage range.
///
/// Values below `0` become `0`, values above `100` become `100`, and values
/// already within range pass through unchanged.
pub fn clamp_percent(value: i64) -> u8 {
    value.clamp(0, 100) as u8
}

/// Adds two percentages, saturating at `100` when the sum would exceed it.
pub fn add_percent(a: u8, b: u8) -> u8 {
    clamp_percent(i64::from(a) + i64::from(b))
}

/// Returns what percentage `part` is of `whole`, rounded down and passed
/// through [`clamp_percent`].
///
/// Returns `None` when `whole` is `0`.
pub fn percent_of(part: u32, whole: u32) -> Option<u8> {
    if whole == 0 {
        return None;
    }

    let percent = u64::from(part) * 100 / u64::from(whole);
    Some(clamp_percent(percent as i64))
}

/// Renders `p` as a percentage string, like `"42%"`.
///
/// Values over `100` are passed through [`clamp_percent`] first.
pub fn format_percent(p: u8) -> String {
    format!("{}%", clamp_percent(i64::from(p)))
}

/// Draws `p` as a text bar of `width` characters, like `"[####------]"`.
///
/// Values over `100` are passed through [`clamp_percent`] first. The filled
/// portion is rounded down to the nearest whole character.
pub fn percent_bar(p: u8, width: usize) -> String {
    let percent = clamp_percent(i64::from(p));
    let filled = width * usize::from(percent) / 100;
    let empty = width - filled;

    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
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

    #[test]
    fn add_percent_below_hundred_sums_exactly() {
        assert_eq!(add_percent(30, 40), 70);
    }

    #[test]
    fn add_percent_above_hundred_saturates() {
        assert_eq!(add_percent(70, 50), 100);
    }

    #[test]
    fn percent_of_zero_part_is_zero() {
        assert_eq!(percent_of(0, 50), Some(0));
    }

    #[test]
    fn percent_of_partial_value_rounds_down() {
        assert_eq!(percent_of(1, 3), Some(33));
    }

    #[test]
    fn percent_of_exact_whole_is_hundred() {
        assert_eq!(percent_of(50, 50), Some(100));
    }

    #[test]
    fn percent_of_part_larger_than_whole_clamps_to_hundred() {
        assert_eq!(percent_of(150, 50), Some(100));
    }

    #[test]
    fn percent_of_zero_whole_is_none() {
        assert_eq!(percent_of(1, 0), None);
    }

    #[test]
    fn format_percent_zero() {
        assert_eq!(format_percent(0), "0%");
    }

    #[test]
    fn format_percent_mid_range() {
        assert_eq!(format_percent(42), "42%");
    }

    #[test]
    fn format_percent_hundred() {
        assert_eq!(format_percent(100), "100%");
    }

    #[test]
    fn format_percent_above_hundred_clamps() {
        assert_eq!(format_percent(250), "100%");
    }

    #[test]
    fn percent_bar_zero_is_empty() {
        assert_eq!(percent_bar(0, 10), "[----------]");
    }

    #[test]
    fn percent_bar_fifty_is_half_filled() {
        assert_eq!(percent_bar(50, 10), "[#####-----]");
    }

    #[test]
    fn percent_bar_hundred_is_full() {
        assert_eq!(percent_bar(100, 10), "[##########]");
    }
}
