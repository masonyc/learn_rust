use kvs::KvStore;
use std::process;

fn main() {
    let _kvs = KvStore::new();
    let cmd = clap::Command::new("kvs")
        .bin_name("kvs")
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(clap::command!("-V"))
        .subcommand(
            clap::command!("get").arg(clap::arg!(<KEY>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            clap::command!("rm").arg(clap::arg!(<KEY>).value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            clap::command!("set")
                .arg(clap::arg!(<KEY>).value_parser(clap::value_parser!(String)))
                .arg(clap::arg!(<VALUE>).value_parser(clap::value_parser!(String))),
        );

    let matches = cmd.get_matches();

    if matches.subcommand_matches("-V").is_some() {
        println!(env!("CARGO_PKG_VERSION"));
    }

    if let Some(_matches) = matches.subcommand_matches("get") {
        // let k = matches.get_one::<String>("KEY");
        eprintln!("unimplemented");
        process::exit(1);
        // kvs.get(k.unwrap().to_string());
    }

    if let Some(_matches) = matches.subcommand_matches("rm") {
        // let k = matches.get_one::<String>("KEY");
        eprintln!("unimplemented");
        process::exit(1);
        // kvs.remove(k.unwrap().to_string());
    }

    if let Some(_matches) = matches.subcommand_matches("set") {
        // let k = matches.get_one::<String>("KEY");
        // let v = matches.get_one::<String>("VALUE");
        eprintln!("unimplemented");
        process::exit(1);
        // kvs.set(k.unwrap().to_string(), v.unwrap().to_string());
    }
}
