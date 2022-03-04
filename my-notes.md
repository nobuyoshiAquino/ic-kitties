# My Notes on the `ic-kitties` Project
This document attempts to describe my experience and thoughts on building this project.

## Step 1. Build and Deploy `substrate-node-template`
The first thing I did was use [substrate-developer-hub/substrate-node-template](https://github.com/substrate-developer-hub/substrate-node-template) as a template for my new [ic-kitties](https://github.com/nobuyoshiAquino/ic-kitties) repository.  
Then, I cloned `ic-kitties` to my local environment and ran the following commands:
```
// To build my project (node + runtime)
$ cargo build --release

// To run it locally
$ cargo run --release -- --dev
```

**to-do**: Explain flag `--release`.  
**to-do**: Explain flag `--` and `--dev`.

So like this, I got a base project that works locally and can be built upon it.

## Step 2. Explore `Runtime` and find `TemplateModule`
I knew two things in advance. First, I knew that `runtime/src/lib.rs` is the file where pallets are added to the runtime through the `construct_runtime!` macro.  
Second, I knew that `substrate-node-template` has a custom pallet that 
works as an example.  
So with this in mind, I searched for the `construct_runtime!` macro and checked that the last pallet added in the `enum Runtime` was `TemplateModule`, the custom pallet I was looking.

I CTRL + F'd `pallet_template` and verified that:
1. It's imported with `pub use pallet_template;`.
2. Its configurations are implemented in `pallet_template::Config for Runtime` block.
3. It's added as `enum Runtime` variant in `construct_runtime!` macro.
4. It's added as parameter in `define_benchmarks!`.

My first instinct was to remove the whole `pallet/template` directory and the code lines listed above. But I decided to keep the directory and comment out items 1 to 4 to exclude the custom pallet from the runtime.  

Extra: I went to [Polkadot App](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.polkadot.io#/explorer) and switched to Development/Local Node 127.0.0.1:9944. Then, in the Developer/Extrinsic section, I checked that the template pallet was no longer available as an option in the pallet selector.  

## Step 3. Create new pallet `kitties`
I created the kitties directory with a Cargo file and a `src/lib.rs` file where the kitties module are defined and decorated with the `pallet` macro. Mandatory `pallet::config` and `pallet::pallet` attributes are defined too. They will be parsed by the `pallet` macro.  
Then, I added the kitties pallet as a dependency to the runtime Cargo file and updated `runtime/src/lib.rs` to include the kitties pallet.
