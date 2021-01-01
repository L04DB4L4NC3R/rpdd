use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about="Research Paper of the Day Daemon")]
struct Cli {
    // config file path
    #[structopt(short="-c", long="--config-path", default_value="-", parse(from_os_str))]
    cfg_path: std::path::PathBuf,

    // comma separated interests
    #[structopt(short="-i", long="--interests", default_value="dummy1,dummy2", required_if("cfg_path", "-"))]
    interests: String,

    // cron duration
    #[structopt(short="-t", long="--time", default_value="1440")]
    duration_minutes: u32,

    // run in background?
    #[structopt(short="-d", long="--daemonize")]
    is_daemonized: bool,
}

pub fn parse() -> (std::path::PathBuf, String, u32, bool) {
    let args = Cli::from_args();
    (args.cfg_path, args.interests, args.duration_minutes, args.is_daemonized)
}

pub fn get_default_path() -> std::path::PathBuf {
    std::path::PathBuf::from("-")
}
