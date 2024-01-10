use bit_set::BitSet;
use itertools::Itertools;
use rayon::prelude::*;

use core::sync::atomic::*;

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

fn distance_bitset(u: &Vec<Vec<u8>>) -> BitSet {
    let mut bs = BitSet::new();
    for a in distances(u) {
        bs.insert(a as usize);
    }
    bs
}

fn hypercube_points(dim: usize, bound: u8) -> Vec<Vec<u8>> {
    let jj = (1..=dim).map(|_| 0..bound);
    jj.multi_cartesian_product().collect()
}

fn extend(
    v: Vec<Vec<u8>>,
    target_ln: usize,
    universe: &[Vec<u8>],
    actor: &dyn ResultAcceptor
) {
    if v.len() == target_ln {
        actor.act(&v);
        return;
    }
    let dd0 = distance_bitset(&v);
    for vx in 0..(*universe).len() {
        let np = &universe[vx];
        let mut nds = dd0.clone();
        let mut ok = true;
        for w in &v {
            let xd = dist(w, &np) as usize;
            if nds.contains(xd) {
                ok = false;
                break;
            }
            nds.insert(xd);
        }
        if ok {
            let mut nv = v.clone();
            nv.push((*np).clone());
            extend(nv, target_ln, &universe[(1+vx)..], actor);
        }
    }
}


trait ResultAcceptor
{
    fn act(&self, v: &[Vec::<u8>]);
}

struct ResultCounter
{
    c: AtomicU64
}

struct ResultPrinter
{
}

impl ResultAcceptor for ResultPrinter
{
    fn act(&self, v: &[Vec<u8>]) {
        let mut sds = distances(v);
        sds.sort();
        println!("Found {:?} {:?}", v, sds);
    }    
}

impl ResultAcceptor for ResultCounter
{
    fn act(&self, _v: &[Vec::<u8>])
    {
        _ = self.c.fetch_add(1, Ordering::SeqCst);   
    }
}

impl ResultCounter
{
    fn new() -> ResultCounter
    {
        ResultCounter { c: AtomicU64::new(0) }     
    }
    fn get_count(&self) -> u64
    {
        return self.c.load(Ordering::SeqCst);
    }
}

fn main() {
    const SZ: u8 = 7;
    const N: usize = 7;
    let pts = hypercube_points(2, SZ);
    let tasks = (0..(pts.len() - 1))
        .tuple_combinations::<(_, _)>()
        .collect::<Vec<(usize, usize)>>();
    let counter = ResultCounter::new();
    let printer = ResultPrinter {};
    let _ = tasks
        .par_iter()
        .map(|a| {
            eprint!("  {} {}    \r", a.0, a.1);
            extend(
                vec![pts[a.0].clone(), pts[a.1].clone()],
                N,
                &pts[(1 + a.1)..],
                &counter
            )
        })
        .count();
    println!("\n\n  Found {} answers", counter.get_count());
}