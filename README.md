# X86-demo

This is just a test demo to build a simple bare-metal x86 application.

Mainly refer to the implementation in https://github.com/phil-opp/blog_os/tree/post-11.

It can be run on qemu-system-x86_64 with https://github.com/hky1999/rboot as bootloader.

For build and run commands, see `Makefile` for details.

## ref:
* hermit-core
  * https://github.com/ssrg-vt/libhermitMPK/tree/master/loader
  * https://github.com/hermitcore/rusty-loader
* rCore
  * https://github.com/rcore-os/rboot
* Philipp Oppermann's blog
  * https://os.phil-opp.com/zh-CN/minimal-rust-kernel/
