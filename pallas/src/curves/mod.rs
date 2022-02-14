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
use ark_ff::{field_new, BigInteger256};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<PallasParameters>;
pub type EdwardsProjective = GroupProjective<PallasParameters>;

pub type SWAffine = SWGroupAffine<PallasParameters>;
pub type SWProjective = SWGroupProjective<PallasParameters>;

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct PallasParameters;

pub type EdwardsParameters = PallasParameters;
pub type SWParameters = PallasParameters;

impl ModelParameters for PallasParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

pub type Affine = GroupAffine<PallasParameters>;
pub type Projective = GroupProjective<PallasParameters>;


impl TEModelParameters for PallasParameters {
    // !
    // ! ONLY FOR TESTS!
    // ! VESTA DOES NOT HAVE A TEMODEL REPRESENTATION!
    // ! We use r = 64986179019709162267305887466992519
    // !


    /// COFACTOR = 445448905382629498772542304731918159944924
    const COFACTOR: &'static [u64] = &[13494692613552184540, 1045554871037551030, 1369];

    /// COFACTOR_INV = 6978197180436701685802849249375484
    const COFACTOR_INV: Fr = field_new!(Fr, "6978197180436701685802849249375484");
    
    /// COEFF_A = 16858055109312665414105024874435768498496575809450416791472468096749319906086
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "16858055109312665414105024874435768498496575809450416791472468096749319906086");

    /// COEFF_D = 12086426843685563803235642148383612254048046014280565859482012690131627431415
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "12086426843685563803235642148383612254048046014280565859482012690131627431415");

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (G_GENERATOR_X, G_GENERATOR_Y);

    type MontgomeryModelParameters = PallasParameters;

    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::from(BigInteger256([
            3920351669790597926,1700265065924678470,12113966149038550178,2685643123207111582
        ])) * elem
    }
}

impl MontgomeryModelParameters for PallasParameters {
    // !
    // !
    // ! ONLY FOR TESTS
    // !
    // !

    /// COEFF_A = 2588297946893274575681831065640871032829097155322336866345830064645569423767
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "2588297946893274575681831065640871032829097155322336866345830064645569423767");
    /// COEFF_B = 10738900529984456918477655432620483953351965896515462668448048097007571237740
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "10738900529984456918477655432620483953351965896515462668448048097007571237740");

    type TEModelParameters = PallasParameters;
}

impl SWModelParameters for PallasParameters {
    // !
    // !
    // ! ONLY FOR TESTS
    // !
    // !

    /// COEFF_A = 15472856019507685383953719555290536676206397041956899727279880177693842823075
    const COEFF_A: Fq = field_new!(Fq, "15472856019507685383953719555290536676206397041956899727279880177693842823075");

    /// COEFF_B = 24419954351426524683892915143322266162905124037100658665599160753587064349619
    const COEFF_B: Fq = field_new!(Fq, "24419954351426524683892915143322266162905124037100658665599160753587064349619");

    /// COFACTOR = 445448905382629498772542304731918159944924
    const COFACTOR: &'static [u64] = &[13494692613552184540, 1045554871037551030, 1369];

    /// COFACTOR_INV = 6978197180436701685802849249375484
    const COFACTOR_INV: Fr = field_new!(Fr, "6978197180436701685802849249375484");
    
    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_G_GENERATOR_X, SW_G_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::from(BigInteger256([
            7560272183354021795,10530780597364327372,17401572595672324949,2464968176679476679
        ])) * elem
    }
}

/// G_GENERATOR_X = 20568677711284214498556798082413645008797599346644409175270758450731449158763
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const G_GENERATOR_X: Fq = field_new!(Fq, "20568677711284214498556798082413645008797599346644409175270758450731449158763");

/// G_GENERATOR_Y = 28670977297975866030422267628390914140655890325650091445984717476629237001575
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const G_GENERATOR_Y: Fq = field_new!(Fq, "28670977297975866030422267628390914140655890325650091445984717476629237001575");

//

/// SW_G_GENERATOR_X = 10587733790699609394260227672753853646051912153269630680244231899621041722577
/// Encoded in Montgomery form, so the value here is -R mod p.
pub const SW_G_GENERATOR_X: Fq = field_new!(Fq, "10587733790699609394260227672753853646051912153269630680244231899621041722577");

/// SW_G_GENERATOR_Y = 22041641853171658531264670636745512177619060565997377443928240445297773222283
/// Encoded in Montgomery form, so the value here is 2R mod p.
pub const SW_G_GENERATOR_Y: Fq = field_new!(Fq, "22041641853171658531264670636745512177619060565997377443928240445297773222283");