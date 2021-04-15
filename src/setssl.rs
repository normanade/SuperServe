//! Setup SSL using Rustls

use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

use std::fs::File;
use std::io::BufReader;

use crate::config::get_config;

/// use rustls as ssl loader
pub fn getssl_config() -> ServerConfig {
    let mut ssl_config = ServerConfig::new(NoClientAuth::new());
    let config = get_config();
    let cert_file = &mut BufReader::new(
        File::open(
            config["ssl"]["cert"].to_string().replace("\"", "")
        ).expect(
            "Config Error: `cert.pem` missing or needs permission!"
        )
    );
    let key_file = &mut BufReader::new(
        File::open(
            config["ssl"]["key"].to_string().replace("\"", "")
        ).expect(
            "Config Error: `key.pem` missing or needs permission!"
        )
    );
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    ssl_config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    ssl_config
}
