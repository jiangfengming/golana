use crate::config::Provider;
use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::read_keypair_file;
use anchor_client::{Client, Cluster, Program};
use anyhow::Result;
use std::{rc::Rc, str::FromStr};

/// new_vm_program Creates a Program with info from provider
pub fn new_vm_program(provider: &Provider) -> Result<Program> {
    let cluster = Cluster::from_str(&provider.cluster)?;
    let payer =
        read_keypair_file(&*shellexpand::tilde(&provider.wallet)).expect("Bad keypair file");
    let client = Client::new_with_options(cluster, Rc::new(payer), CommitmentConfig::confirmed());
    let program = client.program(Pubkey::from_str(&provider.golana_id)?);
    Ok(program)
}
