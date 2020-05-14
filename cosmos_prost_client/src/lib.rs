use cosmos_prost_types::cosmos_sdk::tx::v1::{Tx, SignDoc};
use prost::{EncodeError, Message};

pub struct AccountInfo {
    account_number: u64,
    account_sequence: u64,
}

pub trait AccountRetriever {
    fn get_account_info(&self, conn: &dyn Connection, address: &[u8]) -> AccountInfo;
}

pub trait Connection {
    fn chain_id(&self) -> String;
}

pub fn sign_doc(conn: &dyn Connection, account_retriever: &dyn AccountRetriever, tx: &Tx, address: &[u8]) -> SignDoc {
    let chain_id = conn.chain_id();
    let account_info = account_retriever.get_account_info(conn, address);
    SignDoc {
        chain_id,
        account_number: account_info.account_number,
        account_sequence: account_info.account_sequence,
        body: tx.body.clone(),
        auth_info: tx.auth_info.clone(),
    }
}

pub fn sign_bytes(conn: &dyn Connection, account_retriever: &dyn AccountRetriever, tx: &Tx, address: &[u8]) -> Result<Vec<u8>, EncodeError> {
    let doc = sign_doc(conn, account_retriever, tx, address);
    let mut buf = Vec::new();
    let _ = doc.encode(&mut buf)?;
    Ok(buf)
}
