use {super::*, crate::okx::datastore::brc20 as brc20_store, axum::Json, utoipa::ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::TxEvent)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum TxEvent {
  /// Event generated by deployed ticker.
  #[schema(value_type = brc20::DeployEvent)]
  Deploy(DeployEvent),
  /// Event generated by mining.
  #[schema(value_type = brc20::MintEvent)]
  Mint(MintEvent),
  /// Event generated by pretransfer.
  #[schema(value_type = brc20::InscribeTransferEvent)]
  InscribeTransfer(InscribeTransferEvent),
  #[schema(value_type = brc20::TransferEvent)]
  /// Event generated by transfer.
  Transfer(TransferEvent),
  /// Event generated by the execution has failed.
  #[schema(value_type = brc20::ErrorEvent)]
  Error(ErrorEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::ErrorEvent)]
#[serde(rename_all = "camelCase")]
pub struct ErrorEvent {
  /// Event type.
  #[serde(rename = "type")]
  pub event: String,
  /// The inscription id.
  pub inscription_id: String,
  /// The inscription number.
  pub inscription_number: i64,
  /// The inscription satpoint of the transaction input.
  pub old_satpoint: String,
  /// The inscription satpoint of the transaction output.
  pub new_satpoint: String,
  /// The message sender which is an address or script pubkey hash.
  pub from: ScriptPubkey,
  /// The message receiver which is an address or script pubkey hash.
  pub to: ScriptPubkey,
  /// Executed state.
  pub valid: bool,
  /// Error message.
  pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::DeployEvent)]
#[serde(rename_all = "camelCase")]
pub struct DeployEvent {
  /// Event type.
  #[serde(rename = "type")]
  pub event: String,
  /// The ticker deployed.
  pub tick: String,
  /// The inscription id.
  pub inscription_id: String,
  /// The inscription number.
  pub inscription_number: i64,
  /// The inscription satpoint of the transaction input.
  pub old_satpoint: String,
  /// The inscription satpoint of the transaction output.
  pub new_satpoint: String,
  /// The total supply of the deployed ticker.
  pub supply: String,
  /// The limit per mint of the deployed ticker.
  pub limit_per_mint: String,
  /// The decimal of the deployed ticker.
  pub decimal: u8,
  /// The message sender which is an address or script pubkey hash.
  pub from: ScriptPubkey,
  /// The message receiver which is an address or script pubkey hash.
  pub to: ScriptPubkey,
  /// Executed state.
  pub valid: bool,
  /// Message generated during execution.
  pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::MintEvent)]
#[serde(rename_all = "camelCase")]
pub struct MintEvent {
  #[serde(rename = "type")]
  /// Event type.
  pub event: String,
  /// The ticker minted.
  pub tick: String,
  /// The inscription id.
  pub inscription_id: String,
  /// The inscription number.
  pub inscription_number: i64,
  /// The inscription satpoint of the transaction input.
  pub old_satpoint: String,
  /// The inscription satpoint of the transaction output.
  pub new_satpoint: String,
  /// The amount minted.
  pub amount: String,
  /// The message sender which is an address or script pubkey hash.
  pub from: ScriptPubkey,
  /// The message receiver which is an address or script pubkey hash.
  pub to: ScriptPubkey,
  /// Executed state.
  pub valid: bool,
  /// Message generated during execution.
  pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::InscribeTransferEvent)]
#[serde(rename_all = "camelCase")]
pub struct InscribeTransferEvent {
  /// Event type.
  #[serde(rename = "type")]
  pub event: String,
  /// The ticker of pretransfer.
  pub tick: String,
  /// The inscription id.
  pub inscription_id: String,
  /// The inscription number.
  pub inscription_number: i64,
  /// The inscription satpoint of the transaction input.
  pub old_satpoint: String,
  /// The inscription satpoint of the transaction output.
  pub new_satpoint: String,
  /// The amount of pretransfer.
  pub amount: String,
  /// The message sender which is an address or script pubkey hash.
  pub from: ScriptPubkey,
  /// The message receiver which is an address or script pubkey hash.
  pub to: ScriptPubkey,
  /// Executed state.
  pub valid: bool,
  /// Message generated during execution.
  pub msg: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::TransferEvent)]
