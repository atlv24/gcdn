# gcdn

`gcdn` is an algorithmically faster implementation of variadic GCD.
It outperforms chaining implementations such as `gcd(gcd(a,b), c)`
by over 40% in some workloads.

This is a novel algorithm that I wrote in 2017 as part of some
experiments in C# and ported to Rust in 2021, but have continually
delayed releasing out of hopes of writing a paper about it someday.

The main idea is to sort, and then do some binary-euclidean-algorithm
tricks that use knowledge of all arguments to accelerate evaluation.

Please consult [**the documentation**](https://docs.rs/gcdn) for more information.

Add it to your Cargo.toml:

```toml
[dependencies]
gcdn = "0.1"
```

# Example

```rs
assert_eq!(gcd4(15, 120, 30, 25), 5u32);

// needs mutable access because it runs the algorithm in-place to avoid allocation
assert_eq!(gcdn(&mut [15, 120, 30, 25]), 5u32);
```

## License

`gcdn` is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

### Your contributions

Unless you explicitly state otherwise,
any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license,
shall be dual licensed as above,
without any additional terms or conditions.