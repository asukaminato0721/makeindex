use anyhow::{Result, anyhow};

use crate::{
    entry::{Entry, FIELD_MAX, Group, RangeOp},
    page::PageParser,
    style::Style,
};

pub struct ParseConfig {
    pub compress_blanks: bool,
    pub german_sort: bool,
}

const ARGUMENT_MAX: usize = 1024;

pub struct IndexParser<'a> {
    style: &'a Style,
    config: ParseConfig,
    page: PageParser<'a>,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub file: String,
    pub line: usize,
    pub message: String,
}

impl ParseError {
    fn new(file: &str, line: usize, err: anyhow::Error) -> Self {
        Self {
            file: file.to_string(),
            line,
            message: format!("{err:#}"),
        }
    }
}

pub struct ParseResults {
    pub entries: Vec<Entry>,
    pub accepted: usize,
    pub rejected: usize,
}

impl<'a> IndexParser<'a> {
    pub fn new(style: &'a Style, config: ParseConfig) -> Self {
        Self {
            style,
            config,
            page: PageParser::new(style),
        }
    }

    pub fn parse_str<F>(&self, name: &str, contents: &str, mut on_error: F) -> ParseResults
    where
        F: FnMut(ParseError),
    {
        let mut entries = Vec::new();
        let mut rejected = 0usize;
        let keyword = self.style.idx_keyword.as_str();
        let line_offsets = compute_line_offsets(contents);
        let mut cursor = 0usize;

        while cursor < contents.len() {
            let search = &contents[cursor..];
            let Some(rel) = search.find(keyword) else {
                break;
            };
            let start = cursor + rel;
            cursor = start + keyword.len();
            let line_no = line_number_at(start, &line_offsets);

            let (rest, consumed_ws) = trim_leading_ws(&contents[cursor..]);
            cursor += consumed_ws;
            let (arg1, after_first) = match extract_argument(
                rest,
                self.style.idx_aopen,
                self.style.idx_aclose,
                self.style.idx_escape,
                Some(self.style.idx_quote),
                self.config.compress_blanks,
            ) {
                Ok(v) => v,
                Err(err) => {
                    rejected += 1;
                    let wrapped = anyhow!("invalid first argument: {err:#}");
                    on_error(ParseError::new(name, line_no, wrapped));
                    continue;
                }
            };
            cursor += rest.len() - after_first.len();
            if arg1.len() >= ARGUMENT_MAX {
                rejected += 1;
                on_error(ParseError::new(
                    name,
                    line_no,
                    anyhow!("First argument too long (max {}).", ARGUMENT_MAX),
                ));
                continue;
            }
            let (rest2, consumed_ws2) = trim_leading_ws(after_first);
            cursor += consumed_ws2;
            let (arg2, remainder) = match extract_argument(
                rest2,
                self.style.idx_aopen,
                self.style.idx_aclose,
                self.style.idx_escape,
                None,
                false,
            ) {
                Ok(v) => v,
                Err(err) => {
                    rejected += 1;
                    let wrapped = anyhow!("invalid second argument: {err:#}");
                    on_error(ParseError::new(name, line_no, wrapped));
                    continue;
                }
            };
            cursor += rest2.len() - remainder.len();

            let key = match self.parse_key(&arg1) {
                Ok(v) => v,
                Err(err) => {
                    rejected += 1;
                    on_error(ParseError::new(name, line_no, err));
                    continue;
                }
            };
            let page = match self.page.parse(&arg2) {
                Ok(v) => v,
                Err(err) => {
                    rejected += 1;
                    let wrapped = anyhow!("invalid page {arg2}: {err:#}");
                    on_error(ParseError::new(name, line_no, wrapped));
                    continue;
                }
            };

            let mut entry = Entry::new(name.to_string(), line_no);
            for (i, level) in key.levels.iter().enumerate() {
                entry.sort_keys[i] = level.sort.clone();
                entry.actual_keys[i] = level.actual.clone();
            }
            entry.encap = key.encap.clone();
            entry.range_op = key.range_op;
            entry.literal_page = page.literal;
            entry.page_fields = page.fields;
            entry.page_type = page.kind;
            entry.group = determine_group(entry.sort_keys[0].as_str());

            entries.push(entry);
        }

        ParseResults {
            accepted: entries.len(),
            rejected,
            entries,
        }
    }

    fn parse_key(&self, raw: &str) -> Result<KeyData> {
        let KeySplit {
            levels,
            encap,
            range_op,
        } = parse_key_levels(
            raw,
            self.style,
            self.config.compress_blanks,
            self.config.german_sort,
        )?;

        Ok(KeyData {
            levels,
            encap,
            range_op,
        })
    }
}

struct KeyData {
    levels: Vec<Level>,
    encap: String,
    range_op: RangeOp,
}

struct Level {
    sort: String,
    actual: String,
}

