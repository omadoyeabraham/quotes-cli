use anyhow::Result;
use clap::Parser;
use cli_table::{print_stdout, Color, Table, WithTitle};
use serde::Deserialize;

const MAX_NUMBER_OF_QUOTES: u32 = 10;

/// Display amazing quotes on the command line.
#[derive(Parser, Debug)]
struct Cli {
    /// The number of quotes to be displayed
    number_of_quotes: u32,
}

#[derive(Debug, Deserialize)]
struct QuotesApiResponse {
    q: String,
    a: String,
    #[allow(dead_code)]
    c: String,
    h: String,
}

/// An inspirational quote
#[derive(Table)]
struct Quote {
    /// The ID of the quote
    #[table(title = "S/N")]
    id: u32,
    /// The content quote
    #[table(title = "Quote", color = "Color::Green")]
    content: String,
    /// The author of the quote
    #[table(title = "Author")]
    author: String,
    /// The HTML representation of the quote
    #[table(skip)]
    #[allow(dead_code)]
    content_html: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let quotes = get_quotes(args.number_of_quotes)?;

    print_stdout(quotes.with_title())?;
    Ok(())
}

fn get_quotes(number_of_quotes: u32) -> Result<Vec<Quote>, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://zenquotes.io/api/quotes/")?
        .json::<Vec<QuotesApiResponse>>()?;

    let quotes: Vec<Quote> = response
        .iter()
        .enumerate()
        .take(std::cmp::min(
            number_of_quotes as usize,
            MAX_NUMBER_OF_QUOTES as usize,
        ))
        .map(|(index, value)| Quote {
            id: (index + 1) as u32,
            content: value.q.clone(),
            author: value.a.clone(),
            content_html: value.h.clone(),
        })
        .collect();

    return Ok(quotes);
}
