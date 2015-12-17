pub type Version = i64;

use std::vec::Vec;

#[derive(Clone)]
pub struct Proposal<T: Clone> {
    pub version: Version,
    pub value: T,
}

pub struct Acceptor<T: Clone> {
    max_prepare_version: Option<Version>,
    accepted_proposal: Option<Proposal<T>>,
}

impl<T: Clone> Acceptor<T> {
    pub fn prepare(&mut self, version: Version) -> Option<Proposal<T>> {
        // TODO(mrjones): mutex
        if self.max_prepare_version.map_or(true, |old_ver| old_ver < version) {
            self.max_prepare_version = Some(version)
        }

        return self.accepted_proposal.clone();
    }

    pub fn accept(&mut self, proposal: Proposal<T>) -> Option<Version> {
        if self.max_prepare_version.map_or(false, |v| v == proposal.version) {

            self.accepted_proposal = Some(proposal);
        }

        return self.accepted_proposal.as_ref().map(|p| p.version);
    }
}

pub struct Proposer<'a, T: Clone + 'a> {
    acceptors: Vec<&'a mut Acceptor<T>>,
    round: i64,
    my_id: i64,
}

impl<'a, T: Clone + 'a> Proposer<'a, T> {
    pub fn new(id: i64, acceptors: Vec<&'a mut Acceptor<T>>) -> Proposer<T> {
               return Proposer{
                   acceptors: acceptors,
                   round: 0,
                   my_id: id,
               }
               }

               fn version(&self) -> Version {
                   // TODO(mrjones): check for overflow
                   return self.round << 8 + self.my_id;
               }

    pub fn run(&mut self, initial_value: T) {
        let mut proposals = Vec::new();
        let version = self.version();
        for acceptor in self.acceptors.iter_mut() {
            match acceptor.prepare(version) {
                Some(p) => proposals.push(p),
                None => {},
            }
        }

        if proposals.len() == 0 {
            for acceptor in self.acceptors.iter_mut() {
                // TODO(mrjones): handle failure
                assert_eq!(version, acceptor.accept(Proposal{version: version, value: initial_value.clone()}).unwrap());
            }
        } else {
            // try again
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn simple() {
    }
}
