// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2025 Björn Busse <bj.rn@baerlin.eu>
use std::io::{self, Read};
use rss::Channel;
use clap::{Parser, ValueEnum};
use html2text::from_read;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// Output format (text, json, xml)
	#[arg(long, default_value = "text")]
	output_format: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
	Text,
	Json,
	Xml,
}

fn main() {
	let cli = Cli::parse();

	// Read all input from stdin
	let mut buffer = String::new();
	io::stdin().read_to_string(&mut buffer).expect("Failed to read from stdin");

	// Check if input is HTML (simple heuristic: starts with <html or <!DOCTYPE html)
	let is_html = buffer.trim_start().to_lowercase().starts_with("<html") || buffer.trim_start().to_lowercase().starts_with("<!doctype html");

	if is_html {
		// If HTML, use html2text to convert to text and print
		let text = from_read(buffer.as_bytes(), 80);
		println!("{}", text);
		return;
	}

	// Try to parse as RSS
	match Channel::read_from(buffer.as_bytes()) {
		Ok(channel) => {
			match cli.output_format {
				OutputFormat::Text => {
					println!("Feed: {}", channel.title());
					println!("Description: {}", channel.description());
					println!("Link: {}", channel.link());
					println!("Items:");
					for item in channel.items() {
						println!("- Title: {}", item.title().unwrap_or("(no title)"));
						println!("  Link: {}", item.link().unwrap_or("(no link)"));
						println!("  Description: {}", item.description().unwrap_or("(no description)"));
						println!("");
					}
				}
				OutputFormat::Json => {
					// Simple JSON output
					let items: Vec<_> = channel.items().iter().map(|item| {
						serde_json::json!({
							"title": item.title(),
							"link": item.link(),
							"description": item.description(),
						})
					}).collect();
					let out = serde_json::json!({
						"feed": channel.title(),
						"description": channel.description(),
						"link": channel.link(),
						"items": items
					});
					println!("{}", serde_json::to_string_pretty(&out).unwrap());
				}
				OutputFormat::Xml => {
					// Output original XML
					println!("{}", buffer);
				}
			}
		}
		Err(_) => {
			// If not RSS and not HTML, just print the input as plain text
			let text = from_read(buffer.as_bytes(), 80);
			println!("{}", text);
		}
	}
}
