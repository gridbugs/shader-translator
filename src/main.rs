use std::io::{self, Read, Write};
use std::process;

struct Args {
    shader_kind: shaderc::ShaderKind,
}

impl Args {
    fn parser() -> impl meap::Parser<Item = Self> {
        meap::let_map! {
            let {
                shader_kind = meap::choose_at_most_one! {
                    flag('f').name("fragment").desc("fragment shader").some_if(shaderc::ShaderKind::Fragment),
                    flag('v').name("vertex").desc("vertex shader").some_if(shaderc::ShaderKind::Vertex),
                }.required_general("choose a type of shader (--fragment or --vertex)");
            } in {
                Args { shader_kind }
            }
        }
    }
}

fn main() {
    env_logger::init();
    use meap::Parser;
    let Args { shader_kind } = Args::parser().with_help_default().parse_env_or_exit();
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read from stdin");
    let mut compiler = shaderc::Compiler::new().expect("failed to instantiate shader compiler");
    match compiler.compile_into_spirv(&buffer, shader_kind, "stdin", "main", None) {
        Ok(artefact) => io::stdout()
            .write_all(artefact.as_binary_u8())
            .expect("failed to write to stdout"),
        Err(error) => {
            eprint!("{}", error);
            process::exit(1);
        }
    }
}
