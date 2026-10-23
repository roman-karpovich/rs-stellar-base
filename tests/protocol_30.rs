use stellar_baselib::address::{Address, AddressTrait};
use stellar_baselib::xdr::{self, Limits, ReadXdr, WriteXdr};

// SEP-23 vectors cover zero and an id above i64::MAX.
const MUXED_CONTRACTS: [&str; 2] = [
    "WA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUAAAAAAAAAAAAAWWC",
    "WA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVAAAAAAAAAAAACWJY",
];

#[test]
fn protocol_30_muxed_contract_addresses_round_trip() {
    for (encoded, id) in MUXED_CONTRACTS.into_iter().zip([0, 1_u64 << 63]) {
        let address = Address::new(encoded).expect("valid SEP-23 muxed contract");
        assert_eq!(address.to_string(), encoded);
        let value = address.to_sc_val().unwrap();
        assert_eq!(Address::from_sc_val(&value).unwrap().to_string(), encoded);
        let xdr::ScVal::Address(xdr::ScAddress::MuxedContract(contract)) = value else {
            panic!("expected muxed contract");
        };
        assert_eq!(contract.id, id);
        assert_eq!(
            hex::encode(contract.contract_id.0 .0),
            "3f0c34bf93ad0d9971d04ccc90f705511c838aad9734a4a2fb0d7a03fc7fe89a"
        );
        assert_eq!(
            Address::muxed_contract(&address.to_buffer()).unwrap(),
            address
        );
        assert_ne!(
            address,
            Address::new("CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA").unwrap()
        );
    }
}

#[test]
fn protocol_30_muxed_contract_rejects_malformed_payloads() {
    for length in [0, 32, 39, 41] {
        assert!(Address::muxed_contract(&vec![0; length]).is_err());
    }
    assert!(Address::new(&MUXED_CONTRACTS[0][..68]).is_err());
    assert!(Address::new(&format!("{}A", &MUXED_CONTRACTS[0][..68])).is_err());
}

#[test]
fn protocol_30_muxed_contract_is_only_an_invocation_argument() {
    use stellar_baselib::contract::{ContractBehavior, Contracts};
    use stellar_baselib::operation::Operation;
    assert!(Contracts::new(MUXED_CONTRACTS[0]).is_err());
    assert!(Operation::new()
        .invoke_contract(MUXED_CONTRACTS[0], "transfer", vec![], None)
        .is_err());
    assert!(Operation::new()
        .create_contract(MUXED_CONTRACTS[0], [0; 32], Some([0; 32]), None, vec![],)
        .is_err());
    assert!(Operation::new()
        .create_contract_from_external_ref(
            MUXED_CONTRACTS[0],
            "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA",
            b"tag",
            Some([0; 32]),
            None,
            vec![],
        )
        .is_err());
    let recipient = Address::new(MUXED_CONTRACTS[0])
        .unwrap()
        .to_sc_val()
        .unwrap();
    assert!(Operation::new()
        .invoke_contract(
            "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA",
            "transfer",
            vec![recipient],
            None,
        )
        .is_ok());
}

#[test]
fn protocol_30_muxed_contract_cannot_authorize() {
    use stellar_baselib::authorize_entry::{authorize_entry, AuthorizeEntryParams};
    use stellar_baselib::keypair::{Keypair, KeypairBehavior};
    let signer = Keypair::random().unwrap();
    let entry = xdr::SorobanAuthorizationEntry {
        credentials: xdr::SorobanCredentials::AddressV2(xdr::SorobanAddressCredentials {
            address: Address::new(MUXED_CONTRACTS[0])
                .unwrap()
                .to_sc_address()
                .unwrap(),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert_eq!(
        authorize_entry(AuthorizeEntryParams {
            entry,
            signer: &signer,
            valid_until_ledger_seq: 100,
            network_passphrase: "Test SDF Network ; September 2015",
            use_address_v2: None,
        })
        .unwrap_err(),
        "Muxed addresses cannot authorize Soroban entries"
    );
}

#[test]
fn protocol_30_headers_decode_both_millisecond_arms() {
    for (bytes, discriminant) in [
        (include_bytes!("fixtures/signed_ms.xdr").as_slice(), 3),
        (include_bytes!("fixtures/empty_tx_set_ms.xdr").as_slice(), 4),
    ] {
        let entry = xdr::LedgerHeaderHistoryEntry::from_xdr(bytes, Limits::none()).unwrap();
        assert_eq!(entry.header.ledger_seq, 42);
        assert_eq!(entry.header.scp_value.close_time.0, 1_700_000_000);
        let milliseconds = match &entry.header.scp_value.ext {
            xdr::StellarValueExt::SignedMs(value) => value.close_time_ms.0,
            xdr::StellarValueExt::EmptyTxSetMs(value) => value.close_time_ms.0,
            other => panic!("unexpected close-time extension: {other:?}"),
        };
        assert_eq!(milliseconds, 1_700_000_000_123);
        assert_eq!(
            entry.header.scp_value.ext.discriminant() as i32,
            discriminant
        );
        assert_eq!(entry.to_xdr(Limits::none()).unwrap(), bytes);
    }
}

#[test]
fn protocol_30_exposes_ml_dsa_cost_types_and_long_spec_names() {
    for discriminant in 86..=94 {
        assert!(xdr::ContractCostType::try_from(discriminant).is_ok());
    }
    let spec = xdr::ScSpecUdtStructV0 {
        name: vec![b'a'; 1024].try_into().unwrap(),
        ..Default::default()
    };
    let bytes = spec.to_xdr(Limits::none()).unwrap();
    assert_eq!(
        xdr::ScSpecUdtStructV0::from_xdr(bytes, Limits::none())
            .unwrap()
            .name
            .len(),
        1024
    );
    let event = xdr::ScSpecEventV0 {
        name: vec![b'e'; 1024].try_into().unwrap(),
        ..Default::default()
    };
    let bytes = event.to_xdr(Limits::none()).unwrap();
    assert_eq!(
        xdr::ScSpecEventV0::from_xdr(bytes, Limits::none())
            .unwrap()
            .name
            .len(),
        1024
    );
}
