use munin_assembler::Assembler;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to run
    #[arg(short, long)]
    file: String,

    // Inputs
    #[arg(short, long)]
    output: String
}

fn main() {
    let args: Args = Args::parse();
    let output_path = args.output;
    let file_path = args.file;

    let mut assembler = Assembler::new();

    assembler.load_file(file_path);

    assembler.compile_program(&output_path);

}
