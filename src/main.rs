mod core;
mod net;
mod output;
mod cli;
mod registry;

use clap::Parser;
use cli::args::{Cli, Commands, ToolsCommands};

fn main() {
    let cli = Cli::parse();
    let json_mode = cli.json;

    match cli.command {
        Commands::Dns { host } => {
            if json_mode {
                println!(r#"{{"command":"dns","host":"{}","status":"not implemented"}}"#, host);
            } else {
                println!("🔍 DNS lookup for: {} (not yet implemented)", host);
            }
        }
        Commands::Icmp { host } => {
            if json_mode {
                println!(r#"{{"command":"icmp","host":"{}","status":"not implemented"}}"#, host);
            } else {
                println!("📡 ICMP ping to: {} (not yet implemented)", host);
            }
        }
        Commands::Http { url } => {
            if json_mode {
                println!(r#"{{"command":"http","url":"{}","status":"not implemented"}}"#, url);
            } else {
                println!("🌐 HTTP probe: {} (not yet implemented)", url);
            }
        }
        Commands::Trace { host } => {
            if json_mode {
                println!(r#"{{"command":"trace","host":"{}","status":"not implemented"}}"#, host);
            } else {
                println!("🗺️  Traceroute to: {} (not yet implemented)", host);
            }
        }
        Commands::Version => {
            if json_mode {
                println!(r#"{{"name":"staxping","version":"2.0.0"}}"#);
            } else {
                println!("staxping v2.0.0 — Network diagnostics CLI");
            }
        }
        Commands::Tools(sub) => match sub {
            ToolsCommands::List => {
                if json_mode {
                    println!(r#"{{"tools":[]}}"#);
                } else {
                    println!("🛠️  Available tools: (none installed)");
                }
            }
            ToolsCommands::Install { tool } => {
                if json_mode {
                    println!(r#"{{"command":"tools install","tool":"{}","status":"not implemented"}}"#, tool);
                } else {
                    println!("📦 Installing tool: {} (not yet implemented)", tool);
                }
            }
        },
    }
}
