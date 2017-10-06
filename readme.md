

Experimental, example Vagga configuration files that create Vagga containers for developing in Rust language.
Use these to quickly get a non-invasive development environment for programming in Rust, especially embedded Rust for ARM.

The goal is to have a dev environment that is reproducible and deletable.
Anything you do to set up your dev environment (system administration) goes into a Vagga script.
You can have many dev environments.
You can hack at the dev environments without worrying about root priveleges.

This is particularly appropriate when you:

   - are just learning Rust and embedded programming.
   - are shopping for tools and want to try them in various combinations
   - are using tools that change daily (like embedded Rust tools)

One disadvantage is that you must learn about Vagga and where it puts files.

Status:  work in progress.  Some scripts don't work, and include much cruft.

May be portable to Windows and OSX since vagga is?  But the container contains Linux, so best if you know Linux.


Example vagga scripts for other languages:
-

 - https://github.com/andreaferretti/vagga-examples.git


Example vagga scripts in this repository:
-

 - rustDev: for compiling Rust to the target host in the container
 - rustCrossDev: for cross-compiling Rust to a different target than the host of the container (e.g. embedded ARM) using Xargo


Alternatives/References:
-

  - use the Rust playground.
  - https://github.com/japaric/cross  a Docker virtual machine for cross-compiling in Rust
  - https://github.com/cbiffle/minimal-embedded-rust a Vagrant virtual machine for "
  - https://users.rust-lang.org/t/feedbackwanted-idea-cargo-wrapper-for-zero-setup-cross-compilation/8141  discusses alternatives

References about Rust embedded ARM (sans containers/vm):
  - https://github.com/hannobraun/embedded
  - https://github.com/japaric/f3   STM32 F3 is an ARM Cortex-M4F similar to the chip I want to target?


See my post at https://wordpress.com/post/plashless.wordpress.com/3454


Vagga as an alternative to Vagrant or Docker
-

For all alternatives:

 - you must install the container/vm tool
 - you must learn the container/vm tool's language
 - you can quickly delete the containers/vm

I like the Vagga language.  All configuration of containers is mostly in one script file, instead of spread around in many config files and shell scripts.  Also, Vagga is written in Rust, so using Vagga is eating your own dog food.

One advantage of Vagrant/Dock is that there exist repositories which create containers/vm's for rust embedded.
This repository aims to do the same for Vagga.


Quick start
-

 - install Vagga
 - clone this repository
 - open a terminal
 - >cd to one of the example directories
 - >vagga   (ask vagga to print list of commands defined in the config file)
 - >vagga foo  (e.g. testCargo, or testXargo or any other defined  command)
 - expect possible many minutes of downloading
 - expect see the final output from the command e.g. "rustc 1.15.1 (021bd294c 2017-02-08)"


In rustDev:

 - >vagga run
 - expect "Hello world" 

In rustCrossDev:

 - >vagga run
 - expect a clean compile and link
 - future: expect LED to blink (embedded.)

Vagga and commands
-

Commands are defined in vagga.yaml.  Every command invocation is on a local shell, in the form "vagga foo".  You don't have a Vagga "shell" having a prompt for commands.

Invoking "vagga" without a command will show you the available (that you have defined in vagga.yaml) commands.

Two commands are Vagga conventions:  (you still must define them, and others will expect you to.)

 - run   run the project
 - test  run the test suite of the project
    
For a normal SW development project, they would run you application and its test suite.  Here, in these examples, they don't.
    

Where a container lives
-

When a vagga container configuration script first runs, it does much downloading.  When the script succeeds, it creates a hidden directory .vagga, that holds the contents of the container.  Subsequent runs are faster as long as .vagga exists.

To remove a container:

>rm -Rf .vagga


To install vagga:
-

    echo 'deb [arch=amd64 trusted=yes] https://ubuntu.zerogw.com vagga main' | sudo tee /etc/apt/sources.list.d/vagga.list
    sudo apt-get update
    sudo apt-get install vagga


About the contents and what you can delete:
-

Part of the example:

 - src/main.rs   part of the example, you should augment
 - Cargo.toml  manifest of the project/program/app, you should augment

 - foo.json   target specification, with link flags for bare, no changes usually unless you change ARM architecture from Cortex M4
 - layout.ld  linker script (.ld) for a specific chip, change it to describe memory and peripherals of your chip

 - vagga.yml   configuration for vagga, no changes usually unless you rename files e.g. foo.json
 - .gitignore git config that excludes artifacts (see below) from the repository, no changes usually


Hidden files: delete them at will, vagga will recreate

 - .vagga  directory created by vagga to store the container, delete it at will, it will be recreated

Artifacts created by Rust

 - target   a directory, where Rust caches builds
 - Cargo.lock



Notes about Vagga language
-

The language is vagga configuration scripts:

    - is YAML
    - is domain specific
    
So certain syntax comes from YAML, some from vagga.

For example, | is yaml syntax for "multi-line literal."

For example, !Sh is vagga syntax for a command.


Notes:
-

Linker flags are in foo.json (the target spec.)  Some people put them in cargo.toml
Omitted post link flags   "-lm", "-lgcc", "-lnosys"
Added to pre-link-args  "-nostartfiles",     to omit linking crt0

The Rust book says to add this to cargo.toml to get a crt0 but it might be outdated.
[dependencies]
libc = { version = "0.2.14", default-features = false }





