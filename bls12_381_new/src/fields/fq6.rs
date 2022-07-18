use crate::*;
use ark_ff::{field_new, fields::*};

pub type Fq6 = Fp6<Fq6Parameters>;

#[derive(Clone, Copy)]
pub struct Fq6Parameters;

impl Fp6Parameters for Fq6Parameters {
    type Fp2Params = Fq2Parameters;

    /// NONRESIDUE = 2*U + 3
	// this is a non-cube in Fq2, defining Fq6/Fq2
    #[rustfmt::skip]
    const NONRESIDUE: Fq2 = field_new!(Fq2,
		field_new!(Fq, "3"),
		field_new!(Fq, "2"),
    );

	// these are NONRESIDUE^((p^j-1)/3) for j < 6
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
		// Fp2(2*u+3)^(((q^0) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(2*u+3)^(((q^1) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1299818015494292088105446783060438427891131812204492106986268054896610609280511382410141402766362857835195262967386"),
			field_new!(Fq, "341271319708803213020263867634975777161930113570579678857021592772510761250115379665947893149424902561574727029022"),
		),
		// Fp2(2*u+3)^(((q^2) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "568069564169735887784523595428185157672868461154544401167503000216125933363689723611179842011137"),
			field_new!(Fq, "0"),
		),
		// Fp2(2*u+3)^(((q^3) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1089868072997204305446337911517999430801885948297603518328129824758724688053803760199139187816677592904486659407166"),
			field_new!(Fq, "6001812092242436681138596122146195032835985843506017544428097327879010780129094672745597976884595104912131708629"),
		),
		// Fp2(2*u+3)^(((q^4) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "2680159072491083434283635177082100889479298905786357595840289005788905202900434977410295569058861055203330140143615"),
			field_new!(Fq, "0"),
		),
		// Fp2(2*u+3)^(((q^5) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "290472983999587041299920046673398918570804740712447128198759587288114306733622835017140911839510328074828059780201"),
			field_new!(Fq, "2332885940690037785150302277494714805069056401800457057111707776843059832037693503287728011296241281148023123417102"),
		),
    ];

	// these are NONRESIDUE^((2*p^j-1)/3) for j < 6
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
		// Fp2(2*u+3)^(((2q^0) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(2*u+3)^(((2q^1) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "2179444459186748273112551997010384670375589528884670695987044927030785248858612513613180171673355844517681812527850"),
			field_new!(Fq, "487434218004211063327557595469157431073051587159035649286351658516756471014046227002760437066099235147820265302119"),
		),
		// Fp2(2*u+3)^(((2q^2) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "2680159072491083434283635177082100889479298905786357595840289005788905202900434977410295569058861055203330140143615"),
			field_new!(Fq, "0"),
		),
		// Fp2(2*u+3)^(((2q^3) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1884666182199079045190158128871350137613953722361364451157028371088535444355133186141561055723008284210988565443871"),
			field_new!(Fq, "1584520781098346576030137031954069795708367568669905674149676394698547039704496911268682909560059191904117694597017"),
		),
		// Fp2(2*u+3)^(((2q^4) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "568069564169735887784523595428185157672868461154544401167503000216125933363689723611179842011137"),
			field_new!(Fq, "0"),
		),
		// Fp2(2*u+3)^(((2q^5) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1296207503596339551400699356621938746538101751183050359882241635767578514922130255498101777448737428900349586337785"),
			field_new!(Fq, "608204073388525795494010113828609550482403345385601430077129413728146093349394839354978155796392351762572022255617"),
		),
    ];

    /// Multiply this element by the nonresidue 3+2u.
    /// Make this generic.
    fn mul_fp2_by_nonresidue(fe: &Fq2) -> Fq2 {
        // (x+uy) * (3+2u) = 3x+3uy+2ux+2u²y = (3x-10y) + u * (3y+2x)
        let mut copy = *fe;
        let x = copy.c0;
        let y = copy.c1;
        let x_2 = x.double();
        let y_2 = y.double();
        copy.c0 = x_2 + x - y_2.double().double() - y_2;
        copy.c1 = y_2 + y + x_2;
        copy
    }
}
