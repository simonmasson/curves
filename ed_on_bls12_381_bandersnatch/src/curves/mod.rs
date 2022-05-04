use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    short_weierstrass_jacobian::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    twisted_edwards_extended::{GroupAffine, GroupProjective},
    SWModelParameters,
};
use ark_ff::{Field, MontFp};

use crate::{Fq, Fr};

mod glv;

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<BandersnatchParameters>;
pub type EdwardsProjective = GroupProjective<BandersnatchParameters>;

pub type SWAffine = SWGroupAffine<BandersnatchParameters>;
pub type SWProjective = SWGroupProjective<BandersnatchParameters>;

/// `bandersnatch` is a twisted Edwards curve. These curves have equations of
/// the form: ax² + y² = 1 - dx²y².
/// over some base finite field Fq.
///
/// bandersnatch's curve equation: -5x² + y² = 1 - dx²y²
///
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513.
///
/// a = -5.
/// d = (138827208126141220649022263972958607803/
///     171449701953573178309673572579671231137) mod q
///   = 45022363124591815672509500913686876175488063829319466900776701791074614335719.
///
/// Sage script to calculate these:
///
/// ```text
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
/// Fq = GF(q)
/// d = (Fq(138827208126141220649022263972958607803)/Fq(171449701953573178309673572579671231137))
/// ```
/// These parameters and the sage script obtained from:
/// <https://github.com/asanso/Bandersnatch/>
///
/// bandersnatch also has a short Weierstrass curve form, following the
/// form: y² = x³ + A * x + B
/// where
///
/// A = 10773120815616481058602537765553212789256758185246796157495669123169359657269
/// B = 29569587568322301171008055308580903175558631321415017492731745847794083609535
///
/// Script to transfer between different curves are available
/// <https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/bandersnatch.sage>
#[derive(Clone, Default, PartialEq, Eq)]
pub struct BandersnatchParameters;

pub type EdwardsParameters = BandersnatchParameters;
pub type SWParameters = BandersnatchParameters;

impl ModelParameters for BandersnatchParameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 4
    const COFACTOR: &'static [u64] = &[4];

    /// COFACTOR^(-1) mod r =
    /// 9831726595336160714896451345284868594481866920080427688839802480047265754601
    const COFACTOR_INV: Fr = MontFp!(
        Fr,
        "9831726595336160714896451345284868594481866920080427688839802480047265754601"
    );
}

impl TEModelParameters for BandersnatchParameters {
    /// COEFF_A = -5
    const COEFF_A: Fq = MontFp!(Fq, "-5");

    /// COEFF_D = (138827208126141220649022263972958607803/
    /// 171449701953573178309673572579671231137) mod q
    const COEFF_D: Fq = MontFp!(
        Fq,
        "45022363124591815672509500913686876175488063829319466900776701791074614335719"
    );

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontgomeryModelParameters = BandersnatchParameters;

    /// Multiplication by `a` is multiply by `-5`.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        let t = (*elem).double().double();
        -(t + *elem)
    }
}

impl MontgomeryModelParameters for BandersnatchParameters {
    /// COEFF_A = 29978822694968839326280996386011761570173833766074948509196803838190355340952
    const COEFF_A: Fq = MontFp!(
        Fq,
        "29978822694968839326280996386011761570173833766074948509196803838190355340952"
    );

    /// COEFF_B = 25465760566081946422412445027709227188579564747101592991722834452325077642517
    const COEFF_B: Fq = MontFp!(
        Fq,
        "25465760566081946422412445027709227188579564747101592991722834452325077642517"
    );

    type TEModelParameters = BandersnatchParameters;
}

// The TE form generator is generated following Zcash's fashion:
//  "The generators of G1 and G2 are computed by finding the lexicographically
//   smallest valid x-coordinate, and its lexicographically smallest
//   y-coordinate and scaling it by the cofactor such that the result is not
//   the point at infinity."
// The SW form generator is the same TE generator converted into SW form,
// obtained from the scripts:
//   <https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/bandersnatch.sage>

/// x coordinate for TE curve generator
const TE_GENERATOR_X: Fq = MontFp!(
    Fq,
    "18886178867200960497001835917649091219057080094937609519140440539760939937304"
);

/// y coordinate for TE curve generator
const TE_GENERATOR_Y: Fq = MontFp!(
    Fq,
    "19188667384257783945677642223292697773471335439753913231509108946878080696678"
);

/// x coordinate for SW curve generator
const SW_GENERATOR_X: Fq = MontFp!(
    Fq,
    "30900340493481298850216505686589334086208278925799850409469406976849338430199"
);

/// y coordinate for SW curve generator
const SW_GENERATOR_Y: Fq = MontFp!(
    Fq,
    "12663882780877899054958035777720958383845500985908634476792678820121468453298"
);

impl SWModelParameters for BandersnatchParameters {
    /// COEFF_A = 10773120815616481058602537765553212789256758185246796157495669123169359657269
    const COEFF_A: Self::BaseField = MontFp!(
        Fq,
        "10773120815616481058602537765553212789256758185246796157495669123169359657269"
    );

    /// COEFF_B = 29569587568322301171008055308580903175558631321415017492731745847794083609535
    const COEFF_B: Self::BaseField = MontFp!(
        Fq,
        "29569587568322301171008055308580903175558631321415017492731745847794083609535"
    );

    /// generators
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_GENERATOR_X, SW_GENERATOR_Y);
}