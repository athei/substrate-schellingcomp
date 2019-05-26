# SchellingComp

This is a substrate Node hosting the SRML-based `schellingcomp` Module that allows
offloading non-verifiable computations to untrusted nodes. It is based around the
[Schellingcoin](https://blog.ethereum.org/2014/03/28/schellingcoin-a-minimal-trust-universal-data-feed/) ideas
as described by Vitalik Buterin.


# Run Tests
```
cargo test --package schellingcomp-runtime
```

# View Documentation
```
cargo doc  --package schellingcomp-runtime --open
```