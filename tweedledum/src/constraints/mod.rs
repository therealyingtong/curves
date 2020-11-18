//! This module implements the R1CS equivalent of `ark_tweedledum`.
//!
//! It implements field variables for `crate::Fp`,
//! and group variables for `crate::GroupProjective`.
//!
//! The field underlying these constraints is `crate::Fp`.
//!
//! # Examples
//!
//! One can perform standard algebraic operations on `FBaseVar`:
//!
//! ```
//! # fn main() -> Result<(), ark_relations::r1cs::SynthesisError> {
//! use ark_ff::UniformRand;
//! use ark_relations::r1cs::*;
//! use ark_r1cs_std::prelude::*;
//! use ark_tweedledum::{*, constraints::*};
//!
//! let cs = ConstraintSystem::<Fp>::new_ref();
//! // This rng is just for test purposes; do not use it
//! // in real applications.
//! let mut rng = ark_ff::test_rng();
//!
//! // Generate some random `Fp` elements.
//! let a_native = Fp::rand(&mut rng);
//! let b_native = Fp::rand(&mut rng);
//!
//! // Allocate `a_native` and `b_native` as witness variables in `cs`.
//! let a = FBaseVar::new_witness(ark_relations::ns!(cs, "generate_a"), || Ok(a_native))?;
//! let b = FBaseVar::new_witness(ark_relations::ns!(cs, "generate_b"), || Ok(b_native))?;
//!
//! // Allocate `a_native` and `b_native` as constants in `cs`. This does not add any
//! // constraints or variables.
//! let a_const = FBaseVar::new_constant(ark_relations::ns!(cs, "a_as_constant"), a_native)?;
//! let b_const = FBaseVar::new_constant(ark_relations::ns!(cs, "b_as_constant"), b_native)?;
//!
//! let one = FBaseVar::one();
//! let zero = FBaseVar::zero();
//!
//! // Sanity check one + one = two
//! let two = &one + &one + &zero;
//! two.enforce_equal(&one.double()?)?;
//!
//! assert!(cs.is_satisfied()?);
//!
//! // Check that the value of &a + &b is correct.
//! assert_eq!((&a + &b).value()?, a_native + &b_native);
//!
//! // Check that the value of &a * &b is correct.
//! assert_eq!((&a * &b).value()?, a_native * &b_native);
//!
//! // Check that operations on variables and constants are equivalent.
//! (&a + &b).enforce_equal(&(&a_const + &b_const))?;
//! assert!(cs.is_satisfied()?);
//! # Ok(())
//! # }
//! ```
//!
//! One can also perform standard algebraic operations on `GVar`:
//!
//! ```
//! # fn main() -> Result<(), ark_relations::r1cs::SynthesisError> {
//! # use ark_ff::UniformRand;
//! # use ark_relations::r1cs::*;
//! # use ark_r1cs_std::prelude::*;
//! # use ark_tweedledum::{*, constraints::*};
//!
//! # let cs = ConstraintSystem::<Fp>::new_ref();
//! # let mut rng = ark_ff::test_rng();
//!
//! // Generate some random `Projective` elements.
//! let a_native = Projective::rand(&mut rng);
//! let b_native = Projective::rand(&mut rng);
//!
//! // Allocate `a_native` and `b_native` as witness variables in `cs`.
//! let a = GVar::new_witness(ark_relations::ns!(cs, "a"), || Ok(a_native))?;
//! let b = GVar::new_witness(ark_relations::ns!(cs, "b"), || Ok(b_native))?;
//!
//! // Allocate `a_native` and `b_native` as constants in `cs`. This does not add any
//! // constraints or variables.
//! let a_const = GVar::new_constant(ark_relations::ns!(cs, "a_as_constant"), a_native)?;
//! let b_const = GVar::new_constant(ark_relations::ns!(cs, "b_as_constant"), b_native)?;
//!
//! // This returns the identity.
//! let zero = GVar::zero();
//!
//! // Sanity check one + one = two
//! let two_a = &a + &a + &zero;
//! two_a.enforce_equal(&a.double()?)?;
//!
//! assert!(cs.is_satisfied()?);
//!
//! // Check that the value of &a + &b is correct.
//! assert_eq!((&a + &b).value()?, a_native + &b_native);
//!
//! // Check that operations on variables and constants are equivalent.
//! (&a + &b).enforce_equal(&(&a_const + &b_const))?;
//! assert!(cs.is_satisfied()?);
//! # Ok(())
//! # }
//! ```

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
