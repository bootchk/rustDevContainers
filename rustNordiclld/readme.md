

Derived from rustNordicBlinky

Crux:  Changed to use llvm's lld instead of arm-none-eabi-ld linker.
Installs base of Ubuntu artufl version (since its repository contains package lld-5.0)
Installs package lld-5.0

blinky.rs is for Nordic platform.

vagga.yaml builds vagga container for Rust Nordic platform.
Containing a /demo playground.

After running vagga.yaml, the /demo playground will have a "hello" main
and various vagga commands will build it.

Other vagga commands copy blinky.rs (which you can edit)
into demo/src/main.rs and build that custom demo.

link.x is copied from demo/.../..m-rt/out/link.x so that lld can find it
(lld doesn't seem to search for it like ld does, in the dependencies.)
link.x is hacked from the original, since ld syntax checking of scripts seems different.
