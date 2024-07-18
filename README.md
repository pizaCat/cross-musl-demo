Demo project for a compilation failure with the latest cross (7b79041).

Cross must be installed with 
```sh
cargo install cross --git https://github.com/cross-rs/cross.git --rev 7b79041`
```

The following fails:
```sh
cargo clean
cross build --target x86_64-unknown-linux-musl
```

With the following error:
```
error: failed to run custom build command for `rdkafka-sys v4.7.0+2.3.0`

Caused by:
  process didn't exit successfully: `/target/debug/build/rdkafka-sys-f2824ebbed8581bd/build-script-build` (exit status: 101)
  --- stdout
  Configuring and compiling librdkafka
  CMAKE_TOOLCHAIN_FILE_x86_64-unknown-linux-musl = None
  CMAKE_TOOLCHAIN_FILE_x86_64_unknown_linux_musl = Some("/opt/toolchain.cmake")
  CMAKE_GENERATOR_x86_64-unknown-linux-musl = None
  CMAKE_GENERATOR_x86_64_unknown_linux_musl = None
  TARGET_CMAKE_GENERATOR = None
  CMAKE_GENERATOR = None
  CMAKE_PREFIX_PATH_x86_64-unknown-linux-musl = None
  CMAKE_PREFIX_PATH_x86_64_unknown_linux_musl = None
  TARGET_CMAKE_PREFIX_PATH = None
  CMAKE_PREFIX_PATH = None
  CMAKE_x86_64-unknown-linux-musl = None
  CMAKE_x86_64_unknown_linux_musl = None
  TARGET_CMAKE = None
  CMAKE = None
  running: cd "/target/x86_64-unknown-linux-musl/debug/build/rdkafka-sys-e2f2066daeed4ea9/out/build" && CMAKE_PREFIX_PATH="/target/x86_64-unknown-linux-musl/debug/build/openssl-sys-994c1be9da62cf5b/out/openssl-build/install:" "cmake" "/home/marsad1/.cargo/registry/src/index.crates.io-6f17d22bba15001f/rdkafka-sys-4.7.0+2.3.0/librdkafka" "-DRDKAFKA_BUILD_STATIC=1" "-DRDKAFKA_BUILD_TESTS=0" "-DRDKAFKA_BUILD_EXAMPLES=0" "-DCMAKE_INSTALL_LIBDIR=lib" "-DWITH_ZLIB=0" "-DWITH_CURL=0" "-DWITH_SSL=1" "-DWITH_SASL_SCRAM=1" "-DWITH_SASL_OAUTHBEARER=1" "-DWITH_SASL=0" "-DWITH_ZSTD=0" "-DENABLE_LZ4_EXT=0" "-DCMAKE_TOOLCHAIN_FILE=/opt/toolchain.cmake" "-DCMAKE_INSTALL_PREFIX=/target/x86_64-unknown-linux-musl/debug/build/rdkafka-sys-e2f2066daeed4ea9/out" "-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_CXX_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC -m64" "-DCMAKE_BUILD_TYPE=Debug"
  -- The C compiler identification is GNU 9.2.0
  -- The CXX compiler identification is GNU 9.2.0
  -- Detecting C compiler ABI info
  -- Detecting C compiler ABI info - done
  -- Check for working C compiler: /usr/local/bin/x86_64-linux-musl-gcc - skipped
  -- Detecting C compile features
  -- Detecting C compile features - done
  -- Detecting CXX compiler ABI info
  -- Detecting CXX compiler ABI info - done
  -- Check for working CXX compiler: /usr/local/bin/x86_64-linux-musl-g++ - skipped
  -- Detecting CXX compile features
  -- Detecting CXX compile features - done
  -- Looking for pow in m
  -- Looking for pow in m - found
  -- Checking for module 'libsasl2'
  --   No package 'libsasl2' found
  -- Configuring incomplete, errors occurred!
  See also "/target/x86_64-unknown-linux-musl/debug/build/rdkafka-sys-e2f2066daeed4ea9/out/build/CMakeFiles/CMakeOutput.log".

  --- stderr
  Building and linking librdkafka statically
  CMake Error at /usr/local/share/cmake-3.23/Modules/FindPackageHandleStandardArgs.cmake:230 (message):
    Could NOT find OpenSSL, try to set the path to OpenSSL root folder in the
    system variable OPENSSL_ROOT_DIR (missing: OPENSSL_CRYPTO_LIBRARY
    OPENSSL_INCLUDE_DIR)
  Call Stack (most recent call first):
    /usr/local/share/cmake-3.23/Modules/FindPackageHandleStandardArgs.cmake:594 (_FPHSA_FAILURE_MESSAGE)
    /usr/local/share/cmake-3.23/Modules/FindOpenSSL.cmake:578 (find_package_handle_standard_args)
    src/CMakeLists.txt:232 (find_package)


  thread 'main' panicked at /home/marsad1/.cargo/registry/src/index.crates.io-6f17d22bba15001f/cmake-0.1.50/src/lib.rs:1098:5:

  command did not execute successfully, got: exit status: 1

  build script failed, must exit now
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```

When forcing image `0.2.5`, the build succeeds:
```sh
cargo clean
CROSS_TARGET_X86_64_UNKNOWN_LINUX_MUSL_IMAGE=ghcr.io/cross-rs/x86_64-unknown-linux-musl:0.2.5 cross build --target x86_64-unknown-linux-musl
```