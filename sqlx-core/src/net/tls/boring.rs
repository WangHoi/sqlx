use crate::net::CertificateInput;
use crate::error::Error;
use boring::ssl::{SslConnector, SslMethod};

pub async fn configure_tls_connector(
    accept_invalid_certs: bool,
    accept_invalid_hostnames: bool,
    root_cert_path: Option<&CertificateInput>,
) -> Result<sqlx_rt::TlsConnector, Error> {
    let builder = SslConnector::builder(SslMethod::tls_client())?;
    let config = builder.build().configure()?;
    Ok(config.into())
}
