//! TLS over the AA control channel: we are the client, using the well-known head unit certificate.
//! The phone's certificate is not verified (no head unit does; there is nothing to check it against).
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{CryptoProvider, ring};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime, pem::PemObject};
use rustls::sign::{CertifiedKey, SingleCertAndKey};
use rustls::{ClientConfig, ClientConnection, DigitallySignedStruct, SignatureScheme};
use std::io::{self, Read, Write};
use std::sync::Arc;

const CERT: &[u8] = include_bytes!("../certs/headunit.crt");
const KEY: &[u8] = include_bytes!("../certs/headunit.key");

pub struct Tls(ClientConnection);

impl Tls {
    pub fn new() -> Self {
        let provider = Arc::new(ring::default_provider());
        let mut cfg = ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .expect("tls versions")
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AnyCert(provider.clone())))
            .with_client_cert_resolver(Arc::new(SingleCertAndKey::from(client_cert(&provider))));
        cfg.enable_sni = false;
        let name = ServerName::try_from("android-auto").unwrap();
        Tls(ClientConnection::new(Arc::new(cfg), name).expect("tls client"))
    }

    pub fn handshaking(&self) -> bool {
        self.0.is_handshaking()
    }

    /// Feeds handshake bytes from the phone; returns what to send back.
    pub fn handshake(&mut self, input: &[u8]) -> io::Result<Vec<u8>> {
        self.feed(input)?;
        let mut out = Vec::new();
        self.flush(&mut out)?;
        Ok(out)
    }

    /// Appends the ciphertext of `plain` to `out`.
    pub fn encrypt(&mut self, plain: &[u8], out: &mut Vec<u8>) -> io::Result<()> {
        self.0.writer().write_all(plain)?;
        self.flush(out)
    }

    /// Appends the plaintext of `cipher` to `out`.
    pub fn decrypt(&mut self, cipher: &[u8], out: &mut Vec<u8>) -> io::Result<()> {
        self.feed(cipher)?;
        match self.0.reader().read_to_end(out) {
            Err(e) if e.kind() != io::ErrorKind::WouldBlock => Err(e),
            _ => Ok(()),
        }
    }

    fn feed(&mut self, mut input: &[u8]) -> io::Result<()> {
        while !input.is_empty() {
            self.0.read_tls(&mut input)?;
            self.0.process_new_packets().map_err(io::Error::other)?;
        }
        Ok(())
    }

    fn flush(&mut self, out: &mut Vec<u8>) -> io::Result<()> {
        while self.0.wants_write() {
            self.0.write_tls(out)?;
        }
        Ok(())
    }
}

/// The head unit certificate is X.509 v1, which rustls refuses to parse, so skip `from_der`'s
/// cert/key match check (it would parse it) and hand the raw certificate over as is.
fn client_cert(provider: &CryptoProvider) -> CertifiedKey {
    let key = PrivateKeyDer::from_pem_slice(KEY).expect("key");
    let key = provider
        .key_provider
        .load_private_key(key)
        .expect("signing key");
    CertifiedKey::new(
        vec![CertificateDer::from_pem_slice(CERT).expect("cert")],
        key,
    )
}

#[derive(Debug)]
struct AnyCert(Arc<CryptoProvider>);

impl ServerCertVerifier for AnyCert {
    fn verify_server_cert(
        &self,
        _: &CertificateDer<'_>,
        _: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        _: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
    fn verify_tls12_signature(
        &self,
        _: &[u8],
        _: &CertificateDer<'_>,
        _: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn verify_tls13_signature(
        &self,
        _: &[u8],
        _: &CertificateDer<'_>,
        _: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}
