use crate::{
    types::{RetRandomCancel, RetRandomEntry, RetRandomPoll, RetVsCpuEntry, WhoGoesFirst},
    BotToken, GameState, RoomId, RoomInfoWithPerspective,
};
use crate::{AccessToken, AppState, MsgWithAccessToken};
use actix_web::web;
use big_s::S;
use cetkaik_core::absolute;
use cetkaik_full_state_transition::{Rate, Season};
use uuid::Uuid;
pub fn random_entrance_poll_(
    _is_staging: bool,
    msg: &web::Json<MsgWithAccessToken>,
    data: &web::Data<AppState>,
) -> RetRandomPoll {
    if let Ok(access_token) = AccessToken::parse_str(&msg.access_token) {
        let person_to_room = data.person_to_room.lock().unwrap();
        if let Some(room_id) = (*person_to_room).get(&access_token) {
            // You already have a room
            RetRandomPoll::Ok {
                ret: RetRandomEntry::RoomAlreadyAssigned {
                    access_token: access_token.to_string(),
                    is_first_move_my_move: room_id.is_first_move_my_move[0].clone(),
                    is_ia_down_for_me: room_id.is_ia_down_for_me,
                },
            }
        } else {
            let waiting_list = data.waiting_list.lock().unwrap();
            if (*waiting_list).contains(&access_token) {
                // not yet assigned a room, but is in the waiting list
                RetRandomPoll::Ok {
                    ret: RetRandomEntry::InWaitingList {
                        access_token: access_token.to_string(),
                    },
                }
            } else {
                RetRandomPoll::Err {
                    why_illegal: format!(
                        r#"Invalid access token:
I don't know {}, which is the access token that you sent me.
Please reapply by sending an empty object to random/entry ."#,
                        access_token
                    ),
                }
            }
        }
    } else {
        RetRandomPoll::Err {
            why_illegal: S("access token could not be parsed"),
        }
    }
}

pub trait RemoveRandom {
    type Item;

    fn remove_random<R: rand::Rng>(&mut self, rng: &mut R) -> Option<Self::Item>;
}

// code adopted from https://stackoverflow.com/questions/53755017/can-i-randomly-sample-from-a-hashset-efficiently
impl<T> RemoveRandom for Vec<T> {
    type Item = T;

    fn remove_random<R: rand::Rng>(&mut self, rng: &mut R) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            let index = rng.gen_range(0..self.len());
            Some(self.swap_remove(index))
        }
    }
}

pub fn random_entry_(is_staging: bool, data: &web::Data<AppState>) -> RetRandomEntry {
    use rand::Rng;
    let new_token = AccessToken(Uuid::new_v4());
    let mut rng = rand::thread_rng();
    let mut waiting_list = data.waiting_list.lock().unwrap();
    let mut person_to_room = data.person_to_room.lock().unwrap();
    let mut room_to_gamestate = data.room_to_gamestate.lock().unwrap();
    let mut waiting_list_vec: Vec<AccessToken> = (*waiting_list).iter().copied().collect();
    let opt_token = waiting_list_vec.remove_random(&mut rng);
    if let Some(token) = opt_token {
        (*waiting_list).remove(&token);
        let room_id = open_a_room(token, new_token, is_staging);

        let is_first_turn_newtoken_turn: [WhoGoesFirst; 4] = [
            WhoGoesFirst::new(&mut rng),
            WhoGoesFirst::new(&mut rng),
            WhoGoesFirst::new(&mut rng),
            WhoGoesFirst::new(&mut rng),
        ];

        let is_ia_down_for_newtoken: bool = rng.gen();

        person_to_room.insert(
            new_token,
            RoomInfoWithPerspective {
                room_id,
                is_first_move_my_move: is_first_turn_newtoken_turn.clone(),
                is_ia_down_for_me: is_ia_down_for_newtoken,
            },
        );
        person_to_room.insert(
            token,
            RoomInfoWithPerspective {
                room_id,
                is_first_move_my_move: [
                    is_first_turn_newtoken_turn[0].not(),
                    is_first_turn_newtoken_turn[1].not(),
                    is_first_turn_newtoken_turn[2].not(),
                    is_first_turn_newtoken_turn[3].not(),
                ],
                is_ia_down_for_me: !is_ia_down_for_newtoken,
            },
        );

        let is_ia_owner_s_turn =
            is_first_turn_newtoken_turn[0 /* spring */].result == is_ia_down_for_newtoken;
        room_to_gamestate.insert(
            room_id,
            GameState {
                tam_itself_is_tam_hue: true,
                season: Season::Iei2,
                rate: Rate::X1,
                ia_owner_s_score: 20,
                is_ia_owner_s_turn,
                f: absolute::Field {
                    board: absolute::yhuap_initial_board(),
                    a_side_hop1zuo1: vec![],
                    ia_side_hop1zuo1: vec![],
                },
                waiting_for_after_half_acceptance: None,
                moves_to_be_polled: [vec![], vec![], vec![], vec![]],
            },
        );
        return RetRandomEntry::RoomAlreadyAssigned {
            access_token: format!("{}", new_token),
            is_first_move_my_move: is_first_turn_newtoken_turn[0 /* spring */].clone(),
            is_ia_down_for_me: is_ia_down_for_newtoken,
        };
    }

    todo!()
}

