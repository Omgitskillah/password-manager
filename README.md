# password-manager
This is a simple password manager written in Rust.

## Concept

With the continued development in decentralized ledger technology, one very promising organization [Radix DLT](https://www.radixdlt.com/) has managed to build an affordable and scalable Distributed Ledger.  
This ledger is expected to support decentralized apps written in a modified version of Rust to suit it's native system.  
Because of its expected affordability, it will be possible to run low cost user applications such as password managers that would do well to leverage the forcoming ubiquity of distributed technologies.  

## How it Works
The application only runs on the command line for now but can be wrapped modified to run on a browser, a desktop or on a mobile app. 
The application taked in various credentials and encrypts the password for storage, currently on the local machine and later on the Radix Distribute Ledger.

Donload and run `password-manager.exe` from `password-manager/target/release/`. You may need Admin previlages to run this executable

### How to storing a password works
When you choose the save password option from the application command line menu, you will be required to enter the `URL` or `Application Name` for which you would like to encrypt and store its password. 
You will then be asked to enter the corresponding `username` or `email address` and then finally the strong `*password` you wish to secure. 

You will then be required to provide your `radix wallet ID` as an identifier your strong `Secret key` that only you would know.

All this information will be used to encrypt and store your application's password.

### How to retrieve your password
When you choose to retrieve your password, you will be required to enter the `URL` or `Application Name` for which you would like to retrieve its password.  
You will then be asked to enter the corresponding `username` or `email address` and then

You will then be required to provide your `radix wallet ID` as an identifier your strong `Secret key` that only you would know.

Your password `*if found` will be displayed on the command line.



`NOTES: *password, to store can only be a max of 13 characters in length`  

`NOTES: *if found, it is possible to request a password that doesn't exist, because the application has no knowledge of what is stored after the fact, retrieving the password will necessitate that you provide the exact same credentials you gave while storing your password`
