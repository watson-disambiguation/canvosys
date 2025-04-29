use std::{error::Error, io, process, fs, collections::HashMap, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResultatsBureauRecord {

    #[serde(rename="Electoral District Number/Numéro de circonscription")]
    electoral_district_number: String,
    #[serde(rename="Electoral District Name_English/Nom de circonscription_Anglais")]
    electoral_district_name: String,
    #[serde(rename="Polling Station Number/Numéro du bureau de scrutin")]
    polling_station_number: String,
    #[serde(rename = "Candidate Poll Votes Count/Votes du candidat pour le bureau")]
    votes: usize,
    #[serde(rename = "Political Affiliation Name_English/Appartenance politique_Anglais")]
    party_name: String,
}

#[derive(Debug,Clone)]
struct Riding {
    name: String,
    id: String,
}

impl Riding {
    fn new(name: &str, id: &str) -> Self {
        Self {
            name: String::from(name),
            id: String::from(id),
        }
    }
}
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Party {
    name: String,
    short_form: String,
}

impl Party {
    fn new(name: &str, short_form: &str) -> Self {
        Self {
            name: String::from(name),
            short_form: String::from(short_form)
        }
    }
}

struct PartyList {
    map: HashMap<String,Party>,
    other_party: Party,
}

impl PartyList {
    fn new() -> Self {
        Self {
            map: HashMap::from([
                (String::from("NDP-New Democratic Party"),Party::new("New Democratic Party", "NDP")),
                (String::from("Liberal"),Party::new("Liberal","LIB")),
                (String::from("Conservative"),Party::new("Conservative","CON")),
                (String::from("People's Party - PPC"),Party::new("People's Party","PPC")),
                (String::from("Bloc Québécois"),Party::new("Bloc Quebecois","BQ")),
                (String::from("Green Party"),Party::new("Green Party","GRN"))
            ]),
            other_party: Party::new("Independent/Other","IND"),
        }
    }

    fn get_party(&self, identifier: &str) -> Party {
        self.map.get(identifier).unwrap_or(&self.other_party).clone()
    }

    fn get_party_vote(&self) -> HashMap<Party,usize> {
        let mut party_vote: HashMap<Party,usize> = HashMap::new();
        for party in self.map.values() {
            party_vote.insert(party.clone(),0);
        };
        party_vote.insert(self.other_party.clone(),0);
        return party_vote;
    }
}


#[derive(Debug, Clone)]
struct RidingResult {
    riding: Riding,
    votes: HashMap<Party,usize>,
}



fn calculate_riding_result(polling_stations: Vec<ResultatsBureauRecord>, party_list: &PartyList) -> RidingResult {
    let mut result: HashMap<Party,usize> = party_list.get_party_vote();
    for station in polling_stations.iter() {
        let party = party_list.get_party(&station.party_name);
        let curr_votes: usize = match result.get(&party) {
            Some(n) => *n,
            None => panic!("There should always be a matching party, leftovers should be mapped to other, so None shouldn't occur"),
        };
        let curr_votes = curr_votes + station.votes;
        result.insert(party, curr_votes);
    };
    let first_line = polling_stations.get(0).expect("Should always be at least one polling station in a riding");

    return RidingResult {
        riding: Riding::new(&first_line.electoral_district_name,&first_line.electoral_district_number),
        votes: result,
    };
}

fn read_csv<P: AsRef<Path>>(path: P) -> Result<Vec<ResultatsBureauRecord>,Box<dyn Error>> {
    let csv_file = fs::File::open(path)?;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(csv_file);
    Ok(rdr.deserialize::<ResultatsBureauRecord>().collect::<Result<Vec<ResultatsBureauRecord>,csv::Error>>()?)
}

fn print_result(riding_result: &RidingResult) {
    println!("{}: {}", riding_result.riding.id, riding_result.riding.name);
    for (party, votes) in riding_result.votes.iter() {
        println!("{}: {}",party.short_form, votes)
    }
}

struct FinalRidingResult {
    riding: Riding,
    winner: Party,
}

impl FinalRidingResult {
    fn new(riding: &Riding, winner: &Party) -> Self {
        Self {
            riding: riding.clone(),
            winner: winner.clone(),
        }
    }
}

trait VotingSystem {
    fn vote(&self, riding_results: Vec<RidingResult> ) -> Vec<FinalRidingResult>;
}

struct FirstPastThePost {}

impl FirstPastThePost {
    fn new() -> Self {
        Self {}
    }
}

impl VotingSystem for FirstPastThePost {
    fn vote(&self, riding_results: Vec<RidingResult> ) -> Vec<FinalRidingResult> {
        riding_results.iter()
            .map(|riding_result| {
                let winner = riding_result.votes.iter()
                .max_by(|x, y| x.1.cmp(&y.1))
                .expect("There will always at least one party").0;
                return FinalRidingResult::new(&riding_result.riding,winner);
            })
            .collect()
    }
}

fn summarize(final_riding_results: Vec<FinalRidingResult>, party_list: &PartyList ) -> HashMap<Party,usize> {
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
    result
} 

fn run() -> Result<(), Box<dyn Error>> {
    let party_list = PartyList::new();
    let voting_system = FirstPastThePost::new();
    let election_folder = "./data/elections/2021/";
    let riding_results: Vec<RidingResult> = fs::read_dir(election_folder)?
        .map(|file| file.expect("All paths in the data folder should be valid").path())
        .map(|path| read_csv(path).expect("All CSVs should be valid"))
        .map(|polling_stations| calculate_riding_result(polling_stations,&party_list))
        .collect(); 
    let election_result = voting_system.vote(riding_results);
    println!("{:?}",summarize(election_result,&party_list));
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
