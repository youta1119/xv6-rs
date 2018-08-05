# xv6-rs
xv6-rs is re-implementation xv6 in rust

## setup

### linux , windows
```bash
cd docker
docker build -t {tag} .
``` 
create `.cargo/config`
```toml
[target.i386-unknown-none]
    linker = "ld.lld-6.0"
    ar = "llvm-ar-6.0"
```
### mac os
1. install llvm 
`brew install llvm`
2. install rust
```bash
$ curl https://sh.rustup.rs -sSf | sh -s -- -y  --default-toolchain nightly
source $HOME/.cargo/env
$ cargo install xargo
$ rustup component add rust-src
```

## run

```
$ cd bootloader 
$ make bootloader
$ qemu-system-i386 -d int -no-reboot  -hda obj/bootloader.bin
```