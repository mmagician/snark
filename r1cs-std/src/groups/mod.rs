use crate::prelude::*;
use algebra::{Field, ProjectiveCurve};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use r1cs_core::{Namespace, SynthesisError};

use core::{borrow::Borrow, fmt::Debug};

pub mod curves;

pub use self::curves::short_weierstrass::bls12;
pub use self::curves::short_weierstrass::mnt4;
pub use self::curves::short_weierstrass::mnt6;

/// A hack used to work around the lack of implied bounds.
pub trait GroupOpsBounds<'a, F, T: 'a>:
    Sized
    + Add<&'a T, Output = T>
    + Sub<&'a T, Output = T>
    + Add<T, Output = T>
    + Sub<T, Output = T>
    + Add<F, Output = T>
    + Sub<F, Output = T>
{
}

pub trait CurveVar<C: ProjectiveCurve, ConstraintF: Field>:
    'static
    + Sized
    + Clone
    + Debug
    + R1CSVar<ConstraintF, Value = C>
    + ToBitsGadget<ConstraintF>
    + ToBytesGadget<ConstraintF>
    + EqGadget<ConstraintF>
    + CondSelectGadget<ConstraintF>
    + AllocVar<C, ConstraintF>
    + AllocVar<C::Affine, ConstraintF>
    + for<'a> GroupOpsBounds<'a, C, Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + AddAssign<C>
    + SubAssign<C>
    + AddAssign<Self>
    + SubAssign<Self>
{
    fn constant(other: C) -> Self;

    fn zero() -> Self;

    fn is_zero(&self) -> Result<Boolean<ConstraintF>, SynthesisError> {
        self.is_eq(&Self::zero())
    }

    /// Allocate a variable in the subgroup without checking if it's in the
    /// prime-order subgroup
    fn new_variable_omit_prime_order_check(
        cs: impl Into<Namespace<ConstraintF>>,
        f: impl FnOnce() -> Result<C, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError>;

    /// Enforce that `self` is in the prime-order subgroup.
    fn enforce_prime_order(&self) -> Result<(), SynthesisError>;

    fn double(&self) -> Result<Self, SynthesisError> {
        let mut result = self.clone();
        result.double_in_place()?;
        Ok(result)
    }

    fn double_in_place(&mut self) -> Result<(), SynthesisError>;

    fn negate(&self) -> Result<Self, SynthesisError>;

    /// Computes `bits * self`, where `bits` is a little-endian
    /// `Boolean` representation of a scalar.
    fn scalar_mul_le<'a>(
        &self,
        bits: impl Iterator<Item = &'a Boolean<ConstraintF>>,
    ) -> Result<Self, SynthesisError> {
        let mut res = Self::zero();
        let mut multiple = self.clone();
        for bit in bits {
            let tmp = res.clone() + &multiple;
            res = bit.select(&tmp, &res)?;
            multiple.double_in_place()?;
        }
        Ok(res)
    }

    /// Computes a `I * self` in place, where `I` is a `Boolean` *little-endian*
    /// representation of the scalar.
    ///
    /// The base powers are precomputed power-of-two multiples of a single
    /// base.
    fn precomputed_base_scalar_mul_le<'a, I, B>(
        &mut self,
        scalar_bits_with_base_powers: I,
    ) -> Result<(), SynthesisError>
    where
        I: Iterator<Item = (B, &'a C)>,
        B: Borrow<Boolean<ConstraintF>>,
        C: 'a,
    {
        for (bit, base_power) in scalar_bits_with_base_powers {
            let new_encoded = self.clone() + *base_power;
            *self = bit.borrow().select(&new_encoded, self)?;
        }
        Ok(())
    }

    fn precomputed_base_3_bit_signed_digit_scalar_mul<'a, I, J, B>(
        _: &[B],
        _: &[J],
    ) -> Result<Self, SynthesisError>
    where
        I: Borrow<[Boolean<ConstraintF>]>,
        J: Borrow<[I]>,
        B: Borrow<[C]>,
    {
        Err(SynthesisError::AssignmentMissing)
    }

    /// Computes a `\sum I_j * B_j`, where `I_j` is a `Boolean`
    /// representation of the j-th scalar.
    fn precomputed_base_multiscalar_mul_le<'a, T, I, B>(
        bases: &[B],
        scalars: I,
    ) -> Result<Self, SynthesisError>
    where
        T: 'a + ToBitsGadget<ConstraintF> + ?Sized,
        I: Iterator<Item = &'a T>,
        B: Borrow<[C]>,
    {
        let mut result = Self::zero();
        // Compute ∏(h_i^{m_i}) for all i.
        for (bits, base_powers) in scalars.zip(bases) {
            let base_powers = base_powers.borrow();
            let bits = bits.to_bits_le()?;
            result.precomputed_base_scalar_mul_le(bits.iter().zip(base_powers))?;
        }
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use algebra::{test_rng, Field, ProjectiveCurve};
    use r1cs_core::{ConstraintSystem, SynthesisError};

    use crate::prelude::*;

    pub(crate) fn group_test<C: ProjectiveCurve, ConstraintF: Field, GG: CurveVar<C, ConstraintF>>(
    ) -> Result<(), SynthesisError>
    where
        for<'a> &'a GG: GroupOpsBounds<'a, C, GG>,
    {
        let cs = ConstraintSystem::<ConstraintF>::new_ref();

        let mut rng = test_rng();
        let a_native = C::rand(&mut rng);
        let b_native = C::rand(&mut rng);
        let a = GG::new_witness(cs.ns("generate_a"), || Ok(a_native)).unwrap();
        let b = GG::new_witness(cs.ns("generate_b"), || Ok(b_native)).unwrap();

        let zero = GG::zero();
        assert_eq!(zero.value()?, zero.value()?);

        // a == a
        assert_eq!(a.value()?, a.value()?);
        // a + 0 = a
        assert_eq!((&a + &zero).value()?, a.value()?);
        // a - 0 = a
        assert_eq!((&a - &zero).value()?, a.value()?);
        // a - a = 0
        assert_eq!((&a - &a).value()?, zero.value()?);
        // a + b = b + a
        let a_b = &a + &b;
        let b_a = &b + &a;
        assert_eq!(a_b.value()?, b_a.value()?);
        a_b.enforce_equal(&b_a)?;
        assert!(cs.is_satisfied().unwrap());

        // (a + b) + a = a + (b + a)
        let ab_a = &a_b + &a;
        let a_ba = &a + &b_a;
        assert_eq!(ab_a.value()?, a_ba.value()?);
        ab_a.enforce_equal(&a_ba)?;
        assert!(cs.is_satisfied().unwrap());

        // a.double() = a + a
        let a_a = &a + &a;
        let mut a2 = a.clone();
        a2.double_in_place()?;
        a2.enforce_equal(&a_a)?;
        assert_eq!(a2.value()?, a_native.double());
        assert_eq!(a_a.value()?, a_native.double());
        assert_eq!(a2.value()?, a_a.value()?);
        assert!(cs.is_satisfied().unwrap());

        // b.double() = b + b
        let mut b2 = b.clone();
        b2.double_in_place()?;
        let b_b = &b + &b;
        b2.enforce_equal(&b_b)?;
        assert!(cs.is_satisfied().unwrap());
        assert_eq!(b2.value()?, b_b.value()?);

        let _ = a.to_bytes()?;
        assert!(cs.is_satisfied().unwrap());
        let _ = a.to_non_unique_bytes()?;
        assert!(cs.is_satisfied().unwrap());

        let _ = b.to_bytes()?;
        let _ = b.to_non_unique_bytes()?;
        if !cs.is_satisfied().unwrap() {
            println!("{:?}", cs.which_is_unsatisfied().unwrap());
        }
        assert!(cs.is_satisfied().unwrap());
        Ok(())
    }
}
