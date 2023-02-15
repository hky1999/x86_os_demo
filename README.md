# X86-demo

This is just a test demo to build a simple bare-metal x86 application.

Mainly refer to the implementation in https://github.com/phil-opp/blog_os/tree/post-11 .

Currently it can run on qemu-system-x86_64 with the support of bootloader and bootimage.

We need to add a dependency:

```
# in Cargo.toml
[dependencies]
bootloader = "0.9.23"
```

To install the bootimage tool, execute the following command in your terminal:

```
cargo install bootimage
```

After installing bootimage and adding the llvm-tools-preview component, we can create a bootable disk image by executing:
```
cargo bootimage
```

For build and run commands, see `Makefile` for details.

## ref:
* hermit-core
  * https://github.com/ssrg-vt/libhermitMPK/tree/master/loader
  * https://github.com/hermitcore/rusty-loader
* rCore
  * https://github.com/rcore-os/rboot
* Philipp Oppermann's blog
  * https://os.phil-opp.com/zh-CN/minimal-rust-kernel/
