use std::io;

use canvosys::{datatypes::*,parsing,voting_systems::VotingSystem,voting_systems};

fn inputs(party_list: &PartyList) -> Vec<RidingResult> {
    println!("Input election year you wish to use");
    loop {
        let mut election_folder = String::from("./data/elections/");
        let mut election_year = String::new();
        match io::stdin().read_line(&mut election_year) {
            Ok(_) => (),
            Err(e) => {
                println!("{} - Failed to read in election year, try again.", e);
                continue;
            },
        };
        election_folder.push_str(&election_year.trim());
        let csv_files = match parsing::load_files(&election_folder) {
            Ok(f) => f,
            Err(e) => {
                println!("{} - {} - Issue finding files for election, try again.", e, election_folder);
                continue;
            },
        };

        let unparsed_riding_results = match parsing::parse_election_csv(csv_files) {
            Ok(r) => r,
            Err(e) => {
                println!("{} - Files are not correctly formatted as CSV.", e);
                continue;
            },
        };

        return match unparsed_riding_results.into_iter().map(|result| parsing::parse_as_riding(result, party_list)).collect() {
            Ok(r) => r,
            Err(e) => {
                println!("{} - Files are not correctly formatted as CSV.", e);
                continue;
            },
        };
    };
}

fn output(election_result: FinalElectionResult) {
    for (party, seats) in election_result.get_seat_counts().iter() {
        let seats = seats.clone();
        let plural = if seats == 1 {""} else {"s"};
        println!("{} got {} seat{}", party.get_name(), seats, plural );
    }
}

fn main() {
    let voting_system = voting_systems::FirstPastThePost::new();
    let party_list = PartyList::new();
    let riding_results = inputs(&party_list);
    let election_result = voting_system.vote(riding_results, &party_list);
    output(election_result);
}
