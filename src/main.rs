//! oxide-forge CLI entry point.
//!
//! Thin wrapper around the `oxide_forge` library.

use oxide_forge::Runtime;

fn main() {
    let runtime = Runtime::new();
    println!("{} v{} — ready to forge.", Runtime::name(), runtime.version());
}
