use clap::{Parser, Subcommand};

/// StaxPing — Clean, unified network diagnostics CLI
#[derive(Parser, Debug)]
#[command(name = "staxping")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output as JSON instead of pretty-printed text
    #[arg(short, long, global = true)]
    pub json: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Perform a DNS lookup for a hostname
    Dns {
        /// Hostname to resolve
        host: String,
    },
    /// Send an ICMP echo request (ping) to a host
    Icmp {
        /// Target host to ping
        host: String,
    },
    /// Send an HTTP request and report status
    Http {
        /// URL to probe
        url: String,
    },
    /// Run a traceroute to a host
    Trace {
        /// Destination host for traceroute
        host: String,
    },
    /// Show version information
    Version,
    /// Manage diagnostic tools
    #[command(subcommand)]
    Tools(ToolsCommands),
}

#[derive(Subcommand, Debug)]
pub enum ToolsCommands {
    /// List available diagnostic tools
    List,
    /// Install a specific diagnostic tool
    Install {
        /// Name of the tool to install
        tool: String,
    },
}
