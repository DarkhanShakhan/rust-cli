use std::collections::HashMap;
use std::any::Any;

pub struct Key<'a>{
    pub address: &'a[u8], //TODO: define the struct
    pub private_key: String
}

struct EncryptedKey {
    address: String,
    crypto: CryptoJSON
}

impl EncryptedKey {
    pub fn new(address: String, crypto: CryptoJSON) -> Self {
        EncryptedKey { address: address, crypto: crypto }
    }
}

struct CryptoJSON {
    cipher: String, 
    ciphertext: String, 
    cipherparams: CipherParamsJSON,
    kdf: String,
    kdfparams: HashMap<String, Box<dyn Any>>,
    mac: String
}

struct CipherParamsJSON {
    iv: String
}

pub fn encrypt_key(key: &mut Key, auth:String, scryptN:i64,scryptP:i64) -> Result<&[u8]> {
    let key_bytes = key.private_key.as_bytes();
   //TODO: cryptoStruct

   let key_json =  EncryptedKey::new(key.address,)
}


fn encrypt_data_v3(data:&[u8], auth:&[u8],scryptN:i64,scryptP:i64) -> Result<CryptoJSON> {
    let salt = &[0u8; 10];
    
}