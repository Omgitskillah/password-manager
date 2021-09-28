static url_request:&str ="the URL";
static username_request:&str ="your Username";
static password_request:&str ="your Password";
static wallet_id_request:&str ="your Wallet ID";
static secret_key_request:&str ="your Secret Key";

static url_json_key:&str ="URL";
static username_json_key:&str ="Username";
static password_json_key:&str ="Password";
static wallet_id_json_key:&str ="WalletID";
static secret_key_json_key:&str ="SecretKey";


fn request_vault_credentials( request:&str ) -> String
{
    let mut vault_credentials = String::new();

    println!("Enter {}:", request);
    std::io::stdin().read_line(&mut vault_credentials).unwrap();

    return vault_credentials;
}

fn generate_credentials_json_string( json_key_1:&str, json_value_1:String, json_key_2:&str, json_value_2:String ) -> String
{
    use json::object;

    let credentials_json_object = object!{
        json_key_1 => json_value_1,
        json_key_2 => json_value_2
    };

    return credentials_json_object.to_string();
}

fn generate_hash(text:String) -> String
{
    use sha2::{Sha256, Digest};

    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    let result_array = hasher.finalize();
    let output = hex::encode(result_array);

    return output;
}

fn encrypt_password( password:String, aes_key:String ) -> String
{
    use aes::{Aes128, Block};
    use aes::cipher::{BlockEncrypt, NewBlockCipher,
        generic_array::GenericArray,
    };

    let key = GenericArray::from_slice(&aes_key.as_bytes());
    let mut password_padded = format!("{:width$}", password, width=16);
    let mut password_padded_bytes = password_padded.as_bytes();

    let mut block = Block::clone_from_slice(&password_padded_bytes);
    let cipher = Aes128::new(&key);
    cipher.encrypt_block(&mut block);
    let encrypted_string = hex::encode(block.clone());

    return encrypted_string;
}

fn decrypt_password( encrypted_password:String, aes_key:String ) -> String
{
    use std::str;
    use aes::{Aes128, Block};
    use aes::cipher::{BlockDecrypt, NewBlockCipher,
        generic_array::GenericArray,
    };

    let key = GenericArray::from_slice(&aes_key.as_bytes());
    let mut retrieved_slice = [0u8; 16];
    hex::decode_to_slice(encrypted_password, &mut retrieved_slice as &mut [u8]);

    let mut decrypt_block = Block::clone_from_slice(&retrieved_slice);

    let cipher = Aes128::new(&key);

    cipher.decrypt_block(&mut decrypt_block);

    let decrypted_password = str::from_utf8(&decrypt_block).unwrap();

    return String::from(decrypted_password);
}

fn store_encrypted_password( encrypted_password:String, vault_name:String )
{
    use std::fs::File;
    use std::io::prelude::*;

    let mut vault = File::create(format!("{}{}", vault_name, ".pwd")).expect("Could not create file");
    vault.write_all(&mut encrypted_password.as_bytes()).expect("Could not write to file");
}

fn retrieve_encrypted_password( vault_name:String ) -> String 
{
    use std::fs::File;
    use std::io::prelude::*;

    let mut retrieved_vault = File::open(format!("{}{}", vault_name, ".pwd")).expect("Could not create file");
    let mut retrieved_encrypted_password = String::new();
    retrieved_vault.read_to_string(&mut retrieved_encrypted_password).expect("Could not read file");

    return retrieved_encrypted_password;
}

fn main() 
{
    /* now implement the storage process!!! */
}
