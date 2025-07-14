use std::{
    collections::HashMap,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use clap::{Parser, arg, command};
use custom_benchmark::{
    sorts::{
        Element, Sort, naked_verus::NakedVerus, slices::Slices, slices_unchecked::SlicesUnchecked,
        verus::Verus,
    },
    threshold_calc::get_threshold,
    utils::{self, get_input_array},
};

const SEQ_ARRAY_SIZES: &[usize] = &[65_536, 250_000, 1_000_000, 5_000_000];
const PAR_ARRAY_SIZES: &[usize] = &[
    500_000, 1_000_000, 5_000_000,
    50_000_000,
    100_000_000
    ];
const SAMPLES_PER_SIZE: u32 = 100;

fn bench_sort<S: Sort>(input: &Vec<Element>, parallel: bool) -> Duration {
    let size = input.len();
    let input = input.clone();
    let (mut input, mut buf) = S::prepare_array(input);
    let mut run: Box<dyn FnMut()> = if parallel {
        let threshold = get_threshold(size);
        Box::new(move || S::sort_parallel(&mut input, &mut buf, threshold))
    } else {
        Box::new(|| S::sort(&mut input, &mut buf))
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

fn bench_sorts_once(
    input: &Vec<Element>,
    parallel: bool,
) -> (Duration, Duration, Duration, Duration) {
    (
        bench_sort::<Verus>(input, parallel),
        bench_sort::<Slices>(input, parallel),
        bench_sort::<SlicesUnchecked>(input, parallel),
        bench_sort::<NakedVerus>(input, parallel),
    )
}

type Micros = u128;

/// Returns map : sort_name -> [time in micros]
type SortsSample = HashMap<&'static str, Vec<Micros>>;

fn bench_sorts_with_size(size: usize, parallel: bool) -> SortsSample {
    println!("benchmarking size = {size}");

    let mut res = [
        (Verus::name(), vec![]),
        (Slices::name(), vec![]),
        (SlicesUnchecked::name(), vec![]),
        (NakedVerus::name(), vec![]),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let mut progress: f32 = 0.0;
    let fraction = 10. / SAMPLES_PER_SIZE as f32;
    print!("{progress:.0} ");
    for _ in 0..SAMPLES_PER_SIZE {
        let input = utils::get_input_array(size);
        let (verus, slices, slices_unchecked, naked_verus) = {
            let x = bench_sorts_once(&input, parallel);
            (
                x.0.as_micros(),
                x.1.as_micros(),
                x.2.as_micros(),
                x.3.as_micros(),
            )
        };
        res.get_mut(&Verus::name()).unwrap().push(verus);
        res.get_mut(&Slices::name()).unwrap().push(slices);
        res.get_mut(&SlicesUnchecked::name())
            .unwrap()
            .push(slices_unchecked);
        res.get_mut(&NakedVerus::name()).unwrap().push(naked_verus);

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

// fn print_stats(stats: &HashMap<&'static str, SortStats>) {
//     map_values(stats, |stats| map_values(stats, |stats| map_v))
// }

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
