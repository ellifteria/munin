use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use bit_vec::BitVec;

const NUM_FLAGS: usize = 4;
const EQUAL_FLAG: usize = 0;
const GREATER_FLAG: usize = 1;
const CARRY_FLAG: usize = 2;
const UNDERFLOW_FLAG: usize = 3;

#[derive(Debug, Clone)]
pub struct Variable
{
    pub value:      BitVec,
    pub max_size:   usize,
}

impl Variable
{
    fn new(uint: u32) -> Self
    {
        let bits: BitVec = Self::u32_to_bits(uint);

        Self
        {
            value: bits.clone(),
            max_size: bits.len(),
        }
    }

    fn set_value(&mut self, uint: u32)
    {
        let bits: BitVec = Self::u32_to_bits(uint);
        self.value = bits.clone();

        let size = bits.len();

        if size > self.max_size
        {
            self.max_size = size;
        }
    }

    fn get_value(&mut self) -> u32
    {
        return Self::bits_to_u32(self.value.clone());
    }

    #[allow(dead_code)]
    pub fn u32_to_bits(uint: u32) -> BitVec
    {
        let num_bits: usize = ((uint as i32).checked_ilog2().unwrap_or(0) + 1) as usize;
        let mut bits: BitVec = BitVec::from_elem(num_bits, false);

        for i in 0..num_bits
        {
            bits.set(i, ((uint >> i) & (1 as u32)) != 0);
        }

        return bits;
    }

    #[allow(dead_code)]
    pub fn bits_to_u32(bits: BitVec) -> u32
    {
        let mut uint: u32 = 0;

        for i in 0..bits.len()
        {
            uint += (bits[i] as u32) << i;
        }

        return uint;
    }
}

pub struct Device
{
    pub write_variables:        Vec<Variable>,
    pub write_bits:             Vec<Variable>,
    pub input_variables:        Vec<Variable>,
    pub flags:                  [bool; NUM_FLAGS],
    pub program_lines:          Vec<String>,
    pub program_running:        bool,
    pub instruction_pointer:    usize,
    pub has_loaded_input:       bool,
}

impl Device
{
    pub fn new() -> Self
    {
        Self
        {
            write_variables:        Vec::<Variable>::new(),
            write_bits:             Vec::<Variable>::new(),
            input_variables:        Vec::<Variable>::new(),
            flags:                  [false; NUM_FLAGS],
            program_lines:          Vec::new(),
            program_running:        false,
            instruction_pointer:    0,
            has_loaded_input:       false,
        }
    }

    #[allow(dead_code)]
    pub fn count_touched_memory(&mut self) -> (usize, usize)
    {
        let num_touched_flags: usize = self.flags.len();
        let num_touched_bits: &usize = &self.write_bits.len();
        let num_touched_variables: &usize = &self.write_variables.clone()
            .iter()
            .fold(0 as usize, |acc, var| acc + var.max_size);
        
        return (num_touched_flags, *num_touched_bits +  *num_touched_variables);
    }

    fn decipher_immediate(&mut self, immediate_string: &str) -> u32
    {
        if immediate_string.starts_with("0x") // hex encoded
        {
            let immediate_wo_prefix: &str = immediate_string.trim_start_matches("0x");
            let immediate_as_u32: u32 = u32::from_str_radix(immediate_wo_prefix, 16).unwrap();
            return immediate_as_u32;
        }

        // otherwise is int
        let immediate_as_u32: u32 = immediate_string.parse::<i64>().unwrap() as u32;
        return immediate_as_u32;
    }

    fn check_flow_condition(&mut self, condition: &str) -> bool
    {
        match condition
        {
            "e" => {self.flags[EQUAL_FLAG]}
            "ne" => {!self.flags[EQUAL_FLAG]}
            "g" => {self.flags[GREATER_FLAG]}
            "ge" => {self.flags[EQUAL_FLAG] || self.flags[GREATER_FLAG]}
            "l" => {!self.flags[GREATER_FLAG] && !self.flags[EQUAL_FLAG]}
            "le" => {!self.flags[GREATER_FLAG]}
            "c" => {self.flags[CARRY_FLAG]}
            "nc" => {!self.flags[CARRY_FLAG]}
            "u" => {self.flags[UNDERFLOW_FLAG]}
            "nu" => {!self.flags[UNDERFLOW_FLAG]}
            "" => {true}
            _ => panic!("Invalid condition: {}", condition)
        }
    }

