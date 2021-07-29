
# Learn-rCore
稍微对rCore的源码做了一些简化，去掉了makefile, 不用make而是使用cargo exec去组合一些命令，这样更rust原生，然后还加上了单元测试环境，以及cargo runner配置，这样就可以用cargo命令直接运行了，像是普通的rust项目一样。

### 必须的依赖
1. qemu-system-risvc64 5.0.0以上
2. rust(随便先安装一个rust，不限版本，然后clone代码之后再rustup show就会自动安装一些依赖)
3. cargo-binutils (这个需要手动安装，cargo install cargo-binutils --vers ~0.2)
4. riscv64-unknown-elf-gdb(用于gdb调试，教程 [gdb调试](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html#gdb))

### 可选依赖：
1. cargo exec [repo](https://github.com/dunmengjun/cargo-exec) clone完之后
```sh
cargo install --path .
```
就会把exec命令安装到你的path中

## 运行：
```sh
cargo run
```

## 单元测试:
```sh
cargo test
```

ps: 调试请看教程 [gdb调试](https://rcore-os.github.io/rCore-Tutorial-Book-v3/chapter0/5setup-devel-env.html#gdb)
