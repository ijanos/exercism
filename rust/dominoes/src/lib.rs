fn solve(chain: &Vec<(usize, usize)>, pool: &Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    if pool.is_empty() {
        // If the chain is finished (the pool of available domninoes is empty)
        // check that the last and first numbers match
        if chain.len() > 0 && chain.last().unwrap().1 != chain.first().unwrap().0 {
            return None;
        } else {
            return Some(chain.to_owned());
        }
    }

    let link = chain.last().unwrap().1; // get the last number of the current chain
    // collect dominoes that can continue the chain
    let potentials = pool.iter()
                         .enumerate()
                         .filter(|&(_, &(j, k))| link == j || link == k)
                         .collect::<Vec<_>>();
    for (i, &p) in potentials {
        // add the domino to the chain
        let mut chain = chain.to_owned();
        if chain.last().unwrap().1 == p.0 {
            chain.push(p);
        } else {
            // rotate domino if necessary
            chain.push((p.1, p.0))
        }
        let mut pool = pool.to_owned();
        pool.remove(i); // remove domino from the pool
        if let Some(chain) = solve(&chain, &pool) {
            return Some(chain);
        }
    }
    return None;
}

pub fn chain(input: &Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    let mut input = input.to_owned();
    let first = input.pop().unwrap();
    solve(&vec![first], &input)
}
