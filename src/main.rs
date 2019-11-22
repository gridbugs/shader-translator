use simon::Arg;
use std::io::{self, Read, Write};
use std::process;

struct Args {
    shader_kind: shaderc::ShaderKind,
}

impl Args {
    fn arg() -> impl simon::Arg<Item = Self> {
        simon::args_map! {
            let {
                shader_kind = simon::args_choice! {
                    simon::flag("f", "fragment", "fragment shader").some_if(shaderc::ShaderKind::Fragment),
                    simon::flag("v", "vertex", "vertex shader").some_if(shaderc::ShaderKind::Vertex),
                }.required();
            } in {
                Args { shader_kind }
            }
        }
    }
}

fn main() {
    let Args { shader_kind } = Args::arg().with_help_default().parse_env_or_exit();
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
