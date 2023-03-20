

# 编译手册

## MacOS

### linux-musl

经测试，如果直接使用MacOS编译，无论使用gnu还是musl，会有太多问题，建议直接使用docker编译linux版本。

只推荐使用`clux/muslrust`，其他的镜像要么能编译成功但是运行报错，要么无法编译

```shell
cd rabbithole/
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release
# upx is an optional option'
upx ./target/x86_64-unknown-linux-musl/release/rabbithole
```



### windows-gnu
```shell
rustup target add x86_64-pc-windows-gnu
brew install mingw-w64

cargo build --release --target x86_64-pc-windows-gnu
```



## ubuntu

### musl（docker编译）

```shell
docker run -v $PWD:/volume --rm -t clux/muslrust cargo build --release
```

### gnu(动态依赖不推荐)

成功率高，但是因为动态依赖，所以不推荐。

```bash
sudo apt-get install -y openssl pkg-config libssl-dev
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu && strip ./target/x86_64-unknown-linux-gnu/release/rabbithole

upx ./target/x86_64-unknown-linux-gnu/release/rabbithole
```



## ~~Windows~~

Fxxk Windows

No Windows

Not Windows