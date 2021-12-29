use std::net::SocketAddr;

use crate::{manager::connection_map, setup::Channels, util::signing_bytes};

use lib::cs::{
    key::{PublicKey, VerifiablePublicKey},
    protocol::{error::Error, response::RequestConnection},
};

#[inline]
pub async fn request_connection(
    channels: Channels,
    initiator_key: VerifiablePublicKey,
    target_key: PublicKey,
    initiator_address: SocketAddr,
) -> Result<RequestConnection, Error> {
    let signing_bytes = signing_bytes(channels.sign).await?;
    let initiator_key = initiator_key.into_key(&signing_bytes)?;
    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();

    channels
        .conn
        .send(connection_map::Command::RequestConnection {
            initiator_key,
            initiator_address,
            target_key,
            resp: resp_tx,
        })
        .await?;

    let _ = resp_rx.await?;
    Ok(RequestConnection)
}
