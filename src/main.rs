mod pentomino;
mod cell_shape;
mod transform;
mod coord;

use clap::{
    Subcommand,
    Parser,
};

#[derive(Parser)]
#[command(version, about, arg_required_else_help=true, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command
}


#[derive(Subcommand)]
enum Command {
    DescribePentominoes
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::DescribePentominoes => {
            describe_pentominoes();
        }
    }
}

fn describe_pentominoes() {
    pentomino::PENTOMINOES
        .iter()
        .for_each(|p| {

            let shapes = p.shapes();

            println!("================================");
            println!(" {p:?} has {} distinct orientations", shapes.len());
            for (i, s) in shapes.iter().enumerate() {
                println!("{}. ", i + 1);
                s.print_out();
            }
    });

}
