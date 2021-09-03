use clap::{
	crate_authors, crate_description, crate_version, Clap,
};

#[derive(Clap)]
#[clap(name = "tdtxt", version = crate_version!(), author = crate_authors!(), about = crate_description!())]
pub struct Opts {
	#[clap(subcommand)]
	command: Command,
}

#[derive(Clap)]
pub enum Command {
	Add(Add),
}

#[derive(Clap)]
pub struct Add {}
