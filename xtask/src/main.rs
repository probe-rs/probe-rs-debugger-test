use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use miette::miette;
use miette::Context;
use miette::IntoDiagnostic;
use xshell::cmd;

fn repo_root() -> PathBuf {
    let xtask_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    xtask_dir.parent().unwrap().canonicalize().unwrap()
}

fn main() -> miette::Result<()> {
    let build_command = Command::new("build")
        .arg(Arg::new("cmd").long("command").default_value("build"))
        .arg(
            Arg::new("release")
                .long("release")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("reproducible")
                .long("reproducible")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("testcase")
                .long("testcase")
                .default_value("full_unwind"),
        );

    let ci_build_command = Command::new("ci-build");

    let m = Command::new("xtask")
        .subcommand_required(true)
        .subcommand(build_command)
        .subcommand(ci_build_command)
        .get_matches();

    let sh = xshell::Shell::new().into_diagnostic()?;

    match m.subcommand() {
        Some(("build", matches)) => {
            let command = matches.get_one::<String>("cmd").unwrap();
            let release_build: bool = matches.get_flag("release");
            let reproducible: bool = matches.get_flag("reproducible");
            let test_case = matches.get_one::<String>("testcase").unwrap();

            let settings = BuildSettings {
                release: release_build,
                command: command.clone(),
                reproducible,
            };

            let _ = build(&sh, &settings, &test_case)?;

            Ok(())
        }
        Some(("ci-build", _matches)) => {
            let repo_root = repo_root();

            let settings = BuildSettings {
                release: false,
                command: "build".to_string(),
                reproducible: true,
            };

            let output_dir = repo_root.join("test-binaries");

            if !output_dir.exists() {
                std::fs::create_dir(&output_dir).into_diagnostic()?;
            }

            for test_case in TEST_CASES {
                println!("Building test case {test_case}");
                let built_binaries = build(&sh, &settings, test_case)?;

                // TODO: Copy the binaries into the correct location

                for bin in built_binaries {
                    let file_name = bin.file_name().ok_or_else(|| {
                        miette!("Missing file name for binary in path {}", bin.display())
                    })?;

                    let file_name = file_name.to_string_lossy();

                    let output_filename = format!("{file_name}_{test_case}.elf");

                    let new_path = output_dir.join(output_filename);

                    std::fs::copy(&bin, &new_path)
                        .into_diagnostic()
                        .wrap_err_with(|| {
                            format!(
                                "Failed to copy file from '{}' to '{}'",
                                bin.display(),
                                new_path.display()
                            )
                        })?;
                }
            }

            Ok(())
        }
        _ => unreachable!("Subcommand required settings prevents this."),
    }
}

const TEST_CASES: &[&str] = &[
    "full_unwind",
    "systick",
    "svcall",
    "hardfault_from_usagefault",
    "hardfault_from_busfault",
    "hardfault_in_systick",
];

#[derive(Debug)]
struct BuildSettings {
    release: bool,
    command: String,
    reproducible: bool,
}

#[derive(Debug)]
struct BuildTarget {
    chip: Cow<'static, str>,
    rust_target: Cow<'static, str>,
}

impl BuildTarget {
    const fn new_static(name: &'static str, rust_target: &'static str) -> Self {
        Self {
            chip: Cow::Borrowed(name),
            rust_target: Cow::Borrowed(rust_target),
        }
    }
}

const TARGETS: &[BuildTarget] = &[
    BuildTarget::new_static("nRF52833_xxAA", "thumbv7em-none-eabihf"),
    BuildTarget::new_static("RP2040", "thumbv6m-none-eabi"),
    BuildTarget::new_static("esp32c3", "riscv32imc-unknown-none-elf"),
];

const REMAPPED_CARGO_HOME: &str = "/probe-rs/.cargo";
const REMAPPED_COMPILE_DIR: &str = "/probe-rs/compile-dir";

/// Build the test case for all targets
///
/// Return the paths to the built binaries
fn build(
    sh: &xshell::Shell,
    settings: &BuildSettings,
    test_case: &str,
) -> miette::Result<Vec<PathBuf>> {
    let repo_root = repo_root();

    let cargo_home = std::env::var("CARGO_HOME").unwrap();

    let cargo_home = Path::new(&cargo_home).canonicalize().unwrap();

    let path_mappings = [
        (cargo_home.as_path(), REMAPPED_CARGO_HOME),
        (repo_root.as_path(), REMAPPED_COMPILE_DIR),
    ];

    let flags: Vec<String> = path_mappings
        .into_iter()
        .map(|(from, to)| format!("--remap-path-prefix={}={}", from.display(), to))
        .collect();

    println!("Flags: {:?}", flags);

    let rust_flags = flags.join("\x1f");

    println!("encoded flags: {:?}", flags);

    // Try to create deterministic builds
    if settings.reproducible {
        sh.set_var("CARGO_ENCODED_RUSTFLAGS", rust_flags);
    }

    let mut built_binaries = Vec::new();

    for t in TARGETS {
        println!("Building for {}", t.chip);
        println!(" - Installing rust target {}", t.rust_target);

        let rust_target = t.rust_target.as_ref();

        cmd!(sh, "rustup target add {rust_target}")
            .run()
            .into_diagnostic()?;

        let bin_name = t.chip.as_ref();

        let features = vec![t.chip.as_ref(), test_case];
        let features_arg = features.join(",");

        let mut args = vec!["--features", &features_arg];

        if settings.release {
            args.push("--release")
        };

        let command = &settings.command;

        cmd!(
            sh,
            "cargo {command} --bin {bin_name} {args...} --target {rust_target} --locked"
        )
        .run()
        .into_diagnostic()?;

        let subfolder = if settings.release { "release" } else { "debug" };

        let chip = t.chip.as_ref();

        let binary_path = repo_root.join(format!("target/{rust_target}/{subfolder}/{chip}"));

        built_binaries.push(binary_path);
    }

    Ok(built_binaries)
}
