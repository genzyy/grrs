// Crate used for arguments structure.
use structopt::StructOpt;

// Activating Debug Mode
#[derive(StructOpt, Debug)]
// About the app
#[structopt(name = "demo", about = "Greetings from CLI using Rust!")]

struct Opt {
    // Defining cli arguments flag as short and long ones.
    #[structopt(short = "g", long = "greet")]
    // greet vairable defined as public so that there is no pre defined scope of greet variable
    pub greet: bool,
}

// main function here
fn main() {
    // namespace for arguments
    let opt = Opt::from_args();
    if opt.greet {
        println!("Hello, from Rust!");
    } else {
        println!("You missed something...");
    }
}
