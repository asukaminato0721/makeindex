use std::io::Write;

use anyhow::Result;

use crate::{
    entry::{Entry, FIELD_MAX, Group, RangeOp},
    style::Style,
    util,
};

pub struct OutputConfig {
    pub merge_page_ranges: bool,
    pub initial_page: Option<String>,
    pub german_sort: bool,
}

pub fn generate_index<W: Write>(
    entries: &[Entry],
    style: &Style,
    config: OutputConfig,
    writer: &mut W,
) -> Result<()> {
    util::write_latin1(writer, &style.preamble)?;
    if let Some(page) = &config.initial_page {
        util::write_latin1(writer, &style.setpage_open)?;
        util::write_latin1(writer, page)?;
        util::write_latin1(writer, &style.setpage_close)?;
    }
    let mut prev_entry: Option<&Entry> = None;
    let mut prev_heading: Option<String> = None;
    let mut first_entry = true;

    let mut idx = 0usize;
    while idx < entries.len() {
        let entry = &entries[idx];
        let block_change = prev_entry
            .map(|prev| needs_group_skip(prev, entry, config.german_sort))
            .unwrap_or(false);

        if first_entry {
            emit_heading(entry, style, &mut prev_heading, writer)?;
            first_entry = false;
        } else if block_change {
            util::write_latin1(writer, &style.delim_t)?;
            util::write_latin1(writer, &style.group_skip)?;
            emit_heading(entry, style, &mut prev_heading, writer)?;
        } else {
            util::write_latin1(writer, &style.delim_t)?;
        }

        let mut group_refs = Vec::new();
        let mut j = idx;
        while j < entries.len() && same_key(entry, &entries[j]) {
            group_refs.push(&entries[j]);
            j += 1;
        }

        let pages = consolidate_pages(&group_refs, style, config.merge_page_ranges);
        let entry_text = render_entry(entry, prev_entry, &pages, style);
        util::write_latin1(writer, &entry_text)?;
        prev_entry = Some(entry);
        idx = j;
    }

    util::write_latin1(writer, &style.delim_t)?;
    util::write_latin1(writer, &style.postamble)?;
    Ok(())
}

fn render_entry(entry: &Entry, prev: Option<&Entry>, pages: &[String], style: &Style) -> String {
    let depth = entry_depth(entry);
    let diff_level = first_diff(prev, entry).unwrap_or(depth);
    let prev_depth = prev.map(entry_depth);
    let mut segments: Vec<String> = Vec::new();

    if diff_level <= depth {
        let mut current = String::new();
        let template = if prev_depth.is_some_and(|pd| diff_level > pd) {
            &style.item_u[diff_level]
        } else {
            &style.item_r[diff_level]
        };
        current.push_str(template);
        current.push_str(render_level(entry, diff_level));

        for level in diff_level + 1..=depth {
            segments.push(current);
            current = String::new();
            current.push_str(&style.item_x[level]);
            current.push_str(render_level(entry, level));
        }
        current.push_str(&style.delim_p[depth]);
        segments.push(current);
    } else {
        segments.push(String::new());
    }

    let mut rendered = String::new();
    let tail = segments.pop().unwrap_or_default();
    for segment in segments {
        rendered.push_str(&segment);
    }
    rendered.push_str(&format_pages(tail, pages, style));
    rendered
}

fn heading_label(entry: &Entry, style: &Style) -> Option<String> {
    if style.headings_flag == 0 {
        return None;
    }
    let label = match entry.group {
        Group::Symbol => {
            if style.headings_flag > 0 {
                style.symhead_pos.clone()
            } else {
                style.symhead_neg.clone()
            }
        }
        Group::Numeric(_) => {
            if style.headings_flag > 0 {
                style.numhead_pos.clone()
            } else {
                style.numhead_neg.clone()
            }
        }
        Group::Alpha => {
            let mut chars = entry.sort_keys[0].chars();
            let ch = chars.next().unwrap_or('?');
            let c = if style.headings_flag > 0 {
                ch.to_ascii_uppercase()
            } else {
                ch.to_ascii_lowercase()
            };
            c.to_string()
        }
    };
    Some(label)
}

