use crate::parser::ProgramInfo;
use clap::{Arg, ArgGroup, Command};
use clockwork_client::webhook::objects::HttpMethod;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Debug, PartialEq)]
pub enum CliCommand {
    // API commands
    ApiNew {
        ack_authority: Pubkey,
        base_url: String,
    },

    // Config commands
    ConfigGet,
    ConfigSet {
        admin: Option<Pubkey>,
        crank_fee: Option<u64>,
    },

    // Http
    HttpRequestNew {
        api: Pubkey,
        id: String,
        method: HttpMethod,
        route: String,
    },

    Initialize {
        mint: Pubkey,
    },

    // Localnet commands
    Localnet {
        program_infos: Vec<ProgramInfo>,
    },

    // Node commands
    NodeGet {
        worker: Pubkey,
    },
    NodeRegister {
        worker: Keypair,
    },
    NodeStake {
        address: Pubkey,
        amount: u64,
    },

    // Pool commands
    PoolGet,

    // Queue commands
    QueueGet {
        address: Pubkey,
    },
    QueueUpdate {
        address: Pubkey,
        rate_limit: Option<u64>,
    },

    // Registry
    RegistryGet,

    // Snapshot
    SnapshotGet {
        entry_id: Option<u64>,
    },
}

pub fn app() -> Command<'static> {
    Command::new("Clockwork")
        .bin_name("clockwork")
        .about("Automation infrastructure for Solana")
        .version(version!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("config")
                .about("Manage the Clockwork configs")
                .arg_required_else_help(true)
                .subcommand(Command::new("get").about("Get a config value"))
                .subcommand(
                    Command::new("set")
                        .about("Set a config value")
                        .arg(
                            Arg::new("admin")
                                .long("admin")
                                .value_name("PUBKEY")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("worker_fee")
                                .long("worker_fee")
                                .value_name("NUM_LAMPORTS")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("grace_period")
                                .long("grace_period")
                                .value_name("NUM_SECONDS")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::new("spam_penalty")
                                .long("spam_penalty")
                                .value_name("NUM_LAMPORTS")
                                .takes_value(true),
                        )
                        .group(
                            ArgGroup::new("config_settings")
                                .args(&["admin", "worker_fee", "grace_period", "spam_penalty"])
                                .multiple(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("initialize")
                .about("Initialize the Clockwork programs")
                .arg(
                    Arg::new("mint")
                        .long("mint")
                        .short('m')
                        .takes_value(true)
                        .required(true)
                        .help("Mint address of network token"),
                ),
        )
        .subcommand(
            Command::new("localnet")
                .about("Launch a local Clockwork node for development and testing")
                .arg(
                    Arg::with_name("bpf_program")
                        .long("bpf-program")
                        .value_names(&["ADDRESS_OR_KEYPAIR", "BPF_PROGRAM.SO"])
                        .takes_value(true)
                        .number_of_values(2)
                        .multiple(true)
                        .help(
                            "Add a BPF program to the genesis configuration. \
                       If the ledger already exists then this parameter is silently ignored. \
                       First argument can be a pubkey string or path to a keypair",
                        ),
                ),
        )
        .subcommand(
            Command::new("node")
                .about("Manage your nodes")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("get")
                        .about("Lookup a registered worker node")
                        .arg(
                            Arg::new("worker_address")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The worker address to lookup"),
                        ),
                )
                .subcommand(
                    Command::new("register")
                        .about("Register a new worker with the Clockwork network")
                        .arg(
                            Arg::new("worker")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("Filepath to the worker keypair"),
                        ),
                )
                .subcommand(
                    Command::new("stake")
                        .about("Stake CLOCK tokens with a Clockwork worker")
                        .arg(
                            Arg::new("address")
                                .index(2)
                                .takes_value(true)
                                .required(true)
                                .help("The node address to stake tokens with"),
                        )
                        .arg(
                            Arg::new("amount")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The number of tokens to stake"),
                        ),
                ),
        )
        .subcommand(Command::new("pool").about("Get the worker pool info"))
        .subcommand(
            Command::new("queue")
                .about("Manage your queues")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("address")
                        .index(1)
                        .takes_value(true)
                        .required(false)
                        .help("Public address of a queue"),
                )
                .subcommand(Command::new("get").about("Get a queue"))
                .subcommand(
                    Command::new("update")
                        .about("Update a property on a queue")
                        .arg(
                            Arg::new("rate_limit")
                                .long("rate_limit")
                                .short('r')
                                .takes_value(true)
                                .required(false)
                                .help("The maximum allowed cranks per slot"),
                        ),
                ),
        )
        .subcommand(Command::new("registry").about("Get the registry account"))
        .subcommand(
            Command::new("snapshot")
                .about("Lookup the current snapshot")
                .subcommand(
                    Command::new("entry")
                        .about("Lookup an entry in the snapshot")
                        .arg(
                            Arg::new("id")
                                .index(1)
                                .takes_value(true)
                                .required(true)
                                .help("The id of an entry in the snapshot"),
                        ),
                ),
        )
}
