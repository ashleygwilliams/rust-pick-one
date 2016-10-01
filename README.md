# rust-pick-one

Picks one random element from an array

## Usage

In `Cargo.toml`:

```toml
[dependencies]
pick-one = "1.0.0"
```

In your `lib.rs` or `main.rs`:

```rust
extern crate pick_one;

use pick_one;

let random_choice = pick_one::pick_one_str(&["doggo"]);
```
