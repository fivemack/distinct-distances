use argmin::core::{State,CostFunction,Error,Executor};
use argmin::solver::neldermead::NelderMead;

use rand::prelude::*;

use std::fs::File;
use std::io::Write;

fn score(v: &Vec<[f64;2]>) -> f64
{
    for u in v
    {
        if u[0]<0.0 || u[0]>1.0 || u[1]<0.0 || u[1]>1.0
        {
            return 0.0;
        }
    }
    let vx = [vec![[0.0,0.0]],v.clone(),vec![[1.0,1.0]]].concat();
    let n = vx.len();
    let cc0:Vec<_> = {0..n}
    .map(|a| {(a+1..n).map(move |b| (a,b))}.collect::<Vec<_>>()).collect();
    let cc = cc0.concat();

    let mut dd: Vec<f64> = cc.iter()
    .map(|a| [vx[a.0],vx[a.1]])
    .map(|[a,b]| { ((a[0]-b[0])*(a[0]-b[0])+(a[1]-b[1])*(a[1]-b[1])).sqrt()})
    .collect();
    dd.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let ee: Vec<f64> = [vec![dd[0]],dd.iter().zip(dd[1..].iter()).map(|a| a.1-a.0).collect::<Vec<_>>()].concat();
    let mut msf = 1e8_f64;
    for a in ee { if a<msf { msf=a; }}

    msf
}

fn score_flat(v:&Vec<f64>) -> f64
{
    let u: Vec<[f64;2]> = v.chunks(2).map(|p| [p[0],p[1]]).collect();
    score(&u)
}

struct Optimisand {
}

impl CostFunction for Optimisand{
    type Param = Vec<f64>;
    type Output = f64;
    
    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error>
    {
        let u: Vec<[f64;2]> = p.chunks(2).map(|p| [p[0],p[1]]).collect();
        Ok(-score(&u))
    }
}

fn rainbow(x: f64) -> (u8, u8, u8)
{
    if x<0.0 || x>1.0 { panic!(); }
    let xx = 6.0*x;
    let uu = (256.0 * (xx-xx.floor())) as u8;
    let vv = 255-uu;
    let tt = xx.floor() as u8;
    if tt == 0 { return (255,uu,0); }
    if tt == 1 { return (vv,255,0); }
    if tt == 2 { return (0,255,uu); }
    if tt == 3 { return (0,vv,255); }
    if tt == 4 { return (uu,0,255); }
    if tt == 5 { return (255,0,vv); }
    (0,0,0)
}

enum VisualisationMode { Three, Six, Fifteen }

fn visualise(pt : &Vec<[f64;2]>, m: VisualisationMode, filename: &str)
{
    const px: usize = 800;
    const border: usize = 32;

    let xs = 4*px + 3*border;
    let ys = px;

    let mut whole: Vec<(u8,u8,u8)> = Vec::new();
    whole.resize(xs*ys, (255,255,255));

    let G = score(&pt)*1.2;

    for y in 0..px
    {
        let yy = y*xs;
        for x in 0..px
        {
            let XX = (x as f64)/(px as f64);
            let YY = (y as f64)/(px as f64);
            let mut vv = pt.clone();
            let mut old = vv[0];
            vv[0]=[XX,YY];
            whole[x+yy]=rainbow(score(&vv)/G);
            vv[0]=old;
            old = vv[1];
            vv[1]=[XX,YY];
            whole[(x+border+px)+yy]=rainbow(score(&vv)/G);
            vv[1]=old;
            old = vv[2];
            vv[2]=[XX,YY];
            whole[x+2*(border+px)+yy]=rainbow(score(&vv)/G);
            vv[2]=old;
            vv[3]=[XX,YY];
            whole[x+3*(border+px)+yy]=rainbow(score(&vv)/G);

        }
    }

    let mut f = File::create(filename).unwrap();
    write!(f,"P6\n{} {}\n255\n",xs,ys).unwrap();
    f.write_all(&(whole.iter().map(|x| vec![x.0,x.1,x.2]).flatten().collect::<Vec<u8>>())).unwrap();
    
}

const SZ: usize = 2*3;

fn main() -> Result<(), Error> {
//   let init: Vec<[f64;2]> = vec![[0.4407,0.0106],[0.4908,0.3084],[0.9152,0.8882]];

   //visualise(&init, VisualisationMode::Three, "four.ppm");
   //Found -0.08830506712195119 at st=686 t=5 with [0.9146246361229086, 0.7396817429997604, 0.6763131346926479, 0.2559136457959466, 0.7932133717484757, 0.5992275754884522, 0.08290320461129147, 0.04695606306577746]


   let init: Vec<[f64;2]> = vec![[0.0563,0.0835],[0.1221,0.6550],[0.3227,0.2183],[0.8424,0.8887]];
   visualise(&init, VisualisationMode::Three, "four.ppm");

    println!("{}",score(&init));

    let mut rng = thread_rng();

    // Step 0: find the best score out of some number of random points
    let mut bsst = 0.0;
    let mut bx = Vec::new();
    for _r in 0..10000
    {
        let init0 = (1..=SZ).map(|u| rng.gen_range(0.0..1.0)).collect::<Vec<_>>();
        let sc = score_flat(&init0);
        if sc>bsst {bsst=sc; bx=init0.clone()}
    }
    println!("Best start score out of 10^4 samples is {}", bsst);
    let mut bsf = 0.0;
    for r in 0..10000
    {
//    let init0 = init.concat();
//    println!{"{:?}",init0};
//    let mut bsf = score(&init);
        let mut happy = false;
        let mut init0 = bx.clone();
        while !happy
        {
            init0 = (1..=SZ).map(|u| rng.gen_range(0.0..1.0)).collect::<Vec<_>>();
            if (score_flat(&init0)>bsst) { happy=true; }
        } 


    for q in 0..500
    {

        // for an n-dimensional NM we need n+1 init vectors
        // this code is for an axis oriented simplex with the start point inside 
        
        const ssl: f64 = 0.03;
        let dim = init0.len();
        let mut v: Vec<Vec<f64>> = Vec::with_capacity(1+dim);

        /* 
        let init00 = init0.iter().map(|u| u-ssl/(dim as f64)).collect::<Vec<_>>();

        v.push(init00.clone());
        for i in 0..dim
        {
            let mut init_i = init0.clone();
            init_i[i]+=ssl*(1.0 - 1.0/(dim as f64));
            v.push(init_i.clone());
        } */

        for i in 0..=dim
        {
            let mut init_i = init0.clone();
            for j in 0..dim
            {
                init_i[j] += rng.gen_range(-ssl..ssl);
            }
            v.push(init_i.clone());
        }

        let solver : NelderMead<Vec<f64>,f64> = NelderMead::new(v).with_sd_tolerance(1e-9)?;
        let cost = Optimisand {};
        let res = Executor::new(cost, solver);
        let res_aus = res.run()?;
        let sc = -res_aus.state().get_best_cost();
        if sc > bsf
        {
            bsf=sc;
            println!("Found {} at st={} t={} with {:?}", res_aus.state().get_best_cost(), r, q, res_aus.state().get_best_param().unwrap());
        }
    }
}

    Ok(())
}
