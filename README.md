Nft mint contract for solana.
solana-test-validator //run the rpc srver in local... the port is 8899
solana config set --url localhost // set the url to localhost.
cargo build-sbf // build the source file using sbf compiler.
solana program deploy solana_nft_native.so // from target/deploy directory, you can run this command . it means to deploy the program to dev net -- local..
solana logs --url localhost // you can check the logs from request to the client.

