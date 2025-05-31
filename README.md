This reository is used to demonstrate and summarize the following thress issue:

1. [release builds using rustc 1.86.0 on macOS Ventura (intel) SDK exhibit incorrect behaviour #140686](https://github.com/rust-lang/rust/issues/140686)
2. [Enabling LTO causes miscompilation on x86_64-apple-darwin #141306](https://github.com/rust-lang/rust/issues/141306)
3. [failure to link due to unknown relocation type in switch tables (aarch64-linux-*) #141737](https://github.com/rust-lang/rust/issues/141737)

> As an aside, ld64 is the classic linker of Apple and ld-prime is the new linker that is bundled with Xcode 15.

## 1. [release builds using rustc 1.86.0 on macOS Ventura (intel) SDK exhibit incorrect behaviour #140686](https://github.com/rust-lang/rust/issues/140686)

✅ This fixed by ld-prime. You can upgrade to Xcode 15. Apple hasn't release the source of ld-prime, cross-build is not working.

This is a bug of ld64. I've documented some analyses in https://github.com/rust-lang/rust/issues/140686#issuecomment-2869525604.

I can verify this on https://github.com/dianqk/rust-140686-141306-141737/actions/runs/15363331241:

```
the expected value is 8
the actual value is 8 with ld-prime
the actual value is 0 with ld64(-ld_classic)
```

## 2. [Enabling LTO causes miscompilation on x86_64-apple-darwin #141306](https://github.com/rust-lang/rust/issues/141306)

✅ This fixed by Xcode 15.3. You can upgrade to the version or build with `RUSTFLAGS="-Clink-arg=-ld_classic"`.

> If you want to use ld64, you may encounter the first issue mentioned above. :\

I haven't analyze this issue yet, but I can see Xcode 15.3 has fixed it: https://github.com/dianqk/rust-140686-141306-141737/actions/runs/15363331239.

## 3. [failure to link due to unknown relocation type in switch tables (aarch64-linux-*) #141737](https://github.com/rust-lang/rust/issues/141737)

❗ Hmm, you can use lld 18.1 (or newer). The NDK 28.0.13004108 already supports `R_AARCH64_GOTPCREL32`.

LLVM 18.1 introduces a new relocation: `R_AARCH64_GOTPCREL32`, in [PR72584](https://github.com/llvm/llvm-project/pull/72584), which the GNU linker doestn't yet support.

The build log of Android: https://github.com/dianqk/rust-140686-141306-141737/actions/runs/15363331249.
