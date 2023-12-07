use std::{
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::Path,
    collections::HashMap,
};


pub struct Assembler
{
    pub jump_points:    HashMap<String, usize>,
    pub program_lines:  Vec<String>,
    pub line_number:    usize,
}

impl Assembler
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

            match tokens[0]
            {
                "label" =>
                {
                    let name = tokens[1];
                    self.jump_points.insert(name.to_owned(), line_number);
                }
                _other =>{}
            };
        }

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
                ""=>{
                    "non".to_string()
                }
                "label" =>
                {
                    "non".to_string()
                }
                "set" =>
                {
                    let destination = tokens[1];
                    let source = tokens[3];
                    match source {
                        "length-of" =>
                        {
                            let true_source = tokens[4];
                            format!("stl {destination} {true_source}")
                        }
                        "bit" =>
                        {
                            let bit = tokens[4];
                            let true_source = tokens[6];
                            format!("stnb {destination} {true_source} {bit}")
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
                    format!("iadd {destination} {source}")
                }
                "int-subtract" =>
                { 
                    let destination = tokens[3];
                    let source = tokens[1];
                    format!("isub {destination} {source}") 
                }
                "bit-add" =>
                {
                    let destination = tokens[3];
                    let source = tokens[1];
                    let carry = tokens[4];
                    match carry {
                        "with-carry" =>
                        {
                            format!("badc {destination} {source}")
                        }
                        _other =>
                        {
                            format!("badd {destination} {source}")
                        }
                    }
                }
                "bit-subtract" =>
                {
                    let destination = tokens[3];
                    let source = tokens[1];
                    let borrow = tokens[4];
                    match borrow {
                        "with-borrow" =>
                        {
                            format!("bsbu {destination} {source}")
                        }
                        _other =>
                        {
                            format!("bsub {destination} {source}")
                        }
                    }
                }
                "shift"=>
                {
                    let destination = tokens[1];
                    let direction = tokens[2].chars().next().unwrap();
                    let by = tokens[4];
                    format!("bs{direction} {destination} {by}")
                }
                "go-to" =>
                {
                    let line_number: &usize = self.jump_points.get(tokens[1]).unwrap();
                    format!("jmp {line_number}")
                }
                "compare" =>
                {
                    let a = tokens[1];
                    let b = tokens[3];
                    format!("cmp {a} {b}")
                }
                "clear-flags" =>
                {
                    "clf".to_string()
                }
                "skip-next-if" =>
                {
                    let flag = match tokens[1]
                    {
                        "equal" => {"e"}
                        "not-equal" => {"ne"}
                        "greater" => {"g"}
                        "greater-or-equal" => {"ge"}
                        "less" => {"l"}
                        "less-or-equal" => {"le"}
                        "carry" => {"c"}
                        "no-carry" => {"nc"}
                        "borrow" => {"b"}
                        "no-borrow" => {"nb"}
                        "" => {""}
                        _other => panic!("Invalid flag: {}", tokens[1])
                    };
                    format!("jon {flag}")
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
