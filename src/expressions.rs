use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use std::fmt::Write;

use rapidfuzz::distance::;

#[polars_expr(output_type=Float32)]
fn jarowinkler_similarity(inputs: &Series) -> PolarsResult<Series> {
    let s1 = inputs[0].str()?;
    let s2 = inputs[1].str()?;
}