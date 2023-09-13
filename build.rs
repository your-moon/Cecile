fn main() {
    lalrpop::Configuration::new()
        .use_cargo_dir_conventions()
        .process_file("./src/cc_parser/grammar.lalrpop")
        .expect("failed to process grammar.lalrpop");
}
