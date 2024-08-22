use alloy::primitives::Uint;
use derive_more::{Deref, DerefMut};
use hex::{self, FromHexError};
use rand::{Rng, RngCore};
use std::{fmt::Debug, hash::Hash};

#[derive(Deref, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BridgeTransferId<H>(pub H);

impl<H> BridgeTransferId<H> {
	pub fn inner(&self) -> &H {
		&self.0
	}
}

impl BridgeTransferId<[u8; 32]> {
	pub fn parse(s: &str) -> Result<Self, FromHexError> {
		let bytes = hex::decode(s)?;
		let array: [u8; 32] =
			bytes.as_slice().try_into().map_err(|_| FromHexError::InvalidStringLength)?;
		Ok(BridgeTransferId(array))
	}
}

impl<H, O> Convert<BridgeTransferId<O>> for BridgeTransferId<H>
where
	H: Convert<O>,
{
	fn convert(me: &BridgeTransferId<H>) -> BridgeTransferId<O> {
		BridgeTransferId(Convert::convert(&me.0))
	}
}

impl<H> From<H> for BridgeTransferId<H> {
	fn from(hash: H) -> Self {
		BridgeTransferId(hash)
	}
}

pub fn convert_bridge_transfer_id<H: From<O>, O>(
	other: BridgeTransferId<O>,
) -> BridgeTransferId<H> {
	BridgeTransferId(From::from(other.0))
}

impl<H> GenUniqueHash for BridgeTransferId<H>
where
	H: GenUniqueHash,
{
	fn gen_unique_hash<R: Rng>(rng: &mut R) -> Self {
		BridgeTransferId(H::gen_unique_hash(rng))
	}
}

#[derive(Deref, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitiatorAddress<A>(pub A);

impl From<&str> for InitiatorAddress<Vec<u8>> {
	fn from(value: &str) -> Self {
		Self(value.as_bytes().to_vec())
	}
}

impl From<String> for InitiatorAddress<Vec<u8>> {
	fn from(value: String) -> Self {
		Self(value.as_bytes().to_vec())
	}
}

#[derive(Deref, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipientAddress<A>(pub A);

impl From<&str> for RecipientAddress<Vec<u8>> {
	fn from(value: &str) -> Self {
		RecipientAddress(value.as_bytes().to_vec())
	}
}

#[derive(Deref, Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecipientAddressCounterparty<A>(pub A);

#[derive(Deref, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InitiatorAddressCounterParty(pub Vec<u8>);

#[derive(Deref, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HashLock<H>(pub H);

impl<H> HashLock<H> {
	pub fn inner(&self) -> &H {
		&self.0
	}
}

impl HashLock<[u8; 32]> {
	pub fn parse(s: &str) -> Result<Self, FromHexError> {
		let bytes = hex::decode(s)?;
		let array: [u8; 32] =
			bytes.as_slice().try_into().map_err(|_| FromHexError::InvalidStringLength)?;
		Ok(HashLock(array))
	}
}

pub fn convert_hash_lock<H: From<O>, O>(other: HashLock<O>) -> HashLock<H> {
	HashLock(From::from(other.0))
}

#[derive(Deref, Debug, Clone, PartialEq, Eq)]
pub struct HashLockPreImage(pub Vec<u8>);

