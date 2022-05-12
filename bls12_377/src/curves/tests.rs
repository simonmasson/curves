use ark_algebra_test_templates::{
    curves::{curve_tests, edwards_tests, glv_tests, sw_tests},
    generate_bilinearity_test, generate_g1_generator_raw_test, generate_g1_test, generate_g2_test,
    generate_glv_test,
    groups::group_test,
    msm::test_var_base_msm,
};
use ark_ec::{models::SWModelParameters, AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{
    fields::{Field, PrimeField, SquareRootField},
    One, Zero, UniformRand,
};
use ark_std::{rand::Rng, test_rng};
use core::ops::{AddAssign, MulAssign};

use crate::{
    g1, g1::Parameters as G1Parameters, g2, g2::Parameters as G2Parameters, Bls12_377, Fq, Fq12,
    Fr, G1Affine, G1Projective, G1TEProjective, G2Affine, G2Projective,
};

generate_g1_test!(bls12_377; curve_tests; sw_tests; edwards_tests; te_group_tests;);
generate_g2_test!(bls12_377; curve_tests; sw_tests;);
generate_bilinearity_test!(Bls12_377, Fq12);
generate_g1_generator_raw_test!(bls12_377, 1);

generate_glv_test!(G1Parameters, G2Parameters);

#[test]
fn test_g1_subgroup_membership_via_endomorphism() {
    let mut rng = test_rng();
    let generator = G1Projective::rand(&mut rng).into_affine();
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g1_subgroup_non_membership_via_endomorphism() {
    let mut rng = test_rng();
    loop {
        let x = Fq::rand(&mut rng);
        let greatest = rng.gen();

        if let Some(p) = G1Affine::get_point_from_x(x, greatest) {
            if !p.into_projective().mul(Fr::characteristic()).is_zero() {
                assert!(!p.is_in_correct_subgroup_assuming_on_curve());
                return;
            }
        }
    }
}