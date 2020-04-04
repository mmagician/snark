use crate::mnt6_753::{Fq, Fq3, Fq3Parameters, FQ_ONE, FQ_ZERO};
use algebra_core::{
    biginteger::BigInteger768 as BigInteger,
    field_new,
    fields::{
        fp3::Fp3Parameters,
        fp6_2over3::{Fp6, Fp6Parameters},
    },
};

pub type Fq6 = Fp6<Fq6Parameters>;

pub struct Fq6Parameters;

impl Fp6Parameters for Fq6Parameters {
    type Fp3Params = Fq3Parameters;

    #[rustfmt::skip]
    const NONRESIDUE: Fq3 = field_new!(Fq3,
        Fq3Parameters::NONRESIDUE,
        FQ_ZERO,
        FQ_ZERO,
    );

    // Coefficients for the Frobenius automorphism.
    // c1[0] = 1,
    // c1[1] = 24129022407817241407134263419936114379815707076943508280977368156625538709102831814843582780138963119807143081677569721953561801075623741378629346409604471234573396989178424163772589090105392407118197799904755622897541183052133
    // c1[2] = 24129022407817241407134263419936114379815707076943508280977368156625538709102831814843582780138963119807143081677569721953561801075623741378629346409604471234573396989178424163772589090105392407118197799904755622897541183052132
    // c1[3] = 41898490967918953402344214791240637128170709919953949071783502921025352812571106773058893763790338921418070971888458477323173057491593855069696241854796396165721416325350064441470418137846398469611935719059908164220784476160000
    // c1[4] = 17769468560101711995209951371304522748355002843010440790806134764399814103468274958215310983651375801610927890210888755369611256415970113691066895445191924931148019336171640277697829047741006062493737919155152541323243293107868
    // c1[5] = 17769468560101711995209951371304522748355002843010440790806134764399814103468274958215310983651375801610927890210888755369611256415970113691066895445191924931148019336171640277697829047741006062493737919155152541323243293107869
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: [Fq; 6] = [
        FQ_ONE,
        field_new!(Fq, BigInteger([
            2665418275744511426,
            7073776242814464967,
            4441331072847607829,
            5681016258918493042,
            18254896527151449163,
            10681724016023285331,
            1760041123371930134,
            4557299868084578750,
            16702481779049799698,
            14149724469588165150,
            5617650120443517591,
            449252806040736,
        ])),
        field_new!(Fq, BigInteger([
            7739145380395648640,
            1403348385939055902,
            11220424057264707228,
            4567962295300549271,
            5929583493640677751,
            17618207486530478833,
            16600462137977359741,
            16551719371247820635,
            12057922785354578416,
            13022559182829558162,
            13308285686168533250,
            313705269181021,
        ])),
        field_new!(Fq, BigInteger([
            2265581976117350591,
            18442012872391748519,
            3807704300793525789,
            12280644139289115082,
            10655371227771325282,
            1346491763263331896,
            7477357615964975877,
            12570239403004322603,
            2180620924574446161,
            12129628062772479841,
            8853285699251153944,
            362282887012814,
        ])),
        field_new!(Fq, BigInteger([
            12973180669431253567,
            17038664486452692616,
            11034024317238370177,
            7712681843988565810,
            4725787734130647531,
            2175028350442404679,
            9323639551697167751,
            14465264105466053583,
            8569442212929419360,
            17553812953652473294,
            13991744086792172309,
            48577617831792,
        ])),
        field_new!(Fq, BigInteger([
            7899453564780116353,
            4262348269618550065,
            4254931332821270779,
            8825735807606509581,
            17051100767641418943,
            13685288953644762793,
            12929962610801289759,
            2470844602302811697,
            13214001206624640642,
            234234166701528666,
            6301108521067156651,
            184125154691507,
        ])),
    ];
}
