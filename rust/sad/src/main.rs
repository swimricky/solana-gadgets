//! @brief Main entry poiint for CLI

use {
    desertree::Deseriaizer,
    gadgets_common::load_yaml_file,
    sadout::{SadCsvOutput, SadExcelOutput, SadOutput, SadSysOutput},
    solana_clap_utils::{input_validators::normalize_to_url_if_moniker, keypair::DefaultSigner},
    solana_client::rpc_client::RpcClient,
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Signer},
    },
    std::path::Path,
    std::str::FromStr,
    std::{process::exit, sync::Arc},
};

/// sad main module
mod clparse;
mod desertree;
mod errors;
mod sadout;
mod sadtypes;
mod solq;

struct Config {
    commitment_config: CommitmentConfig,
    default_signer: Box<dyn Signer>,
    json_rpc_url: String,
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = clparse::parse_command_line();
    let (sub_command, sub_matches) = app_matches.subcommand();
    let matches = sub_matches.unwrap();
    let mut wallet_manager: Option<Arc<RemoteWalletManager>> = None;
    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_signer =
            DefaultSigner::new("keypair".to_string(), cli_config.keypair_path.clone());

        Config {
            json_rpc_url: normalize_to_url_if_moniker(
                matches
                    .value_of("json_rpc_url")
                    .unwrap_or(&cli_config.json_rpc_url)
                    .to_string(),
            ),
            default_signer: default_signer
                .signer_from_path(matches, &mut wallet_manager)
                .unwrap_or_else(|err| {
                    eprintln!("error: {}", err);
                    exit(1);
                }),
            verbose: matches.is_present("verbose"),
            commitment_config: CommitmentConfig::confirmed(),
        }
    };
    // Change to "solana=debug" if needed
    solana_logger::setup_with_default("solana=info");

    if config.verbose {
        println!("JSON RPC URL: {}", config.json_rpc_url);
    }
    let rpc_client = RpcClient::new(config.json_rpc_url.clone());
    // Get the deserialization descriptor
    let indecl = if let Some(ind) = matches.value_of("decl") {
        load_yaml_file(ind).unwrap_or_else(|err| {
            eprintln!("File error: On {} {}", ind, err);
            exit(1)
        })
    } else {
        eprintln!("Requires -d or --declfile argument");
        exit(1);
    };

    // Setup the account or program public key
    let target_pubkey = if matches.is_present("pkstr") {
        Pubkey::from_str(matches.value_of("pkstr").unwrap())?
    } else {
        let kp = read_keypair_file(Path::new(matches.value_of("keypair").unwrap()))?;
        kp.pubkey()
    };

    // Setup the deserialization tree
    let destree = Deseriaizer::new(&indecl[0]);
    let deserialize_result = match (sub_command, sub_matches) {
        ("account", Some(_)) => solq::deserialize_account(&rpc_client, &target_pubkey, &destree)?,
        ("program", Some(_)) => {
            solq::deserialize_program_accounts(&rpc_client, &target_pubkey, &destree)?
        }
        _ => unreachable!(),
    };
    // Check for output or default to pretty print
    if matches.is_present("output") {
        match matches.value_of("output").unwrap() {
            "excel" => {
                SadExcelOutput::new(deserialize_result, matches.value_of("filename").unwrap())
                    .write()
            }
            "csv" => {
                SadCsvOutput::new(deserialize_result, matches.value_of("filename").unwrap()).write()
            }
            "stdout" => {
                if matches.is_present("filename") {
                    println!(
                        "'--filename {}' argument ignored for screen output",
                        matches.value_of("filename").unwrap()
                    );
                }
                SadSysOutput::new(deserialize_result).write()
            }
            _ => unreachable!(),
        }
    }

    Ok(())
}
