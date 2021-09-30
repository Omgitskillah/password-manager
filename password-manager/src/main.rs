static SPLASH_SCREEN:&str =
"
****************************************************************
**  ______   __    __  ______    __    __  __   __    ____    **
** |   _  \\ |  |/\\|  ||   _  \\  |  \\  /  ||  \\ |  | /  ___|   **
** |  |_|  ||        ||  | \\  | |   \\/   ||   \\|  ||  /  ___  **
** |   ___/ |   /\\   ||  |_/  | |     |  ||  |\\   ||  \\_/  _| **
** |__|     |__/  \\__||_____ /  |__|\\/|__||__| \\__| \\_____/   **
**                                                            **
**          SIMPLE PASSWORD MANAGER WRITTEN IN RUST           **
**              By Jeff Myers & Clarence Alucho               **
****************************************************************
"
;

const TOTAL_APP_STATES: usize = 14;
const PASSWORD_VAULT_OFFSET: usize = 3;
// TOTAL_APP_STATES is the total number of 
// elements in ApplicationStates enum below
#[derive(Clone)]
enum ApplicationStates
{
    Idle,
    GetUrl,
    GetUname,
    GetUrlAndUnameHash,
    GetPwd,
    GetWalletID,
    GetSecretKey,
    GetIdAndSecretHash,
    EncryptPwd,
    SavePwd,
    RetrievePwd,
    DecryptPwd,
    DisplayCredentials,
    ExitApp,
}

struct Vault
{
    url:String,
    username:String,
    password:String,
    vault_hash:String,
    encrypted_password:String,
    wallet_id:String,
    secret_key:String,
    secret_hash:String,
}

enum ApplicationPath 
{
    Store,
    Retrieve,
    Exit,
}

static APPLICATION_PATH_REQUEST:&str ="[s] to create vault, [r] to Retrieve vault or [e] to Exit";

static URL_REQUEST:&str ="URL";
static USERNAME_REQUEST:&str ="Username";
static PASSWORD_REQUEST:&str ="Password";
static WALLET_ID_REQUEST:&str ="Wallet ID";
static SECRET_KEY_REQUEST:&str ="Secret Key";

static URL_JSON_KEY:&str ="URL";
static USERNAME_JSON_KEY:&str ="Username";
static WALLET_ID_JSON:&str ="WalletID";
static SECRET_KEY_JSON_KEY:&str ="SecretKey";

fn request_vault_credentials( request:&str ) -> String
{
    let mut vault_credentials = String::new();

    println!("\nEnter {}:", request);
    std::io::stdin().read_line(&mut vault_credentials).unwrap();
    
    return vault_credentials.trim().to_string();
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
    use aes::{Aes256, Block};
    use aes::cipher::{BlockEncrypt, NewBlockCipher,
        generic_array::GenericArray,
    };
    
    let mut aes_key_hex = [0u8;32];

    assert_eq!(hex::decode_to_slice(aes_key.clone(), &mut aes_key_hex as &mut [u8]), Ok(()));

    let key = GenericArray::from_slice(&aes_key_hex);

    let password_padded = format!("{:02x}:{:width$}",password.len(), password, width=13);

    let password_padded_bytes = password_padded.as_bytes();

    let mut block = Block::clone_from_slice(&password_padded_bytes);
    let cipher = Aes256::new(&key);
    cipher.encrypt_block(&mut block);
    let encrypted_string = hex::encode(block.clone());

    return encrypted_string;
}

fn decrypt_password( encrypted_password:String, aes_key:String ) -> String
{
    use std::str;
    use aes::{Aes256, Block};
    use aes::cipher::{BlockDecrypt, NewBlockCipher,
        generic_array::GenericArray,
    };

    let mut aes_key_hex = [0u8;32];
    assert_eq!(hex::decode_to_slice(aes_key.clone(), &mut aes_key_hex as &mut [u8]), Ok(()));
    let key = GenericArray::from_slice(&aes_key_hex);

    let mut retrieved_slice = [0u8; 16];

    assert_eq!(hex::decode_to_slice(encrypted_password, &mut retrieved_slice as &mut [u8]), Ok(()));

    let mut decrypt_block = Block::clone_from_slice(&retrieved_slice);

    let cipher = Aes256::new(&key);

    cipher.decrypt_block(&mut decrypt_block);

    let decrypted_password_length_str = str::from_utf8(&decrypt_block[..2]).unwrap();

    let mut decrypted_password_length = usize::from_str_radix(decrypted_password_length_str, 16).unwrap();

    decrypted_password_length += PASSWORD_VAULT_OFFSET;

    let decrypted_password = str::from_utf8(&decrypt_block[PASSWORD_VAULT_OFFSET..decrypted_password_length]).unwrap();

    return String::from(decrypted_password);
}

