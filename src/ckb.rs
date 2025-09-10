use crate::{
    cell_data::{Bytes, DidWeb5Data, DidWeb5DataUnion},
    db::insert_record,
    error::AppError,
    types::Web5DocumentData,
    util::{calculate_address, calculate_web5_did, check_did_doc},
};
use ckb_jsonrpc_types::BlockNumber;
use ckb_sdk::{CkbRpcAsyncClient, NetworkType};
use ckb_types::H256;
use diesel::PgConnection;
use molecule::prelude::Entity;
use std::time::Duration;
use tokio::time;

pub const CODE_HASH: &str = "510150477b10d6ab551a509b71265f3164e9fd4137fcb5a4322f49f03092c7c5";

pub async fn rolling(
    start_height: u64,
    client: &CkbRpcAsyncClient,
    conn: &mut PgConnection,
    network: NetworkType,
    target_code_hash: H256,
    mut is_sync: bool,
) -> Result<bool, AppError> {
    trace!("Tracing scanning block #{start_height}");
    match client
        .get_block_by_number(BlockNumber::from(start_height))
        .await
        .map_err(|e| AppError::CkbRpcError(e.to_string()))?
    {
        Some(block) => {
            if start_height % 100 == 0 {
                info!("Scanning block #{start_height}");
                if !is_sync {
                    let tip_number = client
                        .get_tip_block_number()
                        .await
                        .map_err(|e| AppError::CkbRpcError(e.to_string()))?
                        .value();
                    if tip_number > start_height {
                        is_sync = true;
                    }
                }
            }
            let header = block.header.inner;
            for (tx_index, tx) in block.transactions.into_iter().enumerate() {
                for (i, output) in tx.inner.outputs.into_iter().enumerate() {
                    if let Some(type_script) = output.type_ {
                        if type_script.code_hash == target_code_hash {
                            let ckb_addr = calculate_address(&output.lock.into(), network);
                            let args = type_script.args.as_bytes();
                            info!("Get doc cell args: {}", hex::encode(args));
                            let didoc =
                                parse_didoc_cell(tx.inner.outputs_data.get(i).unwrap().as_bytes())?;
                            info!(
                                "Get did document:\n{}",
                                serde_json::to_string_pretty(&didoc)
                                    .map_err(|e| AppError::RunTimeError(e.to_string()))?
                            );
                            let handle = check_did_doc(&didoc)?;
                            insert_record(
                                conn,
                                calculate_web5_did(&args[..20]),
                                handle,
                                header.timestamp.value(),
                                ckb_addr.to_string(),
                                tx.hash.to_string(),
                                tx_index as i32,
                                start_height as i64,
                                didoc,
                                true,
                            )?;
                        }
                    }
                }
            }
        }
        None => {
            if is_sync {
                let tip_number = client
                    .get_tip_block_number()
                    .await
                    .map_err(|e| AppError::CkbRpcError(e.to_string()))?
                    .value();
                if tip_number < start_height {
                    is_sync = false;
                }
            }
        }
    }

    let wait = if is_sync {
        Duration::from_secs(0)
    } else {
        Duration::from_secs(3)
    };
    time::sleep(wait).await;
    Ok(is_sync)
}

fn parse_didoc_cell(cell_data: &[u8]) -> Result<Web5DocumentData, AppError> {
    let did_data = DidWeb5Data::from_slice(cell_data).unwrap();
    let DidWeb5DataUnion::DidWeb5DataV1(did_data_v1) = did_data.to_enum();
    let did_doc: Bytes = did_data_v1.document();
    serde_ipld_dagcbor::from_slice(&did_doc.raw_data())
        .map_err(|e| AppError::DagCborError(e.to_string()))
}
