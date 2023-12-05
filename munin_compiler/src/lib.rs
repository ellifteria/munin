use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
    collections::HashMap,
};


pub struct Compiler
{
    pub jump_points:    HashMap<String, usize>,
    pub program_lines:  Vec<String>,
    pub line_number:    usize,
}

impl Compiler
{
    pub fn new() -> Self
    {
        Self
        {
            jump_points:    HashMap::new(),
            program_lines:  Vec::new(),
            line_number:    0,
            
        }
    }

    pub fn load_file(&mut self, file_path: impl AsRef<Path>)
    {
        let file: File = File::open(file_path).expect("Could not open file");
        let io_file_lines:io::Result<Vec<String>> = BufReader::new(file).lines().collect();
        let program_lines = io_file_lines.expect("Could not read program lines");
        self.program_lines = program_lines.into_iter().filter(|x| !x.starts_with(";")).collect::<Vec<String>>().clone();
    }

    pub fn compile_program(&mut self, path: &str)
    {

        let f = File::create(path).expect("unable to create file");
        let mut f = BufWriter::new(f);

        const MAX_NUM_TOKENS: usize = 8;
        for line_number in 0..self.program_lines.len()
        {
            let line = &self.program_lines.get(line_number).unwrap();

            let mut tokens: Vec<&str> = line.split_whitespace()
                .collect::<Vec<&str>>();
    
            // push empty values to ensure have tokens
            for _ in 0..MAX_NUM_TOKENS
            {
                tokens.push("");
            }
    
            let asm_code: String = match tokens[0]
            {
                ""=>{"do-nothing".to_string()}
                "declare-jump-point" =>
                {
                    let name = tokens[1];
                    self.jump_points.insert(name.to_owned(), line_number);
                    "do-nothing".to_string()
                }
                "set" =>
                {
                    let destination = tokens[1];
                    let source = tokens[3];
                    match source {
                        "length-of" =>
                        {
                            let true_source = tokens[4];
                            format!("set-to-length-of {destination} {true_source}")
                        }
                        "bit" =>
                        {
                            let bit = tokens[4];
                            let true_source = tokens[6];
                            format!("set-to-nth-bit {destination} {true_source} {bit}")
                        }
                        _other =>
                        {
                            format!("set {destination} {source}")
                        }
                    }
                }
                "int-add" =>
                {
                    let destination = tokens[3];
                    let source = tokens[1];
                    format!("int-add {destination} {source}")
                }
                "int-subtract" =>
                { 
                    let destination = tokens[3];
                    let source = tokens[1];
                    format!("int-subtract {destination} {source}") 
                }
                "bit-add" =>
                {
                    let destination = tokens[3];
                    let source = tokens[1];
                    let carry = tokens[4];
                    match carry {
                        "with-carry" =>
                        {
                            format!("bit-add-with-carry {destination} {source}")
                        }
                        _other =>
                        {
                            format!("bit-add {destination} {source}")
                        }
                    }
                }
                "bit-subtract" =>
                {
                    let destination = tokens[3];
                    let source = tokens[1];
                    format!("bit-subtract {destination} {source}")
                }
                "shift"=>
                {
                    let destination = tokens[1];
                    let direction = tokens[2];
                    let by = tokens[4];
                    format!("bit-shift-{direction} {destination} {by}")
                }
                "go-to" =>
                {
                    let line_number: &usize = self.jump_points.get(tokens[1]).unwrap();
                    format!("jump-to {line_number}")
                }
                "compare" =>
                {
                    let a = tokens[1];
                    let b = tokens[3];
                    format!("compare {a} {b}")
                }
                "skip-next-if" =>
                {
                    let flag = tokens[1];
                    format!("jump-over-next-if {flag}")
                }
                _other =>
                {
                    panic!("Invalid syntax: {}", line);
                }
            };
            write!(f, "{}\n", asm_code).expect("unable to write");
        }
        write!(f, "end\n").expect("unable to write");
    }

}
