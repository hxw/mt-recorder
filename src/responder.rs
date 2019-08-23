// responder.rs

use base64;
use base64_serde::base64_serde_type;
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;

base64_serde_type!(Base64Standard, base64::STANDARD);

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

use super::block;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "Job")]
    job: String,

    #[serde(rename = "Header")]
    header: block::Header,

    #[serde(rename = "TxZero", with = "Base64Standard")]
    tx_zero: Vec<u8>,

    //#[serde(rename = "TxIds", with = "hex_serde")]
    #[serde(rename = "TxIds")]
    tx_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(rename = "Request")]
    pub request: String,

    #[serde(rename = "Job")]
    pub job: String,

    #[serde(rename = "Packed", with = "Base64Standard")]
    pub packed: Vec<u8>,
}

pub fn send_job(
    s: &str,
    txs: &mut Vec<spmc::Sender<(bytes::Bytes, u64, std::string::String)>>,
) -> MyResult<()> {
    let p: Job = serde_json::from_str(s)?;

    // debugging
    log::debug!("job:    {}", p.job);
    log::debug!("tx_0:   {:02x?}", p.tx_zero);
    log::debug!("header: {:?}", p.header);

    let h = p.header;

    let mut nnn = u64::from_le_bytes(h.nonce);

    let buf = bytes::Bytes::from(h);
    if buf.len() != 100 - 8 {
        bail!("block header wrong");
    }

    let mut w = 1;
    for tx in txs.iter_mut() {
        log::info!("send: {}  nonce: {:016x}", w, nnn);
        let blk = buf.clone();
        tx.send((blk, nnn, p.job.clone()))?;
        nnn += 0x100000000;
        w += 1;
    }

    Ok(())
}
