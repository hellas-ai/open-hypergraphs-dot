use crate::{generate_dot_with, Options};
use open_hypergraphs::lax::OpenHypergraph;

use graphviz_rust::{
    cmd::{CommandArg, Format},
    exec,
    printer::PrinterContext,
};

pub fn to_svg_with<O: PartialEq + Clone, A: PartialEq + Clone>(
    term: &OpenHypergraph<O, A>,
    opts: &Options<O, A>,
) -> Result<Vec<u8>, std::io::Error> {
    let dot_graph = generate_dot_with(term, opts);

    exec(
        dot_graph,
        &mut PrinterContext::default(),
        vec![CommandArg::Format(Format::Svg)],
    )
}