fn emit_heading<W: Write>(
    entry: &Entry,
    style: &Style,
    prev_heading: &mut Option<String>,
    writer: &mut W,
) -> Result<()> {
    if style.headings_flag == 0 {
        return Ok(());
    }
    if let Some(label) = heading_label(entry, style)
        && prev_heading.as_ref().map(String::as_str) != Some(label.as_str())
    {
        util::write_latin1(writer, &style.heading_pre)?;
        util::write_latin1(writer, &label)?;
        util::write_latin1(writer, &style.heading_suf)?;
        *prev_heading = Some(label);
    }
    Ok(())
}

fn needs_group_skip(prev: &Entry, curr: &Entry, german_sort: bool) -> bool {
    let prev_kind = group_kind(&prev.group);
    let curr_kind = group_kind(&curr.group);
    if prev_kind != curr_kind {
        return true;
    }
    if prev_kind == GroupKind::Alpha && curr_kind == GroupKind::Alpha {
        return entry_letter(prev) != entry_letter(curr);
    }
    if german_sort && prev_kind == GroupKind::Alpha && curr_kind != GroupKind::Alpha {
        return true;
    }
    false
}

#[derive(PartialEq, Eq)]
enum GroupKind {
    Symbol,
    Numeric,
    Alpha,
}

fn group_kind(group: &Group) -> GroupKind {
    match group {
        Group::Symbol => GroupKind::Symbol,
        Group::Numeric(_) => GroupKind::Numeric,
        Group::Alpha => GroupKind::Alpha,
    }
}

fn entry_letter(entry: &Entry) -> Option<char> {
    entry
        .sort_keys
        .first()
        .and_then(|s| s.chars().next())
        .map(|c| c.to_ascii_lowercase())
}

fn same_key(a: &Entry, b: &Entry) -> bool {
    for i in 0..FIELD_MAX {
        if a.sort_keys[i] != b.sort_keys[i] || a.actual_keys[i] != b.actual_keys[i] {
            return false;
        }
    }
    true
}

fn entry_depth(entry: &Entry) -> usize {
    for level in (0..FIELD_MAX).rev() {
        if !entry.sort_keys[level].is_empty() {
            return level;
        }
    }
    0
}

fn first_diff(prev: Option<&Entry>, current: &Entry) -> Option<usize> {
    if let Some(prev_entry) = prev {
        for level in 0..FIELD_MAX {
            let prev_empty = prev_entry.sort_keys[level].is_empty();
            let cur_empty = current.sort_keys[level].is_empty();
            if prev_empty && cur_empty {
                break;
            }
            if prev_entry.sort_keys[level] != current.sort_keys[level]
                || prev_entry.actual_keys[level] != current.actual_keys[level]
            {
                return Some(level);
            }
        }
        Some(entry_depth(current))
    } else {
        Some(0)
    }
}

fn render_level(entry: &Entry, level: usize) -> &str {
    if entry.actual_keys[level].is_empty() {
        entry.sort_keys[level].as_str()
    } else {
        entry.actual_keys[level].as_str()
    }
}

fn consolidate_pages(entries: &[&Entry], style: &Style, merge_ranges: bool) -> Vec<String> {
    let mut collector = PageCollector::new(style, merge_ranges);
    for entry in entries {
        collector.push(entry);
    }
    collector.finish()
}

struct PageRun<'a> {
    entries: Vec<&'a Entry>,
    encap: &'a str,
    encap_range: bool,
}

impl<'a> PageRun<'a> {
    fn new(entry: &'a Entry) -> Self {
        Self {
            entries: vec![entry],
            encap: entry.encap.as_str(),
            encap_range: false,
        }
    }

