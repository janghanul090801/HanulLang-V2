mod runtime;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap[author, version, about]]
pub struct Arg {
    pub path: String,
    // #[arg(short, long)]
    // pub flag: Option<String>,
}

fn main() {
    let mut runtime = runtime::Janghanul::new();

    let args = Arg::parse();
    let path = args.path.clone();
    let code = std::fs::read_to_string(&path).unwrap();
    match runtime.compile(&code, true, 100000) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }

    // println!("{tokens:?}");
}
