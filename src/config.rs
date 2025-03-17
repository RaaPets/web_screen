use clap::Parser;

//  //  //  //  //  //  //  //
pub fn parse_cli() -> Config {
    Config::parse()
}

#[derive(Parser)]
#[command(version, about)]
pub struct Config {
    #[arg(short,long, default_value = "127.0.0.1")]
    pub bind: String,
    #[arg(short,long, default_value_t = 8088)]
    pub port: u16,
    #[arg(long, default_value = "admin")]
    pub user_name: String,
    #[arg(long, default_value = "admin")]
    pub pin: String,
}

