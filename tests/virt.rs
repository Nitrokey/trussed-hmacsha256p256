// Copyright (C) Nitrokey GmbH
// SPDX-License-Identifier: Apache-2.0 or MIT

#![cfg(all(feature = "virt"))]

use trussed::client::CryptoClient;
use trussed::key::Kind;
use trussed::syscall;
use trussed::types::{Location::*, Mechanism, SignatureSerialization};

use trussed::types::Location;

use trussed_hmacsha256p256::virt::with_ram_client;

use trussed::client::P256;
use trussed_hmacsha256p256::HmacSha256P256Client;

#[test]
fn hmac_inject_any() {
    use trussed::types::Message;
    with_ram_client("staging-tests", |mut client| {
        let client = &mut client;

        let key = syscall!(client.inject_any_key(
            Message::from_slice(b"12345678123456781234567812345678").unwrap(),
            Volatile,
            Kind::P256
        ))
        .key
        .unwrap();

        let _pk = syscall!(client.derive_p256_public_key(key, Location::Volatile)).key;

        let signature =
            syscall!(client.sign(Mechanism::P256, key, &[], SignatureSerialization::Raw)).signature;
        assert!(signature.len() > 0);
    });
}
