// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

mod cli;

use std::sync::Arc;

use cli::{AuthoringPolicy, RelayChainCli, Subcommand, TestCollatorCli};
use cumulus_primitives_core::relay_chain::CollatorPair;
use cumulus_test_service::{chain_spec, new_partial, AnnounceBlockFn};
use polkadot_service::IdentifyNetworkBackend;
use sc_cli::{CliConfiguration, SubstrateCli};
use sp_core::Pair;

pub fn wrap_announce_block() -> Box<dyn FnOnce(AnnounceBlockFn) -> AnnounceBlockFn> {
	tracing::info!("Block announcements disabled.");
	Box::new(|_| {
		// Never announce any block
		Arc::new(|_, _| {})
	})
}

fn main() -> Result<(), sc_cli::Error> {
	let cli = TestCollatorCli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},

		Some(Subcommand::ExportGenesisHead(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|mut config| {
				let partial = new_partial(&mut config, false)?;
				cmd.run(partial.client)
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(&*config.chain_spec))
		},
		None => {
			let log_filters = cli.run.normalize().log_filters();
			let mut builder = sc_cli::LoggerBuilder::new(log_filters.unwrap_or_default());
			builder.with_colors(false);
			let _ = builder.init();

			let collator_options = cli.run.collator_options();
			let tokio_runtime = sc_cli::build_runtime()?;
			let tokio_handle = tokio_runtime.handle();
			let parachain_config = cli
				.run
				.normalize()
				.create_configuration(&cli, tokio_handle.clone())
				.expect("Should be able to generate config");

			let relay_chain_cli = RelayChainCli::new(
				&parachain_config,
				[RelayChainCli::executable_name()].iter().chain(cli.relaychain_args.iter()),
			);
			let tokio_handle = parachain_config.tokio_handle.clone();
			let relay_chain_config = SubstrateCli::create_configuration(
				&relay_chain_cli,
				&relay_chain_cli,
				tokio_handle,
			)
			.map_err(|err| format!("Relay chain argument error: {}", err))?;

			let parachain_id = chain_spec::Extensions::try_get(&*parachain_config.chain_spec)
				.map(|e| e.para_id)
				.ok_or("Could not find parachain extension in chain-spec.")?;

			tracing::info!("Parachain id: {:?}", parachain_id);
			tracing::info!(
				"Is collating: {}",
				if parachain_config.role.is_authority() { "yes" } else { "no" }
			);
			if cli.fail_pov_recovery {
				tracing::info!("PoV recovery failure enabled");
			}

			let collator_key =
				parachain_config.role.is_authority().then(|| CollatorPair::generate().0);

			let consensus = cli
				.use_null_consensus
				.then(|| {
					tracing::info!("Using null consensus.");
					cumulus_test_service::Consensus::Null
				})
				.unwrap_or(cumulus_test_service::Consensus::Aura);

			// If the network backend is unspecified, use the default for the given chain.
			let default_backend = relay_chain_config.chain_spec.network_backend();
			let network_backend =
				relay_chain_config.network.network_backend.unwrap_or(default_backend);
			let use_slot_based_collator = cli.authoring == AuthoringPolicy::SlotBased;
			let (mut task_manager, _, _, _, _, _) = tokio_runtime
				.block_on(async move {
					match network_backend {
						sc_network::config::NetworkBackendType::Libp2p =>
							cumulus_test_service::start_node_impl::<
								_,
								sc_network::NetworkWorker<_, _>,
							>(
								parachain_config,
								collator_key,
								relay_chain_config,
								parachain_id.into(),
								cli.disable_block_announcements.then(wrap_announce_block),
								cli.fail_pov_recovery,
								|_| Ok(jsonrpsee::RpcModule::new(())),
								consensus,
								collator_options,
								true,
								use_slot_based_collator,
							)
							.await,
						sc_network::config::NetworkBackendType::Litep2p =>
							cumulus_test_service::start_node_impl::<
								_,
								sc_network::Litep2pNetworkBackend,
							>(
								parachain_config,
								collator_key,
								relay_chain_config,
								parachain_id.into(),
								cli.disable_block_announcements.then(wrap_announce_block),
								cli.fail_pov_recovery,
								|_| Ok(jsonrpsee::RpcModule::new(())),
								consensus,
								collator_options,
								true,
								use_slot_based_collator,
							)
							.await,
					}
				})
				.expect("could not create Cumulus test service");

			tokio_runtime
				.block_on(task_manager.future())
				.expect("Could not run service to completion");
			Ok(())
		},
	}
}
