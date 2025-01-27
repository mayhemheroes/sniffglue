use structopt::StructOpt;
use structopt::clap::{AppSettings, Shell};

#[derive(Debug, StructOpt)]
#[structopt(global_settings = &[AppSettings::ColoredHelp])]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Set device to promiscuous mode
    #[structopt(short="p", long="promisc")]
    pub promisc: bool,
    /// Show fully dissected packets with all headers for development
    #[structopt(long="debugging")]
    pub debugging: bool,
    /// Json output (unstable)
    #[structopt(short="j", long="json")]
    pub json: bool,
    #[structopt(short="v", long="verbose",
                parse(from_occurrences),
                help="Increase filter sensitivity to show more (possibly less useful) packets.
The default only shows few packets, this flag can be specified multiple times. (maximum: 4)")]
    pub verbose: u8,
    /// Open a pcap file instead of a device
    #[structopt(short="r", long="read")]
    pub read: bool,
    // --cpus is a legacy alias and going to be removed in the future
    /// Number of packet parsing threads (defaults to number of cpu cores)
    #[structopt(short="n", long="threads", alias="cpus")]
    pub threads: Option<usize>,
    /// Disable syscall filter sandbox,
    /// this flag disables security features in sniffglue,
    /// please file a bug report if you need this option
    #[structopt(long)]
    pub insecure_disable_seccomp: bool,
    /// Generate shell completions
    #[structopt(long, hidden=true)]
    pub gen_completions: Option<Shell>,
    /// The device or file to read packets from
    pub device: Option<String>,
}
