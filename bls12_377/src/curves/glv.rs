use crate::{g1::Parameters, Fq, Fr};
use ark_ec::{glv::GLVParameters, msm::ScalarMul, ModelParameters};
use ark_ff::MontFp;

impl ScalarMul for Parameters {
    type CurveAffine = crate::G1Affine;
}

impl GLVParameters for Parameters {
    type CurveProjective = crate::G1Projective;

    const COEFFS_ENDOMORPHISM: &'static[Self::BaseField] = &[
        MontFp!(Fq, "258664426012969093929703085429980814127835149614277183275038967946009968870203535512256352201271898244626862047231"),
    ];

    const LAMBDA: Self::ScalarField = MontFp!(
        Fr,
        "8444461749428370424248824938781546531284005582649182570233710176290576793600"
    );

    const COEFF_N: [<Self as ModelParameters>::ScalarField; 4] = [
        MontFp!(Fr, "91893752504881257701523279626832445441"),
        MontFp!(Fr, "1"),
        MontFp!(Fr, "1"),
        MontFp!(Fr, "91893752504881257701523279626832445440"),
    ];
    const SGN_N: [bool; 4] = [true, true, false, true];

    fn endomorphism(base: &Self::CurveAffine) -> Self::CurveAffine {
        Self::CurveAffine::new(Self::COEFFS_ENDOMORPHISM[0] * base.x, base.y, false)
    }
}