struct KeySplit {
    levels: Vec<Level>,
    encap: String,
    range_op: RangeOp,
}

fn parse_key_levels(
    raw: &str,
    style: &Style,
    compress_blanks: bool,
    german_sort: bool,
) -> Result<KeySplit> {
    if raw.is_empty() {
        return Err(anyhow!("Empty index key"));
    }
    let chars: Vec<char> = raw.chars().collect();
    let mut idx = 0usize;
    let len = chars.len();
    let mut levels: Vec<Level> = Vec::new();
    let mut current_level: Option<usize> = None;
    let mut need_level_delim = false;
    let mut encap = String::new();

    while idx < len {
        let ch = chars[idx];
        if ch == style.idx_encap {
            idx += 1;
            encap = parse_field(
                &chars,
                &mut idx,
                style,
                false,
                false,
                false,
                compress_blanks,
            )?;
            break;
        } else if ch == style.idx_actual {
            idx += 1;
            let level_index = current_level
                .ok_or_else(|| anyhow!("Actual text specified before primary index key"))?;
            let allow_level = level_index < FIELD_MAX - 1;
            let actual = parse_field(
                &chars,
                &mut idx,
                style,
                allow_level,
                true,
                false,
                compress_blanks,
            )?;
            levels[level_index].actual = actual;
        } else {
            if need_level_delim {
                if ch != style.idx_level {
                    return Err(anyhow!(format!(
                        "Missing '{}' separator before subentry.",
                        style.idx_level
                    )));
                }
                idx += 1;
            }
            if levels.len() >= FIELD_MAX {
                return Err(anyhow!("Too many nesting levels (max {}).", FIELD_MAX));
            }
            let allow_level = levels.len() < FIELD_MAX - 1;
            let mut sort = parse_field(
                &chars,
                &mut idx,
                style,
                allow_level,
                true,
                true,
                compress_blanks,
            )?;
            if sort.is_empty() {
                return Err(anyhow!("Illegal null field in index key."));
            }
            let mut actual = String::new();
            if german_sort {
                if let Some(transformed) = german_sort_transform(&sort) {
                    actual = sort.clone();
                    sort = transformed;
                }
            }
            levels.push(Level { sort, actual });
            current_level = Some(levels.len() - 1);
            need_level_delim = true;
        }
    }

    if levels.is_empty() {
        return Err(anyhow!("Empty index key"));
    }

    validate_levels(&levels)?;

    let (encap_text, range_op) = normalize_encap(encap, style);

    Ok(KeySplit {
        levels,
        encap: encap_text,
        range_op,
    })
}

fn parse_field(
    chars: &[char],
    idx: &mut usize,
    style: &Style,
    allow_level: bool,
    allow_encap: bool,
    allow_actual: bool,
    compress_blanks: bool,
) -> Result<String> {
    skip_leading_blanks(chars, idx, compress_blanks);
    let len = chars.len();
    let mut out = String::new();
    while *idx < len {
        let mut escape_count = 0;
        while *idx < len && chars[*idx] == style.idx_escape {
            escape_count += 1;
            out.push(chars[*idx]);
            *idx += 1;
            if *idx >= len {
                break;
            }
        }
        if *idx >= len {
            break;
        }
        let ch = chars[*idx];
        if ch == style.idx_quote {
            if escape_count % 2 == 0 {
                *idx += 1;
                if *idx >= len {
                    return Err(anyhow!("Dangling quote in index key"));
                }
                out.push(chars[*idx]);
                *idx += 1;
                continue;
            } else {
                out.push(ch);
                *idx += 1;
                continue;
            }
        }

        if ch == style.idx_level {
            if allow_level {
                break;
            } else {
                return Err(anyhow!(
                    "Extra '{}' at position {} of first argument.",
                    style.idx_level,
                    *idx + 1
                ));
            }
        }
        if ch == style.idx_encap {
            if allow_encap {
                break;
            } else {
                return Err(anyhow!(
                    "Extra '{}' at position {} of first argument.",
                    style.idx_encap,
                    *idx + 1
                ));
            }
        }
        if ch == style.idx_actual {
            if allow_actual {
                break;
            } else {
                return Err(anyhow!(
                    "Extra '{}' at position {} of first argument.",
                    style.idx_actual,
                    *idx + 1
                ));
            }
        }

        out.push(ch);
        *idx += 1;
    }

    if compress_blanks {
        while out.ends_with(' ') {
            out.pop();
        }
    }

    Ok(out)
}

fn skip_leading_blanks(chars: &[char], idx: &mut usize, compress_blanks: bool) {
    if !compress_blanks {
        return;
    }
    let len = chars.len();
    while *idx < len {
        match chars[*idx] {
            ' ' | '\t' => *idx += 1,
            _ => break,
        }
    }
}

