//! Compiles the documentation views into Rust, so a release build renders them
//! without a template interpreter. The generated module is included in `views.rs`.

fn main() {
    const PAGE: &str = "crate::nav::DocPage";
    propel_compiler::build::Views::new("resources/views")
        .view("docs.introduction", PAGE)
        .view("docs.install", PAGE)
        .view("docs.connecting", PAGE)
        .view("docs.models", PAGE)
        .view("docs.queries", PAGE)
        .view("docs.transactions", PAGE)
        .view("docs.errors", PAGE)
        .view("docs.migrations", PAGE)
        .view("docs.blocking", PAGE)
        .view("docs.differences", PAGE)
        .view("home", PAGE)
        .view("errors.not_found", PAGE)
        .build();
}
