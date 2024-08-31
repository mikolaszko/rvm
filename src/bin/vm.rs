use rvm::vm::Machine;

pub fn main() -> Result<(), &'static str> {
    let mut vm = Machine::new();
    vm.step()
}
