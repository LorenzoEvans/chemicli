mod commands;
mod utilities;
use crate::commands::element::{exec as exec_element, ElementsArgs};
use crate::commands::metals::{exec as exec_metals, MetalsSubcommands};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "chemicli",
    author = "Lorenzo Evans <lorenzo.evans94@gmail.com>",
    version = "0.1.0",
    about = "A CLI for querying the periodic table of elements."
)]
pub struct Chemicli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[command(name = "element", override_usage = " chemicli.exe element <ATOMIC_SYMBOL> [OPTIONS]")]
    Element(ElementsArgs),
    #[clap(subcommand, alias = "mtl")]
    #[command(name = "metals")]
    Metals(MetalsSubcommands),
    #[command(name = "metalloids")]
    Metalloids,
    #[clap(subcommand)]
    #[command(name = "nonmetals")]
    NonMetals,
}

fn main() {
    let chemicli = Chemicli::parse();
    
    match &chemicli.commands {
        Commands::Element(args) => {
            exec_element(args);
        }
        Commands::Metals(args) => {
            exec_metals(args);
        }
        _ => todo!(),
    }
}
