
Experimental example configuration files for Vagga containers for developing in Rust language.

Status:  work in progress.  Some scripts don't work, and include much cruft.

Two example containers:

    - for compiling to the target host in the container
    - for cross-compiling to a different target than the host of the container (e.g. embedded ARM) using Xargo

Use these to quickly install Rust so you play.  (Alternative: use the Rust playground.)

Quick start: testing compiler is installed
-

    - install Vagga
    - clone this repository
    - open a terminal
    - >cd to one of the example directories
    - >vagga test
    - expect possible many minutes of downloading
    - expect "rustc 1.15.1 (021bd294c 2017-02-08)"

Explanation: "test" is a command defined in the vagga.yaml.  The "test" command simply invokes the rust compiler (or cross compiler) with the -v flag.  If so, it prints out its version.  Otherwise you will see something like "Command foo not found. Run vagga without arguments to see the list. "

Vagga and commands
-

Commands are defined in vagga.yaml.  Every command invocation is on a local shell, in the form "vagga foo".  You don't have a Vagga "shell" having a prompt for commands.

The exception is that invoking "vagga" without a command will show you the available (that you have defined in vagga.yaml) commands.

Two commands are Vagga conventions:  (you still must define them, and others will expect you to.)

    - run   run the project
    - test  run the test of the project
    
For a normal SW development project, they would run you application and its test suite.  Here, in these examples, they don't.
    

Flailing
-

When a vagga container configuration script first runs, it does much downloading.  When the script succeeds, it creates a directory .vagga, that holds the contents of the container.

To remove it:

>rm -Rf .vagga


References
-

See my post at https://wordpress.com/post/plashless.wordpress.com/3454




