// Copyright (c) 2022, Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::{
    crypto::{AccountKeyPair, SuiSignature},
    intent::{ChainId, Intent, IntentMessage, IntentScope, IntentVersion, PersonalMessage},
};

#[test]
fn test_personal_message_intent() {
    use crate::crypto::{get_key_pair, Signature};
    let (addr1, sec1): (_, AccountKeyPair) = get_key_pair();
    let message = "Hello".as_bytes().to_vec();
    let p_message = PersonalMessage { message };
    let p_message_bcs = bcs::to_bytes(&p_message).unwrap();

    let intent = Intent::default_with_scope(IntentScope::PersonalMessage);
    let intent_bcs = bcs::to_bytes(&IntentMessage::new(intent, &p_message)).unwrap();
    assert_eq!(intent_bcs.len(), p_message_bcs.len() + 3);

    // Check that the first 3 bytes are the domain separation information.
    assert_eq!(
        &intent_bcs[..3],
        vec![
            IntentVersion::V0 as u8,
            ChainId::Testing as u8,
            IntentScope::PersonalMessage as u8,
        ]
    );

    // Check that intent's last bytes match the p_message's bsc bytes.
    assert_eq!(&intent_bcs[3..], &p_message_bcs);

    // Let's ensure we can sign and verify intents.
    let s = Signature::new_secure(&p_message, intent, &sec1);
    let verification = s.verify_secure(&p_message, intent, addr1);
    assert!(verification.is_ok())
}
