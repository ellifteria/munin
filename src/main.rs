use crate::device::Variable;

mod device;


fn main() {
    let mut device: device::Device = device::Device::new();

    device.read_program("examples/add.asm");

    for j in 1..=30
    {
        eprintln!("j={}", j);
        eprintln!("{:->36}","");
        let i = 1 << j;
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
