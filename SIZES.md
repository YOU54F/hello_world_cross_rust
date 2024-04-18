
```sh
cargo +nightly build --release -Zbuild-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=abort",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    "-C","strip=symbols",
    "-C","lto=true",
    "-C","opt-level=z",
    "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```

```console
du -sh target/*/release/hello_world
 52K    target/aarch64-apple-darwin/release/hello_world
 24K    target/aarch64-unknown-linux-gnu/release/hello_world
 32K    target/aarch64-unknown-linux-musl/release/hello_world
 28K    target/arm-unknown-linux-gnueabi/release/hello_world
 32K    target/arm-unknown-linux-gnueabihf/release/hello_world
 32K    target/arm-unknown-linux-musleabi/release/hello_world
 32K    target/arm-unknown-linux-musleabihf/release/hello_world
 28K    target/i586-unknown-linux-musl/release/hello_world
 28K    target/i686-unknown-linux-gnu/release/hello_world
 28K    target/i686-unknown-linux-musl/release/hello_world
 68K    target/powerpc64le-unknown-linux-musl/release/hello_world
 24K    target/riscv64gc-unknown-linux-musl/release/hello_world
 32K    target/s390x-unknown-linux-musl/release/hello_world
 52K    target/x86_64-apple-darwin/release/hello_world
 28K    target/x86_64-unknown-linux-gnu/release/hello_world
 28K    target/x86_64-unknown-linux-musl/release/hello_world
```


```sh
cargo +nightly build --release -Zbuild-std=std,panic_abort
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=abort",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    "-C","strip=symbols",
    "-C","lto=true",
    "-C","opt-level=z",
    "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```

```console
 84K    target/aarch64-apple-darwin/release/hello_world
 52K    target/aarch64-unknown-linux-gnu/release/hello_world
 56K    target/aarch64-unknown-linux-musl/release/hello_world
 56K    target/arm-unknown-linux-gnueabi/release/hello_world
 56K    target/arm-unknown-linux-gnueabihf/release/hello_world
 60K    target/arm-unknown-linux-musleabi/release/hello_world
 60K    target/arm-unknown-linux-musleabihf/release/hello_world
 52K    target/i586-unknown-linux-musl/release/hello_world
 52K    target/i686-unknown-linux-gnu/release/hello_world
 52K    target/i686-unknown-linux-musl/release/hello_world
132K    target/powerpc64le-unknown-linux-musl/release/hello_world
 48K    target/riscv64gc-unknown-linux-musl/release/hello_world
 64K    target/s390x-unknown-linux-musl/release/hello_world
 84K    target/x86_64-apple-darwin/release/hello_world
 48K    target/x86_64-unknown-linux-gnu/release/hello_world
 52K    target/x86_64-unknown-linux-musl/release/hello_world
```


```sh
cargo +nightly build --release -Zbuild-std=std
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=unwind",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    "-C","strip=symbols",
    "-C","lto=true",
    "-C","opt-level=z",
    "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```


```console
 88K    target/aarch64-apple-darwin/release/hello_world
 64K    target/aarch64-unknown-linux-gnu/release/hello_world
 72K    target/aarch64-unknown-linux-musl/release/hello_world
 56K    target/arm-unknown-linux-gnueabi/release/hello_world
 56K    target/arm-unknown-linux-gnueabihf/release/hello_world
 68K    target/arm-unknown-linux-musleabi/release/hello_world
 68K    target/arm-unknown-linux-musleabihf/release/hello_world
 68K    target/i586-unknown-linux-musl/release/hello_world
 72K    target/i686-unknown-linux-gnu/release/hello_world
 68K    target/i686-unknown-linux-musl/release/hello_world
132K    target/powerpc64le-unknown-linux-musl/release/hello_world
 64K    target/riscv64gc-unknown-linux-musl/release/hello_world
 80K    target/s390x-unknown-linux-musl/release/hello_world
 84K    target/x86_64-apple-darwin/release/hello_world
 68K    target/x86_64-unknown-linux-gnu/release/hello_world
 68K    target/x86_64-unknown-linux-musl/release/hello_world
```

```sh
cargo +nightly build --release -Zbuild-std=std
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=unwind",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    "-C","strip=symbols",
    # "-C","lto=true",
    "-C","opt-level=z",
    "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```


