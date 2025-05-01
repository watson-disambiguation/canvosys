use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Riding {
    name: String,
    id: String,
}

impl Riding {
    pub fn new(name: &str, id: &str) -> Self {
        Self {
            name: String::from(name),
            id: String::from(id),
        }
    }
}
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct Party {
    name: String,
    short_form: String,
}

impl Party {
    pub fn new(name: &str, short_form: &str) -> Self {
        Self {
            name: String::from(name),
            short_form: String::from(short_form)
        }
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }
}

pub struct PartyList {
    map: HashMap<String,Party>,
    short_form_map: HashMap<String,Party>,
    other_party: Party,
}

impl PartyList {
    pub fn new() -> Self {
        let map = HashMap::from([
                (String::from("NDP-New Democratic Party"),Party::new("New Democratic Party", "NDP")),
                (String::from("Liberal"),Party::new("Liberal","LIB")),
                (String::from("Conservative"),Party::new("Conservative","CON")),
                (String::from("People's Party - PPC"),Party::new("People's Party","PPC")),
                (String::from("Bloc Québécois"),Party::new("Bloc Quebecois","BQ")),
                (String::from("Green Party"),Party::new("Green Party","GRN"))
            ]);
       ;
        Self {
            map: map.clone(),
            short_form_map: map.values().map(|party| (party.short_form.clone(),party.clone())).collect(),
            other_party: Party::new("Independent/Other","IND"),
        }
    }

    pub fn get_party(&self, identifier: &str) -> Party {
        self.map.get(identifier).unwrap_or(&self.other_party).clone()
    }

    pub fn get_party_short(&self, identifier: &str) -> Party {
        self.map.get(identifier).unwrap_or(&self.other_party).clone()
    }

    pub fn get_party_vote(&self) -> HashMap<Party,usize> {
        let mut party_vote: HashMap<Party,usize> = HashMap::new();
        for party in self.map.values() {
            party_vote.insert(party.clone(),0);
        };
        party_vote.insert(self.other_party.clone(),0);
        return party_vote;
    }
}

#[derive(Debug, Clone)]
pub struct RidingResult {
    total_votes: usize,
    riding: Riding,
    votes: HashMap<Party,usize>,
}

impl RidingResult {
    pub fn new(riding: Riding, votes: HashMap<Party,usize>) -> Self {
        let total_votes = votes.values().fold(0,|sum, cur| sum + cur);
        Self {
            total_votes,
            riding,
            votes,
        }
    }

    pub fn get_riding(&self) -> &Riding {
        return &self.riding;
    }

    pub fn get_votes(&self) -> &HashMap<Party,usize> {
        return &self.votes;
    }

    pub fn get_total_votes(&self) -> usize {
        return self.total_votes;
    }
}

pub struct FinalRidingResult {
    riding: Riding,
    winner: Party,
}

impl FinalRidingResult {
    pub fn new(riding: &Riding, winner: &Party) -> Self {
        Self {
            riding: riding.clone(),
            winner: winner.clone(),
        }
    }

}

pub struct FinalElectionResult {
    seat_counts: HashMap<Party,usize>,
    riding_results: HashMap<Riding,FinalRidingResult>,
}

impl FinalElectionResult {
    pub fn new(final_riding_results: Vec<FinalRidingResult>, party_list: &PartyList) -> Self {
        let mut result: HashMap<Party,usize> = party_list.get_party_vote();
        for riding in final_riding_results.iter() {
            let party = riding.winner.clone();
            let curr_seats: usize = match result.get(&party) {
                Some(n) => *n,
                None => 0
            };
            let curr_seats = curr_seats + 1;
            result.insert(party, curr_seats);
        };
        ;
        return FinalElectionResult {
            seat_counts: result,
            riding_results: final_riding_results.into_iter().map(|result| (result.riding.clone(), result)).collect(),
        };
    }

    pub fn get_seat_counts(&self) -> &HashMap<Party,usize> {
        return &self.seat_counts;
    }
}









