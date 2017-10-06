
How to follow the Cortex-m-quickstart tutorial and how this diverges

This if for the NRF52DK development kit.

The vagga scripts follow the tutorial up to the point where you begin debugging.
IOW, through the build phase.

Plug DK into USB port first.
Expect (possibly) an app to open showing the device as a removeable drive.
You can just close that app.

Start gdbserver in one terminal:

    JLinkGDBServer -device nRF52832_xxAA -if SWD

In another terminal, cd to the src dir of the project.
If you are in the src dir, gdb will search it for source files.
Gdb must find the source files to display them.
The executable still contains the source file names and line numbers for the assembled code , whether or not gdb has found the source.
For example:

    cd ~/git/rustD*/r*y/demo/src

Start gdb:

    arm-none-eabi-gdb ../target/thumbv7m-none-eabi/debug/demo


Expect gdb to start.  It knows the program executable to be debugged.  Gdb now prompts with >>>

Connect gdb to the target.  In the gdb shell:

    target remote localhost:2331
    
Expect gdb to respond "Remote debugging using localhost:2331"
and, if you are using gdb-dashboard, for the dashboard to appear at the top of the terminal, in color.
It may show state of the target from the last debugging session, which you usually ignore.

Reset the target and upload your executable:

(Aside: gdb is not the bash shell.  It allows no wildcarding in file names.  It does have a history, use the up arrow key.)

     monitor reset
     load
     
Expect gdb to respond "....Start address 0x400, load size 4150
Transfer rate: 68 KB/sec, 1383 bytes/write."

(On the host, you could enter "start".  The target doesn't support that.)
Set a breakpoint at main and run the program:

     tbreak demo::main
     continue
     
If your program is correct, you should see your output or your led blinking...
If not, start over and single step through your program.

If the source code was not found, you may need to step through assembler instructions instead of source code lines.

    step
    stepi  
   
      