    pub fn get_source_value(&mut self, operand: &str) -> u32
    {
        let source: u32;

        if operand.starts_with("v")
        {
            let variable_index: usize = operand.trim_start_matches("v").parse::<usize>().unwrap();
            source = self.write_variables[variable_index].get_value();
        } else if operand.starts_with("b")
        {
            let bit_index: usize = operand.trim_start_matches("b").parse::<usize>().unwrap();
            source = self.write_bits[bit_index].get_value();
        } else if operand.starts_with("i")
        {
            let input_index: usize = operand.trim_start_matches("i").parse::<usize>().unwrap();
            source = self.input_variables[input_index].get_value();
        } else
        {
            source = self.decipher_immediate(operand);
        }

        return source;
    }

    pub fn set_destination(&mut self, operand: &str, value: u32)
    {
        if operand.starts_with("v")
        {
            let variable_index: usize = operand.trim_start_matches("v").parse::<usize>().unwrap();
            if variable_index == self.write_variables.len()
            {
                let new_variable = Variable::new(value);
                self.write_variables.push(new_variable);
            } else
            {
                self.write_variables[variable_index].set_value(value);
            }
        } else if operand.starts_with("b")
        {
            if value > 1
            {
                panic!("Invalid bit value: {}", value)
            }

            let bit_index: usize = operand.trim_start_matches("b").parse::<usize>().unwrap();
            if bit_index == self.write_bits.len()
            {
                let new_variable = Variable::new(value);
                self.write_bits.push(new_variable);
            } else
            {
                self.write_bits[bit_index].set_value(value);
            }
        } else if operand.starts_with("i")
        {
            let input_index: usize = operand.trim_start_matches("i").parse::<usize>().unwrap();
            if input_index == self.input_variables.len()
            {
                let new_variable = Variable::new(value);
                self.input_variables.push(new_variable);
            } else
            {
                self.input_variables[input_index].set_value(value);
            }
        } else
        {
            panic!("Unknown location in memory: {}", operand);
        } 
    }

