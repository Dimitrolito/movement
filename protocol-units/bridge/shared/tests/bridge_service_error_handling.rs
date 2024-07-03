use futures::StreamExt;
use test_log::test;

use bridge_shared::{
	bridge_contracts::{
		BridgeContractCounterparty, BridgeContractCounterpartyError, BridgeContractInitiator,
	},
	types::{
		Amount, BridgeTransferId, HashLock, HashLockPreImage, InitiatorAddress, RecipientAddress,
		TimeLock,
	},
};

mod shared;

use crate::shared::{
	setup_bridge_service, testing::blockchain::client::MethodName, B2Client, BC1Address, BC1Hash,
	BC2Hash, SetupBridgeServiceResult,
};

use self::shared::testing::blockchain::client::{CallConfig, ErrorConfig};

#[test(tokio::test(flavor = "multi_thread", worker_threads = 4))]
async fn test_bridge_service_error_handling() {
	let SetupBridgeServiceResult(
		mut bridge_service,
		mut blockchain_1_client,
		mut blockchain_2_client,
		blockchain_1,
		blockchain_2,
	) = setup_bridge_service();

	tokio::spawn(blockchain_1);
	tokio::spawn(blockchain_2);

	// Lets make the blockchain_2_client fail on the locking of assets
	blockchain_2_client.set_call_config(
		MethodName::LockBridgeTransferAssets,
		1,
		CallConfig {
			error: ErrorConfig::CounterpartyError(
				BridgeContractCounterpartyError::LockTransferAssetsError,
			),
			delay: None,
		},
	);

	// Step 1: Initiating the swap on Blockchain 1 with an invalid hash lock

	tracing::debug!("Initiating bridge transfer with invalid hash lock");
	blockchain_1_client
		.initiate_bridge_transfer(
			InitiatorAddress(BC1Address("initiator")),
			RecipientAddress::from(BC1Address("recipient")),
			HashLock(BC1Hash::from("invalid_hash_lock")),
			TimeLock(100),
			Amount(1000),
		)
		.await
		.expect("initiate_bridge_transfer failed");

	// B1I Initiated
	let event = bridge_service.next().await.expect("No event");
	tracing::debug!(?event);

	// B2C Locked
	let event = bridge_service.next().await.expect("No event");
	tracing::debug!(?event);

	// Step 2: Attempting to complete the swap on Blockchain 2 with an invalid secret

	tracing::debug!("Attempting to complete bridge transfer with invalid secret");
	<B2Client as BridgeContractCounterparty>::complete_bridge_transfer(
		&mut blockchain_2_client,
		BridgeTransferId(BC2Hash::from("non_existent_transfer_id")),
		HashLockPreImage(b"invalid_secret".to_vec()),
	)
	.await
	.expect("complete_bridge_transfer failed");

	// The team has decided not to monitor for incorrect secret errors at this time.

	// let event = bridge_service.next().await.expect("No event");
	// tracing::debug!(?event);
}
