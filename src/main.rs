use deadlines::{
    operations,
    opts::{get_file, Args, Opts},
};
use structopt::StructOpt;

fn main() {
    env_logger::init();

    let opts: Opts = Opts::from_args();
    if opts.add {
        let args: Args = TryFrom::try_from(&opts).unwrap();
        operations::add(args);
    } else {
        operations::display::display(&get_file(&opts.deadlines_file).expect("File opening error"));
    }
}
