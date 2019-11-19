# Junto Rust Application
## Getting Started

To get started running the Holochain Junto Application it is easiest to use [holoscape](https://github.com/holochain/holoscape).

---

## Development

First install holochain. This is done using nix-os, the guide to do that can be found [here](https://developer.holochain.org/docs/install/).
To run our holochain application with a http interface, run the following command: `hc run --interface http`. That will run a developer container of our holochain application at port `8888`. This will be local only and wont gossip with anyone else on the network.

To find out how to make the appropriate HTTP requests to our application please view: https://developer.holochain.org/docs/guide/json_rpc_http/

Documentation of HTTP endpoints can be seen [here](./docs/)

---

## Current Status: Pre-Alpha (unstable)