fn store_encrypted_password( encrypted_password:String, vault_name:String )
{
    use std::fs::File;
    use std::io::prelude::*;

    let mut vault = match File::create(format!("{}{}", vault_name, ".pwd")) {
        Ok(vault) => vault,
        _ => {
            println!("\n**********************\nCould Not create Vault\n**********************\n");
            std::process::exit(0);
        }
    };

    vault.write_all(&mut encrypted_password.as_bytes()).expect("Could not write to file");
}

fn retrieve_encrypted_password( vault_name:String ) -> String 
{
    use std::fs::File;
    use std::io::prelude::*;

    let mut retrieved_vault = match File::open(format!("{}{}", vault_name, ".pwd")) {
        Ok(retrieved_vault) => retrieved_vault,
        _ => {
            println!("\n*****************\nVault Unavailable\n*****************\n");
            std::process::exit(0);
        }
    };

    let mut retrieved_encrypted_password = String::new();
    retrieved_vault.read_to_string(&mut retrieved_encrypted_password).expect("Could not read file");

    println!("Password: -----> {}", retrieved_encrypted_password );

    return retrieved_encrypted_password;
}

fn process_application_path_input( input:String ) ->ApplicationPath
{
    match input.trim()
    {
        "e" => ApplicationPath::Exit,
        "E" => ApplicationPath::Exit,

        "s" => ApplicationPath::Store,
        "S" => ApplicationPath::Store,

        "r" => ApplicationPath::Retrieve,
        "R" => ApplicationPath::Retrieve,
        // Exit if anythin else
        _ => ApplicationPath::Exit,
    }
}

fn show_credentials( url:String, uname:String, pwd:String )
{
    println!("\n*****************\n{}: {}\n{}: {}\n{}: {}\n*****************\n",
              URL_REQUEST,url,
              USERNAME_REQUEST,uname,
              PASSWORD_REQUEST,pwd );
}

fn terminate_app()
{
    println!("exiting app...\n");
}

