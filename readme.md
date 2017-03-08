
Example config files for vagga brand of container, where the created container supports developing in Rust language.
Use these to quickly get a non-invasive development environment for programming in Rust.

Status:  work in progress, some containers don't work yet.  I'm just learning it myself.  I expose almost everything I work on in Git, much of it is experimental.

Example containers:
 - rustDev: compiling target is identical to host in the container (working)
 - cross-compiling to a different target than the host of the container (e.g. embedded ARM)
  + rustCrossDev:  using Xargo (working at least to compile the project, not working to download to target and execute)
  + rustCrossDevRaw: w/o Xargo, instead using dependency on libcore: (not working)

This may be portable to Windows and OSX since vagga is?  But the container contains Linux, so best if you know Linux.

Alternatives/References:
-

 - use the Rust playground.
 - https://github.com/japaric/cross  a Docker virtual machine for cross-compiling in Rust
 - https://github.com/cbiffle/minimal-embedded-rust a Vagrant virtual machine for "
 - https://users.rust-lang.org/t/feedbackwanted-idea-cargo-wrapper-for-zero-setup-cross-compilation/8141  discusses alternatives

References about Rust embedded ARM (sans containers/vm):
 - https://github.com/hannobraun/embedded
 - https://github.com/japaric/f3   STM32 F3 is an ARM Cortex-M4F similar to the chip I want to target?

Using Vagga instead of Vagrant or Docker?  For all alternatives:
- you must install the container/vm tool
- you must learn the container/vm tool's language
- you can quickly delete the containers/vm

I like the Vagga language, and the fact that all configuration of containers is mostly in one script file, instead of spread around in many config files and shell scripts.

One advantage of Vagrant/Dock is that there exist repositories which create containers/vm's for rust embedded.
This aims to do the same for Vagga.

Quick start:
- install Vagga
- clone this repository
- open a terminal
- >cd to one of the example container directories e.g. rustDev
- >vagga   (ask vagga to print list of commands defined in the config file)
- >vagga <command>  (e.g. testCargo, or testXargo or any other defined  command)
- expect a minutes-long download and install, then see the final output from the command

In rustDev:
- >vagga run
- expect "Hello world" 

In rustCrossDev:
- >vagga run
- expect a clean compile and link
- future: expect LED to blink (embedded.)

See my post at https://wordpress.com/post/plashless.wordpress.com/3454

To install vagga:

    echo 'deb [arch=amd64 trusted=yes] https://ubuntu.zerogw.com vagga main' | sudo tee /etc/apt/sources.list.d/vagga.list
    sudo apt-get update
    sudo apt-get install vagga


About the contents and what you can delete:
-

Part of the example:

src/main.rs   part of the example, you should augment
Cargo.toml  manifest of the project/program/app, you should augment

foo.json   target specification, with link flags for bare, no changes usually unless you change ARM architecture from Cortex M4
layout.ld  linker script (.ld) for a specific chip, change it to describe memory and peripherals of your chip

vagga.yml   configuration for vagga, no changes usually unless you rename files e.g. foo.json
.gitignore git config that excludes artifacts (see below) from the repository, no changes usually


Hidden files: delete them at will, vagga will recreate

.vagga  directory created by vagga to store the container, delete it at will, it will be recreated
.home   directory created during container build

Artifacts created by Rust

target   a directory, where Rust caches builds
Cargo.lock



Notes:

Linker flags are in foo.json (the target spec.)
Omitted post link flags   "-lm", "-lgcc", "-lnosys"
Added to pre-link-args  "-nostartfiles",     to omit linking crt0

The Rust book says to add this to cargo.toml to get a crt0 but it might be outdated.
[dependencies]
libc = { version = "0.2.14", default-features = false }





