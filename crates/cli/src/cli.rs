use super::CliInput;
use anyhow::Result;
use compiler::{
    codegen::{self, CodeGenerator},
    converter::{self, Converter},
    parser, scanner,
    transformer::{self, Transformer},
    compiler::get_base_pass,
};
use serde_yaml::to_writer;
use std::io;

pub(super) fn compile_to_stdout(debug: CliInput) -> Result<()> {
    let (source, option, show) = debug;
    let sfc_info = Default::default();
    let pass = get_base_pass(&sfc_info, &option);
    let eh = || option.error_handler.clone();

    let scanner = scanner::Scanner::new(option.scanning());
    let tokens = scanner.scan(&source, eh());
    if show.dump_scan {
        let tokens: Vec<_> = scanner.scan(&source, eh()).collect();
        println!(r#"============== Tokens ============="#);
        let stdout = io::stdout();
        to_writer(stdout.lock(), &tokens)?;
        println!(r#"========== End of Tokens =========="#);
    }

    let parser = parser::Parser::new(option.parsing());
    let ast = parser.parse(tokens, eh());
    if show.dump_parse {
        println!(r#"=============== AST =============="#);
        let stdout = io::stdout();
        to_writer(stdout.lock(), &ast)?;
        println!(r#"=========== End of AST ==========="#);
    }

    let converter = converter::BaseConverter {
        err_handle: eh(),
        sfc_info: Default::default(),
        option: option.converting(),
    };
    let mut ir = converter.convert_ir(ast);
    if show.dump_convert {
        println!(r#"============= IR ============"#);
        to_writer(io::stdout(), &ir)?;
        println!(r#"========== End of IR ==========="#);
    }

    let mut transformer = transformer::BaseTransformer::new(pass);
    transformer.transform(&mut ir);
    if show.dump_transform {
        println!(r#"======= Transformed ========="#);
        to_writer(io::stdout(), &ir)?;
        println!(r#"======== End of Transform ========"#);
    }

    let mut generator =
        codegen::CodeWriter::new(io::stdout(), option.codegen(), Default::default());
    generator.generate(ir)?;
    Ok(())
}
