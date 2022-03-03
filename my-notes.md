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

## Step 2. Remove `TemplateModule` from `Runtime`
**in-progres...**
