# Golem Unlimited

Golem Unlimited utilizes **trusted** heterogeneous computing resources which are part time idle. It is meant for data center-like setup (e.g., render farms, or desktops within organisation LAN) where network participants trust each other, but it will also support trusted P2P subnetworks (e.g., distributed team machines).

It features the [hub](gu-hub) acting as a requestor and additional worker nodes in the hub’s trusted network acting as [providers](gu-provider).

Trust within Golem Unlimited subnetwork allows simplifying its design and taking care of only the computation layer. Other components such as economic layers, reputation systems, verification algorithms, and sandboxing (in contrast to the public Golem Network) can be skipped altogether or implemented optionally.

Golem Unlimited joint resources can be used to perform tasks for internal requestor - the hub operator - with no fee. At the same time the hub will be able to expose its subordinate trusted providers to the public [Golem](../../../golem) Network. In such a setting hub will act as a provider and earn GNTs.  

The latter broadens [Golem](../../../golem) reach. Now Golem Node can be a single machine, with Golem Unlimited it can be more complex component, such as a whole subnetwork.

# Use cases
So far we have prepared plugins for two use cases:
* Integer factorization
* Mining 

We will open source those plugins soon.

# Installing and testing

Please bear in mind that Golem Unlimited is in its [Alpha](https://en.wikipedia.org/wiki/Software_release_life_cycle#Alpha) stage.

## binary
To install you can use the [released](../../releases) Ubuntu `deb` and MacOs `dmg` binary packages.

The detailed steps can be found in our demo https://youtu.be/J0LBdg2j6Tk

## from source
To run the hub, go to the `gu-hub` subdir and perform
```
$ cargo run -- -vv server run
```

To run the provider and connect to your hub at 192.168.1.1 go to `gu-provider` subdir and run
```
$ cargo run -- -vv -a 192.168.1.1:61622 server run
```

# Usage

See our demo for sample usage
https://youtu.be/J0LBdg2j6Tk


# Project layout

*  [`gu-actix`]: small util crate defining flatten trait for ActixWeb future
*  [`gu-base`]: implementation of common parts of Provider and Hub servers
*  [`gu-ethkey`]: Ethereum keys management
*  [`gu-event-bus`]: event-bus implementation - publish-subscribe communication between components
*  [`gu-hardware`]: discovery of hardware resources - GPU, disk space, RAM
*  [`gu-hub`]: binary of Hub server
*  [`gu-lan`]: mDNS services discovery
*  [`gu-net`]: network layer of the application
*  [`gu-persist`]: filesystem, persistent storage of files
*  [`gu-provider`]: binary of Provider service
*  [`gu-webapp`]: web application building development tools

[`gu-actix`]: gu-actix
[`gu-base`]: gu-base
[`gu-ethkey`]: gu-ethkey
[`gu-event-bus`]: gu-event-bus
[`gu-hardware`]: gu-hardware
[`gu-hub`]: gu-hub
[`gu-lan`]: gu-lan
[`gu-net`]: gu-net
[`gu-persist`]: gu-persist
[`gu-provider`]: gu-provider
[`gu-webapp`]: gu-webapp


# How to Contribute to Unlimited
[Here](../../wiki/Contributing) you can find information in order to give us feedback  and contribute to the project.
