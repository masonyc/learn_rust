use kvs::KvStore;
use std::env::current_dir;

fn main() -> anyhow::Result<()> {
    let cmd = clap::Command::new("kvs")
        .bin_name("kvs")
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .subcommand(clap::command!("-V"))
        .subcommand(
            clap::command!("get")
                .arg(clap::Arg::new("KEY").value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            clap::command!("rm")
                .arg(clap::Arg::new("KEY").value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            clap::command!("set")
                .arg(clap::Arg::new("KEY").value_parser(clap::value_parser!(String)))
                .arg(clap::Arg::new("VALUE").value_parser(clap::value_parser!(String))),
        );

    let matches = cmd.get_matches();

    if matches.subcommand_matches("-V").is_some() {
        println!(env!("CARGO_PKG_VERSION"));
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let key = matches.get_one::<String>("KEY").unwrap();
        let mut store = KvStore::open(current_dir()?)?;
        if let Some(value) = store.get(key.to_string())? {
            println!("{}", value);
        } else {
            println!("Key not found");
        }
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        let k = matches.get_one::<String>("KEY").unwrap();
        let mut store = KvStore::open(current_dir()?)?;
        match store.remove(k.to_string()) {
            Ok(()) => {}
            Err(err) => {
                println!("Key not found");
                std::process::exit(1);
            }
        }
    } else if let Some(matches) = matches.subcommand_matches("set") {
        let k = matches.get_one::<String>("KEY").unwrap();
        let v = matches.get_one::<String>("VALUE").unwrap();

        let mut store = KvStore::open(current_dir()?)?;
        store.set(k.to_string(), v.to_string())?;
    }
    Ok(())
}
