# Custom-Small-OS-in-Rust
Inspiration-Blog.os.

Hello Everyone.

I am currently  working on this Project.
# Compiler
1.You need to install a rust compiler in your code editor,I am using VS code in this case but you use whatever code editor you wantt so let's start this journey of making badass projects to become better at coding.

2.Install Rust nightly tools for congfig.toml for targeting json file beacause it will not set as a target until you install nightly and install  tool required for your build.

3.After installing QEMU virtual matchine,U need to bootload the main file with nightly tools,I am mentioning the commands below i used to print the hello world program.

4.You need to install all the tools for executing the commands below but don't worry I will soon add dockers for this after a finish this project.
# COMMANDS 
cargo run //For running virtual machine.

cargo test //For running tests.
# Progress vlog and challenges
Day 1&2:Trying to learn the basics of Rust like how to print a word or comment and making  a baic project file of rust and make a project repo for version control of the project to show case the project after completion.

Day 3:Making json file for disabling and enabling important functions in this file we have to add a lot of things which i don't know but it helps in learning more things.

DAY4:Adding new code to the main file which give us text on a virtual matchine from a vga buffer and you need some tools related to it like a virtual matchine like QEMU.

DAY5:Adding some code for text vga buffer, A new rs file is added in sre folder named as "vga_buffer.rs" for printing text on bootloaderscreen.The vga_buffer is not fully finished but it will be finished soon as i get more time to work on this file.

DAY6:Learning more about rust programming for complete understanding of code and concept of further development and debugging.

DAY7:Fix the code and dependency on long command for running QEMU and now it can run  by "cargo run"

DAY8&9:Completed the vga text buffer code to print text in our virtual machine and add println fuction to print panic and hello world without any lengthy code.

DAY10:Making main modifiactation for testing in the rust kernal because we cannot use the standard library for our kernal testing.