use std::{
    fs,
    io::{self, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, anyhow};

use crate::{
    cli::{self, CliOptions, InitialPage},
    generator,
    idx::{IndexParser, ParseConfig, ParseError},
    sort,
    style::Style,
    util,
};

pub fn run() -> Result<()> {
    let opts = cli::parse()?;
    let mut app = App::new(opts)?;
    app.execute()
}

struct App {
    opts: CliOptions,
    style: Style,
    transcript: Transcript,
    initial_page: Option<String>,
    input_files: Vec<PathBuf>,
    output_path: Option<PathBuf>,
}

impl App {
    fn new(opts: CliOptions) -> Result<Self> {
        let input_files = resolve_input_files(&opts)?;
        let output_path = resolve_output_path(&opts, &input_files);
        let transcript_path = resolve_transcript_path(&opts, &input_files);

        let mut transcript = Transcript::new(transcript_path.as_deref(), opts.verbose)?;
        transcript.log("makeindex (Rust rewrite) starting...")?;

        let style = match &opts.style_file {
            Some(path) => {
                transcript.log(format!("Loading style file {}", path))?;
                Style::load_from_path(PathBuf::from(path).as_path())
                    .with_context(|| format!("while reading style file {}", path))?
            }
            None => {
                transcript.log("Using built-in default style.")?;
                Style::default()
            }
        };

        let initial_page =
            compute_initial_page(&opts, transcript_path.as_deref(), &mut transcript)?;

        Ok(Self {
            opts,
            style,
            transcript,
            initial_page,
            input_files,
            output_path,
        })
    }

    fn execute(&mut self) -> Result<()> {
        if self.input_files.is_empty() && !self.opts.use_stdin {
            anyhow::bail!("No input files specified (pass -i for stdin or list .idx files).");
        }

        let style = self.style.clone();
        let parser = IndexParser::new(
            &style,
            ParseConfig {
                compress_blanks: self.opts.compress_blanks,
                german_sort: self.opts.german_sort,
            },
        );

        let mut entries = Vec::new();
        if self.opts.use_stdin {
            self.transcript.log("Reading index entries from stdin...")?;
            let mut buf = Vec::new();
            io::stdin().read_to_end(&mut buf)?;
            let text = util::decode_latin1(&buf);
            let mut errors = Vec::new();
            let parsed = parser.parse_str("stdin", &text, |err| errors.push(err));
            for err in errors {
                self.report_parse_error(&err)?;
            }
            self.report_parse_summary("stdin", parsed.accepted, parsed.rejected)?;
            entries.extend(parsed.entries);
        }

        let file_list = self.input_files.clone();
        for path in &file_list {
            self.transcript
                .log(format!("Reading index file {}", path.display()))?;
            let bytes =
                fs::read(path).with_context(|| format!("while reading {}", path.display()))?;
            let text = util::decode_latin1(&bytes);
            let display_name = path.to_string_lossy().into_owned();
            let mut errors = Vec::new();
            let parsed = parser.parse_str(display_name.as_str(), &text, |err| errors.push(err));
            for err in errors {
                self.report_parse_error(&err)?;
            }
            self.report_parse_summary(display_name.as_str(), parsed.accepted, parsed.rejected)?;
            entries.extend(parsed.entries);
        }

        if entries.is_empty() {
            self.transcript
                .log("No valid index entries found; aborting.")?;
            return Err(anyhow!("No valid index entries found."));
        }

        sort::sort_entries(
            &mut entries,
            self.opts.letter_ordering,
            self.opts.german_sort,
        );

        self.transcript
            .log(format!("{} entries ready after sorting.", entries.len()))?;

        let mut output: Box<dyn io::Write> = match &self.output_path {
            Some(path) => Box::new(
                fs::File::create(path)
                    .with_context(|| format!("unable to create {}", path.display()))?,
            ),
            None => Box::new(io::stdout()),
        };

        generator::generate_index(
            &entries,
            &self.style,
            generator::OutputConfig {
                merge_page_ranges: self.opts.merge_page_ranges,
                initial_page: self.initial_page.clone(),
                german_sort: self.opts.german_sort,
            },
            &mut output,
        )?;
        match &self.output_path {
            Some(path) => self
                .transcript
                .log(format!("Index written to {}.", path.display()))?,
            None => self.transcript.log("Index written to stdout.")?,
        }
        Ok(())
    }

    fn report_parse_error(&mut self, error: &ParseError) -> Result<()> {
        self.transcript.input_error(error)
    }

    fn report_parse_summary(
        &mut self,
        source: &str,
        accepted: usize,
        rejected: usize,
    ) -> Result<()> {
        if rejected > 0 {
            self.transcript.log(format!(
                "  + {} entries from {} ({} rejected).",
                accepted, source, rejected
            ))
        } else {
            self.transcript
                .log(format!("  + {} entries from {}.", accepted, source))
        }
    }
}

struct Transcript {
    verbose: bool,
    writer: BufWriter<Box<dyn Write>>,
}

impl Transcript {
    fn new(path: Option<&Path>, verbose: bool) -> Result<Self> {
        let sink: Box<dyn Write> = if let Some(p) = path {
            Box::new(fs::File::create(p)?)
        } else {
            Box::new(io::stderr())
        };
        Ok(Self::with_writer(sink, verbose))
    }

    fn log(&mut self, message: impl AsRef<str>) -> Result<()> {
        let msg = message.as_ref();
        if self.verbose {
            eprintln!("{}", msg);
        }
        writeln!(self.writer, "{}", msg)?;
        Ok(())
    }

    fn input_error(&mut self, error: &ParseError) -> Result<()> {
        if self.verbose {
            eprintln!(
                "!! Input index error (file = {}, line = {}):",
                error.file, error.line
            );
            eprintln!("   -- {}", error.message);
        }
        writeln!(
            self.writer,
            "!! Input index error (file = {}, line = {}):",
            error.file, error.line
        )?;
        writeln!(self.writer, "   -- {}", error.message)?;
        Ok(())
    }

    fn with_writer(writer: Box<dyn Write>, verbose: bool) -> Self {
        Self {
            verbose,
            writer: BufWriter::new(writer),
        }
    }
}

impl Drop for Transcript {
    fn drop(&mut self) {
        let _ = self.writer.flush();
    }
}

fn compute_initial_page(
    opts: &CliOptions,
    transcript_path: Option<&Path>,
    transcript: &mut Transcript,
) -> Result<Option<String>> {
    let Some(request) = &opts.initial_page else {
        return Ok(None);
    };
    match request {
        InitialPage::Literal(text) => Ok(Some(text.clone())),
        InitialPage::NextEven | InitialPage::NextOdd | InitialPage::NextAny => {
            let Some(base) = derive_log_page(transcript_path)? else {
                transcript.log("Could not determine page from log; ignoring -p option.")?;
                return Ok(None);
            };
            let mut number: i32 = base
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid page literal {} in log", base))?;
            match request {
                InitialPage::NextEven => {
                    if number % 2 != 0 {
                        number += 1;
                    }
                }
                InitialPage::NextOdd => {
                    if number % 2 == 0 {
                        number += 1;
                    }
                }
                InitialPage::NextAny => {
                    number += 1;
                }
                _ => {}
            }
            transcript.log(format!("Initial page set to {}", number))?;
            Ok(Some(number.to_string()))
        }
    }
}

fn derive_log_page(path: Option<&Path>) -> Result<Option<String>> {
    let Some(log_path) = path else {
        return Ok(None);
    };
    if log_path.exists() {
        let text = fs::read_to_string(log_path)?;
        if let Some(last) = text.lines().rev().find_map(parse_page_line) {
            return Ok(Some(last));
        }
    }
    Ok(None)
}

fn resolve_input_files(opts: &CliOptions) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for raw in &opts.inputs {
        files.push(resolve_single_input(raw)?);
    }
    Ok(files)
}

