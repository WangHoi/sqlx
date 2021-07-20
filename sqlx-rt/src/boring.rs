pub use tokio_boring::SslStream as TlsStream;
use tokio_boring;
use tokio::io::{AsyncRead, AsyncWrite};

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
impl std::error::Error for TlsError {}
impl std::fmt::Display for TlsError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl TlsConnector {
    pub async fn connect<S>(self, domain: &str, stream: S) -> Result<TlsStream<S>, TlsError>
    where 
        S: AsyncRead + AsyncWrite + Unpin + std::fmt::Debug,
    {
        tokio_boring::connect(self.config, domain, stream).await.map_err(|e| {
            TlsError {
                inner: e.to_string(),
            }
        })
    }
}
