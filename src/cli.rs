use std::env;

use anyhow::{Result, anyhow, bail};

#[derive(Debug, Clone)]
pub enum InitialPage {
    Literal(String),
    NextEven,
    NextOdd,
    NextAny,
}

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub use_stdin: bool,
    pub letter_ordering: bool,
    pub compress_blanks: bool,
    pub merge_page_ranges: bool,
    pub verbose: bool,
    pub german_sort: bool,
    pub style_file: Option<String>,
    pub output_file: Option<String>,
    pub transcript_file: Option<String>,
    pub initial_page: Option<InitialPage>,
    pub inputs: Vec<String>,
}

impl Default for CliOptions {
    fn default() -> Self {
        Self {
            use_stdin: false,
            letter_ordering: false,
            compress_blanks: false,
            merge_page_ranges: true,
            verbose: true,
            german_sort: false,
            style_file: None,
            output_file: None,
            transcript_file: None,
            initial_page: None,
            inputs: Vec::new(),
        }
    }
}

pub fn parse() -> Result<CliOptions> {
    let args: Vec<String> = env::args().skip(1).collect();
    parse_from(&args)
}

pub fn parse_from(argv: &[String]) -> Result<CliOptions> {
    let mut opts = CliOptions::default();
    let mut iter = argv.iter().peekable();
    let mut literal_mode = false;

    while let Some(arg) = iter.next() {
        if literal_mode || !arg.starts_with('-') || arg == "-" {
            opts.inputs.push(arg.clone());
            continue;
        }
        if arg == "--" {
            literal_mode = true;
            continue;
        }

        let mut chars = arg[1..].chars().peekable();
        if chars.peek().is_none() {
            opts.inputs.push(arg.clone());
            continue;
        }

        while let Some(flag) = chars.next() {
            match flag {
                'i' => opts.use_stdin = true,
                'l' => opts.letter_ordering = true,
                'r' => opts.merge_page_ranges = false,
                'q' => opts.verbose = false,
                'c' => opts.compress_blanks = true,
                'g' => opts.german_sort = true,
                's' | 'o' | 't' | 'p' => {
                    if chars.peek().is_some() {
                        bail!("Option -{} requires a separate argument.", flag);
                    }
                    let value = iter
                        .next()
                        .ok_or_else(|| anyhow!("Expected -{} <value>", flag))?;
                    match flag {
                        's' => opts.style_file = Some(value.clone()),
                        'o' => opts.output_file = Some(value.clone()),
                        't' => opts.transcript_file = Some(value.clone()),
                        'p' => {
                            let normalized = value.to_ascii_lowercase();
                            let page = match normalized.as_str() {
                                "even" => InitialPage::NextEven,
                                "odd" => InitialPage::NextOdd,
                                "any" => InitialPage::NextAny,
                                _ => InitialPage::Literal(value.clone()),
                            };
                            opts.initial_page = Some(page);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => bail!("Unknown option -{}.", flag),
            }
        }
    }

    Ok(opts)
}
