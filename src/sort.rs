use std::cmp::Ordering;

use crate::entry::{Entry, Group, RangeOp};

pub fn sort_entries(entries: &mut [Entry], letter_ordering: bool, german_sort: bool) {
    entries.sort_by(|a, b| compare_entries(a, b, letter_ordering, german_sort));
}

fn compare_entries(a: &Entry, b: &Entry, letter_ordering: bool, german_sort: bool) -> Ordering {
    match group_rank(&a.group).cmp(&group_rank(&b.group)) {
        Ordering::Equal => {}
        other => return other,
    }

    if let (Group::Numeric(av), Group::Numeric(bv)) = (&a.group, &b.group) {
        return av.cmp(bv);
    }

    for idx in 0..a.sort_keys.len() {
        let sort_cmp = compare_field(
            &a.sort_keys[idx],
            &b.sort_keys[idx],
            letter_ordering,
            german_sort,
        );
        if sort_cmp != Ordering::Equal {
            return sort_cmp;
        }
        let actual_cmp = compare_field(
            &a.actual_keys[idx],
            &b.actual_keys[idx],
            letter_ordering,
            german_sort,
        );
        if actual_cmp != Ordering::Equal {
            return actual_cmp;
        }
    }

    compare_pages(a, b)
}

fn compare_field(left: &str, right: &str, letter_ordering: bool, german_sort: bool) -> Ordering {
    if left.is_empty() && right.is_empty() {
        return Ordering::Equal;
    }
    if left.is_empty() {
        return Ordering::Less;
    }
    if right.is_empty() {
        return Ordering::Greater;
    }

    let left_kind = classify_field(left);
    let right_kind = classify_field(right);

    match (left_kind, right_kind) {
        (FieldCategory::Numeric(lv), FieldCategory::Numeric(rv)) => lv.cmp(&rv),
        (FieldCategory::Numeric(_), _) => {
            if german_sort {
                Ordering::Greater
            } else if matches!(right_kind, FieldCategory::Alpha) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        (_, FieldCategory::Numeric(_)) => {
            if german_sort {
                Ordering::Less
            } else if matches!(left_kind, FieldCategory::Alpha) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
        (FieldCategory::Symbol, FieldCategory::Symbol) => compare_symbol_strings(left, right),
        (FieldCategory::Symbol, _) => Ordering::Less,
        (_, FieldCategory::Symbol) => Ordering::Greater,
        (FieldCategory::Alpha, FieldCategory::Alpha) => {
            compare_alpha_strings(left, right, letter_ordering, german_sort)
        }
        _ => Ordering::Equal,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FieldCategory {
    Empty,
    Numeric(i32),
    Symbol,
    Alpha,
}

fn classify_field(text: &str) -> FieldCategory {
    if text.is_empty() {
        FieldCategory::Empty
    } else if text.chars().all(|c| c.is_ascii_digit()) {
        let value = text.parse().unwrap_or(0);
        FieldCategory::Numeric(value)
    } else if text.chars().next().is_some_and(is_symbol) {
        FieldCategory::Symbol
    } else {
        FieldCategory::Alpha
    }
}

fn is_symbol(ch: char) -> bool {
    matches!(ch, '!'..='@' | '['..='`' | '{'..='~')
}

fn compare_symbol_strings(left: &str, right: &str) -> Ordering {
    let left_is_digit = left.chars().next().is_some_and(|c| c.is_ascii_digit());
    let right_is_digit = right.chars().next().is_some_and(|c| c.is_ascii_digit());
    match (left_is_digit, right_is_digit) {
        (true, false) => Ordering::Greater,
        (false, true) => Ordering::Less,
        _ => left.cmp(right),
    }
}

fn compare_alpha_strings(
    left: &str,
    right: &str,
    letter_ordering: bool,
    german_sort: bool,
) -> Ordering {
    let mut iter_left = left.chars();
    let mut iter_right = right.chars();

    loop {
        let next_left = next_alpha_char(&mut iter_left, letter_ordering);
        let next_right = next_alpha_char(&mut iter_right, letter_ordering);
        match (next_left, next_right) {
            (None, None) => break,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(cl), Some(cr)) => {
                let al = cl.to_ascii_lowercase();
                let bl = cr.to_ascii_lowercase();
                if al != bl {
                    return al.cmp(&bl);
                }
            }
        }
    }

    if german_sort {
        compare_case_sensitive_german(left, right)
    } else {
        left.cmp(right)
    }
}

fn compare_case_sensitive_german(left: &str, right: &str) -> Ordering {
    let mut left_chars = left.chars();
    let mut right_chars = right.chars();
    loop {
        match (left_chars.next(), right_chars.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(lc), Some(rc)) => {
                if lc == rc {
                    continue;
                }
                let left_upper = lc.is_ascii_uppercase();
                let right_upper = rc.is_ascii_uppercase();
                return match (left_upper, right_upper) {
                    (true, false) => Ordering::Greater,
                    (false, true) => Ordering::Less,
                    _ => lc.cmp(&rc),
                };
            }
        }
    }
}

fn next_alpha_char(iter: &mut std::str::Chars<'_>, letter_ordering: bool) -> Option<char> {
    for ch in iter.by_ref() {
        if letter_ordering && ch == ' ' {
            continue;
        }
        return Some(ch);
    }
    None
}

fn group_rank(group: &Group) -> i32 {
    match group {
        Group::Symbol => 0,
        Group::Numeric(_) => 1,
        Group::Alpha => 2,
    }
}

fn compare_pages(a: &Entry, b: &Entry) -> Ordering {
    match compare_page_numbers(a, b) {
        Ordering::Equal => compare_page_encap(a, b),
        other => other,
    }
}

fn compare_page_numbers(a: &Entry, b: &Entry) -> Ordering {
    let a_vals = &a.page_fields.values;
    let b_vals = &b.page_fields.values;
    let mut idx = 0usize;
    while idx < a_vals.len() && idx < b_vals.len() {
        match a_vals[idx].cmp(&b_vals[idx]) {
            Ordering::Equal => idx += 1,
            other => return other,
        }
    }
    a_vals.len().cmp(&b_vals.len())
}

fn compare_page_encap(a: &Entry, b: &Entry) -> Ordering {
    let a_range = matches!(a.range_op, RangeOp::Open | RangeOp::Close);
    let b_range = matches!(b.range_op, RangeOp::Open | RangeOp::Close);
    if a_range && b_range {
        return a.line.cmp(&b.line);
    }
    if a.encap == b.encap {
        return Ordering::Equal;
    }
    if a_range || b_range {
        return a.line.cmp(&b.line);
    }
    a.encap.cmp(&b.encap)
}