impl AsRef<[u8]> for HashLockPreImage {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl HashLockPreImage {
	/// Generate a cryptographically secure random secret
	pub fn random() -> Self {
		let mut rng = rand::thread_rng();
		let mut secret = vec![0u8; 32];
		rng.fill_bytes(&mut secret);
		HashLockPreImage(secret)
	}
}

#[derive(Deref, Debug, Clone, PartialEq, Eq)]
pub struct TimeLock(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum BridgedToken {
	Weth(u64),
	Eth(u64),
	WethAndEth((u64, u64)),
}
impl From<Uint<256, 4>> for TimeLock {
	fn from(value: Uint<256, 4>) -> Self {
		// Extract the lower 64 bits.
		let lower_64_bits = value.as_limbs()[0];
		TimeLock(lower_64_bits)
	}
}

#[derive(Deref, DerefMut, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Amount<V>(pub V);
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EthValue {
	Weth(u64),
	Eth(u64),
	WethAndEth((u64, u64)),
}

impl Amount<EthValue> {
    pub fn weth(&self) -> u64 {
        match self.0 {
            EthValue::Weth(value) => value,
            EthValue::WethAndEth((weth_value, eth_value)) => weth_value,
			_ => 0, 
        }
	}
	pub fn eth(&self) -> u64 {
		match self.0 {
			EthValue::Eth(value) => value,
			EthValue::WethAndEth((weth_value, eth_value)) => eth_value,
			_ => 0, 
		}
	}
}

impl From<Uint<256, 4>> for Amount<EthValue> {
	fn from(value: Uint<256, 4>) -> Self {
		// Extract the lower 64 bits.
		let lower_64_bits: u64 = value.as_limbs()[0];
		//TODO: unit test this for Weth and Eth
		Amount(EthValue::Eth(lower_64_bits))
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BridgeTransferDetails<A, H, V> {
	pub bridge_transfer_id: BridgeTransferId<H>,
	pub initiator_address: InitiatorAddress<A>,
	pub recipient_address: RecipientAddress<Vec<u8>>,
	pub hash_lock: HashLock<H>,
	pub time_lock: TimeLock,
	pub amount: Amount<V>,
}

impl<A, H, V> Default for BridgeTransferDetails<A, H, V> {
	fn default() -> Self {
		todo!()
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LockDetails<A, H, V> {
	pub bridge_transfer_id: BridgeTransferId<H>,
	pub initiator_address: InitiatorAddress<Vec<u8>>,
	pub recipient_address: RecipientAddress<A>,
	pub hash_lock: HashLock<H>,
	pub time_lock: TimeLock,
	pub amount: Amount<V>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CounterpartyCompletedDetails<A, H, V> {
	pub bridge_transfer_id: BridgeTransferId<H>,
	pub initiator_address: InitiatorAddress<Vec<u8>>,
	pub recipient_address: RecipientAddress<A>,
	pub hash_lock: HashLock<H>,
	pub secret: HashLockPreImage,
	pub amount: Amount<V>,
}

impl<A, H, V> CounterpartyCompletedDetails<A, H, V>
where
	InitiatorAddress<Vec<u8>>: From<InitiatorAddress<A>>,
	RecipientAddress<A>: From<RecipientAddress<Vec<u8>>>,
{
	pub fn from_bridge_transfer_details(
		bridge_transfer_details: BridgeTransferDetails<A, H, V>,
		secret: HashLockPreImage,
	) -> Self {
		CounterpartyCompletedDetails {
			bridge_transfer_id: bridge_transfer_details.bridge_transfer_id,
			initiator_address: From::from(bridge_transfer_details.initiator_address),
			recipient_address: From::from(bridge_transfer_details.recipient_address),
			hash_lock: bridge_transfer_details.hash_lock,
			secret,
			amount: bridge_transfer_details.amount,
		}
	}
}

impl<A, H, V> CounterpartyCompletedDetails<A, H, V> {
	pub fn from_lock_details(lock_details: LockDetails<A, H, V>, secret: HashLockPreImage) -> Self {
		CounterpartyCompletedDetails {
			bridge_transfer_id: lock_details.bridge_transfer_id,
			initiator_address: lock_details.initiator_address,
			recipient_address: lock_details.recipient_address,
			hash_lock: lock_details.hash_lock,
			secret,
			amount: lock_details.amount,
		}
	}
}

// Types
pub trait BridgeHashType: Debug + PartialEq + Eq + Hash + Unpin + Send + Sync + Clone {}
pub trait BridgeAddressType:
	Debug + PartialEq + Eq + Hash + Unpin + Send + Sync + Clone + From<Vec<u8>>
{
}
pub trait BridgeValueType: Debug + PartialEq + Eq + Clone + Send + Sync + Unpin {}

pub trait Convert<O> {
	fn convert(other: &Self) -> O;
}

// Blankets
impl<T> BridgeHashType for T where T: Debug + PartialEq + Eq + Hash + Unpin + Send + Sync + Clone {}
impl<T> BridgeAddressType for T where
	T: Debug + PartialEq + Eq + Hash + Unpin + Send + Sync + Clone + From<Vec<u8>>
{
}

pub trait GenUniqueHash {
	fn gen_unique_hash<R: Rng>(rng: &mut R) -> Self;
}
