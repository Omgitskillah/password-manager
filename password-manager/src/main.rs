use aes::{Aes256, Block, ParBlocks};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, NewBlockCipher,
    generic_array::GenericArray,
};
use hex::encode;
use json::object;
use sha2::{Sha256, Digest};

struct Credentials
{
    in_json:String,
    hash:String
}


fn main() {
    let credentials = get_username_and_password();
    let vault_credentials = get_vault_credentials();
}



// fn encrypt_credentials( credentials:String, key:String ) -> String 
// {
//     let mut block = Block::default();
//     let key_bytes = key.as_bytes();

//     let cipher = Aes256::new(&key_bytes);

// }

fn generate_hash(text:String) -> String
{
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());

    let result = hasher.finalize();
    let output = hex::encode(result);

    return output;
}

fn get_vault_credentials() -> Credentials
{
    let mut wallet_id = String::new();
    let mut secret_key = String::new();

    println!("Enter wallet ID:");
    std::io::stdin().read_line(&mut wallet_id).unwrap();
    println!("Enter your secret key:");
    std::io::stdin().read_line(&mut secret_key).unwrap();

    let vault_credentials_json_object = object!{
        "wallet_id" => wallet_id.clone(),
        "secret_key" => secret_key.clone(),
    };

    let vault_credentials_json_string = vault_credentials_json_object.to_string();
    println!("VAULT JSON STRING: {}", vault_credentials_json_string);

    let vault_credentials_hash = generate_hash( vault_credentials_json_string.clone() );
    println!("VAULT CREDENTIALS HASH: {}", vault_credentials_hash);

    let vault_credentials_to_store = Credentials {
        in_json: String::from(vault_credentials_json_string),
        hash: String::from(vault_credentials_hash)
    };

    return vault_credentials_to_store;
}

fn get_username_and_password() -> Credentials
{
    let mut url = String::new();
    let mut uname = String::new();
    let mut pwd = String::new();

    println!("Enter the URL to manage:");
    std::io::stdin().read_line(&mut url).unwrap();
    println!("Enter your Username:");
    std::io::stdin().read_line(&mut uname).unwrap();
    println!("Enter your password:");
    std::io::stdin().read_line(&mut pwd).unwrap();

    let credentials_json_object = object!{
        "url" => url.clone(),
        "username" => uname.clone(),
        "password" => pwd.clone()
    };
    
    let credentials_json_string = credentials_json_object.to_string();

    println!("CREDENTIALS JSON STRING: {}", credentials_json_string);
    
    let credentials_hash = generate_hash( credentials_json_string.clone() );

    println!("CREDENTIALS HASH: {}", credentials_hash);

    let credentials_to_store = Credentials {
        in_json: String::from(credentials_json_string),
        hash: String::from(credentials_hash)
    };

    return credentials_to_store;
}
