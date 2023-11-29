use crate::device::Variable;

mod device;

fn main() {
    let mut device: device::Device = device::Device::new();

    // println!("\n---\nFinal modified register values:\n---");

    device.read_program("examples/pal-add.asm");

    for j in 1..=24
    {
        let i = 1 << j;
        device.clear_device_execution_memory();
        device.load_input_variable("i00", i-1);
        device.load_input_variable("i01", 1);
        device.load_input_variable("i02", i);
        device.execute_program(None);
        let (touched_flags, touched_registers) = device.count_touched_memory();
        println!("{},{}", Variable::u32_to_bits(i).len(), touched_flags + touched_registers);
    }
    // device.load_input_variable("i00", 12);
    //     device.load_input_variable("i01", 15);
    // device.execute_program(None);

    // dbg!(device.write_variables.clone());
    // dbg!(device.input_variables.clone());
    // dbg!(device.write_bits.clone());

    // println!("\n---\nMemory usage:\n---");

    // let (touched_flags, touched_registers) = device.count_touched_memory();
    // println!("Flags:\t\t\t{}", touched_flags);
    // println!("Registers:\t\t{}", touched_registers);
    // println!("---\nTotal memory used:\t{} bytes\n---\n", touched_flags + touched_registers);
}
