use clap::{Arg, ArgMatches, Command};

pub fn get_args() {
    let matches = register_args();

    // Matches the commands and performs actions
    match matches.subcommand() {
        Some(("configure", sub_m)) => {
            if sub_m.contains_id("provider") {
                let packages: Vec<_> = sub_m
                    .get_many::<String>("provider")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Configuring provider: {}", values);
            }
            // TODO: Implement the configuration logic here
        },
        Some(("get", sub_m)) => {
            if sub_m.contains_id("address") {
                let packages: Vec<_> = sub_m
                    .get_many::<String>("address")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let address = packages.join(", ");
                println!("Configuring provider: {}", address);
            }
            if sub_m.contains_id("date") {
                let packages: Vec<_> = sub_m
                    .get_many::<String>("date")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let date = packages.join(", ");
                println!("Configuring provider: {}", date);
            }
            // TODO: Implement the weather fetching logic here
        },
        _ => (),
    }
}

pub fn register_args() -> ArgMatches {
    Command::new("Weather CLI")
        .version("1.0")
        .author("Your Name valentinefilatov2015@gmail.com")
        .about("Shows weather to a user")
        .subcommand(Command::new("configure")
            .about("Configures credentials for a provider")
            .arg(Arg::new("provider")
                .help("Sets the provider to configure")
                .required(true)
                .index(1)))
        .subcommand(Command::new("get")
            .about("Shows weather for the provided address")
            .arg(Arg::new("address")
                .help("Sets the address to get weather for")
                .required(true)
                .index(1))
            .arg(Arg::new("date")
                .help("Sets the date (optional, defaults to now)")
                .required(false)
                .index(2)))
        .get_matches()
}
