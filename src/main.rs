use deadlines::{
    operations,
    opts::{Args, Opts},
};
use structopt::StructOpt;

fn main() {
    env_logger::init();

    let opts: Opts = Opts::from_args();
    if opts.add {
        let args: Args = opts.try_into().unwrap();
        operations::add(args);
    } else {
        operations::display::display(&opts.deadlines_file);
    }
}
