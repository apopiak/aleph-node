// TEMP allow as everything gets plugged into each other.
// TODO: Remove before we do a release to ensure there is no hanging code.
#![allow(dead_code)]
#![allow(clippy::type_complexity)]
use sp_core::traits::BareCryptoStorePtr;

use codec::{Decode, Encode};
use rush::{nodes::NodeIndex, HashT, Unit};
use std::fmt::Debug;

use sp_consensus::BlockImport;

use sc_client_api::{
    backend::{AuxStore, Backend},
    BlockchainEvents, ExecutorProvider, Finalizer, LockImportRun, TransactionFor,
};
use sp_api::ProvideRuntimeApi;
use sp_runtime::traits::Block as BlockT;

pub(crate) mod communication;
pub mod config;
pub(crate) mod environment;
pub mod hash;

mod key_types {
    use sp_runtime::KeyTypeId;

    pub const ALEPH: KeyTypeId = KeyTypeId(*b"alph");
}

mod app {
    use crate::key_types::ALEPH;
    use sp_application_crypto::{app_crypto, ed25519};
    app_crypto!(ed25519, ALEPH);
}

pub type AuthorityId = app::Public;

pub type AuthoritySignature = app::Signature;

pub type AuthorityPair = app::Pair;

#[derive(Clone, Debug, Eq, Hash, Encode, Decode, PartialEq)]
pub struct NodeId {
    auth: AuthorityId,
    index: NodeIndex,
}

impl rush::MyIndex for NodeId {
    fn my_index(&self) -> Option<NodeIndex> {
        unimplemented!()
    }
}

/// Ties an authority identification and a cryptography keystore together for use in
/// signing that requires an authority.
pub struct AuthorityCryptoStore {
    authority_id: AuthorityId,
    crypto_store: BareCryptoStorePtr,
}

impl AuthorityCryptoStore {
    /// Constructs a new authority cryptography keystore.
    pub fn new(authority_id: AuthorityId, crypto_store: BareCryptoStorePtr) -> Self {
        AuthorityCryptoStore {
            authority_id,
            crypto_store,
        }
    }

    /// Returns a references to the authority id.
    pub fn authority_id(&self) -> &AuthorityId {
        &self.authority_id
    }

    /// Returns a reference to the cryptography keystore.
    pub fn crypto_store(&self) -> &BareCryptoStorePtr {
        &self.crypto_store
    }
}

impl AsRef<BareCryptoStorePtr> for AuthorityCryptoStore {
    fn as_ref(&self) -> &BareCryptoStorePtr {
        self.crypto_store()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
pub struct UnitCoord {
    pub creator: NodeIndex,
    pub round: u64,
}

impl<B: HashT, H: HashT> From<Unit<B, H>> for UnitCoord {
    fn from(unit: Unit<B, H>) -> Self {
        UnitCoord {
            creator: unit.creator(),
            round: unit.round() as u64,
        }
    }
}

impl<B: HashT, H: HashT> From<&Unit<B, H>> for UnitCoord {
    fn from(unit: &Unit<B, H>) -> Self {
        UnitCoord {
            creator: unit.creator(),
            round: unit.round() as u64,
        }
    }
}

pub trait ClientForAleph<B, BE>:
    LockImportRun<B, BE>
    + Finalizer<B, BE>
    + AuxStore
    + BlockchainEvents<B>
    + ProvideRuntimeApi<B>
    + ExecutorProvider<B>
    + BlockImport<B, Transaction = TransactionFor<BE, B>, Error = sp_consensus::Error>
where
    BE: Backend<B>,
    B: BlockT,
{
}

impl<B, BE, T> ClientForAleph<B, BE> for T
where
    BE: Backend<B>,
    B: BlockT,
    T: LockImportRun<B, BE>
        + Finalizer<B, BE>
        + AuxStore
        + BlockchainEvents<B>
        + ProvideRuntimeApi<B>
        + ExecutorProvider<B>
        + BlockImport<B, Transaction = TransactionFor<BE, B>, Error = sp_consensus::Error>,
{
}
