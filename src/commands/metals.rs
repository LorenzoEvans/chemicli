use clap::{Args, Subcommand};
use serde_json::{Map, Value};
use crate::utilities::utils::create_periodic_table;

#[derive(Debug, Clone, Subcommand)]
pub enum MetalsSubcommands {
    #[clap(alias = "alki")]
    Alkali(MetalArgs),
    #[clap(alias = "alkn")]
    Alkaline(MetalArgs),
    #[clap(alias = "lan", alias = "lanthanides")]
    Lanthanoids(MetalArgs),
    #[clap(alias = "act", alias = "actinides")]
    Actinoids(MetalArgs),
    #[clap(alias = "trans")]
    Transition(MetalArgs),
    #[clap(alias = "post")]
    PostTransition(MetalArgs),
}

#[derive(Debug, Clone, Args)]
pub struct MetalArgs {
    #[clap(short, long)]
    pub members: bool,
    #[clap(short, long)]
    pub number: bool,
}

pub fn exec(subcommand: &MetalsSubcommands) {
    let periodic_table = create_periodic_table();
    let periodic_map = periodic_table.as_object().unwrap();

    match subcommand {
        MetalsSubcommands::Alkali(args) => {
            handle_metal_query("Alkali Metal", "Alkali Metal", args, periodic_map);
        }
        MetalsSubcommands::Alkaline(args) => {
            handle_metal_query("Alkaline Earth Metal", "Alkaline Earth Metal", args, periodic_map);
        }
        MetalsSubcommands::Lanthanoids(args) => {
            handle_metal_query("Lanthanide", "Lanthanide", args, periodic_map);
        }
        MetalsSubcommands::Actinoids(args) => {
            handle_metal_query("Actinide", "Actinide", args, periodic_map);
        }
        MetalsSubcommands::Transition(args) => {
            handle_metal_query("Transition Metal", "Transition Metal", args, periodic_map);
        }
        MetalsSubcommands::PostTransition(args) => {
            handle_metal_query("Post-transition Metal", "Post-transition Metal", args, periodic_map);
        }
    }
}

fn handle_metal_query(series_name: &str, display_name: &str, args: &MetalArgs, periodic_map: &Map<String, Value>) {
    let members = get_members(series_name, periodic_map);
    
    if args.members {
        if members.is_empty() {
            println!("No members found for the {} group in the current database.", display_name);
        } else {
            for (idx, ele) in members.iter().enumerate() {
                println!("Member number {} is {}.", idx + 1, ele);
            }
        }
    }

    if args.number {
        println!("The number of members in the {} group is {}.", display_name, members.len());
    }

    if !args.members && !args.number {
        println!("Please provide an option, e.g., --members or --number.");
    }
}

fn get_members(series: &str, p_table: &Map<String, Value>) -> Vec<String> {
    let mut members = Vec::new();
    for (_sym, data) in p_table.iter() {
        if let Some(s) = data["series"].as_str() {
            if s == series {
                if let Some(name) = data["element_name"].as_str() {
                    members.push(name.to_string());
                }
            }
        }
    }
    members
}
