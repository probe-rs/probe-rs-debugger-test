use std::borrow::Cow;

use clap::Command;
use miette::IntoDiagnostic;
use xshell::cmd;

fn main() -> miette::Result<()> {
    let build_command = Command::new("build");

    let m = Command::new("xtask")
        .subcommand_required(true)
        .subcommand(build_command)
        .get_matches();

    match m.subcommand() {
        Some(("build", _)) => build(),
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

const TARGETS: &[BuildTarget] = &[BuildTarget::new_static(
    "nRF52833_xxAA",
    "thumbv7em-none-eabihf",
)];

fn build() -> miette::Result<()> {
    let shell = xshell::Shell::new().into_diagnostic()?;

    for t in TARGETS {
        println!("Building for {}", t.chip);
        println!(" - Installing rust target {}", t.rust_target);

        let rust_target = t.rust_target.as_ref();

        cmd!(shell, "rustup target add {rust_target}")
            .run()
            .into_diagnostic()?;

        let bin_name = t.chip.as_ref();

        let features = vec![t.chip.as_ref()];
        let features_arg = features.join(",");

        cmd!(
            shell,
            "cargo build --bin {bin_name} --features {features_arg} --target {rust_target}"
        )
        .run()
        .into_diagnostic()?;
    }

    Ok(())
}
