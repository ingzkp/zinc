//!
//! The `Not` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::Not;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Not {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let value = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let not = gadgets::logical::not::not(cs.namespace(|| "not"), &value)?;

        vm.push(Cell::Value(not))
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigInt;
    use num_traits::One;
    use num_traits::Zero;

    use zinc_build::ScalarType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_not() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(BigInt::zero(), ScalarType::Boolean))
            .push(zinc_build::Not)
            .push(zinc_build::Push::new(BigInt::one(), ScalarType::Boolean))
            .push(zinc_build::Not)
            .test(&[0, 1])
    }
}