    fn base_encap(&self) -> &str {
        self.encap
    }

    fn start(&self) -> &'a Entry {
        self.entries[0]
    }

    fn end(&self) -> &'a Entry {
        self.entries.last().unwrap()
    }

    fn push(&mut self, entry: &'a Entry) {
        self.entries.push(entry);
    }
}

struct PageCollector<'a> {
    style: &'a Style,
    merge_ranges: bool,
    pages: Vec<String>,
    run: Option<PageRun<'a>>,
    in_range: bool,
}

impl<'a> PageCollector<'a> {
    fn new(style: &'a Style, merge_ranges: bool) -> Self {
        Self {
            style,
            merge_ranges,
            pages: Vec::new(),
            run: None,
            in_range: false,
        }
    }

    fn push(&mut self, entry: &'a Entry) {
        let merged = if let Some(run) = self.run.as_mut() {
            PageCollector::merge_into(run, entry, self.merge_ranges, self.in_range)
        } else {
            false
        };
        let encap_fragment = if merged && self.in_range {
            self.run
                .as_ref()
                .and_then(|run| make_inrange_fragment(self.style, run, entry))
        } else {
            None
        };
        if !merged {
            self.flush_run();
            self.run = Some(PageRun::new(entry));
        }
        if let Some(fragment) = encap_fragment {
            self.pages.push(fragment);
        }
        self.update_range_state(entry);
    }

    fn finish(mut self) -> Vec<String> {
        self.flush_run();
        self.pages
    }

    fn flush_run(&mut self) {
        if let Some(run) = self.run.take() {
            let fragments = render_run(run, self.style);
            self.pages.extend(fragments);
        }
    }

    fn update_range_state(&mut self, entry: &'a Entry) {
        match entry.range_op {
            RangeOp::Open => {
                self.in_range = true;
            }
            RangeOp::Close => {
                self.in_range = false;
                if let Some(run) = self.run.as_mut() {
                    run.encap_range = true;
                }
            }
            RangeOp::None => {}
        }
    }
}

impl<'a> PageCollector<'a> {
    fn merge_into(run: &mut PageRun<'a>, entry: &'a Entry, merge: bool, in_range: bool) -> bool {
        let last = run.end();
        if in_range {
            if page_diff(last, entry).is_none() {
                return false;
            }
            run.encap_range = true;
            run.push(entry);
            return true;
        }
        let diff = match page_diff(last, entry) {
            Some(v) => v,
            None => return false,
        };
        let same_encap = run.base_encap() == entry.encap.as_str();

        if diff == 0 {
            return same_encap;
        }
        if diff == 1 && merge && same_encap {
            run.push(entry);
            return true;
        }
        false
    }
}

fn make_inrange_fragment(style: &Style, run: &PageRun<'_>, entry: &Entry) -> Option<String> {
    if entry.range_op != RangeOp::None {
        return None;
    }
    if entry.encap.is_empty() {
        return None;
    }
    if entry.encap.as_str() == run.base_encap() {
        return None;
    }
    Some(wrap_with_encap(
        entry.literal_page.clone(),
        &entry.encap,
        style,
    ))
}

fn page_distance(start: &Entry, end: &Entry) -> i32 {
    let start_val = start.page_fields.values.last().copied().unwrap_or(0);
    let end_val = end.page_fields.values.last().copied().unwrap_or(0);
    end_val - start_val
}

fn render_run(run: PageRun<'_>, style: &Style) -> Vec<String> {
    let start = run.start();
    let end = run.end();
    if start.literal_page == end.literal_page {
        return vec![wrap_page(start, style)];
    }
    let diff = page_distance(start, end);
    let threshold = if style.suffix_2p.is_empty() { 1 } else { 0 };
    if !run.encap_range && diff <= threshold {
        let text = format!(
            "{}{}{}",
            start.literal_page, style.delim_n, end.literal_page
        );
        if start.encap.is_empty() {
            return vec![text];
        } else {
            return vec![wrap_with_encap(text, &start.encap, style)];
        }
    }
    vec![render_span(start, end, style)]
}

