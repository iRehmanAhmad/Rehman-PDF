//! Page-range parsing ("1, 12, 16-18") for the control channel. The Organize Pages toolbar's
//! editable range field was replaced by the T-003 mockup's Select-all action bar; upstream
//! v0.9.0's typed-field widget (`PageSelectionInput`) and its organize panel were removed in
//! T-010 (PM decision: the T-003 canvas action bar is the shipped Organize). `parse` stays
//! for `--control` page selection.
use std::collections::BTreeSet;

use pdfcraft_engine::print::PrintError;

/// Blank clears the explicit selection. Repeated/reversed ranges become a sorted set.
/// Bound both the input and expansion work, including repeated overlapping ranges.
pub(crate) fn parse(text: &str, count: usize) -> Result<Vec<usize>, PrintError> {
    if text.len() > 16_384 {
        return Err(PrintError::NotAPage(text.chars().take(80).collect()));
    }
    if text.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut pages = BTreeSet::new();
    let mut budget = 1_000_000usize;
    let number = |token: &str| {
        let n = token.trim().parse::<usize>().map_err(|_| PrintError::NotAPage(token.into()))?;
        if n == 0 || n > count { Err(PrintError::OutOfRange(n, count)) } else { Ok(n - 1) }
    };
    for part in text.split([',', ';']).map(str::trim) {
        let (a, b) = if let Some((a, b)) = part.split_once('-') {
            (number(a)?, number(b)?)
        } else {
            let p = number(part)?;
            (p, p)
        };
        let (a, b) = (a.min(b), a.max(b));
        let size = b.saturating_sub(a).saturating_add(1);
        budget = budget.checked_sub(size).ok_or_else(|| PrintError::NotAPage(part.into()))?;
        pages.extend(a..=b);
    }
    Ok(pages.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_ranges_are_sorted_unique_and_bounded() {
        let pages = parse("1,12,16-18,89,92,93,97,99", 100).unwrap();
        assert_eq!(pages, [0, 11, 15, 16, 17, 88, 91, 92, 96, 98]);
        assert_eq!(parse("3-1,2", 3).unwrap(), [0, 1, 2]);
        assert!(parse("  ", 0).unwrap().is_empty());
        for text in ["0", "101", "1,", "x", "1--3", "18446744073709551616", "1-100000000"] {
            assert!(parse(text, 100).is_err(), "{text}");
        }
        assert!(parse(&"1,".repeat(10_000), 100).is_err());
        assert!(parse("1-1000000,1-1000000", 1_000_000).is_err());
    }
}
