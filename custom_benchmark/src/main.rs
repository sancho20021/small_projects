use std::{
    collections::HashMap,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::{Parser, arg, command};
use custom_benchmark::{
    sorts::{Element, Sort},
    threshold_calc::get_threshold,
    utils::{self, get_input_array},
};

// const SEQ_ARRAY_SIZES: &[usize] = &[65_536, 250_000, 1_000_000, 5_000_000];
const SEQ_ARRAY_SIZES: &[usize] = &[];
// const SEQ_ARRAY_SIZES: &[usize] = &[65_536];
const PAR_ARRAY_SIZES: &[usize] = &[500_000, 1_000_000, 5_000_000, 50_000_000, 100_000_000];
// const PAR_ARRAY_SIZES: &[usize] = &[1_000_000];
// const SAMPLES_PER_SIZE: u32 = 5;
const SAMPLES_PER_SIZE: u32 = 100;
const BENCHED_SORTS: &[Sort] = &[
    Sort::Slices,
    Sort::SlicesUnchecked,
    Sort::Verus,
    Sort::NakedVerus,
    Sort::SlicesUncheckedVspawn,
];

fn bench_sort(sort: &Sort, input: &Vec<Element>, parallel: bool) -> Duration {
    let size = input.len();
    let input = input.clone();
    let (mut input, mut buf) = sort.prepare_array(input);
    let mut run: Box<dyn FnMut()> = if parallel {
        let threshold = get_threshold(size);
        Box::new(move || sort.sort_parallel(&mut input, &mut buf, threshold))
    } else {
        Box::new(|| sort.sort(&mut input, &mut buf))
    };
    let start = Instant::now();
    run();
    start.elapsed()
}

fn estimate_time(parallel: bool, sequential: bool) -> Duration {
    let mut time = Duration::ZERO;

    if parallel && !PAR_ARRAY_SIZES.is_empty() {
        let mean_size = PAR_ARRAY_SIZES.iter().cloned().sum::<usize>() / PAR_ARRAY_SIZES.len();
        let input = get_input_array(mean_size);
        let start = Instant::now();
        bench_sorts_once(&input, true);
        let time_spent = start.elapsed();
        let time_spent = time_spent * SAMPLES_PER_SIZE * PAR_ARRAY_SIZES.len() as u32;
        time += time_spent;
    };
    if sequential && !SEQ_ARRAY_SIZES.is_empty() {
        let mean_size = SEQ_ARRAY_SIZES.iter().cloned().sum::<usize>() / SEQ_ARRAY_SIZES.len();
        let input = get_input_array(mean_size);
        let start = Instant::now();
        bench_sorts_once(&input, false);
        let time_spent = start.elapsed();
        let time_spent = time_spent * SAMPLES_PER_SIZE * SEQ_ARRAY_SIZES.len() as u32;
        time += time_spent;
    }
    time
}

fn bench_sorts_once(input: &Vec<Element>, parallel: bool) -> HashMap<Sort, Duration> {
    BENCHED_SORTS
        .iter()
        .map(|sort| (*sort, bench_sort(sort, input, parallel)))
        .collect()
}

type Micros = u128;

/// Returns map : sort -> [time in micros]
type SortsSample = HashMap<Sort, Vec<Micros>>;

fn bench_sorts_with_size(size: usize, parallel: bool) -> SortsSample {
    println!("benchmarking size = {size}");

    let mut res = BENCHED_SORTS
        .iter()
        .map(|s| (*s, vec![]))
        .into_iter()
        .collect::<HashMap<_, _>>();

    let mut progress: f32 = 0.0;
    let fraction = 10. / SAMPLES_PER_SIZE as f32;
    print!("{progress:.0} ");
    for _ in 0..SAMPLES_PER_SIZE {
        let input = utils::get_input_array(size);
        let micros = bench_sorts_once(&input, parallel)
            .into_iter()
            .map(|(s, d)| (s, d.as_micros()));
        for (s, d) in micros {
            res.get_mut(&s).unwrap().push(d);
        }

        progress += fraction;
        print!("{progress:.0} ");
        let _ = std::io::stdout().flush();
    }
    println!();
    res
}

type SortStats = HashMap<usize, SortsSample>;

fn _bench_sorts(sizes: &[usize], parallel: bool) -> SortStats {
    let mut res = HashMap::new();
    for size in sizes {
        res.insert(*size, bench_sorts_with_size(*size, parallel));
    }
    res
}

fn bench_sorts() -> SortStats {
    println!("Starting sequential benchmark");
    let res = _bench_sorts(SEQ_ARRAY_SIZES, false);
    println!("Finishing sequential benchmark");
    res
}

fn bench_sorts_parallel() -> SortStats {
    println!("Starting parallel benchmark");
    let res = _bench_sorts(PAR_ARRAY_SIZES, true);
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

    println!("Estimating time");
    let time = estimate_time(args.parallel, args.sequential);
    println!(
        "Benchmarking will take approximately {} minutes",
        time.as_millis() / 1000 / 60
    );

    let mut res: HashMap<&'static str, SortStats> = HashMap::new();

    if args.sequential {
        res.insert("sequential", bench_sorts());
    }

    if args.parallel {
        res.insert("parallel", bench_sorts_parallel());
    }

    let output = format!("{res:#?}\n");

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