fn render_span(start: &Entry, end: &Entry, style: &Style) -> String {
    let diff = page_distance(start, end);
    let text = if diff == 1 && !style.suffix_2p.is_empty() {
        format!("{}{}", start.literal_page, style.suffix_2p)
    } else if diff == 2 && !style.suffix_3p.is_empty() {
        format!("{}{}", start.literal_page, style.suffix_3p)
    } else if diff >= 2 && !style.suffix_mp.is_empty() {
        format!("{}{}", start.literal_page, style.suffix_mp)
    } else {
        format!(
            "{}{}{}",
            start.literal_page, style.delim_r, end.literal_page
        )
    };
    wrap_with_encap(text, &start.encap, style)
}

fn page_diff(a: &Entry, b: &Entry) -> Option<i32> {
    if a.page_type != b.page_type {
        return None;
    }
    let a_vals = &a.page_fields.values;
    let b_vals = &b.page_fields.values;
    if a_vals.len() != b_vals.len() || a_vals.is_empty() {
        return None;
    }
    if a_vals.len() > 1 && a_vals[..a_vals.len() - 1] != b_vals[..b_vals.len() - 1] {
        return None;
    }
    Some(b_vals[b_vals.len() - 1] - a_vals[a_vals.len() - 1])
}

fn wrap_page(entry: &Entry, style: &Style) -> String {
    wrap_with_encap(entry.literal_page.clone(), &entry.encap, style)
}

fn wrap_with_encap(text: String, encap: &str, style: &Style) -> String {
    if encap.is_empty() {
        text
    } else {
        format!(
            "{}{}{}{}{}",
            style.encap_p, encap, style.encap_i, text, style.encap_s
        )
    }
}

fn format_pages(line: String, pages: &[String], style: &Style) -> String {
    if pages.is_empty() {
        return line;
    }
    let mut writer = LineWriter::new(style, line);
    for (idx, page) in pages.iter().enumerate() {
        let last = idx == pages.len() - 1;
        if last {
            writer.push_final(page);
        } else {
            writer.push_chunk(page);
        }
    }
    writer.finish()
}

struct LineWriter<'a> {
    style: &'a Style,
    current: String,
    output: String,
    ind_indent: usize,
}

impl<'a> LineWriter<'a> {
    fn new(style: &'a Style, initial: String) -> Self {
        Self {
            style,
            current: initial,
            output: String::new(),
            ind_indent: 0,
        }
    }

    fn push_chunk(&mut self, chunk: &str) {
        self.flush(chunk, false);
    }

    fn push_final(&mut self, chunk: &str) {
        self.flush(chunk, true);
    }

    fn flush(&mut self, buff: &str, is_final: bool) {
        let len = self.current.len() + buff.len() + self.ind_indent;
        if is_final {
            if len > self.style.linemax {
                self.output.push_str(&self.current);
                self.output.push('\n');
                self.output.push_str(&self.style.indent_space);
                self.ind_indent = self.style.indent_length;
            } else {
                self.output.push_str(&self.current);
            }
            self.output.push_str(buff);
            self.current.clear();
        } else if len > self.style.linemax {
            self.output.push_str(&self.current);
            self.output.push('\n');
            self.current.clear();
            self.current.push_str(&self.style.indent_space);
            self.current.push_str(buff);
            self.current.push_str(&self.style.delim_n);
            self.ind_indent = self.style.indent_length;
        } else {
            self.current.push_str(buff);
            self.current.push_str(&self.style.delim_n);
        }
    }

    fn finish(mut self) -> String {
        if !self.current.is_empty() {
            self.output.push_str(&self.current);
        }
        self.output
    }
}
