use std::{
    collections::HashMap,
    fmt::Debug,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::{Parser, arg, command};
use custom_benchmark::{
    sorts::{Element, HasName, ParSort, SeqSort, Sort},
    threshold_calc::get_threshold,
    utils::{self, get_input_array},
};
use rand::seq::SliceRandom;

// const SEQ_ARRAY_SIZES: &[usize] = &[10_000, 100_000, 1_000_000];
const SEQ_ARRAY_SIZES: &[usize] = &[];
const PAR_ARRAY_SIZES: &[usize] = &[
    // 10_000,
    // 100_000,
    // 1_000_000,
    // 2_000_000,
    // 5_000_000,
    10_000_000,
    20_000_000,
    // 50_000_000,
    // 100_000_000,
];
// const PAR_ARRAY_SIZES: &[usize] = &[1_000_000];
// const SAMPLES_PER_SIZE: u32 = 5;
// const fn samples_per_size(size: usize) -> u32 {
//     1
// }
const fn samples_per_size(size: usize) -> u32 {
    if size <= 100_000 {
        400
    } else if size <= 1_000_000 {
        200
    } else if size <= 10_000_000 {
        100
    } else {
        100
    }
}
const BENCHED_SEQ_SORTS: &[SeqSort] = &[
    SeqSort::SlicesUnchecked, SeqSort::Verus, SeqSort::Slices
    ];
const BENCHED_PAR_SORTS: &[ParSort] = &[
    // ParSort::SlicesUnchecked,
    // ParSort::SlicesUncheckedVspawn,
    // ParSort::Verus,
    // ParSort::VerusNoGhost,
    // ParSort::VerusNoGhostLessArcs,
    // ParSort::VerusNoGhostNoArc,
    // ParSort::VerusNoGhostMuninit,
    ParSort::VerusLessArcs,
    // ParSort::Slices,
    // ParSort::ImposterSlices,
    // ParSort::SlicesBlackbox,
    // ParSort::Rayon,
    ParSort::NakedVerus,
    // ParSort::NakedVerusNoScope,
    // ParSort::NakedVerusArcClone,
    // ParSort::NakedVerusArcCloneNoScope,
    // ParSort::NakedVerusArc,
    // ParSort::NakedVerusArcNoScope,
    // ParSort::NakedVerusRayon,
];

fn bench_sort_seq(sort: &SeqSort, input: &Vec<Element>) -> Duration {
    let input = input.clone();
    let (mut input, mut buf) = Sort::Seq(*sort).prepare_array(input);
    let start = Instant::now();
    sort.sort(&mut input, &mut buf);
    start.elapsed()
}

fn bench_sort_par(sort: &ParSort, input: &Vec<Element>) -> Duration {
    let input = input.clone();
    let threshold = get_threshold(input.len());
    let (mut input, mut buf) = Sort::Par(*sort).prepare_array(input);
    let start = Instant::now();
    sort.sort_parallel(&mut input, &mut buf, threshold);
    start.elapsed()
}

fn bench_sorts_once<S: HasName>(
    sorts: &[S],
    bench: impl Fn(&S, &Vec<Element>) -> Duration,
    input: &Vec<Element>,
) -> HashMap<SortName, Duration> {
    // This is to shuffle the order in which execute the algorithms
    // (to minimize the effect of order)
    let mut sorts = sorts.iter().collect::<Vec<_>>();
    sorts.shuffle(&mut rand::rng());

    sorts
        .iter()
        .map(|sort| (sort.name(), bench(sort, input)))
        .collect()
}

fn estimate_time(parallel: bool, sequential: bool) -> Duration {
    let mut time = Duration::ZERO;

    if parallel {
        for &size in PAR_ARRAY_SIZES {
            let input = get_input_array(size);
            let start = Instant::now();
            bench_sorts_once(BENCHED_PAR_SORTS, bench_sort_par, &input);
            let time_spent = start.elapsed();
            time += time_spent * samples_per_size(size);
        }
    };
    if sequential {
        for &size in SEQ_ARRAY_SIZES {
            let input = get_input_array(size);
            let start = Instant::now();
            bench_sorts_once(BENCHED_SEQ_SORTS, bench_sort_seq, &input);
            let time_spent = start.elapsed();
            time += time_spent * samples_per_size(size);
        }
    }
    time
}

type Micros = u128;
type SortName = &'static str;
type SortsSample = HashMap<SortName, Vec<Micros>>;
type SortStats = HashMap<usize, SortsSample>;

fn _bench_sorts<S: HasName>(
    sorts: &[S],
    bench: impl Fn(&S, &Vec<Element>) -> Duration,
    sizes: &[usize],
) -> SortStats {
    let mut res = HashMap::new();
    for &size in sizes {
        println!("benchmarking size = {size}");
        let mut stats = sorts
            .iter()
            .map(|s| (s.name(), vec![]))
            .into_iter()
            .collect::<HashMap<_, _>>();

        let mut progress: f32 = 0.0;
        let fraction = 10. / samples_per_size(size) as f32;
        print!("{progress:.0} ");
        for _ in 0..samples_per_size(size) {
            let input = utils::get_input_array(size);
            let micros = bench_sorts_once(sorts, &bench, &input)
                .into_iter()
                .map(|(s, d)| (s, d.as_micros()));
            for (s, d) in micros {
                stats.get_mut(&s).unwrap().push(d);
            }

            progress += fraction;
            print!("{progress:.0} ");
            let _ = std::io::stdout().flush();
        }
        println!();

        res.insert(size, stats);
    }
    res
}

fn bench_sorts_seq() -> SortStats {
    println!("Starting sequential benchmark");
    let res = _bench_sorts(BENCHED_SEQ_SORTS, bench_sort_seq, SEQ_ARRAY_SIZES);
    println!("Finishing sequential benchmark");
    res
}

fn bench_sorts_parallel() -> SortStats {
    println!("Starting parallel benchmark");
    let res = _bench_sorts(BENCHED_PAR_SORTS, bench_sort_par, PAR_ARRAY_SIZES);
    println!("Finishing parallel benchmark");
    res
}

#[derive(Parser, Debug)]
#[command(name = "bench-sorts")]
#[command(about = "Runs benchmarks for various sort implementations", long_about = None)]
struct Args {
    /// Run sequential sorts
    #[arg(long, aliases = ["seq"])]
    sequential: bool,

    /// Run parallel sorts
    #[arg(long, aliases = ["par"])]
    parallel: bool,

    #[arg(long = "out", value_name = "FILE")]
    out: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    if !args.sequential && !args.parallel {
        eprintln!("Please specify at least one of --sequential/--seq or --parallel/--par");
        std::process::exit(1);
    }

    // println!("Estimating time");
    // let time = estimate_time(args.parallel, args.sequential);
    // println!(
    //     "Benchmarking will take approximately {} minutes",
    //     time.as_millis() / 1000 / 60
    // );

    let mut res: HashMap<SortName, SortStats> = HashMap::new();

    if args.sequential {
        res.insert("sequential", bench_sorts_seq());
    }

    if args.parallel {
        res.insert("parallel", bench_sorts_parallel());
    }

    let output = serde_json::to_string_pretty(&res).unwrap();

    match &args.out {
        Some(path) => {
            std::fs::write(path, output).expect("Failed to write results to file");
            println!("Results written to {}", path.display());
        }
        None => {
            println!("{output}");
        }
    }
}