fn resolve_single_input(raw: &str) -> Result<PathBuf> {
    let path = PathBuf::from(raw);
    if path.exists() {
        return Ok(path);
    }
    if path.extension().is_none() {
        let mut candidate = path.clone();
        candidate.set_extension("idx");
        if candidate.exists() {
            return Ok(candidate);
        }
    }
    Err(anyhow!("Input file {} not found", raw))
}

fn resolve_output_path(opts: &CliOptions, inputs: &[PathBuf]) -> Option<PathBuf> {
    if let Some(path) = &opts.output_file {
        if path == "-" {
            return None;
        }
        return Some(PathBuf::from(path));
    }
    if opts.use_stdin || inputs.len() != 1 {
        return None;
    }
    Some(util::derive_output_path(&inputs[0], "ind"))
}

fn resolve_transcript_path(opts: &CliOptions, inputs: &[PathBuf]) -> Option<PathBuf> {
    if let Some(path) = &opts.transcript_file {
        if path == "-" {
            return None;
        }
        return Some(PathBuf::from(path));
    }
    if opts.use_stdin || inputs.len() != 1 {
        return None;
    }
    Some(util::derive_output_path(&inputs[0], "ilg"))
}

fn parse_page_line(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        let inner = trimmed.trim_matches(&['[', ']'][..]).trim();
        if inner.chars().all(|c| c.is_ascii_digit()) {
            return Some(inner.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{CliOptions, InitialPage};
    use std::{fs, io, path::PathBuf, time::SystemTime};

    fn temp_path(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let unique = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        path.push(format!("makeindex-test-{}-{}", name, unique));
        path
    }

    fn test_transcript() -> Transcript {
        Transcript::with_writer(Box::new(io::sink()), false)
    }

    #[test]
    fn test_parse_page_line() {
        assert_eq!(parse_page_line("[12 ]"), Some("12".to_string()));
        assert_eq!(parse_page_line(" [ 999 ] "), Some("999".to_string()));
        assert_eq!(parse_page_line("[a12]"), None);
    }

    #[test]
    fn test_initial_page_literal() {
        let opts = CliOptions {
            initial_page: Some(InitialPage::Literal("42".into())),
            ..CliOptions::default()
        };
        let mut transcript = test_transcript();
        let result = compute_initial_page(&opts, None, &mut transcript).unwrap();
        assert_eq!(result, Some("42".into()));
    }

    #[test]
    fn test_initial_page_even_from_log() {
        let path = temp_path("even");
        fs::write(&path, "[17]\n").unwrap();
        let opts = CliOptions {
            initial_page: Some(InitialPage::NextEven),
            ..CliOptions::default()
        };
        let mut transcript = test_transcript();
        let result = compute_initial_page(&opts, Some(path.as_path()), &mut transcript).unwrap();
        assert_eq!(result, Some("18".into()));
    }

    #[test]
    fn test_initial_page_odd_from_log() {
        let path = temp_path("odd");
        fs::write(&path, "ignored\n[18]\n").unwrap();
        let opts = CliOptions {
            initial_page: Some(InitialPage::NextOdd),
            ..CliOptions::default()
        };
        let mut transcript = test_transcript();
        let result = compute_initial_page(&opts, Some(path.as_path()), &mut transcript).unwrap();
        assert_eq!(result, Some("19".into()));
    }

    #[test]
    fn test_initial_page_any_from_log() {
        let path = temp_path("any");
        fs::write(&path, "[5]\n").unwrap();
        let opts = CliOptions {
            initial_page: Some(InitialPage::NextAny),
            ..CliOptions::default()
        };
        let mut transcript = test_transcript();
        let result = compute_initial_page(&opts, Some(path.as_path()), &mut transcript).unwrap();
        assert_eq!(result, Some("6".into()));
    }
}
