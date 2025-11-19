# Open Hypergraphs Dot

A Rust library for visualizing
lax [open hypergraphs](https://crates.io/crates/open-hypergraphs)
using the GraphViz DOT format.

## Get Started

Add the crate with the `svg` feature for simplest use:

    cargo add open-hypergraphs-dot --feature svg

Then you can save an
[Open Hypergraph](https://docs.rs/open-hypergraphs/latest/open_hypergraphs/lax/open_hypergraph/struct.OpenHypergraph.html)
to an SVG in one line:

```rust
use open_hypergraphs_dot::{Options, svg::to_svg_with};
std::fs::write("out.svg", to_svg_with(&term, &Options::default()));
```

If you want non-standard options, you can use the fluent options interface:

```rust
// use defaults in Left-to-right orientation, using Display instance for nodes and edges
let opts = Options::default().display().lr();
std::fs::write("out.svg", to_svg_with(&term, &opts));
```


## Examples

Run the `adder` example:

    cargo run --example adder

This will produce the following depiction of an open hypergraph representing a
2-bit ripple-carry adder:

![](./images/adder.png)

See [./examples/adder.rs](./examples/adder.rs) for the source code producing this example.
