use crate::error::Error;
use crate::net::CertificateInput;
use boring::ssl::{SslConnector, SslMethod, SslVerifyMode};
use boring::x509::X509;

pub async fn configure_tls_connector(
    accept_invalid_certs: bool,
    accept_invalid_hostnames: bool,
    root_cert_path: Option<&CertificateInput>,
) -> Result<sqlx_rt::TlsConnector, Error> {
    let mut config;
    if accept_invalid_certs {
        config = SslConnector::builder(SslMethod::tls_client())?
            .build()
            .configure()?;
        config.set_verify(SslVerifyMode::NONE);
    } else {
        let mut builder = SslConnector::builder(SslMethod::tls_client())?;
        if let Some(ca) = root_cert_path {
            let ca = ca.data().await?;
            let ca = X509::from_pem(&ca)?;
            builder.add_client_ca(&ca)?;
        }
        config = builder.build().configure()?;
        config.set_verify_hostname(!accept_invalid_hostnames);
    }
    Ok(config.into())
}