fn validate_levels(levels: &[Level]) -> Result<()> {
    if levels.is_empty() {
        return Err(anyhow!("Empty index key"));
    }
    for level in levels {
        if level.sort.is_empty() {
            return Err(anyhow!("Illegal null field in index key."));
        }
    }
    Ok(())
}

fn normalize_encap(encap: String, style: &Style) -> (String, RangeOp) {
    if encap.is_empty() {
        return (encap, RangeOp::None);
    }
    let mut chars = encap.chars();
    if let Some(first) = chars.next() {
        if first == style.idx_ropen {
            return (chars.collect(), RangeOp::Open);
        } else if first == style.idx_rclose {
            return (chars.collect(), RangeOp::Close);
        }
    }
    (encap, RangeOp::None)
}

fn extract_argument<'a>(
    text: &'a str,
    open: char,
    close: char,
    escape: char,
    quote: Option<char>,
    compress_blanks: bool,
) -> Result<(String, &'a str)> {
    let trimmed = text.trim_start();
    let mut iter = trimmed.char_indices();
    match iter.next() {
        Some((_, ch)) if ch == open => {}
        _ => return Err(anyhow!("Missing opening delimiter")),
    }
    let mut depth = 1;
    let mut buf = String::new();
    let mut take_literal = false;
    let mut last_char_space = false;
    let quote_char = quote.unwrap_or('\0');

    while let Some((idx, ch)) = iter.next() {
        if take_literal {
            buf.push(ch);
            take_literal = false;
            continue;
        }

        if ch == escape || (quote.is_some() && ch == quote_char) {
            buf.push(ch);
            take_literal = true;
            continue;
        }

        if ch == open {
            depth += 1;
            buf.push(ch);
            continue;
        }
        if ch == close {
            depth -= 1;
            if depth == 0 {
                let remainder = trimmed[idx + ch.len_utf8()..].trim_start();
                if compress_blanks {
                    buf = compress_whitespace(&buf);
                }
                return Ok((buf, remainder));
            }
            buf.push(ch);
            continue;
        }

        if compress_blanks && ch.is_whitespace() {
            if !last_char_space {
                buf.push(' ');
                last_char_space = true;
            }
            continue;
        }
        last_char_space = false;
        buf.push(ch);
    }
    Err(anyhow!("Unterminated argument"))
}

fn compress_whitespace(text: &str) -> String {
    let mut out = String::new();
    let mut prev_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    out.trim().to_string()
}

fn compute_line_offsets(text: &str) -> Vec<usize> {
    let mut offsets = Vec::new();
    offsets.push(0);
    for (idx, ch) in text.char_indices() {
        if ch == '\n' {
            offsets.push(idx + 1);
        }
    }
    offsets
}

fn line_number_at(position: usize, offsets: &[usize]) -> usize {
    match offsets.binary_search(&position) {
        Ok(idx) => idx + 1,
        Err(idx) => idx,
    }
    .max(1)
}

fn trim_leading_ws(text: &str) -> (&str, usize) {
    let trimmed = text.trim_start();
    (trimmed, text.len() - trimmed.len())
}

fn determine_group(first: &str) -> Group {
    if !first.is_empty() && first.chars().all(|c| c.is_ascii_digit()) {
        let value = first.parse().unwrap_or(0);
        Group::Numeric(value)
    } else if first.chars().next().map_or(false, is_symbol_char) {
        Group::Symbol
    } else {
        Group::Alpha
    }
}

fn is_symbol_char(ch: char) -> bool {
    matches!(ch, '!'..='@' | '['..='`' | '{'..='~')
}

fn german_sort_transform(input: &str) -> Option<String> {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut changed = false;
    while let Some(ch) = chars.next() {
        if ch == '"' {
            if let Some(next) = chars.next() {
                if let Some(repl) = german_replacement(next) {
                    out.push(repl.0);
                    if let Some(extra) = repl.1 {
                        out.push(extra);
                    }
                    changed = true;
                    continue;
                } else {
                    out.push('"');
                    out.push(next);
                    continue;
                }
            } else {
                out.push('"');
                break;
            }
        } else {
            out.push(ch);
        }
    }
    if changed { Some(out) } else { None }
}

fn german_replacement(ch: char) -> Option<(char, Option<char>)> {
    match ch {
        'a' => Some(('a', Some('e'))),
        'A' => Some(('A', Some('e'))),
        'o' => Some(('o', Some('e'))),
        'O' => Some(('O', Some('e'))),
        'u' => Some(('u', Some('e'))),
        'U' => Some(('U', Some('e'))),
        's' => Some(('s', Some('s'))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_german_transform_applies_umlauts() {
        assert_eq!(
            german_sort_transform("\"a\"o\"us"),
            Some("aeoeues".to_string())
        );
    }

    #[test]
    fn test_german_transform_no_change() {
        assert_eq!(german_sort_transform("alpha"), None);
    }
}
