target remote 127.0.0.1:1234
file target/x86_64qemu/debug/x86_demo
break _start
set confirm off
display/i $pc
set print asm-demangle on