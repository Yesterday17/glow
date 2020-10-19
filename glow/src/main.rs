use clap::{App, Arg, SubCommand};
use glow_utils::{Result, Singleton};
use nix::unistd::{geteuid, ROOT};
use std::process::exit;

fn main() -> Result<()> {
    // Use geteuid here to follow setuid user
    if geteuid() != ROOT {
        println!("Root privileges are required for Glow to work. Exiting...");
        exit(1);
    }

    let matches = App::new("glow")
        .version("0.1.0")
        .author("Yesterday17 <t@yesterday17.cn>")
        .about("Yet another network manager by Project Glow.")
        .arg(
            Arg::with_name("lib")
                .help("Path of configuration files")
                .default_value("/var/lib/glow"),
        )
        .subcommand(SubCommand::with_name("ip").help("IP configs"))
        .get_matches();

    let lib_path = matches.value_of("lib").unwrap();

    let singleton = Singleton::init(&format!("{}/glow.lock", lib_path.to_owned()))?;

    if let Some(matches) = matches.subcommand_matches("ip") {
        // TODO: ip subcommand
    }
    singleton.exit();
    Ok(())
}
