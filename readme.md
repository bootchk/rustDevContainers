
Example containers for developing in Rust language.

Configuration files for Vagga containers.

Status:  work in progress, I'm just learning it myself.  I expose almost everything I work on in Git, much of it is experimental.

Two example containers:
- one for compiling to the target host in the container
- one for cross-compiling to a different target than the host of the container (e.g. embedded ARM) using Xargo

Use these to quickly get a non-invasive development environment for programming in Rust.  (Alternative: use the Rust playground.)

Quick start:
- install Vagga
- clone this repository
- open a terminal
- >cd to one of the example directories
- >vagga test  (or testCargo, or testXargo or any other defined  command)
- expect a minutes-long download and install, then to see the output from "rustc --version" or "xargo --version"

In the future:
- >vagga run
- expect "Hello world" or LED to blink (embedded.)

See my post at https://wordpress.com/post/plashless.wordpress.com/3454

To install vagga:

    echo 'deb [arch=amd64 trusted=yes] https://ubuntu.zerogw.com vagga main' | sudo tee /etc/apt/sources.list.d/vagga.list
    sudo apt-get update
    sudo apt-get install vagga




