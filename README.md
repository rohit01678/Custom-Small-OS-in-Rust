# Custom-Small-OS-in-Rust
Inspiration-Blog.os.

Hello Everyone.

I am currently  working on this Project.
# Compiler
1.You need to install a rust compiler in your code editor,I am using VS code in this case but you use whatever code editor you wantt so let's start this journey of making badass projects to become better at coding.\
2.Install Rust nightly tools for congfig.toml for targeting json file beacause it will not set as a target until you install nightly and install  tool required for your build.

3.After installing QEMU virtual matchine,U need to bootload the main file with nightly tools,I am mentioning the commands below i used to print the hello world program.
# COMMANDS 
for bootloading.

1.cargo +nightly bootimage.

2.qemu-system-x86_64 -drive format=raw,file=target\x86_64-blog_os\debug\bootimage-rust_os.bin -serial stdio.
# Progress vlog and challenges
Day 1&2:Trying to learn the basics of Rust like how to print a word or comment and making  a baic project file of rust and make a project repo for version control of the project to show case the project after completion.

Day 3:Making json file for disabling and enabling important functions in this file we have to add a lot of things which i don't know but it helps in learning more things.

DAY4:Adding new code to the main file which give us text on a virtual matchine from a vga buffer and you need some tools related to it like a virtual matchine like QEMU.

DAY5:Adding some code for text vga buffer, A new rs file is added in sre folder named as "vga_buffer.rs" for printing text on bootloaderscreen.The vga_buffer is not fully finished but it will be finished soon as i get more time to work on this file.