fn choose_application_path()
{
    let mut app_state = 
    [
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
        ApplicationStates::Idle,
    ];

    // set all empty elements to Idle
    for i in 0..TOTAL_APP_STATES
    {
        app_state[i] = ApplicationStates::Idle;
    }

    // get the path to follow
    let application_path = process_application_path_input( request_vault_credentials(APPLICATION_PATH_REQUEST) );

    // load the path states
    match application_path
    {
        ApplicationPath::Exit =>
        {
            app_state[0] = ApplicationStates::ExitApp;
        },

        ApplicationPath::Store =>
        {
            app_state[0] = ApplicationStates::GetUrl;
            app_state[1] = ApplicationStates::GetUname;
            app_state[2] = ApplicationStates::GetUrlAndUnameHash;
            app_state[3] = ApplicationStates::GetPwd;
            app_state[4] = ApplicationStates::GetWalletID;
            app_state[5] = ApplicationStates::GetSecretKey;
            app_state[6] = ApplicationStates::GetIdAndSecretHash;
            app_state[7] = ApplicationStates::EncryptPwd;
            app_state[8] = ApplicationStates::SavePwd;
            app_state[9] = ApplicationStates::ExitApp;
        },

        ApplicationPath::Retrieve =>
        {
            app_state[0] = ApplicationStates::GetUrl;
            app_state[1] = ApplicationStates::GetUname;
            app_state[2] = ApplicationStates::GetUrlAndUnameHash;
            app_state[3] = ApplicationStates::GetWalletID;
            app_state[4] = ApplicationStates::GetSecretKey;
            app_state[5] = ApplicationStates::GetIdAndSecretHash;
            app_state[6] = ApplicationStates::RetrievePwd;
            app_state[7] = ApplicationStates::DecryptPwd;
            app_state[8] = ApplicationStates::DisplayCredentials;
            app_state[9] = ApplicationStates::ExitApp;
        },
    };

    // declare an empty vault instance
    let mut vault_instance = Vault
    {
        url: String::from(""),
        username: String::from(""),
        password: String::from(""),
        vault_hash: String::from(""),
        encrypted_password: String::from(""),
        wallet_id: String::from(""),
        secret_key: String::from(""),
        secret_hash: String::from(""),
    };

    // loop through loaded states
    for i in 0..app_state.len()
    {
        match app_state[i]
        {
            ApplicationStates::GetUrl => 
            {
                vault_instance.url = request_vault_credentials(URL_REQUEST);

            },
            ApplicationStates::GetUname => 
            {
                vault_instance.username = request_vault_credentials(USERNAME_REQUEST);
            },
            ApplicationStates::GetUrlAndUnameHash => 
            {
                assert!( vault_instance.url != "", "state called with empty values");
                assert!( vault_instance.username != "", "state called with empty values");
                let vault_string = generate_credentials_json_string( URL_JSON_KEY,vault_instance.url.clone(),
                                                                     USERNAME_JSON_KEY,vault_instance.username.clone());
                vault_instance.vault_hash = generate_hash(vault_string);
            },
            ApplicationStates::GetPwd => 
            {
                vault_instance.password = request_vault_credentials(PASSWORD_REQUEST);
            },
            ApplicationStates::GetWalletID =>
            {
                vault_instance.wallet_id = request_vault_credentials(WALLET_ID_REQUEST);
            },
            ApplicationStates::GetSecretKey =>
            {
                vault_instance.secret_key = request_vault_credentials(SECRET_KEY_REQUEST);
            },
            ApplicationStates::GetIdAndSecretHash =>
            {
                assert!( vault_instance.wallet_id != "", "state called with empty values");
                assert!( vault_instance.secret_key != "", "state called with empty values");
                let secret_string = generate_credentials_json_string( WALLET_ID_JSON,vault_instance.wallet_id.clone(),
                                                                      SECRET_KEY_JSON_KEY,vault_instance.secret_key.clone());
                vault_instance.secret_hash = generate_hash(secret_string);
            },
            ApplicationStates::EncryptPwd =>
            {
                assert!( vault_instance.secret_hash != "", "state called with empty values");
                assert!( vault_instance.password != "", "state called with empty values");
                let key = generate_hash( format!("{}{}",vault_instance.vault_hash.clone(), vault_instance.secret_hash.clone()) );
                vault_instance.encrypted_password = encrypt_password( vault_instance.password.clone(), key );
            },
            ApplicationStates::SavePwd =>
            {
                assert!( vault_instance.encrypted_password != "", "state called with empty values");
                assert!( vault_instance.vault_hash != "", "state called with empty values");
                store_encrypted_password( vault_instance.encrypted_password.clone(), 
                                          vault_instance.vault_hash.clone() );
            },
            ApplicationStates::RetrievePwd =>
            {
                assert!( vault_instance.vault_hash != "", "state called with empty values");
                vault_instance.encrypted_password = retrieve_encrypted_password( vault_instance.vault_hash.clone() );
            },
            ApplicationStates::DecryptPwd =>
            {
                assert!( vault_instance.vault_hash != "", "state called with empty values");
                assert!( vault_instance.secret_hash != "", "state called with empty values");
                assert!( vault_instance.encrypted_password != "", "state called with empty values");
                let key = generate_hash( format!("{}{}",vault_instance.vault_hash.clone(), vault_instance.secret_hash.clone()) );
                vault_instance.password = decrypt_password( vault_instance.encrypted_password.clone(), key)
            },
            ApplicationStates::DisplayCredentials =>
            {
                assert!( vault_instance.url != "", "state called with empty values");
                assert!( vault_instance.username != "", "state called with empty values");
                assert!( vault_instance.password != "", "state called with empty values");
                show_credentials( vault_instance.url.clone(), vault_instance.username.clone(), vault_instance.password.clone() );
            },
            ApplicationStates::ExitApp =>
            {
                terminate_app();
                break;
            },
            ApplicationStates::Idle =>
            {
                terminate_app();
                break;
            },
        }
    }

    /*
    // for debug only
    println!("url: {}\nusername: {}\npassword: {}\nvault_hash: {}\nencrypted_password: {}\nwallet_id: {}\nsecret_key: {}\nsecret_hash: {}",
                vault_instance.url,
                vault_instance.username,
                vault_instance.password,
                vault_instance.vault_hash,
                vault_instance.encrypted_password,
                vault_instance.wallet_id,
                vault_instance.secret_key,
                vault_instance.secret_hash);
    */
}

fn main() 
{
    println!("{}",SPLASH_SCREEN);
    choose_application_path();
}
