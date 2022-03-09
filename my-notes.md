# My Notes on the `ic-kitties` Project
This document attempts to describe my experience and thoughts on building this project.  
### :small_red_triangle_down: Important note
- This project is from the [Substrate Runtime Developer Academy](https://www.industryconnect.org/substrate-runtime-developer-academy/) course from Industry Connect.  
I wrote the `Kitties` pallet from scratch using the template project, **but** all the code was already available in advance on Github provided by the course.  
- I'm also planning to review and add some codes from the [`Kitties` tutorial](https://www.youtube.com/watch?v=NrG3co6UWEg) presented by [Parity Technologies](https://www.parity.io/) in ETHDenver 2022.  
<br />

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
<br />

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
<br />

## Step 3. Create `Kitties` pallet :cat2:
Before starting, I should answer some basic questions for my readers. First,  
- What is the `kitties` pallet? and,
- What is a **kitty**?  

A kitty is the representation of a unique collectable virtual cat.  
With the `kitties` pallet, you can **create**, **breed**, **buy** and **sell** kitties.  

**to-do**: Add kitty's pictures here.  
<br />

### User Stories
I rephrased the project's first requirements as user stories.
1. "*As a user, I want to create a kitty and own it.*"  
    1.1. A kitty must have a randomly created DNA property.  
<br />
2. "*As a user, I want to breed a kitty from two kitties of different genders (female + male) that I own.*"  
    2.1. A kitty must have a DNA property created based on its parent's DNA.  
    2.2. A kitty's gender must be derived from its DNA.  
<br />

### Scaffolding
I created the kitties directory with a Cargo file and a `src/lib.rs` file where the kitties module are defined and decorated with the `pallet` macro. Mandatory `pallet::config` and `pallet::pallet` attributes are defined too. They will be parsed by the `pallet` macro.  
Then, I added the kitties pallet as a dependency to the runtime Cargo file and updated `runtime/src/lib.rs` to include the kitties pallet.  
<br />

### Implement `create` kitty call :small_blue_diamond:
Based on user stories' requirements 1.1., a `Kitty` will be defined as a tuple struct with a single field for its DNA.  

I created two storage items: 
- `NextKittyId`, an index value assigned as an identifier for each new kitty created.
- `Kitties`, a key-value mapping where the user id: `<AccountId>` and the kitty id: `<KittyIndex>` are the keys.  

Finally, I created the `create` call where a new kitty is created and inserted in the `Kitties` key-value mapping.  
A `KittyCreated` event is sent on success.  
<br />

### Implement `breed` kitty call :small_blue_diamond:
The `breed` and `create` calls are similar. Both result in the creation of a kitty.   
But, the `breed` call is different from `create` call because:
- I must generate the new kitty's DNA based on its parents' DNA. 
- I have to ensure that the kitties, provided as parameters, have different genders and are owned by the sender.

So, for the `breed` call I created:
  - `KittyGender` enum with two variants: `Female` and `Male`.
  - `gender(&self) -> KittyGender` method to get a kitty's gender (requirement 2.2).
  - `combine_kitties_dna(s: AccountId, k1: Dna, k2: Dna) -> Dna` function to create the new kitty's DNA based on its parents DNA (requirement 2.1).

As stated above, `breed` does the same process of `create` call on
- getting an ID for the new kitty querying the `NextKittyId` storage value,
- generating a DNA,
- inserting the new kitty to the `Kitties` mapping with the user and kitty IDs as keys, and
- emitting an event on success (`KittyCreatedByBreeding`).

I added a couple of unit tests too.  
<br />  

### Implement `transfer` kitty call :small_blue_diamond:
The next feature is defined in the following user story.

3. "*As a user, I want to transfer my kitty to another user.*"  
    3.1. Transfers where the sender and the receiver is the same user, should not end in an error.  

For the `transfer` call, I did not introduce new concepts. It's a simple mutation on kitties mapping where I take (remove) the entry with the old user (sender) and insert a new one with the recipient user. An event is emitted on success.  
I also added unit tests.  
<br />

### Implement `set_price` and `buy` kitty calls :small_blue_diamond:
Now I'm going to implement the last features of the `Kitties` pallet, and the user stories are the following:  

4. "*As a user, I want to put my kitties on sale.*"  
    4.1. User must set a price for the kitty to make it available to the public.

5. "*As a user, I want to take my kitty off the market.*"  
6. "*As a user, I want to buy a kitty.*"
    6.1. Users can only buy listed kitties. In other words, users can't buy kitties that are not for sale.  
    6.2. Users must not be allowed to buy their own listed kitty.  
    6.3. User's bids lower than the seller's request price must result in an error.

I implemented requirements 4 and 5 in the same `set_price` call. A user can call `set_price` to set a kitty's price and make it available. If the kitty is already on sale, the user can either update its price or delist it passing `None` as the price.

To keep track of the kitties on sale, I created the `KittyPrices` storage map that has the kitty's id as key and the price as value. This price value is a `Balance` type.  
**to-do**: Explain `BalanceOf<T>` and its relation with `Balance`, `Currency` and the runtime.  

For requirement 6, the `buy` call, I had to update the `KittyPrices` and `Kitties` maps.  
First, the function had to delete the listed kitty entry on `KittyPrices`. Then in the `Kitties` map, the entry with the old owner (seller) is also deleted. Finally, a new entry with the buyer's id is created. I didn't like the nested `try_mutate_exists` in the function. I'll refactor it later.  
Items 6.1., 6.2. and 6.3. are covered, and error messages: `NotForSale`, `BuyerIsSeller` and `BidPriceTooLow` are thrown, respectively.  
Events and unit tests for `set_price` and `buy` are implemented too.  
<br />

### Weights :stopwatch:
*Status: In progress...*  
<br />

### Open Runtime Module Library (ORML)
*Status: pending*