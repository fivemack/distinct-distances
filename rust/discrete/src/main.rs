use bit_set::BitSet;
use itertools::Itertools;
use rayon::prelude::*;

use core::sync::atomic::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// Number of dimensions
    #[arg(short, long, default_value_t = 2)]
    dimensions: usize,

    /// Number of points to place
    #[arg(long = "pts", default_value_t = 7)]
    n_pts: usize,

    /// Side of hypercube to place points in
    #[arg(long = "side", default_value_t = 7)]
    side: u8,

    /// List or count
    #[arg(long = "display", default_value_t = false)]
    display: bool,
}

fn dist(u: &[u8], v: &[u8]) -> u16 {
    (u.iter()
        .zip(v.iter())
        .map(|a| ((*a.0 as i16 - *a.1 as i16) * (*a.0 as i16 - *a.1 as i16)))
        .sum::<i16>()) as u16
}

fn distances(u: &[Vec<u8>]) -> Vec<u16> {
    u.iter()
        .tuple_combinations::<(_, _)>()
        .map(|a| dist(a.0, a.1))
        .collect()
}

fn distance_bitset(u: &[u8], d: usize, p: usize) -> BitSet {
    let mut bs = BitSet::with_capacity(p);
    for a in u
        .chunks(d)
        .tuple_combinations::<(_, _)>()
        .map(|a| dist(a.0, a.1))
    {
        bs.insert(a as usize);
    }
    bs
}

fn hypercube_points(dim: usize, bound: u8) -> Vec<Vec<u8>> {
    let jj = (1..=dim).map(|_| 0..bound);
    jj.multi_cartesian_product().collect()
}

struct Extender<'a> {
    actor: &'a dyn ResultAcceptor,
    initialised_length: usize,
    initialised_universe_offset: usize,

    dimensionality: usize,
    target_xln: usize,
    max_distance: usize,

    universe: &'a Vec<Vec<u8>>,

    state: Vec<u8>,
}

impl<'a> Extender<'a> {
    fn new(
        target_ln: usize,
        dimensionality: usize,
        universe: &'a Vec<Vec<u8>>,
        actor: &'a dyn ResultAcceptor,
    ) -> Extender<'a> {
        let mut state = Vec::new();
        state.resize(dimensionality * target_ln, 0);
        let target_xln = state.len();
        Extender {
            actor,
            state,
            initialised_length: 0,
            initialised_universe_offset: 0,
            dimensionality,
            target_xln,
            universe,
            max_distance: 0,
        }
    }

    fn initialise(&mut self, start_pts: Vec<Vec<u8>>, universe_offset: usize) {
        self.initialised_length = self.dimensionality * start_pts.len();
        for u in 0..start_pts.len() {
            for v in 0..self.dimensionality {
                self.state[v + u * self.dimensionality] = start_pts[u][v];
            }
        }
        self.initialised_universe_offset = universe_offset;
        let universe_biggest = (self.universe.iter().map(|a| *((*a).iter().max().unwrap())).max().unwrap()) as usize;
        self.max_distance = self.dimensionality * universe_biggest * universe_biggest;
    }

    fn execute(&mut self) {
        self.extend(self.initialised_length, self.initialised_universe_offset);
    }

    fn extend(&mut self, valid_length: usize, universe_offset: usize) {
        if valid_length == self.target_xln {
            let v: Vec<Vec<u8>> = self
                .state
                .chunks(self.dimensionality)
                .map(|x| x.to_vec())
                .collect();
            self.actor.act(&v);
            return;
        }
        let dd0 = distance_bitset(&self.state[0..valid_length], self.dimensionality, self.max_distance);
        let mut nds = BitSet::with_capacity(self.max_distance);
        for vx in universe_offset..self.universe.len() {
            let np = &(self.universe)[vx];
            nds.clone_from(&dd0);
            let mut ok = true;
            for w in self.state[0..valid_length].chunks(self.dimensionality) {
                let xd = dist(w, np) as usize;
                if nds.contains(xd) {
                    ok = false;
                    break;
                }
                nds.insert(xd);
            }
            if ok {
                for i in 0..self.dimensionality {
                    self.state[valid_length + i] = (*np)[i];
                }
                self.extend(valid_length + self.dimensionality, vx);
            }
        }
    }
}

trait ResultAcceptor {
    fn act(&self, v: &[Vec<u8>]);
}

struct ResultCounter {
    c: AtomicU64,
}

struct ResultPrinter {}

impl ResultAcceptor for ResultPrinter {
    fn act(&self, v: &[Vec<u8>]) {
        let mut sds = distances(v);
        sds.sort();
        println!("Found {:?} {:?}", v, sds);
    }
}

impl ResultAcceptor for ResultCounter {
    fn act(&self, _v: &[Vec<u8>]) {
        _ = self.c.fetch_add(1, Ordering::SeqCst);
    }
}

impl ResultCounter {
    fn new() -> ResultCounter {
        ResultCounter {
            c: AtomicU64::new(0),
        }
    }
    fn get_count(&self) -> u64 {
        self.c.load(Ordering::SeqCst)
    }
}

fn main() {
    let args = Args::parse();

    let pts = hypercube_points(args.dimensions, args.side);
    let mut tasks = (0..(pts.len() - 1))
        .tuple_combinations::<(_, _)>()
        .collect::<Vec<(usize, usize)>>();
    // par_iter seems to start at the end of the vector
    // and the larger jobs are at the start
    // so to avoid a single-threaded tail this is useful
    tasks.reverse();

    let counter = ResultCounter::new();
    let printer = ResultPrinter {};
    if args.display {
        let _ = tasks
            .par_iter()
            .map(|a| {
                let mut x = Extender::new(args.n_pts, args.dimensions, &pts, &printer);
                x.initialise(vec![pts[a.0].clone(), pts[a.1].clone()], a.1 + 1);
                eprint!("  {} {}    \r", a.0, a.1);
                x.execute();
            })
            .count();
    } else {
        let _ = tasks
            .par_iter()
            .map(|a| {
                let mut x = Extender::new(args.n_pts, args.dimensions, &pts, &counter);
                x.initialise(vec![pts[a.0].clone(), pts[a.1].clone()], a.1 + 1);
                eprint!("  {} {}    \r", a.0, a.1);
                x.execute();
            })
            .count();
    }
    if !args.display {
        println!("\n\n  Found {} answers", counter.get_count());
    }
}
