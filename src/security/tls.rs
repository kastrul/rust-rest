use std::fs::File;
use std::io::BufReader;

use rustls::{Certificate, Error, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

const CERT_PATH: &str = "cert.pem";
const KEY_PATH: &str = "key.pem";

pub fn get_tls_config() -> Result<ServerConfig, Error> {
    let cert_file = &mut BufReader::new(File::open(CERT_PATH).unwrap());
    let key_file = &mut BufReader::new(File::open(KEY_PATH).unwrap());

    let cert_chain: Vec<Certificate> = certs(cert_file).map(to_cert_vec).expect("Cert failed");
    if cert_chain.is_empty() {
        return Err(Error::General(String::from("Could not locate certs.")));
    }

    let keys = pkcs8_private_keys(key_file).expect("Private key failed");
    if keys.is_empty() {
        return Err(Error::General(String::from(
            "Could not locate PKCS 8 private keys.",
        )));
    }

    let p_key = PrivateKey(keys[0].clone());

    return ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, p_key);
}

fn to_cert_vec(cc: Vec<Vec<u8>>) -> Vec<Certificate> {
    cc.iter().map(|c| Certificate(c.clone())).collect()
}
