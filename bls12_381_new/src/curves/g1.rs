use crate::*;
use ark_ec::{
    bls12,
    models::{ModelParameters, SWModelParameters},
    MontgomeryModelParameters, TEModelParameters,
};
use ark_ff::{field_new, Zero};

pub type G1Affine = bls12::G1Affine<crate::Parameters>;
pub type G1Projective = bls12::G1Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 1
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "1");

    /// COFACTOR = (x - 1)^2 / 3  = 66778914282889904464656736638700879872
    const COFACTOR: &'static [u64] = &[0, 0x323d26ac55830000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 17013441151896256258591749887919689498413710868588296929277
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "17013441151896256258591749887919689498413710868588296929277");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G1_GENERATOR_X =
/// 132958330983984893695165623413810542674704168143441818421981068363929595617071520952605269382545513645734829850271 
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "132958330983984893695165623413810542674704168143441818421981068363929595617071520952605269382545513645734829850271");

/// G1_GENERATOR_Y =
/// -1006336734889717938013612781497829381100950959809844863048520201949401120587783327756385452500762745860458840909358
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "-1006336734889717938013612781497829381100950959809844863048520201949401120587783327756385452500762745860458840909358");

impl TEModelParameters for Parameters {
    /// COEFF_A = 1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "1");

    /// COEFF_D = 234814438330152502739243686393973286577960760665870017848933367403534260634477217842396762854151367301824744571298
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "234814438330152502739243686393973286577960760665870017848933367403534260634477217842396762854151367301824744571298");

    /// COFACTOR = (x - 1)^2 / 3  = 66778914282889904464656736638700879872
    const COFACTOR: &'static [u64] = &[0, 0x323d26ac55830000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 17013441151896256258591749887919689498413710868588296929277
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "17013441151896256258591749887919689498413710868588296929277");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X_TE, GENERATOR_Y_TE)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (GENERATOR_X_TE, GENERATOR_Y_TE);

    type MontgomeryModelParameters = Parameters;

    /// Multiplication by `a` is the identity here.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        *elem
    }
}

impl MontgomeryModelParameters for Parameters {
    /// COEFF_A = 2068822913950850701823589477537370904592357066077374569597101442058470768209572787680415317530450925936338672758891
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "2068822913950850701823589477537370904592357066077374569597101442058470768209572787680415317530450925936338672758891");
    /// COEFF_B = 1097165077010438722626606668322100883311762645450570312476404497276142813308767722524142562438216877230893763850205
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "1097165077010438722626606668322100883311762645450570312476404497276142813308767722524142562438216877230893763850205");

    type TEModelParameters = Parameters;
}

#[rustfmt::skip]
const GENERATOR_X_TE: Fq = field_new!(Fq, "215186258444700131108811539781189140517202315562395092938047581141520334773867781998826665620853693197650416399888");
#[rustfmt::skip]
const GENERATOR_Y_TE: Fq = field_new!(Fq, "791514925387674934103027776198741316774869898393271052315833364400265101396204172439933188092443694783110413692137");
