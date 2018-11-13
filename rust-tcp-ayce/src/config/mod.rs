use clap::{Arg, App};

pub struct Config {
    pub address: String,
    pub port: String,
    pub n_kbytes: usize,
    pub n_rounds: usize,
}

impl Config {
    pub fn address_and_port(&self) -> String {
        format!("{}:{}", &self.address, &self.port)
    }
}
/// Extract the configuration from Command line
pub fn parse_config() -> Config {
    let matches = App::new("Config")
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("address")
            .help("IP4 address to connect to")
            .takes_value(true)
            .default_value("127.0.0.1")
        )
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("port")
            .help("port to connect to, like 127.0.0.1:8000")
            .takes_value(true)
            .default_value("7878")
        )
        .arg(Arg::with_name("n_kbytes")
            .short("k")
            .long("kbytes")
            .value_name("n_kbytes")
            .help("number of kbytes to transfer, must be a multiple of 100")
            .takes_value(true)
            .default_value("10000")
        )
        .arg(Arg::with_name("rounds")
            .short("r")
            .long("rounds")
            .value_name("rounds")
            .help("number of rounds of transfer to perform")
            .takes_value(true)
            .default_value("1000")
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let address = String::from(matches.value_of("address").unwrap());
    let port = String::from(matches.value_of("port").unwrap());
    let n_kbytes = matches.value_of("n_kbytes").unwrap().parse::<usize>().unwrap();
    let n_rounds = matches.value_of("rounds").unwrap().parse::<usize>().unwrap();

    Config {
        address,
        port,
        n_kbytes,
        n_rounds
    }
}