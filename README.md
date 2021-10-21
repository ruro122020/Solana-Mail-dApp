# Notes

## Rust

1. In `state.rs` we use vectors for MailAccounts.
   - A vector allows us to store more than one value in a single data structure that puts all the values next to each other in memory.
   - Vectors can only store values of the same type. If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector.
   - Vectors are useful in situation in which you have a list of items.
2. In `instruction.rs` the `Self` keyword is used to refer to the implementing type within [traits](https://doc.rust-lang.org/reference/items/traits.html) and (implementations)[https://doc.rust-lang.org/reference/items/implementations.html].

- `Self` can only be used as the first segment, without a preceding `::`
  ```
  trait T {
   type Item;
   const C: i32;
   // `Self` will be whatever type that implements `T`.
   fn new() -> Self;
   // `Self::Item` will be the type alias in the implementation.
   fn f(&self) -> Self::Item;
  }
  struct S;
  impl T for S {
   type Item = i32;
   const C: i32 = 9;
   fn new() -> Self {           // `Self` is the type `S`.
       S
   }
   fn f(&self) -> Self::Item {  // `Self::Item` is the type `i32`.
       Self::C                  // `Self::C` is the constant value `9`.
   }
  }
  ```

## entrypoint, programs, and accounts

1. The `entrypoint` is the programs point of entry
2. All calls to the program goes through the `process_instruction` function.

The arguments in process_instruction:

- `program_id` (program's identifier) is the public key of our program
- `accounts` - Solana programs are stateless, so to store data solana uses `accounts`
- `instruction_data` is the data passed to the program by the calling code

## Code structure

```
├─ src
│  ├─ lib.rs -> registering modules
│  ├─ entrypoint.rs -> entrypoint to the program
│  ├─ instruction.rs -> program API, (de)serializing instruction data
│  ├─ processor.rs -> program logic
│  ├─ state.rs -> program objects, (de)serializing state
│  ├─ error.rs -> program specific errors
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ Xargo.toml
```

The program's flow:

      - `entrypoint` fowards incoming calls to the `processor`
      - The `processor` decodes the `instruction_data` using function from `instruction.rs`
      - After decoding the data, the `processor` will then use one of the prepared functions to handle the request
      - `state.rs` contains models of the data used in the project

1. When a user creates an account on our program a few things are generated. A Program Derived Address (PDA) from the user's wallet's public key, a seed phrase, and our program's public key.

2. The PDA doesn't only act as the account id but also as a store for the user's data, this is where all the mail the user has sent and received will be stored.

   - The PDA account is used in our program as the user's account id.

_Ownership Explained_

EVERYTHING IS AN ACCOUNT IN SOLANA.

- A program lives in an account.
- Users data has to be stored in a different account from the program.
- Every account that has data that needs to be modified, accessed, used, debited, etc. must be "owned" by a program. This means that a program must own a user's account to be able to send, receive, modify, manipulate or do anything in a holders account.
- In solana a holder is a user who is in possession and has access of their private keys of their account.

3. The process to send mail.
   User 1 sends mail to User 2.

   - The mail data from user 1 is sent as a request to our program
   - The program's entrypoint function fowards the request and all its data to the `processor`.
   - The `processor` decodes the data with the help of `instructions.rs` and the decoding logic in `state.rs`
   - Finally, the `processor` calls a function to save the mail data, that was sent in the request, to the inbox of the receiver's account.

## State, part 1

4. Data in accounts are stored in Uint8Array format. This means if we want to store the string "Solana is awesome" in an
   account's data, we will need to serialize the string. This is done by converting each character in the string to decimals following the UTF-8 format. For example, the string "Solana is awesome" will be serialized into a Uint8Array as "[83, 111, 108, 97, 110, 97, 32, 105, 115, 32, 97, 119, 101, 115, 111, 109, 101]"

5. To serialize data in an account we use the Borch crate.

## Entrypoint

## Error

### Environment Setup

1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively

```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF

```
$ cargo build-bpf
$ cargo test-bpf
```