#[serde(rename_all = "camelCase")]
pub struct TransferEvent {
  /// Event type.
  #[serde(rename = "type")]
  pub event: String,
  /// The ticker of transfer.
  pub tick: String,
  /// The inscription id.
  pub inscription_id: String,
  /// The inscription number.
  pub inscription_number: i64,
  /// The inscription satpoint of the transaction input.
  pub old_satpoint: String,
  /// The inscription satpoint of the transaction output.
  pub new_satpoint: String,
  /// The amount of transfer.
  pub amount: String,
  /// The message sender which is an address or script pubkey hash.
  pub from: ScriptPubkey,
  /// The message receiver which is an address or script pubkey hash.
  pub to: ScriptPubkey,
  /// Executed state.
  pub valid: bool,
  /// Message generated during execution.
  pub msg: String,
}

impl From<&brc20_store::Receipt> for TxEvent {
  fn from(event: &brc20_store::Receipt) -> Self {
    match &event.result {
      Ok(brc20_store::Event::Deploy(deploy_event)) => Self::Deploy(DeployEvent {
        tick: deploy_event.tick.to_string(),
        inscription_id: event.inscription_id.to_string(),
        inscription_number: event.inscription_number,
        old_satpoint: event.old_satpoint.to_string(),
        new_satpoint: event.new_satpoint.to_string(),
        supply: deploy_event.supply.to_string(),
        limit_per_mint: deploy_event.limit_per_mint.to_string(),
        decimal: deploy_event.decimal,
        from: event.from.clone().into(),
        to: event.to.clone().into(),
        valid: true,
        msg: "ok".to_string(),
        event: "deploy".to_string(),
      }),
      Ok(brc20_store::Event::Mint(mint_event)) => Self::Mint(MintEvent {
        tick: mint_event.tick.to_string(),
        inscription_id: event.inscription_id.to_string(),
        inscription_number: event.inscription_number,
        old_satpoint: event.old_satpoint.to_string(),
        new_satpoint: event.new_satpoint.to_string(),
        amount: mint_event.amount.to_string(),
        from: event.from.clone().into(),
        to: event.to.clone().into(),
        valid: true,
        msg: mint_event.msg.clone().unwrap_or("ok".to_string()),
        event: "mint".to_string(),
      }),
      Ok(brc20_store::Event::InscribeTransfer(trans1)) => {
        Self::InscribeTransfer(InscribeTransferEvent {
          tick: trans1.tick.to_string(),
          inscription_id: event.inscription_id.to_string(),
          inscription_number: event.inscription_number,
          old_satpoint: event.old_satpoint.to_string(),
          new_satpoint: event.new_satpoint.to_string(),
          amount: trans1.amount.to_string(),
          from: event.from.clone().into(),
          to: event.to.clone().into(),
          valid: true,
          msg: "ok".to_string(),
          event: "inscribeTransfer".to_string(),
        })
      }
      Ok(brc20_store::Event::Transfer(trans2)) => Self::Transfer(TransferEvent {
        tick: trans2.tick.to_string(),
        inscription_id: event.inscription_id.to_string(),
        inscription_number: event.inscription_number,
        old_satpoint: event.old_satpoint.to_string(),
        new_satpoint: event.new_satpoint.to_string(),
        amount: trans2.amount.to_string(),
        from: event.from.clone().into(),
        to: event.to.clone().into(),
        valid: true,
        msg: trans2.msg.clone().unwrap_or("ok".to_string()),
        event: "transfer".to_string(),
      }),
      Err(err) => Self::Error(ErrorEvent {
        inscription_id: event.inscription_id.to_string(),
        inscription_number: event.inscription_number,
        old_satpoint: event.old_satpoint.to_string(),
        new_satpoint: event.new_satpoint.to_string(),
        valid: false,
        from: event.from.clone().into(),
        to: event.to.clone().into(),
        msg: err.to_string(),
        event: match event.op {
          brc20_store::OperationType::Deploy => "deploy".to_string(),
          brc20_store::OperationType::Mint => "mint".to_string(),
          brc20_store::OperationType::InscribeTransfer => "inscribeTransfer".to_string(),
          brc20_store::OperationType::Transfer => "transfer".to_string(),
        },
      }),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::TxEvents)]
#[serde(rename_all = "camelCase")]
pub struct TxEvents {
  #[schema(value_type = Vec<brc20::TxEvent>)]
  pub events: Vec<TxEvent>,
  pub txid: String,
}

/// Get transaction events by txid.
///
/// Retrieve all BRC20 events associated with a transaction.
#[utoipa::path(
    get,
    path = "/api/v1/brc20/tx/{txid}/events",
    params(
        ("txid" = String, Path, description = "transaction ID")
  ),
    responses(
      (status = 200, description = "Obtain transaction events by txid", body = BRC20TxEvents),
      (status = 400, description = "Bad query.", body = ApiError, example = json!(&ApiError::bad_request("bad request"))),
      (status = 404, description = "Not found.", body = ApiError, example = json!(&ApiError::not_found("not found"))),
      (status = 500, description = "Internal server error.", body = ApiError, example = json!(&ApiError::internal("internal error"))),
    )
  )]
pub(crate) async fn brc20_tx_events(
  Extension(index): Extension<Arc<Index>>,
  Path(txid): Path<String>,
) -> ApiResult<TxEvents> {
  log::debug!("rpc: get brc20_tx_events: {}", txid);
  let txid = bitcoin::Txid::from_str(&txid).map_err(|e| ApiError::bad_request(e.to_string()))?;
  let tx_events = index
    .brc20_get_tx_events_by_txid(&txid)?
    .ok_or_api_not_found(BRC20Error::EventsNotFound)?;

  log::debug!("rpc: get brc20_tx_events: {} {:?}", txid, tx_events);

  Ok(Json(ApiResponse::ok(TxEvents {
    txid: txid.to_string(),
    events: tx_events.iter().map(|e| e.into()).collect(),
  })))
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(as = brc20::BlockEvents)]
#[serde(rename_all = "camelCase")]
pub struct BlockEvents {
  #[schema(value_type = Vec<brc20::TxEvents>)]
  pub block: Vec<TxEvents>,
}

/// Get block events by blockhash.
///
/// Retrieve all BRC20 events associated with a block.
#[utoipa::path(
    get,
    path = "/api/v1/brc20/block/{blockhash}/events",
    params(
        ("blockhash" = String, Path, description = "block hash")
  ),
    responses(
      (status = 200, description = "Obtain block events by block hash", body = BRC20BlockEvents),
      (status = 400, description = "Bad query.", body = ApiError, example = json!(&ApiError::bad_request("bad request"))),
      (status = 404, description = "Not found.", body = ApiError, example = json!(&ApiError::not_found("not found"))),
      (status = 500, description = "Internal server error.", body = ApiError, example = json!(&ApiError::internal("internal error"))),
    )
  )]
pub(crate) async fn brc20_block_events(
  Extension(index): Extension<Arc<Index>>,
  Path(block_hash): Path<String>,
) -> ApiResult<BlockEvents> {
  log::debug!("rpc: get brc20_block_events: {}", block_hash);

  let blockhash =
    bitcoin::BlockHash::from_str(&block_hash).map_err(|e| ApiError::bad_request(e.to_string()))?;

  let block_events = index
    .brc20_get_block_events_by_blockhash(blockhash)?
    .ok_or_api_not_found(BRC20Error::BlockNotFound)?;

  log::debug!(
    "rpc: get brc20_block_events: {} {:?}",
    block_hash,
    block_events
  );

  Ok(Json(ApiResponse::ok(BlockEvents {
    block: block_events
      .iter()
      .map(|(txid, events)| TxEvents {
        txid: txid.to_string(),
        events: events.iter().map(|e| e.into()).collect(),
      })
      .collect(),
  })))
}
