use clap::Parser;

#[derive(Debug,Parser)]  // Debug macro provides format to display the output. 
#[command(author,long_about=None,version,about="this is copy version of echo cmd")]
struct Args {
   #[arg(required(true))]
    sentence : Vec<String>,
}


fn main() {
    let cli = Args::parse();
    println!("{}",
    cli.sentence.join(" ")
);
}
