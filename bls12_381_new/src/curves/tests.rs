#![allow(unused_imports)]
use ark_ec::{
    bls12::Bls12Parameters, group::Group, models::SWModelParameters,
    short_weierstrass_jacobian::GroupProjective, AffineCurve, PairingEngine, ProjectiveCurve,
};
use ark_ff::{
    field_new,
    fields::{Field, FpParameters, PrimeField, SquareRootField},
    BigInteger, BigInteger256, BigInteger384, Fp12Parameters, One, UniformRand, Zero,
};
use ark_serialize::CanonicalSerialize;
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign};
use std::convert::TryInto;

use crate::{
    g1,
    g2::{self, Parameters},
    Bls12_381New, Fq, Fq12, Fq2, Fq6, FqParameters, Fr, G1Affine, G1Projective, G2Affine,
    G2Projective,
};
use ark_algebra_test_templates::{curves::*, fields::field_serialization_test, groups::*};
use ark_ec::short_weierstrass_jacobian::GroupAffine;

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();
    sw_tests::<g1::Parameters>();
    edwards_tests::<g1::Parameters>();
}

#[test]
fn test_g1_projective_group() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G1Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_projective_curve() {
    curve_tests::<G2Projective>();

    sw_tests::<g2::Parameters>();
}

#[test]
fn test_g2_projective_group() {
    let mut rng = test_rng();
    let a: G2Projective = rng.gen();
    let b: G2Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_fq2_frob() {
    let mut rng = ark_std::test_rng();
    let x = Fq2::rand(&mut rng);
    let x_p = x.pow(FqParameters::MODULUS);
    let mut x_frob = x;
    x_frob.frobenius_map(1);
    assert_eq!(x_frob, x_p);
}

#[test]
fn test_fq6_frob() {
    let mut rng = ark_std::test_rng();
    let x = Fq6::rand(&mut rng);
    let x_p = x.pow(FqParameters::MODULUS);
    let mut x_frob = x;
    x_frob.frobenius_map(1);
    assert_eq!(x_frob, x_p);
}

#[test]
fn test_fq12_frob() {
    let mut rng = ark_std::test_rng();
    let x = Fq12::rand(&mut rng);
    let mut x_p = x.pow(FqParameters::MODULUS);
    for power in 1..12 {
        let mut x_pi = x;
        x_pi.frobenius_map(power);
        assert_eq!(x_pi, x_p);
        x_p = x_p.pow(FqParameters::MODULUS);
    }
}

#[test]
fn test_bilinearity() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G2Projective = rng.gen();
    let s: Fr = rng.gen();

    let mut sa = a;
    sa.mul_assign(s);
    let mut sb = b;
    sb.mul_assign(s);

    let ans1 = Bls12_381New::pairing(sa, b);
    let ans2 = Bls12_381New::pairing(a, sb);
    let ans3 = Bls12_381New::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq12::one());
    assert_ne!(ans2, Fq12::one());
    assert_ne!(ans3, Fq12::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq12::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq12::one());
}

#[test]
fn test_g1_generator_raw() {
    let mut x = Fq::zero();
    let mut i = 0;
    loop {
        // y^2 = x^3 + b
        let mut rhs = x;
        rhs.square_in_place();
        rhs.mul_assign(&x);
        rhs.add_assign(&g1::Parameters::COEFF_B);

        if let Some(y) = rhs.sqrt() {
            let p = G1Affine::new(x, if y < -y { y } else { -y }, false);
            assert!(!p.is_in_correct_subgroup_assuming_on_curve());

            let g1 = p.scale_by_cofactor();
            if !g1.is_zero() {
                assert_eq!(i, 1);
                let g1 = G1Affine::from(g1);

                assert!(g1.is_in_correct_subgroup_assuming_on_curve());

                assert_eq!(g1, G1Affine::prime_subgroup_generator());
                break;
            }
        }

        i += 1;
        x.add_assign(&Fq::one());
    }
}