    pub fn execute_instruction(&mut self, instruction: &str)
    {
        let mut instruction_parts: Vec<&str> = instruction.split_whitespace()
            .collect::<Vec<&str>>();
        instruction_parts.push("");
        instruction_parts.push("");
        instruction_parts.push("");
        let operator: &str = instruction_parts[0].trim();
        let operand1: &str = instruction_parts[1].trim();
        let operand2: &str = instruction_parts[2].trim();
        let operand3: &str = instruction_parts[3].trim();

        let mut increment_instruction_pointer: bool = true;

        match operator
        {
            "set" =>
            {
                let source: u32 = self.get_source_value(operand2);

                self.set_destination(operand1, source);
            }
            "stl" =>
            {
                let source: u32 = self.get_source_value(operand2);
                let source_length: usize = Variable::u32_to_bits(source).len();

                self.set_destination(operand1, source_length as u32);
            }
            "add" =>
            {
                let source: u32 = self.get_source_value(operand2);

                let destination: u32 = self.get_source_value(operand1);

                let new_value: u32 = source + destination;

                self.set_destination(operand1, new_value); 
            }
            "addb" =>
            {
                let source: u32 = self.get_source_value(operand2);

                if source > 1
                {
                    panic!("Invalid value for binary addition: {} at {}", source, operand2);
                }

                let destination: u32 = self.get_source_value(operand1);


                if destination > 1
                {
                    panic!("Invalid value for binary addition: {} at {}", destination, operand1);
                }

                let mut new_value: u32 = source + destination + (self.flags[CARRY_FLAG] as u32);

                self.flags[CARRY_FLAG] = new_value >= 2;

                if self.flags[CARRY_FLAG]
                {
                    new_value -= 2;
                }

                self.set_destination(operand1, new_value);
            }
            "sub" =>
            {
                let source: u32 = self.get_source_value(operand2);

                let destination: u32 = self.get_source_value(operand1);

                let new_value: u32 = destination - source;

                self.set_destination(operand1, new_value); 
            }
            "subb" =>
            {
                let source: u32 = self.get_source_value(operand2);

                if source > 1
                {
                    panic!("Invalid value for binary addition: {} at {}", source, operand2);
                }

                let destination: u32 = self.get_source_value(operand1);


                if destination > 1
                {
                    panic!("Invalid value for binary addition: {} at {}", destination, operand1);
                }

                let mut new_value: i32 = (source as i32) -  (destination as i32) - (self.flags[UNDERFLOW_FLAG] as i32);

                self.flags[UNDERFLOW_FLAG] =  new_value < 0;

                if new_value < 0
                {
                    if new_value < -2 {panic!("Invalid binary subtraction")}
                    new_value += 2;
                }

                self.set_destination(operand1, new_value as u32);
            }
            "stnb" =>
            {
                let source: u32 = self.get_source_value(operand2);
                
                let n: u32 = self.get_source_value(operand3);

                let source_as_bits = Variable::u32_to_bits(source);

                let nth_bit : u32;
                if n as usize >= source_as_bits.len()
                {
                    nth_bit = 0;
                } else
                {
                    nth_bit = source_as_bits[n as usize] as u32;
                }
            
                self.set_destination(operand1, nth_bit);
            }
            "shr" =>
            {
                let source: u32 = self.get_source_value(operand2);

                let destination: u32 = self.get_source_value(operand1);

                let new_value: u32 = destination >> source;

                self.set_destination(operand1, new_value); 
            }
            "clf" =>
            {
                for flag in 0..NUM_FLAGS
                {
                    self.flags[flag] = false;
                }
            }
            "cmp" =>
            {
                let a_value: u32 = self.get_source_value(operand1);
                let b_value: u32 = self.get_source_value(operand2);


                self.flags[EQUAL_FLAG] = a_value == b_value;
                self.flags[GREATER_FLAG] = a_value > b_value;
            }
            "jmp" =>
            {
                let program_line: u32 = self.get_source_value(operand1);
                self.instruction_pointer = program_line as usize;
                increment_instruction_pointer = false;
            }
            "sni" =>
            {
                if !self.check_flow_condition(operand1)
                {
                    self.instruction_pointer += 1;
                    return
                }

                self.instruction_pointer += 1;
            }
            "end" =>
            {
                self.program_running = false;
            }
            _other =>
            {
                panic!("Unknown operator: {}", operator);
            }
        }

        if increment_instruction_pointer{
            self.instruction_pointer += 1;
        }

    }

    pub fn read_program(&mut self, file_path: impl AsRef<Path>)
    {
        let file: File = File::open(file_path).expect("Could not open file");
        let io_file_lines:io::Result<Vec<String>> = BufReader::new(file).lines().collect();
        let program_lines = io_file_lines.expect("Could not read program lines");
        self.program_lines = program_lines.into_iter().filter(|x| !x.starts_with(";")).collect::<Vec<String>>().clone();
    }

    fn run_program_lines(&mut self)
    {
        self.program_running = true;

        while self.program_running {
            let instruction_pointer: usize = self.instruction_pointer;
            let instruction: &String = &self.program_lines[instruction_pointer].clone();
            self.execute_instruction(instruction);
        }
    }

    pub fn execute_program(&mut self, start_point: Option<usize>)
    {
        if !self.has_loaded_input
        {
            println!("WARNING: no input loaded");
        }

        self.instruction_pointer = start_point.unwrap_or(0);

        self.run_program_lines();
    }

    pub fn load_input_variable(&mut self, input_variable: &str, input_value: u32)
    {
        let input_index: usize = input_variable.trim_start_matches("i").parse::<usize>().unwrap();
        if input_index == self.input_variables.len()
        {
            let new_variable = Variable::new(input_value);
            self.input_variables.push(new_variable);
        } else
        {
            self.input_variables[input_index].set_value(input_value);
        }
        self.has_loaded_input = true;
    }

    pub fn clear_device_execution_memory(&mut self)
    {
        self.write_variables = Vec::<Variable>::new();
        self.write_bits = Vec::<Variable>::new();
        self.flags = [false; NUM_FLAGS];
    }

}
