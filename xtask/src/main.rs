use std::borrow::Cow;

use clap::Arg;
use clap::Command;
use miette::IntoDiagnostic;
use xshell::cmd;

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

    let m = Command::new("xtask")
        .subcommand_required(true)
        .subcommand(build_command)
        .get_matches();

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

            build(&settings, &test_case)
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
    "hardfault-in-systick",
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

fn build(settings: &BuildSettings, test_case: &str) -> miette::Result<()> {
    let sh = xshell::Shell::new().into_diagnostic()?;

    // Try to create deterministic builds

    if settings.reproducible {
        sh.set_var(
            "RUSTFLAGS",
            "--remap-path-prefix /Users/tiwalun/.cargo=/Users/jacknoppe/.cargo --remap-path-prefix /Users/tiwalun/code/probe-rs-debugger-test=/Users/jacknoppe/dev/debug/probe-rs-debugger-test");
    }

    for t in TARGETS {
        println!("Building for {}", t.chip);
        println!(" - Installing rust target {}", t.rust_target);

        let rust_target = t.rust_target.as_ref();

        cmd!(sh, "rustup target add {rust_target}")
            .run()
            .into_diagnostic()?;

        let bin_name = t.chip.as_ref();

        let features = vec![t.chip.as_ref()];
        let features_arg = features.join(",");

        let mut args = vec!["--features", &features_arg];

        if settings.release {
            args.push("--release")
        };

        let command = &settings.command;

        cmd!(
            sh,
            "cargo {command} --bin {bin_name} {args...} --target {rust_target} --locked --features {test_case}"
        )
        .run()
        .into_diagnostic()?;
    }

    Ok(())
}
