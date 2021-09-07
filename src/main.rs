#![allow(rustdoc::private_intra_doc_links)]
#![deny(
    // Documentation
	// TODO: rustdoc::broken_intra_doc_links,
	// TODO: rustdoc::missing_crate_level_docs,
	// TODO: missing_docs,
	// TODO: clippy::missing_docs_in_private_items,

    // Other
	deprecated_in_future,
	exported_private_dependencies,
	future_incompatible,
	missing_copy_implementations,
	missing_debug_implementations,
	private_in_public,
	rust_2018_compatibility,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unstable_features,
	unused_import_braces,
	unused_qualifications,

	// clippy attributes
	clippy::missing_const_for_fn,
	clippy::redundant_pub_crate,
	clippy::use_self
)]
#![cfg_attr(docsrs, feature(doc_cfg), feature(doc_alias))]

mod task;

use std::fmt;
use std::path::PathBuf;

use clap::{crate_authors, crate_description, crate_version, Clap};
use task::TaskIter;
use tdtxt::{Component, Priority, Task};
use yansi::{Color, Paint, Style};

#[derive(Debug, Clone, Clap)]
#[clap(name = "tdtxt", version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Debug, Clone, Clap)]
pub enum Command {
	List(List),
}

#[derive(Debug, Clone, Clap)]
pub struct List {
	file: PathBuf,
}

fn main() -> anyhow::Result<()> {
	let opts = Opts::parse();

	match opts.command {
		Command::List(List { file }) => {
			let iter = TaskIter::new(&file).unwrap();

			for (line_nr, task) in iter {
				let line_nr_str =
					format!("{:05}", line_nr + 1).replace('0', " ");

				let task_str = match task {
					Ok(task) => task_fmt(&task),
					Err(err) => err.to_string(),
				};

				println!("{}# {}", line_nr_str, task_str);
			}
		}
	}

	Ok(())
}

fn err_fmt(err: impl fmt::Display) -> String {
	Paint::red(err).to_string()
}

fn task_fmt(task: &Task) -> String {
	let mut out = String::new();

	if task.state().is_done() {
		out.push_str(&format!("{} ", task.state()));
	}

	if let Some(priority) = task.priority() {
		let color = if priority >= &Priority::H {
			Color::Red
		} else if priority >= &Priority::P {
			Color::Yellow
		} else {
			Color::Green
		};

		out.push_str(&Paint::new(priority.as_char()).fg(color).to_string());
		out.push(' ');
	}

	if let Some(compound) = task.date_compound() {
		out.push_str(&format!("{} ", compound));
	}

	if task.state().is_done() {
		out.push_str(&task.description().to_string());
		Paint::white(out).dimmed().wrap().to_string()
	} else {
		let mut color: Option<Color> = None;

		for component in task.description().components() {
			match component {
				Component::Text(txt) => out.push_str(txt),
				Component::Project(s) | Component::Context(s) => {
					out.push_str(&Paint::new(s).underline().bold().to_string())
				}
				Component::Custom { key, separator, value } => match key {
					"color" => {
						color = parse_color(value);
					}
					_ => out.push_str(
						&Paint::new(format!("{}{}{}", key, separator, value))
							.underline()
							.bold()
							.to_string(),
					),
				},
			};
		}

		if let Some(color) = color {
			Paint::new(out).fg(color).wrap().to_string()
		} else {
			out
		}
	}
}

fn parse_color(s: &str) -> Option<Color> {
	let color = match s.to_lowercase().as_str() {
		"black" => Color::Black,
		"red" => Color::Red,
		"green" => Color::Green,
		"yellow" => Color::Yellow,
		"blue" => Color::Blue,
		"magenta" => Color::Magenta,
		"cyan" => Color::Cyan,
		"white" => Color::White,
		_ => return None,
	};

	Some(color)
}
