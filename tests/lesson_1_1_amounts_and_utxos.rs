use bdk_dojo::{Amount, OutPoint, Utxo};

#[test]
fn amount_preserves_sats_exactly() {
    let amount = Amount::from_sats(50_000);

    assert_eq!(amount.to_sats(), 50_000);
}

#[test]
fn utxo_stores_outpoint_and_value() {
    let outpoint = OutPoint {
        txid: "abc".to_string(),
        vout: 1,
    };

    let utxo = Utxo {
        outpoint,
        value: Amount::from_sats(12_345),
    };

    assert_eq!(utxo.outpoint.txid, "abc");
    assert_eq!(utxo.outpoint.vout, 1);
    assert_eq!(utxo.value.to_sats(), 12_345);
}
