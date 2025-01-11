use std::borrow::Cow;

use clap::Arg;
use clap::Command;
use miette::IntoDiagnostic;
use xshell::cmd;

fn main() -> miette::Result<()> {
    let build_command =
        Command::new("build").arg(Arg::new("cmd").long("command").default_value("build"));

    let m = Command::new("xtask")
        .subcommand_required(true)
        .subcommand(build_command)
        .get_matches();

    match m.subcommand() {
        Some(("build", matches)) => build(),
        _ => unreachable!("Subcommand required settings prevents this."),
    }
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

fn build() -> miette::Result<()> {
    let sh = xshell::Shell::new().into_diagnostic()?;

    // Try to create deterministic builds

    sh.set_var(
        "RUSTFLAGS",
        "--remap-path-prefix /Users/tiwalun/.cargo/registry=/Users/jacknoppe/.cargo/registry",
    );

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

        cmd!(
            sh,
            "cargo build --bin {bin_name} --features {features_arg} --target {rust_target} --locked"
        )
        .run()
        .into_diagnostic()?;
    }

    Ok(())
}
