// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk_bindings_core::{call_utils_method, Response, Result, UtilsMethod};

#[tokio::test]
async fn utils() -> Result<()> {
    let response = call_utils_method(UtilsMethod::GenerateMnemonic);
    match response {
        Response::GeneratedMnemonic(mnemonic) => println!("{:?}", serde_json::to_string(&mnemonic).unwrap()),
        _ => panic!("Unexpected response type"),
    };

    let bech32_adddress = "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy".to_string();
    let method = UtilsMethod::Bech32ToHex {
        bech32: bech32_adddress.clone(),
    };

    let response = call_utils_method(method);
    match response {
        Response::Bech32ToHex(hex) => {
            match call_utils_method(UtilsMethod::HexToBech32 {
                hex,
                bech32_hrp: "rms".to_string(),
            }) {
                Response::Bech32Address(address_bech32) => {
                    assert_eq!(address_bech32, bech32_adddress)
                }
                _ => panic!("Unexpected response type"),
            };
        }
        _ => panic!("Unexpected response type"),
    };

    Ok(())
}
