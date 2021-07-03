use std::io::{self, Read, Write};
use std::process;

struct Args {
    shader_kind: shaderc::ShaderKind,
    target_env: shaderc::TargetEnv,
    debug: bool,
}

impl Args {
    fn parser() -> impl meap::Parser<Item = Self> {
        meap::let_map! {
            let {
                shader_kind = meap::choose_at_most_one! {
                    flag("fragment").desc("fragment shader").some_if(shaderc::ShaderKind::Fragment),
                    flag("vertex").desc("vertex shader").some_if(shaderc::ShaderKind::Vertex),
                }.required_general("choose a type of shader (--fragment or --vertex)");
                target_env = meap::choose_at_most_one! {
                    flag("vulkan").desc("target vulkan").some_if(shaderc::TargetEnv::Vulkan),
                    flag("opengl").desc("target opengl").some_if(shaderc::TargetEnv::OpenGL),
                    flag("opengl-compat").desc("target opengl-compat").some_if(shaderc::TargetEnv::OpenGLCompat),
                }.required_general("choose a target env (--vulkan, --opengl or --opengl-compat");
                debug = flag("debug");
            } in {
                Args { shader_kind, target_env, debug }
            }
        }
    }
}

fn main() {
    env_logger::init();
    use meap::Parser;
    let Args {
        shader_kind,
        target_env,
        debug,
    } = Args::parser().with_help_default().parse_env_or_exit();
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read from stdin");
    let mut compiler = shaderc::Compiler::new().expect("failed to instantiate shader compiler");
    let mut compile_options =
        shaderc::CompileOptions::new().expect("falied to initialize compile options");
    let env_version = match target_env {
        shaderc::TargetEnv::Vulkan => shaderc::EnvVersion::Vulkan1_2,
        shaderc::TargetEnv::OpenGL | shaderc::TargetEnv::OpenGLCompat => {
            shaderc::EnvVersion::OpenGL4_5
        }
    };
    compile_options.set_target_env(target_env, env_version as u32);
    if debug {
        compile_options.set_generate_debug_info();
    }
    compile_options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    compile_options.set_warnings_as_errors();
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
