use brass::ir::Documented;
use clap::Parser;



#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    subcommand: SubCommand, 
}

#[derive(clap::Subcommand, Clone)]
pub enum SubCommand {
    ShowIr
}

fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommand::ShowIr => {
            let mut map = std::collections::HashMap::new();
            brass::ir::Code::document(&mut map);
            for (k, v) in map {
                println!("{} = {}", k, v)
            }
        }
    }
}
