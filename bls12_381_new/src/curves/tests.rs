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
    let generator_te = g1::G1TEAffine::prime_subgroup_generator();
    assert!(generator_te.is_on_curve());
    assert!(generator_te.is_in_correct_subgroup_assuming_on_curve());
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
fn test_fq2_mul_magma() {
    // from magma
    // c11 = 2047204127881377338023965213551819441417205156999526177052724631150735103\
    // 298394122934756810787628829755890431423457*u +
    // 2337415152124335664867613331351620137783262809102406038291273560987369412247071\
    // 884003566334068613834667118017756696
    // c11^2 = 11771590844495974506550102200903888745028818982944044495476246099097804\
    // 22041100779983989261397360364488823207024393*u +
    // 1198343634478615150052897261954110491356266945218311478206573991473538969173076\
    // 854897389881227192574857304990133676
    // c111 = 233741515212433566486761333135162013778326280910240603829127356098736941\
    // 2247071884003566334068613834667118017756696
    // c112 = 204720412788137733802396521355181944141720515699952617705272463115073510\
    // 3298394122934756810787628829755890431423457

    let c111 = field_new!(Fq, "2337415152124335664867613331351620137783262809102406038291273560987369412247071884003566334068613834667118017756696");
    let c112 = field_new!(Fq, "2047204127881377338023965213551819441417205156999526177052724631150735103298394122934756810787628829755890431423457");
    let c11 = field_new!(Fq2, c111, c112);

    let c11_square = field_new!(Fq2,
        field_new!(Fq, "1198343634478615150052897261954110491356266945218311478206573991473538969173076854897389881227192574857304990133676"),
        field_new!(Fq, "1177159084449597450655010220090388874502881898294404449547624609909780422041100779983989261397360364488823207024393")
    );
    assert_eq!(c11_square, c11 * c11);
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

// let mut a_q = a.pow(&characteristic);
// for power in 1..maxpower {
//     let mut a_qi = a;
//     a_qi.frobenius_map(power);
//     assert_eq!(a_qi, a_q, "failed on power {}", power);

//     a_q = a_q.pow(&characteristic);
// }

#[test]
fn test_final_expo_magma() {
    // WORKS, warning the exponent is 3* (p^12-1)/r
    let y2 = field_new!(
        Fq12,
        field_new!(
            Fq6,
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "225556747335374118604884683518929409094606022428261383513008674\
    1849629059011562257356085699027007953609145655484195"
                ),
                field_new!(
                    Fq,
                    "211826192996536259778944696350238387777308949557969304667098461\
    0580700180682098692221869073941890795160749357521782"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "994477624426897377359500256670239473040582320857262365691078400\
    817029159069156226470470149334978386144220741977311"
                ),
                field_new!(
                    Fq,
                    "125064243095994340315366052917017417615193091528281213787079081\
    4398905692622780459357878710539341371409862107888697"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "953069969264226691360339784547581956611041802787853442723854916\
    488773831179183520961178767989820219218535799454857"
                ),
                field_new!(
                    Fq,
                    "212127512219228614267323817904794869324729005299024972159892221\
    0423801880424106052084645368368814805311423157390140"
                ),
            ),
        ),
        field_new!(
            Fq6,
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "223218500304795929348571476268718460462822738708235343628434436\
    2596150267781651307583174434056266993106256176004968"
                ),
                field_new!(
                    Fq,
                    "128805294098236780027113922314028643595345975829826138836502677\
    8775480105469232044582825569722350010469151127414900"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "139177352786242653816187174863017413429926700930308577818822392\
    9029695856326635149537042474716662155282300051199756"
                ),
                field_new!(
                    Fq,
                    "266630907816643916104095437145354915344673353396249349902197135\
    6979869524380896754328600663400041130616338314631423"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "201625576500415358351086175640410317386915168865985354994154377\
    4954523241060375220735516545060531536501359783003987"
                ),
                field_new!(
                    Fq,
                    "998852777585804453813631779789132335281082658146140946456807665\
    809146048840544407396700853085095065968965768282965"
                ),
            ),
        ),
    );
    let expo = field_new!(
        Fq12,
        field_new!(
            Fq6,
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "534802315403745411336915962014051198774768282352275719914758357\
    956921134518083456617440297074999395362099194977238"
                ),
                field_new!(
                    Fq,
                    "128857167038275625177669152862243611081656521121849010191589809\
    6830345453957071289533384263831054553476595293084871"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "189670115701510439683939955692425203136543005265459415707975178\
    6706747200481366361968858878076120227992297866707079"
                ),
                field_new!(
                    Fq,
                    "231972646622727308585503357664618678620978443762306537312972338\
    7083508978178975676796981498097668865838503026858233"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "166052061005387772069566293439726373495260825592373636700016245\
    7805738839993776843490501015980505239449757446264580"
                ),
                field_new!(
                    Fq,
                    "302268731971303725075639696398464349671016512824763136644655580\
    948342945276946756338874800608357447311828975106239"
                ),
            ),
        ),
        field_new!(
            Fq6,
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "146951846387197082061586108722277880921101859535446340111507930\
    1403788081602938765772392202328780534143908717010290"
                ),
                field_new!(
                    Fq,
                    "104428385097572561121979637580985241041028649734824229776451747\
    8225163045106151370893483745194728340073773561198537"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "237507238040042344415534385974756538949365318265312037505222972\
    8729174278409414276339804148430450081482223948909789"
                ),
                field_new!(
                    Fq,
                    "113017557711486834952425873582536616408964291106005507148702533\
    9395443433463777179950600556735563648072590391997716"
                ),
            ),
            field_new!(
                Fq2,
                field_new!(
                    Fq,
                    "107218195535951964328920727400637543152399403409817428121329348\
    1703252130051328173126282095259644932631272492943446"
                ),
                field_new!(
                    Fq,
                    "254801503598651408690470060727157667591332360115527891220118588\
    9115610094086940398425657514199680589583688919995057"
                ),
            ),
        ),
    );
    let fe = <Bls12_381New as PairingEngine>::final_exponentiation(&y2).unwrap();
    assert_eq!(fe, expo);
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
