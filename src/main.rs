use clap::Parser;

use crate::device::Variable;

mod device;

#[allow(dead_code)]
fn memory_profile_add()
{
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/add.asm");

    for i in [0x1, 0x8, 0x80, 0x8000]
    {
        eprintln!("i={}", i);
        eprintln!("{:->36}","");
        device.clear_device_execution_memory();
        device.load_input_variable("i00", i-1);
        device.load_input_variable("i01", 1);
        device.load_input_variable("i02", i);
        device.execute_program(None);
        let (input_used, touched_flags, touched_registers) = device.count_touched_memory();
        println!("{},{},{}", Variable::u32_to_bits(i).len(), input_used, touched_flags + touched_registers);
        device.pretty_print_memory();
        eprintln!();
    }
}

#[allow(dead_code)]
fn memory_profile_lin_add()
{
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/lin-add.asm");

    for i in [0x1, 0x8, 0x80, 0x8000]
    {
        eprintln!("i={}", i);
        eprintln!("{:->36}","");
        device.clear_device_execution_memory();
        device.load_input_variable("i00", i-1);
        device.load_input_variable("i01", 1);
        device.load_input_variable("i02", i);
        device.execute_program(None);
        let (input_used, touched_flags, touched_registers) = device.count_touched_memory();
        println!("{},{},{}", Variable::u32_to_bits(i).len(), input_used, touched_flags + touched_registers);
        device.pretty_print_memory();
        eprintln!();
    }
}

#[allow(dead_code)]
fn memory_profile_pal_add()
{
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/pal-add.asm");

    for i in [0x1, 0x8, 0x80, 0x8000]
    {
        eprintln!("i={}", i);
        eprintln!("{:->36}","");
        device.clear_device_execution_memory();
        device.load_input_variable("i00", i | (1u32));
        device.load_input_variable("i01", 0);
        device.load_input_variable("i02", i | (1u32));
        device.execute_program(None);
        let (input_used, touched_flags, touched_registers) = device.count_touched_memory();
        println!("{},{},{}", Variable::u32_to_bits(i).len(), input_used, touched_flags + touched_registers);
        device.pretty_print_memory();
        eprintln!();
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Algorithm to profile
    #[arg(short, long)]
    algorithm: u32,
}

fn main() {
    let args: Args = Args::parse();

    let algorithm = args.algorithm;
    match &algorithm
    {
        0 =>
        {
            memory_profile_add();
        }
        1 =>
        {
            memory_profile_pal_add();
        }
        2 =>
        {
            memory_profile_lin_add();
        }
        _other=>
        {
            println!();
            eprintln!("Unknown algorithm: {}", &algorithm)
        }
    }
}
