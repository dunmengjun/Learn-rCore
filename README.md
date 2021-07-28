
# Learn-rCore

### 必须的依赖
1. qemu-system-risvc64 5.0.0以上
2. rust(随便先安装一个rust，不限版本，然后clone代码之后再cargo show就会自动安装一些依赖)
3. cargo-binutils (这个需要手动安装，cargo install cargo-binutils --vers ~0.2)
4. riscv64-unknown-elf-gdb(用于gdb调试，教程 [gdb调试](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html#gdb))

### 可选依赖：
1. exec (安装cargo install exec，其实在工程中有替代的，就是script-runner工程，可直接进script-runner目录，然后)
```sh
cargo install --path .
```

## 运行：
```sh
cargo run
```

## 单元测试:
```sh
cargo test
```

ps: 调试请看教程 [gdb调试](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html#gdb)
