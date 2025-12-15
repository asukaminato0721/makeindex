use std::{fs, path::Path};

use anyhow::{Result, anyhow, bail};

use crate::{entry::FIELD_MAX, page::PageType};

const PREAMBLE_DEF: &str = "\\begin{theindex}\n";
const POSTAMBLE_DEF: &str = "\n\n\\end{theindex}\n";
const SETPAGEOPEN_DEF: &str = "\n  \\setcounter{page}{";
const SETPAGECLOSE_DEF: &str = "}\n";
const GROUPSKIP_DEF: &str = "\n\n  \\indexspace\n";
const HEADINGPRE_DEF: &str = "";
const HEADINGSUF_DEF: &str = "";
const SYMHEADPOS_DEF: &str = "Symbols";
const SYMHEADNEG_DEF: &str = "symbols";
const NUMHEADPOS_DEF: &str = "Numbers";
const NUMHEADNEG_DEF: &str = "numbers";
const ITEM0_DEF: &str = "\n  \\item ";
const ITEM1_DEF: &str = "\n    \\subitem ";
const ITEM2_DEF: &str = "\n      \\subsubitem ";
const DELIM_DEF: &str = ", ";
const DELIMR_DEF: &str = "--";
const DELIMT_DEF: &str = "";
const ENCAP0_DEF: &str = "\\";
const ENCAP1_DEF: &str = "{";
const ENCAP2_DEF: &str = "}";
const INDENTSPC_DEF: &str = "\t\t";
const COMPOSITOR_DEF: &str = "-";
const PRECEDENCE_DEF: &str = "rnaRA";
const INDEX_KEYWORD: &str = "\\indexentry";
const INDEX_AOPEN: char = '{';
const INDEX_ACLOSE: char = '}';
const INDEX_LEVEL: char = '!';
const INDEX_ROPEN: char = '(';
const INDEX_RCLOSE: char = ')';
const INDEX_QUOTE: char = '"';
const INDEX_ACTUAL: char = '@';
const INDEX_ENCAP: char = '|';
const INDEX_ESCAPE: char = '\\';

#[derive(Debug, Clone)]
pub struct Style {
    pub idx_keyword: String,
    pub idx_aopen: char,
    pub idx_aclose: char,
    pub idx_level: char,
    pub idx_ropen: char,
    pub idx_rclose: char,
    pub idx_quote: char,
    pub idx_actual: char,
    pub idx_encap: char,
    pub idx_escape: char,
    pub preamble: String,
    pub postamble: String,
    pub setpage_open: String,
    pub setpage_close: String,
    pub group_skip: String,
    pub headings_flag: i32,
    pub heading_pre: String,
    pub heading_suf: String,
    pub symhead_pos: String,
    pub symhead_neg: String,
    pub numhead_pos: String,
    pub numhead_neg: String,
    pub item_r: [String; FIELD_MAX],
    pub item_u: [String; FIELD_MAX],
    pub item_x: [String; FIELD_MAX],
    pub delim_p: [String; FIELD_MAX],
    pub delim_n: String,
    pub delim_r: String,
    pub delim_t: String,
    pub suffix_2p: String,
    pub suffix_3p: String,
    pub suffix_mp: String,
    pub encap_p: String,
    pub encap_i: String,
    pub encap_s: String,
    pub linemax: usize,
    pub indent_space: String,
    pub indent_length: usize,
    pub page_compositor: String,
    pub page_precedence: String,
    pub page_offsets: [i32; 5],
}

