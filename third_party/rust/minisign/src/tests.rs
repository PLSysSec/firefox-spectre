#[test]
fn byte_array_store() {
    use crate::store_u64_le;

    assert_eq!([0xFF, 0, 0, 0, 0, 0, 0, 0], store_u64_le(0xFF));
}

#[test]
fn byte_array_load() {
    use crate::load_u64_le;

    assert_eq!(255, load_u64_le(&[0xFF, 0, 0, 0, 0, 0, 0, 0]));
}

#[test]
fn pk_key_struct_conversion() {
    use crate::{KeyPair, PublicKey};

    let KeyPair { pk, .. } = KeyPair::generate_unencrypted_keypair().unwrap();
    assert_eq!(pk, PublicKey::from_bytes(&pk.to_bytes()).unwrap());
}

#[test]
fn sk_key_struct_conversion() {
    use crate::{KeyPair, SecretKey};

    let KeyPair { sk, .. } = KeyPair::generate_unencrypted_keypair().unwrap();
    assert_eq!(sk, SecretKey::from_bytes(&sk.to_bytes()).unwrap());
}

#[test]
fn xor_keynum() {
    use crate::KeyPair;
    use getrandom::getrandom;

    let KeyPair { mut sk, .. } = KeyPair::generate_unencrypted_keypair().unwrap();
    let mut key = vec![0u8; sk.keynum_sk.len()];
    getrandom(&mut key).unwrap();
    let original_keynum = sk.keynum_sk.clone();
    sk.xor_keynum(&key);
    assert_ne!(original_keynum, sk.keynum_sk);
    sk.xor_keynum(&key);
    assert_eq!(original_keynum, sk.keynum_sk);
}

#[test]
fn sk_checksum() {
    use crate::KeyPair;

    let KeyPair { mut sk, .. } = KeyPair::generate_unencrypted_keypair().unwrap();
    assert!(sk.write_checksum().is_ok());
    assert_eq!(sk.keynum_sk.chk.to_vec(), sk.read_checksum().unwrap());
}

#[test]
fn load_public_key_string() {
    use crate::PublicKey;

    assert!(
        PublicKey::from_base64("RWRzq51bKcS8oJvZ4xEm+nRvGYPdsNRD3ciFPu1YJEL8Bl/3daWaj72r").is_ok()
    );
    assert!(
        PublicKey::from_base64("RWQt7oYqpar/yePp+nonossdnononovlOSkkckMMfvHuGc+0+oShmJyN5Y")
            .is_err()
    );
}

#[test]
fn signature() {
    use crate::{sign, verify, KeyPair};
    use std::io::Cursor;

    let KeyPair { pk, sk } = KeyPair::generate_unencrypted_keypair().unwrap();
    let data = b"test";
    let signature_box = sign(None, &sk, Cursor::new(data), false, None, None).unwrap();
    verify(&pk, &signature_box, Cursor::new(data), true, false).unwrap();
    let data = b"test2";
    assert!(verify(&pk, &signature_box, Cursor::new(data), true, false).is_err());
}

#[test]
fn signature_bones() {
    use crate::{sign, verify, KeyPair, SignatureBones};
    use std::io::Cursor;

    let KeyPair { pk, sk } = KeyPair::generate_unencrypted_keypair().unwrap();
    let data = b"test";
    let signature_box = sign(None, &sk, Cursor::new(data), false, None, None).unwrap();
    let signature_bones: SignatureBones = signature_box.into();
    verify(
        &pk,
        &signature_bones.clone().into(),
        Cursor::new(data),
        true,
        false,
    )
    .unwrap();
    let data = b"test2";
    assert!(verify(&pk, &signature_bones.into(), Cursor::new(data), true, false).is_err());
}
