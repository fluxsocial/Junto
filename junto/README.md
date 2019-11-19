# Junto Rust Application
## Application Design Notes
This application is still in its very early stages and is likely to change dramatically as holochain develops further along with development abilities/understanding at Junto.

Currently there is only one zome with a module for each expression possibility on Junto. Expression on Junto is broken down into the following modules: channel, group, user & post.

When we started to initially think about how the DHT structure would look for Junto we wanted to keep its blueprint as open and expandable as possible. We wanted to ensure we could change the functionality of the application as easily as possible. We also wanted to ensure a maximum level of "queryability". 

## Code Related Notes

In order to provide this maximum "queryability" we discovered we need links, a lot of links. In order to manage the links we would need we have two helper functions: handle_hooks & handle_contextual links. handle_hooks hooks allows us to call functions to create all links/expression "objects" any time an action happens on our application. handle_contextual allows us to create all the "contextual links" that need to be made in order to make a highly searchable DHT network, a detailed explanation of how this functions will come soon.

## Getting Started

To get started running the Holochain Junto Application on your computer you will need to make sure that you have, hc and holochain installed on your system and in line with the current version we use: `0.0.12-alpha1`.
You must also have rust and cargo installed. The best and easiest way to manage your rust versions is using rustup. You can install with the following command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Your rust version should be: `nightly-2019-01-24`. You can install this by running the following:

```
rustup toolchain install nightly-2019-01-24
rustup default nightly-2019-01-24
rustup target add wasm32-unknown-unknown --toolchain nightly-2019-01-24
```

To install HC visit: https://github.com/holochain/holochain-rust/releases Download the correct version and type from your system. Unpackage the downloaded file, you should receive three files: hc, license and readme. 
I recommend making a folder somewhere on your computer called Holochain where you can put all of the hc cli files as they release - so that if needed you can easily switch between hc versions.

Once you have your hc file in your chosen directory you need to next add this to your path. Add the following line to your ./~bash_profile (with the correct path to HC). `export PATH="path/to/hc:$PATH"`. This will allow your bash command line to use hc. After adding this line run `source ~/.bash_profile`.

Next you need to install "Holochain" this is your "Holochain Conductor". You can do so by running the following command: `cargo install holochain --force --git https://github.com/holochain/holochain-rust.git --tag v0.0.12-alpha1`.

To verify you have all the correct dependencies installed you can run: `hc --version` & `holochain --version`. It should output: `hc 0.0.12-alpha1` & `holochain 0.0.12-alpha1`. You can view your rust toolchain version by running: `rustup show` and checking the `active toolchain` section.

Next you need to package our holochain application. You can do this by navigating to the directory where you cloned this repo. Then run `cd junto-rust/` & `hc package`. That should install all needed rust crates and package our application as wasm code to `./dist/junto-rust.dna.json`. Nice work! The Junto holochain application has now been packaged and can be ran from your computer :+1:

## Calling the API

To run our holochain application with a http interface inside `junto-rust` run the following command: `hc run --interface http`. That will run a developer container of our holochain application at port `8888`.

To find out how to make the appropriate HTTP requests to our application please view: https://developer.holochain.org/guide/0.0.12-alpha1/conductor_json_rpc_api.html 

Viewing the `junto-rust/test/` will allow you to see our integration tests. This can be used as a reference to how our API functions.

## Current Status: Pre-Alpha (unstable)