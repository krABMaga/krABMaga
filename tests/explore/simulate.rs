#[cfg(test)]
#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
use {
    crate::model::flockers::state::*, krABMaga::engine::schedule::Schedule,
    krABMaga::engine::state::State, krABMaga::*,
};

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn simulate() {
    let step = 10;

    let dim = (200., 200.);
    let num_agents = 100;

    let res = simulate_old!(step, Flocker::new(dim, num_agents), 0, Info::Normal);
    assert_eq!(res.len(), 0);

    let res = simulate_old!(step, Flocker::new(dim, num_agents), 1, Info::Normal);
    assert_eq!(res.len(), 1);

    let res = simulate_old!(step, Flocker::new(dim, num_agents), 2, Info::Normal);
    assert_eq!(res.len(), 2);

    for r in res {
        let (duration, step_per_sec) = r;
        assert!(duration.as_secs_f32() > 0.);
        assert!(step_per_sec > 0.);
    }
}

#[cfg(not(any(
    feature = "visualization",
    feature = "visualization_wasm",
    feature = "parallel"
)))]
#[test]
fn simulate_verbose() {
    let step = 10;

    let dim = (200., 200.);
    let num_agents = 100;

    let state = Flocker::new(dim, num_agents);
    simulate_old!(step, state, 1, Info::Verbose);
}
