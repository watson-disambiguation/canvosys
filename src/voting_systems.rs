use crate::datatypes::*;
use std::collections::HashMap;

pub trait VotingSystem {
    fn vote(&self, riding_results: Vec<RidingResult>, party_list: &PartyList) -> FinalElectionResult;
}

pub struct FirstPastThePost {}

impl FirstPastThePost {
    pub fn new() -> Self {
        Self {}
    }
}

impl VotingSystem for FirstPastThePost {
    fn vote(&self, riding_results: Vec<RidingResult> , party_list: &PartyList) -> FinalElectionResult {
        let final_riding_results = riding_results.iter()
            .map(|riding_result| {
                let winner = riding_result.get_votes().iter()
                .max_by(|x, y| x.1.cmp(&y.1))
                .expect("There will always at least one party").0;
                return FinalRidingResult::new(riding_result.get_riding(),winner);
            })
            .collect();
        return FinalElectionResult::new(final_riding_results, party_list);
    }
}

struct RankingProportions {
    percentage: usize,
    ranking: Vec<Party>,
}

struct Ranking {
    count: usize,
    ranking: Vec<Party>,
}

pub struct RankedChoice {
    ranking_map: HashMap<Party,Vec<RankingProportions>>
}

struct RankedRidingResult {
    total_votes: usize,
    riding: Riding,
    party_rankings: HashMap<Party,Ranking>
}

struct PartyRanking {
    count: usize,
    ranking: Vec<Party>,
}

impl RankedChoice {

    fn convert(&self, riding_result: RidingResult) -> RankedRidingResult {
        let rankings = riding_result.get_votes().iter().map(|(party,votes)| {
            todo!()
        });
        RankedRidingResult { 
            total_votes: riding_result.get_total_votes(), 
            riding: riding_result.get_riding().clone(), 
            party_rankings: todo!()
        }
    }
}

fn evaluate_riding_ranked(ranked_riding_result: RankedRidingResult) -> FinalRidingResult {
    todo!()
}

impl VotingSystem for RankedChoice {
    fn vote(&self, riding_results: Vec<RidingResult>, party_list: &PartyList) -> FinalElectionResult {
        let final_riding_results = riding_results.into_iter()
            .map(|riding_result| self.convert(riding_result))
            .map(|ranked_riding_result| evaluate_riding_ranked(ranked_riding_result))
            .collect();
        return FinalElectionResult::new(final_riding_results, party_list);
    }
}