use anyhow::{Result, anyhow};

use crate::style::Style;

pub const PAGEFIELD_MAX: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageType {
    Empty,
    RomanLower,
    RomanUpper,
    Arabic,
    AlphaLower,
    AlphaUpper,
}

impl PageType {
    pub fn precedence_index(self) -> usize {
        match self {
            PageType::RomanLower => 0,
            PageType::RomanUpper => 1,
            PageType::Arabic => 2,
            PageType::AlphaLower => 3,
            PageType::AlphaUpper => 4,
            PageType::Empty => usize::MAX,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PageFields {
    pub values: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct PageNumber {
    pub literal: String,
    pub fields: PageFields,
    pub kind: PageType,
}

pub struct PageParser<'a> {
    style: &'a Style,
}

impl<'a> PageParser<'a> {
    pub fn new(style: &'a Style) -> Self {
        Self { style }
    }

    pub fn parse(&self, literal: &str) -> Result<PageNumber> {
        let trimmed = literal.trim();
        if trimmed.is_empty() {
            return Err(anyhow!("Empty page number"));
        }
        let mut collector = PageFields { values: Vec::new() };
        let mut remaining = trimmed;
        let mut first_type = PageType::Empty;

        while !remaining.is_empty() {
            let (fragment, rest) = split_fragment(remaining, &self.style.page_compositor);
            let fragment = fragment.trim();
            if fragment.is_empty() {
                return Err(anyhow!("Malformed composite page '{}'", literal));
            }
            let (value, kind) = parse_component(fragment)?;
            let offset = self.style.page_offset(kind);
            collector.values.push(value + offset);
            if collector.values.len() > PAGEFIELD_MAX {
                return Err(anyhow!(
                    "Page number '{}' has too many fields (max {}).",
                    literal,
                    PAGEFIELD_MAX
                ));
            }
            if matches!(first_type, PageType::Empty) {
                first_type = kind;
            }
            remaining = rest;
        }

        Ok(PageNumber {
            literal: literal.to_string(),
            fields: collector,
            kind: first_type,
        })
    }
}

fn split_fragment<'a>(text: &'a str, compositor: &str) -> (&'a str, &'a str) {
    if compositor.is_empty() {
        return (text, "");
    }
    if let Some(idx) = text.find(compositor) {
        let (head, tail) = text.split_at(idx);
        let rest = &tail[compositor.len()..];
        (head, rest)
    } else {
        (text, "")
    }
}

fn parse_component(fragment: &str) -> Result<(i32, PageType)> {
    if fragment.chars().all(|c| c.is_ascii_digit()) {
        let value: i32 = fragment.parse()?;
        return Ok((value, PageType::Arabic));
    }
    if fragment.chars().all(|c| {
        matches!(
            c,
            'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm' | 'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'
        )
    }) {
        if fragment.chars().next().unwrap().is_lowercase() {
            let val = roman_to_int(fragment)?;
            return Ok((val, PageType::RomanLower));
        } else {
            let val = roman_to_int(fragment)?;
            return Ok((val, PageType::RomanUpper));
        }
    }
    let mut chars = fragment.chars();
    if let Some(ch) = chars.next() {
        if ch.is_ascii_lowercase() && chars.as_str().is_empty() {
            return Ok(((ch as u8 - b'a') as i32, PageType::AlphaLower));
        }
        if ch.is_ascii_uppercase() && chars.as_str().is_empty() {
            return Ok(((ch as u8 - b'A') as i32, PageType::AlphaUpper));
        }
    }
    Err(anyhow!("Illegal page number '{}'", fragment))
}

fn roman_to_int(text: &str) -> Result<i32> {
    let mut total = 0;
    let mut prev = 0;
    for ch in text.chars() {
        let val = roman_value(ch).ok_or_else(|| anyhow!("Illegal Roman digit '{}'", ch))?;
        if val > prev {
            total = total - prev + (val - prev);
        } else {
            total += val;
        }
        prev = val;
    }
    Ok(total)
}

fn roman_value(ch: char) -> Option<i32> {
    match ch {
        'i' | 'I' => Some(1),
        'v' | 'V' => Some(5),
        'x' | 'X' => Some(10),
        'l' | 'L' => Some(50),
        'c' | 'C' => Some(100),
        'd' | 'D' => Some(500),
        'm' | 'M' => Some(1000),
        _ => None,
    }
}
