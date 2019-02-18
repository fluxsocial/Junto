# Junto Rust Application
## Application Design Notes
This application is still in its very early stages and is likely to change dramatically as holochain develops further along with development abilities/understanding at Junto.

Currently there is only one zome with a module for each expression possibility on Junto. Expression on Junto is broken down into the following modules: channel, group, user & post.

When we started to initially think about how the DHT structure would look for Junto we wanted to keep its blueprint as open and expandable as possible. We wanted to ensure we could change the functionality of the application as easily as possible. We also wanted to ensure a maximum level of "queryability". 

First we began to consider every kind of interaction on Junto as one of the same thing - an expression. We also realised that when expression comes into contact with one another, a link should be made, ensuring that all data is connected and thus searchable through its natural path of interaction, where allowed. This started to lead to some interesting strategies for the linking of data and the code to make this happen. More to follow...

## Code Related Notes

In order to provide this maximum "queryability" we discovered we need links, a lot of links. In order to manage the links we would need we have two helper functions: handle_hooks & handle_contextual links. handle_hooks hooks allows us to call functions to create all links/expression "objects" any time an action happens on our application. handle_contextual allows us to create all the "contextual links" that need to be made in order to make a highly searchable DHT network, a detailed explanation of how this functions will come soon.