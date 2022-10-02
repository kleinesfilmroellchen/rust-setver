# rust-setver

[SetVer](https://github.com/RocketRace/setver) implementation for Rust.

This is the `setver` crate which allows your Rust application or library to comprehend SetVer version specifications. SetVer is a versioning scheme created by Olivia Palmu and based on set theory concepts. It is fantastically simple and powerful, yet entirely unpractical. See the SetVer repository above if you want to learn more.

## Example

```rust
use setver::SetVersion;
let first_version: SetVersion = "{}".parse().unwrap();
let second_version: SetVersion = "{{}}".parse().unwrap();
assert!(first_version.is_subset(&second_version));
```

## Installation & Usage

Add it to your project with Cargo:

```shell
cargo add setver
```

Read the [documentation](https://docs.rs/setver/latest/setver/) for more information about how to use setver.

## Changelog

### 0.1.0

Initial release.

### 0.2.0

Add the `add_child_version` function which allows for easier building of related SetVers.

## License

Only MIT right now, I can't be bothered with Apache at the moment.
