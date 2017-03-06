
Example containers that support developing in Rust language.

Configuration files for Vagga brand of containers.

Status:  work in progress, I'm just learning it myself.  I expose almost everything I work on in Git, much of it is experimental.

Two example containers:
- rustDev: compiling target is identical to host in the container
- rustCrossDev: cross-compiling to a different target than the host of the container (e.g. embedded ARM) using Xargo

Use these to quickly get a non-invasive development environment for programming in Rust.

Alternatives/References:
-

 - use the Rust playground.
 - https://github.com/japaric/cross  a Docker virtual machine for cross-compiling in Rust
 - https://github.com/cbiffle/minimal-embedded-rust a Vagrant virtual machine for "
 - https://users.rust-lang.org/t/feedbackwanted-idea-cargo-wrapper-for-zero-setup-cross-compilation/8141  a thread discussing alternatives


Quick start:
- install Vagga
- clone this repository
- open a terminal
- >cd to one of the example directories
- >vagga   (ask vagga to print list of commands defined in the config file)
- >vagga test  (or testCargo, or testXargo or any other defined  command)
- expect a minutes-long download and install, then e.g. to see the output from "rustc --version" or "xargo --version"

In rustDev:
- >vagga run
- expect "Hello world" 

In rustCrossDev:
- > vaggar run
- > future: expect LED to blink (embedded.)

See my post at https://wordpress.com/post/plashless.wordpress.com/3454

To install vagga:

    echo 'deb [arch=amd64 trusted=yes] https://ubuntu.zerogw.com vagga main' | sudo tee /etc/apt/sources.list.d/vagga.list
    sudo apt-get update
    sudo apt-get install vagga




