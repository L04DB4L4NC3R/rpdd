pub mod cli;

pub fn handle_cli_args() {
    let (cfg_path,
        _interests,
        _duration_minutes,
        is_daemonized) = cli::parse();

    if is_daemonized {
        // run everything async
    }

    let default_cfg_path = cli::get_default_path();

    // if config flag not passed then use interests passed in the cli option
    if !default_cfg_path.eq(&cfg_path) {
        // use cfg file args
    }

    // use cli args
}
