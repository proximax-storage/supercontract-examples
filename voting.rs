pub mod blockchain;
pub mod file;

use std::io::Read;
use std::io::Write;

use crate::file::FileReader;
use crate::file::FileWriter;

#[no_mangle]
pub extern "C" fn run() -> i32 {
    let mut election = Election {
        candidates: Vec::new(),
        voters: Vec::new(),
    };
    // Register candidates
    let candidate1 = Candidate {
        id: 1,
        name: String::from("Candidate 1"),
        photo: vec![1, 2, 3], // Placeholder binary photo data
        propaganda: String::from("Vote for me!"),
    };
    let candidate2 = Candidate {
        id: 2,
        name: String::from("Candidate 2"),
        photo: vec![4, 5, 6], // Placeholder binary photo data
        propaganda: String::from("I'm the best choice!"),
    };
    election.register_candidate(candidate1);
    election.register_candidate(candidate2);

    // Register voters
    let voter1 = Voter {
        public_key: [0u8; 32], //[0;32]
        has_voted: false,
    };
    let voter2 = Voter {
        public_key: [4u8; 32], // Placeholder public key
        has_voted: false,
    };

    election.register_voter(voter1);
    election.register_voter(voter2);
    // Voters cast their votes
    let x = election.cast_vote(0);
    assert_eq!(x, 1); // return 1 vote successfully
    let y = election.cast_vote(1);
    assert_eq!(y, 2); // return 2 voter has already voted
    //Check if all voters have cast their votes
    return election.check_votes();
}
// Candidate structure
#[allow(dead_code)]
struct Candidate {
    id: u8,
    name: String,
    photo: Vec<u8>,
    propaganda: String,
}
// Voter structure
struct Voter {
    public_key: [u8; 32],
    has_voted: bool,
}
// Election structure
struct Election {
    candidates: Vec<Candidate>,
    voters: Vec<Voter>,
}
impl Election {
    // Register a candidate
    pub fn register_candidate(&mut self, candidate: Candidate) -> u8 {
        self.candidates.push(candidate);

        return 0;
    }

    // Register a voter
    pub fn register_voter(&mut self, voter: Voter) {
        self.voters.push(voter);
    }
    // Cast a vote for a candidate
    pub fn cast_vote(&mut self, candidate_id: u8) -> i32 {
        let voter_key = blockchain::get_caller_public_key();
        let voter = self
            .voters
            .iter_mut()
            .find(|voter| voter.public_key == voter_key)
            .unwrap();
        if voter.has_voted == true {
            return 2;
        } else {
            voter.has_voted = true;

            let mut concatenated_vec = Vec::new();
            concatenated_vec.push(candidate_id);
            concatenated_vec.extend_from_slice(&voter_key);

            {
                let mut file = FileWriter::new("voter.txt").unwrap();
                file.write(concatenated_vec.as_mut_slice()).unwrap();
                file.flush().unwrap();

            }
            let mut buffer = vec![0u8; 33]; // Create a buffer to store the read data
            {
                let mut file_read = FileReader::new("voter.txt").unwrap();
                let len = file_read.read(&mut buffer[..]).unwrap();
                assert_eq!(len, 33);
            }

            // check vote
            if concatenated_vec != buffer {
                return 3;
            }

            return 1;
        }
    }

    // Check if all voters have cast their votes
    pub fn check_votes(&self) -> i32 {
        for voter in &self.voters {
            if !voter.has_voted {
                return 0;
            }
        }
        return 1;
    }
}
