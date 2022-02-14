use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{
        ModelParameters, 
        MontgomeryModelParameters, 
        SWModelParameters, 
        TEModelParameters
    },
    twisted_edwards_extended::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::{field_new, Zero, BigInteger256};

#[cfg(test)]
mod tests;


pub type EdwardsAffine = GroupAffine<VestaParameters>;
pub type EdwardsProjective = GroupProjective<VestaParameters>;

pub type SWAffine = SWGroupAffine<VestaParameters>;
pub type SWProjective = SWGroupProjective<VestaParameters>;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct VestaParameters;


pub type EdwardsParameters = VestaParameters;
pub type SWParameters = VestaParameters;

impl ModelParameters for VestaParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

}

pub type Affine = GroupAffine<VestaParameters>;
pub type Projective = GroupProjective<VestaParameters>;

impl TEModelParameters for VestaParameters {
    // !
    // ! ONLY FOR TESTS!
    // ! VESTA DOES NOT HAVE A TEMODEL REPRESENTATION!
    // !

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[3544928777400];

    /// COFACTOR_INV = 1974227602028601682287777587513404591299892938882253386062334684
    const COFACTOR_INV: Fr = field_new!(Fr, "1974227602028601682287777587513404591299892938882253386062334684");

    /// COEFF_A = 18882546820327010691852883245023291371663161054248913913882393733229978544603
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "18882546820327010691852883245023291371663161054248913913882393733229978544603");

    /// COEFF_D = 1344397884135987551249732079497516440402877072853346240649512048082701365364
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "1344397884135987551249732079497516440402877072853346240649512048082701365364");

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (G_GENERATOR_X, G_GENERATOR_Y);

    type MontgomeryModelParameters = VestaParameters;

    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::from(BigInteger256([
            263294614177016283, 14751359859395633415, 845045001895911217, 3008163260104276743
        ])) * elem
    }

}

impl MontgomeryModelParameters for VestaParameters {
    // !
    // !
    // ! ONLY FOR TESTS
    // !
    // !

    /// COEFF_A = 11895231804162141719591236115765134026545847010052284623160100874727159560879
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "11895231804162141719591236115765134026545847010052284623160100874727159560879");
    /// COEFF_B = 19443588303588037647598153854229482937342123604162571818821744411187050018097
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "19443588303588037647598153854229482937342123604162571818821744411187050018097");

    type TEModelParameters = VestaParameters;
}

impl SWModelParameters for VestaParameters {
    // THIS IS NOT VESTA ! WARNING!

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[3544928777400];

    /// COFACTOR_INV = 1974227602028601682287777587513404591299892938882253386062334684
    const COFACTOR_INV: Fr = field_new!(Fr, "1974227602028601682287777587513404591299892938882253386062334684");
    
    /// COEFF_A = 12033307035803878849683165116917446042199434082332695997656512932932991033641
    const COEFF_A: Fq = field_new!(Fq, "12033307035803878849683165116917446042199434082332695997656512932932991033641");

    /// COEFF_B = 21529462793858023443483451437869885304357989953298403038493222394236648576789
    const COEFF_B: Fq = field_new!(Fq, "21529462793858023443483451437869885304357989953298403038493222394236648576789");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_G_GENERATOR_X, SW_G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}


/// G_GENERATOR_X = 17867118842613120718213207777974927098207803448714025363887649166769138968825
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fq = field_new!(Fq, "17867118842613120718213207777974927098207803448714025363887649166769138968825");

/// G_GENERATOR_Y = 2890381160784175018304077481667798917018184370055689611110281600530100509087
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fq = field_new!(Fq, "2890381160784175018304077481667798917018184370055689611110281600530100509087");

//

/// SW_G_GENERATOR_X = 4356316195798653406553170569412604812177689345415962621018877128289734404066
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const SW_G_GENERATOR_X: Fq = field_new!(Fq, "4356316195798653406553170569412604812177689345415962621018877128289734404066");

/// SW_G_GENERATOR_Y = 4833359151334879449080234494095206083548078745547043403722701446818691755949
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const SW_G_GENERATOR_Y: Fq = field_new!(Fq, "4833359151334879449080234494095206083548078745547043403722701446818691755949");