impl Default for Style {
    fn default() -> Self {
        let mut style = Self {
            idx_keyword: INDEX_KEYWORD.to_string(),
            idx_aopen: INDEX_AOPEN,
            idx_aclose: INDEX_ACLOSE,
            idx_level: INDEX_LEVEL,
            idx_ropen: INDEX_ROPEN,
            idx_rclose: INDEX_RCLOSE,
            idx_quote: INDEX_QUOTE,
            idx_actual: INDEX_ACTUAL,
            idx_encap: INDEX_ENCAP,
            idx_escape: INDEX_ESCAPE,
            preamble: PREAMBLE_DEF.to_string(),
            postamble: POSTAMBLE_DEF.to_string(),
            setpage_open: SETPAGEOPEN_DEF.to_string(),
            setpage_close: SETPAGECLOSE_DEF.to_string(),
            group_skip: GROUPSKIP_DEF.to_string(),
            headings_flag: 0,
            heading_pre: HEADINGPRE_DEF.to_string(),
            heading_suf: HEADINGSUF_DEF.to_string(),
            symhead_pos: SYMHEADPOS_DEF.to_string(),
            symhead_neg: SYMHEADNEG_DEF.to_string(),
            numhead_pos: NUMHEADPOS_DEF.to_string(),
            numhead_neg: NUMHEADNEG_DEF.to_string(),
            item_r: [
                ITEM0_DEF.to_string(),
                ITEM1_DEF.to_string(),
                ITEM2_DEF.to_string(),
            ],
            item_u: [String::new(), ITEM1_DEF.to_string(), ITEM2_DEF.to_string()],
            item_x: [String::new(), ITEM1_DEF.to_string(), ITEM2_DEF.to_string()],
            delim_p: [
                DELIM_DEF.to_string(),
                DELIM_DEF.to_string(),
                DELIM_DEF.to_string(),
            ],
            delim_n: DELIM_DEF.to_string(),
            delim_r: DELIMR_DEF.to_string(),
            delim_t: DELIMT_DEF.to_string(),
            suffix_2p: String::new(),
            suffix_3p: String::new(),
            suffix_mp: String::new(),
            encap_p: ENCAP0_DEF.to_string(),
            encap_i: ENCAP1_DEF.to_string(),
            encap_s: ENCAP2_DEF.to_string(),
            linemax: 72,
            indent_space: INDENTSPC_DEF.to_string(),
            indent_length: 16,
            page_compositor: COMPOSITOR_DEF.to_string(),
            page_precedence: PRECEDENCE_DEF.to_string(),
            page_offsets: [0; 5],
        };
        style.recompute_page_offsets().ok();
        style
    }
}

impl Style {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)?;
        let mut style = Style::default();
        parse_style_file(&text, &mut style)
            .map_err(|e| anyhow!("{} (while parsing {:?})", e, path))?;
        Ok(style)
    }

    pub fn page_offset(&self, kind: PageType) -> i32 {
        match kind {
            PageType::RomanLower => self.page_offsets[0],
            PageType::RomanUpper => self.page_offsets[1],
            PageType::Arabic => self.page_offsets[2],
            PageType::AlphaLower => self.page_offsets[3],
            PageType::AlphaUpper => self.page_offsets[4],
            PageType::Empty => 0,
        }
    }

    pub fn recompute_page_offsets(&mut self) -> Result<()> {
        let mut order = Vec::new();
        for ch in self.page_precedence.chars() {
            match ch {
                'r' => push_once(&mut order, PageType::RomanLower)?,
                'R' => push_once(&mut order, PageType::RomanUpper)?,
                'n' => push_once(&mut order, PageType::Arabic)?,
                'a' => push_once(&mut order, PageType::AlphaLower)?,
                'A' => push_once(&mut order, PageType::AlphaUpper)?,
                _ => bail!("Unknown page precedence spec '{}'", ch),
            }
        }
        for kind in [
            PageType::RomanLower,
            PageType::RomanUpper,
            PageType::Arabic,
            PageType::AlphaLower,
            PageType::AlphaUpper,
        ] {
            if !order.contains(&kind) {
                order.push(kind);
            }
        }

        let mut offsets = [0; 5];
        let mut current = 0;
        for kind in order {
            match kind {
                PageType::RomanLower => offsets[0] = current,
                PageType::RomanUpper => offsets[1] = current,
                PageType::Arabic => offsets[2] = current,
                PageType::AlphaLower => offsets[3] = current,
                PageType::AlphaUpper => offsets[4] = current,
                PageType::Empty => {}
            }
            current += match kind {
                PageType::RomanLower | PageType::RomanUpper | PageType::Arabic => 10000,
                PageType::AlphaLower | PageType::AlphaUpper => 26,
                PageType::Empty => 0,
            };
        }
        self.page_offsets = offsets;
        Ok(())
    }
}

