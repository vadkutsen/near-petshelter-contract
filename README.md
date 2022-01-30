Pet Shelter Contract
====================

This repository includes NEAR smart contract written in Rust programming language.


Step 0: Install near-cli
------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. To install it globally:

    yarn install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `your-sub-account.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `your-sub-account.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` and `YOUR-SUB-ACCOUNT` below with your actual account name):

      near create-account YOUR-SUB-ACCOUNT.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: Make sure the contract works as expected - run tests
------------------------------------------------------------

You can easily test the smart contract code using cargo:

    cargo test -- --nocapture


Step 3: Build the contract
--------------------------

Assuming that all the tests passed ok, you can go ahead and compile the smart contract:

    cargo build --target wasm32-unknown-unknown --release


Step 4: Deploy!
---------------

For development purposes you can use `dev-deploy` command and your smart contract will be deployed to the live NEAR TestNet with a throwaway account:

    near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/YOUR-CONTRACT-NAME.wasm

 When you're ready to make it permanent, here's how:

    near deploy --wasmFile target/wasm32-unknown-unknown/release/YOUR-CONTRACT-NAME.wasm --accountId YOUR_ACCOUNT_HERE


  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli

