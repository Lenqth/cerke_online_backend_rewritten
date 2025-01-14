use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// A type that serialize into `{}`.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Unit {}

#[test]
fn unit_serde() {
    assert_eq!("{}", serde_json::to_string(&Unit {}).unwrap());
    let g: Unit = serde_json::from_str("{}").unwrap();
    assert_eq!(Unit {}, g);
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(into = "&'static str")]
#[serde(try_from = "&str")]
pub enum TacticsKey {
    VictoryAlmostCertain,
    StrengthenedShaman,
    FreeLunch,
    AvoidDefeat,
    LossAlmostCertain,
    Neutral,
}

impl TryFrom<&str> for TacticsKey {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "victory_almost_certain" => Ok(TacticsKey::VictoryAlmostCertain),
            "strengthened_shaman" => Ok(TacticsKey::StrengthenedShaman),
            "free_lunch" => Ok(TacticsKey::FreeLunch),
            "avoid_defeat" => Ok(TacticsKey::AvoidDefeat),
            "loss_almost_certain" => Ok(TacticsKey::LossAlmostCertain),
            "neutral" => Ok(TacticsKey::Neutral),
            s => Err(format!("unknown tactics name `{}` found. Please edit cerke_online_backend_rewritten repository.", s)),
        }
    }

    type Error = String;
}

impl From<TacticsKey> for &'static str {
    fn from(a: TacticsKey) -> &'static str {
        match a {
            TacticsKey::VictoryAlmostCertain => "victory_almost_certain",
            TacticsKey::StrengthenedShaman => "strengthened_shaman",
            TacticsKey::FreeLunch => "free_lunch",
            TacticsKey::AvoidDefeat => "avoid_defeat",
            TacticsKey::LossAlmostCertain => "loss_almost_certain",
            TacticsKey::Neutral => "neutral",
        }
    }
}

pub type AbsoluteCoord = cetkaik_core::absolute::Coord;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Kok1 = 0,
    Huok2 = 1,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum Profession {
    Nuak1 = 0,
    Kauk2 = 1,
    Gua2 = 2,
    Kaun1 = 3,
    Dau2 = 4,
    Maun1 = 5,
    Kua2 = 6,
    Tuk2 = 7,
    Uai1 = 8,
    Io = 9,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
// Using boolean is natural, and this is also necessary to allow easy interop with the frontend
#[allow(clippy::struct_excessive_bools)]
pub struct Ciurl(bool, bool, bool, bool, bool);

use rand::{prelude::ThreadRng, Rng};
impl Ciurl {
    pub fn new(rng: &mut ThreadRng) -> Ciurl {
        Ciurl(rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
    pub fn count(self) -> usize {
        self.0 as usize + self.1 as usize + self.2 as usize + self.3 as usize + self.4 as usize
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum NormalMove {
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[serde(tag = "type")]

pub enum NonTamMoveDotData {
    FromHand {
        color: Color,
        profession: Profession,
        dest: AbsoluteCoord,
    },
    SrcDst {
        src: AbsoluteCoord,
        dest: AbsoluteCoord,
        water_entry_ciurl: Option<Ciurl>,
    },
    SrcStepDstFinite {
        src: AbsoluteCoord,
        step: AbsoluteCoord,
        dest: AbsoluteCoord,
        water_entry_ciurl: Option<Ciurl>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
#[serde(tag = "stepStyle")]
pub enum TamMoveInternal {
    NoStep {
        src: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },

    StepsDuringFormer {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },

    StepsDuringLatter {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "firstDest")]
        first_dest: AbsoluteCoord,

        #[serde(rename = "secondDest")]
        second_dest: AbsoluteCoord,
    },
}

/* InfAfterStep | AfterHalfAcceptance | NormalMove*/
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum MainMessage {
    InfAfterStep {
        #[serde(flatten)]
        flatten: InfAfterStepInternal,
    },
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum AfterHalfAcceptanceMessage {
    AfterHalfAcceptance { dest: Option<AbsoluteCoord> },
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct InfAfterStepInternal {
    src: AbsoluteCoord,
    step: AbsoluteCoord,

    #[serde(rename = "plannedDirection")]
    coord_signifying_planned_direction: AbsoluteCoord,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct WhoGoesFirst {
    pub result: bool,
    pub process: Vec<[Ciurl; 2]>,
}

impl WhoGoesFirst {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut process: Vec<[Ciurl; 2]> = Vec::new();
        loop {
            let ciurl1 = Ciurl::new(rng);
            let ciurl2 = Ciurl::new(rng);
            process.push([ciurl1, ciurl2]);
            if ciurl1.count() > ciurl2.count() {
                return WhoGoesFirst {
                    process,
                    result: true,
                };
            }
            if ciurl1.count() < ciurl2.count() {
                return WhoGoesFirst {
                    process,
                    result: false,
                };
            }
        }
    }

    pub fn not(&self) -> Self {
        WhoGoesFirst {
            process: self.process.iter().map(|[a, b]| [*b, *a]).collect(),
            result: !self.result,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetTyMok {
    Err,
    Ok,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetTaXot {
    Err,
    Ok {
        is_first_move_my_move: Option<WhoGoesFirst>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetInfAfterStep {
    Ok { ciurl: Ciurl },
    Err { why_illegal: String },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetNormalMove {
    Err { why_illegal: String },
    WithWaterEntry { ciurl: Ciurl },
    WithoutWaterEntry,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetAfterHalfAcceptance {
    Err { why_illegal: String },
    WithWaterEntry { ciurl: Ciurl },
    WithoutWaterEntry,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetRandomEntry {
    InWaitingList {
        access_token: String,
    },

    #[serde(rename = "LetTheGameBegin")]
    RoomAlreadyAssigned {
        access_token: String,
        is_first_move_my_move: WhoGoesFirst,

        #[serde(rename = "is_IA_down_for_me")]
        is_ia_down_for_me: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetVsCpuEntry {
    LetTheGameBegin {
        access_token: String,
        is_first_move_my_move: WhoGoesFirst,

        #[serde(rename = "is_IA_down_for_me")]
        is_ia_down_for_me: bool,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetRandomPoll {
    Err { why_illegal: String },
    Ok { ret: RetRandomEntry },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetRandomCancel {
    Err { why_illegal: String },
    Ok { cancellable: bool },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetWhetherTyMokPoll {
    TyMok,
    TaXot {
        is_first_move_my_move: Option<WhoGoesFirst>,
    },
    NotYetDetermined,
    Err {
        why_illegal: String,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetMainPoll {
    MoveMade {
        content: MoveToBePolled,
        message: Option<TacticsKey>,
    },
    NotYetDetermined,
    Err {
        why_illegal: String,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RetInfPoll {
    MoveMade { content: MoveToBePolled },
    NotYetDetermined,
    Err { why_illegal: String },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MoveToBePolled {
    NonTamMove {
        data: NonTamMoveDotData,
    },
    TamMove {
        #[serde(flatten)]
        flatten: TamMoveInternal,
    },
    InfAfterStep {
        src: AbsoluteCoord,
        step: AbsoluteCoord,

        #[serde(rename = "plannedDirection")]
        coord_signifying_planned_direction: AbsoluteCoord,
        stepping_ciurl: Ciurl,

        #[serde(rename = "finalResult")]
        final_result: Option<FinalResult>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FinalResult {
    dest: AbsoluteCoord,
    water_entry_ciurl: Option<Ciurl>,
    thwarted_by_failing_water_entry_ciurl: Option<Ciurl>,
}