fn push_once(vec: &mut Vec<PageType>, kind: PageType) -> Result<()> {
    if vec.contains(&kind) {
        bail!(
            "Multiple instances of {:?} in page precedence specification",
            kind
        );
    }
    vec.push(kind);
    Ok(())
}

fn parse_style_file(text: &str, style: &mut Style) -> Result<()> {
    let mut scanner = StyScanner::new(text);
    while let Some(ident) = scanner.read_identifier()? {
        match ident.as_str() {
            "preamble" => style.preamble = scanner.read_string()?,
            "postamble" => style.postamble = scanner.read_string()?,
            "group_skip" => style.group_skip = scanner.read_string()?,
            "headings_flag" => style.headings_flag = scanner.read_number()?,
            "heading_prefix" => style.heading_pre = scanner.read_string()?,
            "heading_suffix" => style.heading_suf = scanner.read_string()?,
            "symhead_positive" => style.symhead_pos = scanner.read_string()?,
            "symhead_negative" => style.symhead_neg = scanner.read_string()?,
            "numhead_positive" => style.numhead_pos = scanner.read_string()?,
            "numhead_negative" => style.numhead_neg = scanner.read_string()?,
            "setpage_prefix" => style.setpage_open = scanner.read_string()?,
            "setpage_suffix" => style.setpage_close = scanner.read_string()?,
            "item_0" => style.item_r[0] = scanner.read_string()?,
            "item_1" => style.item_r[1] = scanner.read_string()?,
            "item_2" => style.item_r[2] = scanner.read_string()?,
            "item_01" => style.item_u[1] = scanner.read_string()?,
            "item_12" => style.item_u[2] = scanner.read_string()?,
            "item_x1" => style.item_x[1] = scanner.read_string()?,
            "item_x2" => style.item_x[2] = scanner.read_string()?,
            "encap_prefix" => style.encap_p = scanner.read_string()?,
            "encap_infix" => style.encap_i = scanner.read_string()?,
            "encap_suffix" => style.encap_s = scanner.read_string()?,
            "delim_0" => style.delim_p[0] = scanner.read_string()?,
            "delim_1" => style.delim_p[1] = scanner.read_string()?,
            "delim_2" => style.delim_p[2] = scanner.read_string()?,
            "delim_n" => style.delim_n = scanner.read_string()?,
            "delim_r" => style.delim_r = scanner.read_string()?,
            "delim_t" => style.delim_t = scanner.read_string()?,
            "suffix_2p" => style.suffix_2p = scanner.read_string()?,
            "suffix_3p" => style.suffix_3p = scanner.read_string()?,
            "suffix_mp" => style.suffix_mp = scanner.read_string()?,
            "line_max" => {
                let v = scanner.read_number()?;
                if v <= 0 {
                    bail!("line_max must be positive");
                }
                style.linemax = v as usize;
            }
            "indent_space" => style.indent_space = scanner.read_string()?,
            "indent_length" => {
                let v = scanner.read_number()?;
                if v < 0 {
                    bail!("indent_length must be nonnegative");
                }
                style.indent_length = v as usize;
            }
            "page_compositor" => style.page_compositor = scanner.read_string()?,
            "page_precedence" => {
                style.page_precedence = scanner.read_string()?;
                style.recompute_page_offsets()?;
            }
            "keyword" => style.idx_keyword = scanner.read_string()?,
            "arg_open" => style.idx_aopen = scanner.read_char()?,
            "arg_close" => style.idx_aclose = scanner.read_char()?,
            "level" => style.idx_level = scanner.read_char()?,
            "range_open" => style.idx_ropen = scanner.read_char()?,
            "range_close" => style.idx_rclose = scanner.read_char()?,
            "quote" => style.idx_quote = scanner.read_char()?,
            "actual" => style.idx_actual = scanner.read_char()?,
            "encap" => style.idx_encap = scanner.read_char()?,
            "escape" => style.idx_escape = scanner.read_char()?,
            ident => bail!("Unknown specifier '{}'", ident),
        }
    }
    if style.idx_quote == style.idx_escape {
        bail!("Quote and escape symbols must be distinct");
    }
    Ok(())
}

