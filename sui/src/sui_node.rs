// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;

use crate::{
    api::{RpcGatewayServer, SignedTransaction, TransactionBytes},
    config::{GatewayConfig, PersistedConfig},
    rpc_gateway::responses::{GetObjectInfoResponse, ObjectResponse, SuiTypeTag},
};
use anyhow::anyhow;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use move_core_types::identifier::Identifier;
use tokio::task::JoinHandle;

use sui_core::gateway_state::{
    gateway_responses::{TransactionEffectsResponse, TransactionResponse},
    GatewayTxSeqNumber, //GatewayState, GatewayClient
};
use sui_core::{
    authority::{AuthorityState, AuthorityStore},
    authority_active::ActiveAuthority,
    authority_client::NetworkAuthorityClient,
    sui_json::SuiJsonValue,
};
use sui_types::{
    base_types::{AuthorityName, ObjectID, SuiAddress, TransactionDigest},
    crypto::get_key_pair,
    error::SuiResult,
    json_schema::Base64,
    object::ObjectRead,
};

pub struct SuiNode {
    //gateway: GatewayClient
    state: Arc<AuthorityState>,
    authority_clients: BTreeMap<AuthorityName, NetworkAuthorityClient>,
    //active_authority: ActiveAuthority<NetworkAuthorityClient>,
}

impl SuiNode {
    pub async fn new(config_path: &Path) -> anyhow::Result<Self> {
        // TODO: For now, nodes just use a GatewayConfig because the only thing
        // we need to know about are the validators.
        let config: GatewayConfig = PersistedConfig::read(config_path).map_err(|e| {
            anyhow!(
                "Failed to read config file at {:?}: {}. Have you run `sui genesis` first?",
                config_path,
                e
            )
        })?;
        let committee = config.make_committee();
        let authority_clients = config.make_authority_clients();

        let dummy_key = get_key_pair();
        let name = dummy_key.1.public_key_bytes();

        let store = Arc::new(AuthorityStore::open(&config.db_folder_path, None));
        let state = AuthorityState::new_without_genesis(
            committee.clone(),
            *name,
            Arc::pin(dummy_key.1.copy()),
            store,
        )
        .await;

        let state = Arc::new(state);

        Ok(Self {
            state,
            authority_clients,
        })
    }

    pub async fn spawn(&self) -> SuiResult<Option<JoinHandle<()>>> {
        let active_authority =
            ActiveAuthority::new(self.state.clone(), self.authority_clients.clone())?;
        Ok(active_authority.spawn_all_active_processes().await)
    }
}

#[async_trait]
impl RpcGatewayServer for SuiNode {
    async fn transfer_coin(
        &self,
        _signer: SuiAddress,
        _object_id: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
        _recipient: SuiAddress,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Sui Node only supports read-only methods").into())
    }

    async fn publish(
        &self,
        _sender: SuiAddress,
        _compiled_modules: Vec<Base64>,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Sui Node only supports read-only methods").into())
    }

    async fn split_coin(
        &self,
        _signer: SuiAddress,
        _coin_object_id: ObjectID,
        _split_amounts: Vec<u64>,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Sui Node only supports read-only methods").into())
    }

    async fn merge_coin(
        &self,
        _signer: SuiAddress,
        _primary_coin: ObjectID,
        _coin_to_merge: ObjectID,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Sui Node only supports read-only methods").into())
    }

    async fn get_owned_objects(&self, _owner: SuiAddress) -> RpcResult<ObjectResponse> {
        todo!()
    }

    async fn get_object_info(&self, _object_id: ObjectID) -> RpcResult<ObjectRead> {
        todo!()
    }

    async fn get_object_typed_info(
        &self,
        _object_id: ObjectID,
    ) -> RpcResult<GetObjectInfoResponse> {
        todo!()
    }

    async fn execute_transaction(
        &self,
        _signed_tx: SignedTransaction,
    ) -> RpcResult<TransactionResponse> {
        Err(anyhow!("Sui Node only supports read-only methods").into())
    }

    async fn move_call(
        &self,
        _signer: SuiAddress,
        _package_object_id: ObjectID,
        _module: Identifier,
        _function: Identifier,
        _type_arguments: Vec<SuiTypeTag>,
        _rpc_arguments: Vec<SuiJsonValue>,
        _gas: Option<ObjectID>,
        _gas_budget: u64,
    ) -> RpcResult<TransactionBytes> {
        Err(anyhow!("Sui Node only supports read-only methods").into())
    }

    async fn sync_account_state(&self, _address: SuiAddress) -> RpcResult<()> {
        todo!()
    }

    async fn get_total_transaction_number(&self) -> RpcResult<u64> {
        todo!()
    }

    async fn get_transactions_in_range(
        &self,
        _start: GatewayTxSeqNumber,
        _end: GatewayTxSeqNumber,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        todo!()
    }

    async fn get_recent_transactions(
        &self,
        _count: u64,
    ) -> RpcResult<Vec<(GatewayTxSeqNumber, TransactionDigest)>> {
        todo!()
    }

    async fn get_transaction(
        &self,
        _digest: TransactionDigest,
    ) -> RpcResult<TransactionEffectsResponse> {
        todo!()
    }
}
