use rvm::vm::Machine;

pub fn main() -> Result<(), String> {
    let mut vm = Machine::new();
    /*
     * PUSH 10
     * PUHS 8
     * ADDSTACK
     *  POPREG A
     */
    vm.memory.write(0, 0x1);
    vm.memory.write(1, 10);
    vm.memory.write(2, 0x1);
    vm.memory.write(3, 8);
    vm.memory.write(4, 0x3);
    vm.memory.write(6, 0x2);
    vm.memory.write(7, 0);
    vm.step()?;
    vm.step()?;
    vm.step()?;
    vm.step()?;
    println!("A = {}", vm.get_register(rvm::Register::A));
    Ok(())
}
