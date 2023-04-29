use getopts::Options;
use std::env;

pub struct OptsArgs {
    pub scene_file: String,
    pub external_file: Option<String>
}

pub enum OptsResult<T> {
    Err,
    Help,
    Ok(T)
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("USAGE: {} <SCENE_FILE>", program);
    print!("{}", opts.usage(&brief));
}

fn create_options() -> Options {
    let mut opts = Options::new();
    opts.optopt("", "export", "save output in a file instead of using graphical", "<file>");
    opts.optflag("h", "help", "print this help menu");
    opts
}

pub fn check_opts() -> OptsResult<OptsArgs> {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let opts = create_options();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => {
            eprintln!("Error: {}", f);
            return OptsResult::Err
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return OptsResult::Help
    }
    let scene_file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return OptsResult::Help
    };
    OptsResult::Ok(OptsArgs{ scene_file, external_file: matches.opt_str("export") })
}
