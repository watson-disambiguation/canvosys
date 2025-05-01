
use std::{fs::*,io, collections::HashMap, error::Error, fmt};

use serde::Deserialize;

use crate::datatypes::*;

#[derive(Debug, Deserialize)]
pub struct ResultatsBureauRecord {

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

pub fn load_files(folder: &str) -> Result<Vec<File>, io::Error> {
    return read_dir(folder)?
        .collect::<Result<Vec<DirEntry>,io::Error>>()?
        .into_iter()
        .map(|dir_entry| dir_entry.path())
        .map(|path| File::open(path))
        .collect();
}

pub fn parse_election_csv(csv_files: Vec<File>) -> Result<Vec<Vec<ResultatsBureauRecord>>,csv::Error>  {
    return csv_files.into_iter()
        .map(|file| csv::Reader::from_reader(file))
        .map(|mut rdr| rdr.deserialize::<ResultatsBureauRecord>().collect::<Result<Vec<ResultatsBureauRecord>,csv::Error>>())
        .collect::<Result<Vec<Vec<ResultatsBureauRecord>>,csv::Error>>();
}

#[derive(Debug)]
pub struct ParsingError {
    reason: String,
}

impl ParsingError {
    fn new(message: &str) -> Self {
        Self {
            reason: message.into(),
        }
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Error for ParsingError {
}

pub fn parse_as_riding(raw_results: Vec<ResultatsBureauRecord>, party_list: &PartyList) -> Result<RidingResult,ParsingError> {
    let mut votes: HashMap<Party,usize> = party_list.get_party_vote();
    for station in raw_results.iter() {
        let party = party_list.get_party(&station.party_name);
        let curr_votes: usize = match votes.get(&party) {
            Some(n) => *n,
            None => panic!("There should always be a matching party, leftovers should be mapped to Other, so None shouldn't occur"),
        };
        let curr_votes = curr_votes + station.votes;
        votes.insert(party, curr_votes);
    };
    let first_line = match raw_results.get(0) {
        Some(line) => line,
        None => return Err(ParsingError::new("CSV file was empty")),
    };
    return Ok(RidingResult::new(Riding::new(&first_line.electoral_district_name, &first_line.electoral_district_number),votes))
}
