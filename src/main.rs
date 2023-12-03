use clap::Parser;

use crate::device::Variable;

mod device;

const VALUES_FOR_PROFILING: [u32; 5] = [0x1, 0x2, 0x8, 0x80, 0x8000];
const NUM_VALUES: usize = VALUES_FOR_PROFILING.len();

#[allow(dead_code)]
fn memory_profile_add(pretty_print_values: bool)
{
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/add.asm");

    let mut input_lengths: [usize; NUM_VALUES] = [0usize; NUM_VALUES];
    let mut execution_memory_usages: [usize; NUM_VALUES] = [0usize; NUM_VALUES];

    for i in 0..NUM_VALUES
    {
        let input: u32 = VALUES_FOR_PROFILING[i];
        device.clear_device_execution_memory();
        device.load_input_variable("i00", input-1);
        device.load_input_variable("i01", 1);
        device.load_input_variable("i02", input);
        device.execute_program(None);
        let (input_used, touched_flags, touched_registers) = device.count_touched_memory();
        input_lengths[i] = Variable::u32_to_bits(input).len();
        execution_memory_usages[i] = touched_flags + touched_registers;
        println!("{},{},{}", Variable::u32_to_bits(input).len(), input_used, touched_flags + touched_registers);
        if pretty_print_values
        {
            eprintln!("i={}", input);
            eprintln!("{:->36}","");
            device.pretty_print_memory();
            eprintln!();
        }
    }
    eprintln!("\n COMPLEXITY ANALYSIS");
    eprintln!("{:->36}","");
    eprintln!("{: ^14}|{: ^21}", "INPUT LENGTH", "MEMORY USED");
    for i in 0..NUM_VALUES
    {
        eprintln!("{:->15}{:->21}", "+", "");
        eprintln!(" {: <13}| {: <20}",  input_lengths[i], execution_memory_usages[i]);
    }
    eprintln!();
}

#[allow(dead_code)]
fn memory_profile_lin_add(pretty_print_values: bool)
{
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/lin-add.asm");

    let mut input_lengths: [usize; NUM_VALUES] = [0usize; NUM_VALUES];
    let mut execution_memory_usages: [usize; NUM_VALUES] = [0usize; NUM_VALUES];

    for i in 0..NUM_VALUES
    {
        let input: u32 = VALUES_FOR_PROFILING[i];
        device.clear_device_execution_memory();
        device.load_input_variable("i00", input | (1u32));
        device.load_input_variable("i01", 0);
        device.load_input_variable("i02", input | (1u32));
        device.execute_program(None);
        let (input_used, touched_flags, touched_registers) = device.count_touched_memory();
        input_lengths[i] = Variable::u32_to_bits(input).len();
        execution_memory_usages[i] = touched_flags + touched_registers;
        println!("{},{},{}", Variable::u32_to_bits(input).len(), input_used, touched_flags + touched_registers);
        if pretty_print_values
        {
            eprintln!("i={}", input);
            eprintln!("{:->36}","");
            device.pretty_print_memory();
            eprintln!();
        }
    }
    eprintln!("\n COMPLEXITY ANALYSIS");
    eprintln!("{:->36}","");
    eprintln!("{: ^14}|{: ^21}", "INPUT LENGTH", "MEMORY USED");
    for i in 0..NUM_VALUES
    {
        eprintln!("{:->15}{:->21}", "+", "");
        eprintln!(" {: <13}| {: <20}",  input_lengths[i], execution_memory_usages[i]);
    }
    eprintln!();
}

#[allow(dead_code)]
fn memory_profile_pal_add(pretty_print_values: bool)
{
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/pal-add.asm");

    let mut input_lengths: [usize; NUM_VALUES] = [0usize; NUM_VALUES];
    let mut execution_memory_usages: [usize; NUM_VALUES] = [0usize; NUM_VALUES];

    for i in 0..NUM_VALUES
    {
        let input: u32 = VALUES_FOR_PROFILING[i];
        device.clear_device_execution_memory();
        device.load_input_variable("i00", input | (1u32));
        device.load_input_variable("i01", 0);
        device.load_input_variable("i02", input | (1u32));
        device.execute_program(None);
        let (input_used, touched_flags, touched_registers) = device.count_touched_memory();
        input_lengths[i] = Variable::u32_to_bits(input).len();
        execution_memory_usages[i] = touched_flags + touched_registers;
        println!("{},{},{}", Variable::u32_to_bits(input).len(), input_used, touched_flags + touched_registers);
        if pretty_print_values
        {
            eprintln!("i={}", input);
            eprintln!("{:->36}","");
            device.pretty_print_memory();
            eprintln!();
        }
    }
    eprintln!("\n COMPLEXITY ANALYSIS");
    eprintln!("{:->36}","");
    eprintln!("{: ^14}|{: ^21}", "INPUT LENGTH", "MEMORY USED");
    for i in 0..NUM_VALUES
    {
        eprintln!("{:->15}{:->21}", "+", "");
        eprintln!(" {: <13}| {: <20}",  input_lengths[i], execution_memory_usages[i]);
    }
    eprintln!();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Algorithm to profile
    #[arg(short, long)]
    algorithm: u32,

    // 1 if pretty printing values; else 0
    #[arg(short, long, default_value_t=0)]
    pretty_print_values: u32
}

fn main() {
    let args: Args = Args::parse();

    let pretty_print_values: bool = args.pretty_print_values == 1;

    let algorithm = args.algorithm;
    match &algorithm
    {
        0 =>
        {
            eprintln!("\n{:->36}","");
            eprintln!(" COMPLEXITY ANALYSIS OF ADD");
            eprintln!("{:->36}","");
            memory_profile_add(pretty_print_values);
        }
        1 =>
        {
            eprintln!("\n{:->36}","");
            eprintln!(" COMPLEXITY ANALYSIS OF PAL-ADD");
            eprintln!("{:->36}","");
            memory_profile_pal_add(pretty_print_values);
        }
        2 =>
        {
            eprintln!("\n{:->36}","");
            eprintln!(" COMPLEXITY ANALYSIS OF LIN-ADD");
            eprintln!("{:->36}","");
            memory_profile_lin_add(pretty_print_values);
        }
        _other=>
        {
            println!();
            eprintln!("Unknown algorithm: {}", &algorithm)
        }
    }
}