pub fn open_a_room(_token: AccessToken, _new_token: AccessToken, _is_staging: bool) -> RoomId {
    RoomId(Uuid::new_v4())
}

pub fn open_a_room_against_bot(
    _token: BotToken,
    _new_token: AccessToken,
    _is_staging: bool,
) -> RoomId {
    RoomId(Uuid::new_v4())
}

pub fn vs_cpu_entry_(is_staging: bool, data: &web::Data<AppState>) -> RetVsCpuEntry {
    use rand::Rng;
    let new_token = AccessToken(Uuid::new_v4());
    let bot_token = BotToken(Uuid::new_v4());
    let room_id = open_a_room_against_bot(bot_token, new_token, is_staging);
    let mut rng = rand::thread_rng();
    let is_first_turn_newtoken_turn = [
        WhoGoesFirst::new(&mut rng),
        WhoGoesFirst::new(&mut rng),
        WhoGoesFirst::new(&mut rng),
        WhoGoesFirst::new(&mut rng),
    ];

    let is_ia_down_for_newtoken: bool = rng.gen();
    let mut person_to_room = data.person_to_room.lock().unwrap();
    let mut room_to_gamestate = data.room_to_gamestate.lock().unwrap();
    let mut rooms_where_opponent_is_bot = data.rooms_where_opponent_is_bot.lock().unwrap();
    person_to_room.insert(
        new_token,
        RoomInfoWithPerspective {
            room_id,
            is_first_move_my_move: is_first_turn_newtoken_turn.clone(),
            is_ia_down_for_me: is_ia_down_for_newtoken,
        },
    );

    rooms_where_opponent_is_bot.insert(room_id);
    room_to_gamestate.insert(
        room_id,
        GameState {
            tam_itself_is_tam_hue: true,
            season: Season::Iei2,
            rate: Rate::X1,
            ia_owner_s_score: 20,
            is_ia_owner_s_turn: is_first_turn_newtoken_turn[0].result == is_ia_down_for_newtoken,
            f: absolute::Field {
                board: absolute::yhuap_initial_board(),
                a_side_hop1zuo1: vec![],
                ia_side_hop1zuo1: vec![],
            },
            waiting_for_after_half_acceptance: None,
            moves_to_be_polled: [vec![], vec![], vec![], vec![]],
        },
    );

    RetVsCpuEntry::LetTheGameBegin {
        access_token: format!("{}", new_token),
        is_first_move_my_move: is_first_turn_newtoken_turn[0].clone(),
        is_ia_down_for_me: is_ia_down_for_newtoken,
    }
}

pub fn random_entrance_cancel(
    _is_staging: bool,
    msg: &web::Json<MsgWithAccessToken>,
    data: &web::Data<AppState>,
) -> RetRandomCancel {
    if let Ok(access_token) = AccessToken::parse_str(&msg.access_token) {
        let person_to_room = data.person_to_room.lock().unwrap();
        let mut waiting_list = data.waiting_list.lock().unwrap();
        match person_to_room.get(&access_token) {
            // you already have a room. you cannot cancel
            Some(_) => RetRandomCancel::Ok { cancellable: false },
            None => {
                if waiting_list.contains(&access_token) {
                    // not yet assigned a room, but is in the waiting list
                    waiting_list.remove(&access_token);
                    RetRandomCancel::Ok { cancellable: true }
                } else {
                    // You told me to cancel, but I don't know you. Hmm...
                    // well, at least you can cancel
                    RetRandomCancel::Ok { cancellable: true }
                }
            }
        }
    } else {
        RetRandomCancel::Err {
            why_illegal: S("access token could not be parsed"),
        }
    }
}