struct StyScanner<'a> {
    data: &'a str,
    bytes: &'a [u8],
    pos: usize,
    len: usize,
}

impl<'a> StyScanner<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            data: text,
            bytes: text.as_bytes(),
            pos: 0,
            len: text.len(),
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.len {
            let ch = self.bytes[self.pos] as char;
            if ch == '%' {
                while self.pos < self.len && self.bytes[self.pos] != b'\n' {
                    self.pos += 1;
                }
            }
            if self.pos >= self.len {
                break;
            }
            let ch = self.bytes[self.pos] as char;
            if ch.is_whitespace() {
                self.pos += 1;
                continue;
            }
            break;
        }
    }

    fn read_identifier(&mut self) -> Result<Option<String>> {
        self.skip_ws();
        if self.pos >= self.len {
            return Ok(None);
        }
        let start = self.pos;
        while self.pos < self.len {
            let ch = self.bytes[self.pos] as char;
            if ch.is_whitespace() {
                break;
            }
            self.pos += 1;
        }
        let ident = &self.data[start..self.pos];
        Ok(Some(ident.to_ascii_lowercase()))
    }

    fn read_number(&mut self) -> Result<i32> {
        self.skip_ws();
        let start = self.pos;
        while self.pos < self.len {
            let ch = self.bytes[self.pos] as char;
            if ch.is_ascii_digit() || ch == '-' || ch == '+' {
                self.pos += 1;
            } else {
                break;
            }
        }
        if start == self.pos {
            bail!("Expected number");
        }
        let txt = &self.data[start..self.pos];
        txt.parse().map_err(|_| anyhow!("Invalid number {}", txt))
    }

    fn read_char(&mut self) -> Result<char> {
        self.skip_ws();
        if self.pos >= self.len || self.bytes[self.pos] != b'\'' {
            bail!("Expected character literal");
        }
        self.pos += 1;
        if self.pos >= self.len {
            bail!("Unterminated character literal");
        }
        let ch = self.bytes[self.pos] as char;
        self.pos += 1;
        if self.pos >= self.len || self.bytes[self.pos] != b'\'' {
            bail!("Character literal must contain exactly one character");
        }
        self.pos += 1;
        Ok(ch)
    }

    fn read_string(&mut self) -> Result<String> {
        self.skip_ws();
        if self.pos >= self.len || self.bytes[self.pos] != b'"' {
            bail!("Expected string literal");
        }
        self.pos += 1;
        let mut buf = String::new();
        while self.pos < self.len {
            let ch = self.bytes[self.pos] as char;
            self.pos += 1;
            match ch {
                '"' => return Ok(buf),
                '\\' => {
                    if self.pos >= self.len {
                        bail!("Unterminated escape sequence");
                    }
                    let esc = self.bytes[self.pos] as char;
                    self.pos += 1;
                    match esc {
                        'n' => buf.push('\n'),
                        't' => buf.push('\t'),
                        '\\' => buf.push('\\'),
                        '"' => buf.push('"'),
                        _ => buf.push(esc),
                    }
                }
                _ => buf.push(ch),
            }
        }
        bail!("Unterminated string literal");
    }
}
