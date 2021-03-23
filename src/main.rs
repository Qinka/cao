mod args;

use crate::args::Args;


fn main() {
    let args = Args::args();

    println!("{:?}", args);
}
