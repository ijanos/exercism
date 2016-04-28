fn solve(chain: &Vec<Domino>, pool: &Vec<Domino>) -> Option<Vec<Domino>> {
    if pool.is_empty() {
        // If the chain is finished (the pool of available domninoes is empty)
        // check that the last and first numbers match
        if chain.len() > 0 && chain.last().unwrap().end() != chain.first().unwrap().front() {
            return None;
        } else {
            return Some(chain.to_owned());
        }
    }

    let end = chain.last().unwrap().end(); // get the last number on the current chain
    // collect dominoes that can continue the chain
    let potentials = pool.iter().enumerate().filter(|&(_, &d)| d.has(end)).collect::<Vec<_>>();
    for (i, &p) in potentials {
        let mut chain = chain.to_owned();
        // rotate domino if necessary
        let p = if chain.last().unwrap().end() != p.front() {
            p.flipped()
        } else {
            p
        };
        // add the domino to the chain
        chain.push(p);
        let mut pool = pool.to_owned();
        pool.remove(i); // remove domino from the pool
        if let Some(chain) = solve(&chain, &pool) {
            return Some(chain);
        }
    }
    return None;
}

pub fn chain(input: &Vec<Domino>) -> Option<Vec<Domino>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    let mut input = input.to_owned();
    let first = input.pop().unwrap();
    solve(&vec![first], &input)
}

pub type Domino = (usize, usize);

trait DominoTools {
    fn flipped(&self) -> Domino;
    fn has(&self, usize) -> bool;
    fn front(&self) -> usize;
    fn end(&self) -> usize;
}

impl DominoTools for Domino {
    fn flipped(&self) -> Domino {
        (self.1, self.0)
    }

    fn has(&self, n: usize) -> bool {
        self.0 == n || self.1 == n
    }

    fn front(&self) -> usize {
        self.0
    }

    fn end(&self) -> usize {
        self.1
    }
}
