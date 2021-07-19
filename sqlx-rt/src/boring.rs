pub use tokio_boring::SslStream as TlsStream;
use tokio_boring::{self, HandshakeError};
use tokio::io::{AsyncRead, AsyncWrite};
use std::error::Error as StdError;

pub struct TlsConnector {
    config: boring::ssl::ConnectConfiguration,
}

impl From<boring::ssl::ConnectConfiguration> for TlsConnector {
    fn from(config: boring::ssl::ConnectConfiguration) -> Self {
        Self {
            config,
        }
    }
}

#[derive(Debug)]
pub struct TlsError {
    inner: String,
}
impl std::error::Error for TlsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
impl std::fmt::Display for TlsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner)
    }
}

impl TlsConnector {
    pub async fn connect<S>(self, domain: &str, stream: S) -> Result<TlsStream<S>, TlsError>
    where 
        S: AsyncRead + AsyncWrite + Unpin,
    {
        tokio_boring::connect(self.config, domain, stream).await.map_err(|e| TlsError {
            inner: String::from("handshake error"),
        })
    }
}