```console
320K    target/aarch64-apple-darwin/release/hello_world
304K    target/aarch64-unknown-linux-gnu/release/hello_world
312K    target/aarch64-unknown-linux-musl/release/hello_world
272K    target/arm-unknown-linux-gnueabi/release/hello_world
272K    target/arm-unknown-linux-gnueabihf/release/hello_world
280K    target/arm-unknown-linux-musleabi/release/hello_world
280K    target/arm-unknown-linux-musleabihf/release/hello_world
312K    target/i586-unknown-linux-musl/release/hello_world
392K    target/i686-unknown-linux-gnu/release/hello_world
312K    target/i686-unknown-linux-musl/release/hello_world
452K    target/powerpc64le-unknown-linux-musl/release/hello_world
292K    target/riscv64gc-unknown-linux-musl/release/hello_world
316K    target/s390x-unknown-linux-musl/release/hello_world
332K    target/x86_64-apple-darwin/release/hello_world
352K    target/x86_64-unknown-linux-gnu/release/hello_world
324K    target/x86_64-unknown-linux-musl/release/hello_world
```

```sh
cargo +nightly build --release -Zbuild-std=std
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=unwind",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    "-C","strip=symbols",
    # "-C","lto=true",
    # "-C","opt-level=z",
    "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```


```console
308K    target/aarch64-apple-darwin/release/hello_world
288K    target/aarch64-unknown-linux-gnu/release/hello_world
292K    target/aarch64-unknown-linux-musl/release/hello_world
308K    target/arm-unknown-linux-gnueabi/release/hello_world
308K    target/arm-unknown-linux-gnueabihf/release/hello_world
316K    target/arm-unknown-linux-musleabi/release/hello_world
316K    target/arm-unknown-linux-musleabihf/release/hello_world
308K    target/i586-unknown-linux-musl/release/hello_world
364K    target/i686-unknown-linux-gnu/release/hello_world
304K    target/i686-unknown-linux-musl/release/hello_world
452K    target/powerpc64le-unknown-linux-musl/release/hello_world
260K    target/riscv64gc-unknown-linux-musl/release/hello_world
432K    target/s390x-unknown-linux-musl/release/hello_world
320K    target/x86_64-apple-darwin/release/hello_world
328K    target/x86_64-unknown-linux-gnu/release/hello_world
324K    target/x86_64-unknown-linux-musl/release/hello_world
```

```sh
cargo +nightly build --release -Zbuild-std=std
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=unwind",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    "-C","strip=symbols",
    # "-C","lto=true",
    # "-C","opt-level=z",
    # "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```


```console
340K    target/aarch64-apple-darwin/release/hello_world
312K    target/aarch64-unknown-linux-gnu/release/hello_world
316K    target/aarch64-unknown-linux-musl/release/hello_world
328K    target/arm-unknown-linux-gnueabi/release/hello_world
328K    target/arm-unknown-linux-gnueabihf/release/hello_world
336K    target/arm-unknown-linux-musleabi/release/hello_world
336K    target/arm-unknown-linux-musleabihf/release/hello_world
328K    target/i586-unknown-linux-musl/release/hello_world
392K    target/i686-unknown-linux-gnu/release/hello_world
324K    target/i686-unknown-linux-musl/release/hello_world
452K    target/powerpc64le-unknown-linux-musl/release/hello_world
280K    target/riscv64gc-unknown-linux-musl/release/hello_world
464K    target/s390x-unknown-linux-musl/release/hello_world
336K    target/x86_64-apple-darwin/release/hello_world
356K    target/x86_64-unknown-linux-gnu/release/hello_world
352K    target/x86_64-unknown-linux-musl/release/hello_world

```

```sh
cargo +nightly build --release -Zbuild-std=std
```

```toml
    "-Z","location-detail=none",
    "-C", "target-feature=+crt-static", # musl only
    "-C","panic=unwind",
    "-C","relocation-model=pie",  # musl only
    "-C","relocation-model=static",  # musl only
    # "-C","strip=symbols",
    # "-C","lto=true",
    # "-C","opt-level=z",
    # "-C","codegen-units=1",
    "-C","embed-bitcode=true",
```


```console
440K    target/aarch64-apple-darwin/release/hello_world
452K    target/aarch64-unknown-linux-gnu/release/hello_world
464K    target/aarch64-unknown-linux-musl/release/hello_world
464K    target/arm-unknown-linux-gnueabi/release/hello_world
464K    target/arm-unknown-linux-gnueabihf/release/hello_world
476K    target/arm-unknown-linux-musleabi/release/hello_world
476K    target/arm-unknown-linux-musleabihf/release/hello_world
424K    target/i586-unknown-linux-musl/release/hello_world
484K    target/i686-unknown-linux-gnu/release/hello_world
420K    target/i686-unknown-linux-musl/release/hello_world
564K    target/powerpc64le-unknown-linux-musl/release/hello_world
492K    target/riscv64gc-unknown-linux-musl/release/hello_world
560K    target/s390x-unknown-linux-musl/release/hello_world
436K    target/x86_64-apple-darwin/release/hello_world
456K    target/x86_64-unknown-linux-gnu/release/hello_world
460K    target/x86_64-unknown-linux-musl/release/hello_world
```