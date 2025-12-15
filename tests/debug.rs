use makeindex::idx::{IndexParser, ParseConfig};
use makeindex::sort;
use makeindex::style::Style;

#[test]
#[ignore]
fn dump_b211e() {
    let style = Style::default();
    let parser = IndexParser::new(
        &style,
        ParseConfig {
            compress_blanks: false,
            german_sort: false,
        },
    );
    let text = std::fs::read("makeindex/test/b211e.idx").unwrap();
    let text = makeindex::util::decode_latin1(&text);
    let parsed = parser.parse_str("b211e", &text, |_| {});
    let mut entries = parsed.entries;
    sort::sort_entries(&mut entries, false, false);
    for entry in entries.iter().take(250) {
        println!(
            "sort0={:?} actual0={:?} literal={:?} group={:?}",
            entry.sort_keys[0], entry.actual_keys[0], entry.literal_page, entry.group
        );
    }
}

#[test]
#[ignore]
fn dump_b211g() {
    let style = Style::default();
    let parser = IndexParser::new(
        &style,
        ParseConfig {
            compress_blanks: false,
            german_sort: false,
        },
    );
    let text = std::fs::read("makeindex/test/b211g.idx").unwrap();
    let text = makeindex::util::decode_latin1(&text);
    let parsed = parser.parse_str("b211g", &text, |_| {});
    let mut entries = parsed.entries;
    sort::sort_entries(&mut entries, false, false);
    println!("entries parsed: {}", entries.len());
    for entry in entries.iter().take(10) {
        println!(
            "sort={:?} actual={:?} depth={}",
            entry.sort_keys,
            entry.actual_keys,
            entry_depth(entry)
        );
    }
    println!("-- delta entries --");
    for entry in entries.iter().filter(|e| e.sort_keys[0] == "delta") {
        println!(
            "delta entry: sort={:?} literal={} type={:?} range={:?} encap={:?} values={:?}",
            entry.sort_keys,
            entry.literal_page,
            entry.page_type,
            entry.range_op,
            entry.encap,
            entry.page_fields.values
        );
    }
}

fn entry_depth(entry: &makeindex::entry::Entry) -> usize {
    for level in (0..makeindex::entry::FIELD_MAX).rev() {
        if !entry.sort_keys[level].is_empty() {
            return level;
        }
    }
    0
}
