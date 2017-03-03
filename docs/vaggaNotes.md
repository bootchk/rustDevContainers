
Notes about Vagga


How your source gets in the container

Your project directory (where your source code lives) also lives in the container!  (This fact is obscure in the Vagga documentation.)  I think that Vagga assumes that the present working directory (where you invoked vagga and where vagga finds its configuration file vagga.yaml) is the home of the project.  I think vagga mounts said directory as /work in the container.  Typically any commands in the container that actually build something look for guidance e.g. a Makefile in that directory.

Vagga also creates hidden files in said directory (as viewed from outside the container):  .vagga, and typically .home.  (Use ctrl-H to see hidden files in your GUI file browser.)


Ephemeral parts of the container

This repository should NOT be storing any ephemeral parts of the container i.e. .vagga since that directory is specified in the .gitignore file.



Caching and rebuilding

Vagga understands dependencies and state and won't rebuild the container unless necessary.  However, while you are thrashing to learn (like I did), it seems to repeatedly download etc. (and take a few minutes) until the configuration file is completed without errors?
