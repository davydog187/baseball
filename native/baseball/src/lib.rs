use rustler::types::atom::ok;
use rustler::{Encoder, Env, Error, NifUnitEnum, ResourceArc, Term};
use std::sync::RwLock;

#[derive(NifUnitEnum)]
pub enum Incident {
    HomeScore,
    AwayScore,
}

pub struct GameState {
    home_score: RwLock<f64>,
    away_score: RwLock<f64>,
    model_data: Vec<f64>
}

impl GameState {
    pub fn update(&self, incident: Incident) {
        use Incident::*;

        match incident {
            HomeScore => {
                let mut score = self.home_score.write().unwrap();

                *score += 1.0;
            },
            AwayScore => {
                let mut score = self.away_score.write().unwrap();

                *score += 1.0;
            }
        };

    }

    pub fn home_score(&self) -> f64 {
        *self.home_score.read().unwrap()
    }

    pub fn away_score(&self) -> f64 {
        *self.away_score.read().unwrap()
    }
}

pub fn on_load<'a>(env: Env, _load_info: Term<'a>) -> bool {
    rustler::resource_struct_init!(GameState, env);
    true
}

rustler::rustler_export_nifs! {
    "Elixir.Baseball",
    [
        ("prepare", 1, prepare),
        ("update", 2, update),
        ("get_scores", 1, get_scores),
        ("predict", 2, predict)
    ],
    Some(on_load)
}

fn prepare<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let data: Vec<f64> = args[0].decode()?;

    let state = GameState {
        home_score: RwLock::new(0.0),
        away_score: RwLock::new(0.0),
        model_data: data
    };

    let resource = ResourceArc::new(state);

    Ok((ok(), resource).encode(env))
}

fn update<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let state: ResourceArc<GameState> = args[0].decode()?;
    let incident: Incident = args[1].decode()?;

    state.update(incident);

    Ok((ok()).encode(env))
}

fn get_scores<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let state: ResourceArc<GameState> = args[0].decode()?;

    Ok((ok(), state.home_score() as i64, state.away_score() as i64).encode(env))
}


fn predict<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let state: ResourceArc<GameState> = args[0].decode()?;
    let multiplier: f64 = args[1].decode()?;

    let sum: f64 = state.model_data.iter().sum();

    let result = (sum + state.home_score() + state.away_score()) * multiplier / 1000.0;

    Ok((ok(), result).encode(env))
}
