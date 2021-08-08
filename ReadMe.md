# Zaryn  Crypto Wallet
A simple wallet for peer 2 peer crypto currency

### Dependencies

* rust and cargo set up
* postgresql database

### Executing program

* set database using diesel and diesel_cli

    ```
    cargo install diesel_cli --no-default-features --features postgres

    ```

* first change your database url in .env file to your database correct url then run database migration file
    ```
    diesel migration run

    ```
* then run the app

    ```
     cargo run

    ```

## How Zaryn Crypto Wallet Works

* Create a Wallet
    Send a post request to http://localhost:4000/v1/wallet as json request body and new generated encryted wallet_address,private_key,public_key as key-value pairs

    ```
      curl -d '{"wallet_address":"value1", "private_key":"value2", "public_key":"value2", }' -H "Content-Type: application/json" -X POST http://localhost:4000/v1/wallet
    
    ```
* Get All Wallet 
 
    Send a get request to http://localhost:4000/v1/wallet 

        ```
        curl -d  -X GET http://localhost:4000/v1/wallet

        ```
* Get A Wallet Information with wallet public key
 
    Send a get request to http://localhost:4000/v1/wallet/wallet_public_key 

        ```
        curl -d  -X GET http://localhost:4000/v1/wallet/info/wallet_public_key

        ```

* Make a Transfer to another wallet
    
    Send a put request to http://localhost:4000/v1/wallet as json request body and sender_wallet_address, receiver_wallet_address, sender_private_key, sender_public_key, amount  
    
    ```
      curl -d '{ "sender_wallet_address": "user1", "receiver_wallet_address": "user2", "sender_key": "generated_key", "sender_public_key": "generated_key", "amount": "450"}' -H "Content-Type: application/json" -X PUT http://localhost:4000/v1/wallet/transfer
    
    ```
* Remove a wallet
 
    Send a delete request to http://localhost:4000/v1/wallet/wallet_address 

        ```
        curl -d  -X DELETE http://localhost:4000/v1/wallet/wallet_address

        ```

* Get All Transactions
 
    Send a get request to http://localhost:4000/v1/transaction 

        ```
        curl -d  -X GET http://localhost:4000/v1/transaction

        ```
* Get all transactions performed by a wallet_address
 
    Send a get request to http://localhost:4000/v1/transaction/wallet_address 

        ```
        curl -d  -X GET http://localhost:4000/v1/transaction/wallet_address

        ```
* Get  transaction information using transaction address, because for each transaction perform transaction address and transaction  
    signature is generated

    Send a get request to http://localhost:4000/v1/transaction/info/transaction_address 

        ```
        curl -d  -X GET http://localhost:4000/v1/transaction/info/transaction_address

        ```

* Service Health Check
        Send a get request to http://localhost:4000/health
        ```
        curl -d  -X GET http://localhost:4000/v1/health

        ```