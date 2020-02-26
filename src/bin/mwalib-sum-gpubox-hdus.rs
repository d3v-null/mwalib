/// Given gpubox files, add the contents of their HDUs and report the sum.
use anyhow::*;
use fitsio::FitsFile;
use structopt::StructOpt;

use mwalib::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "mwalib-sum-gpubox-hdus", author)]
struct Opt {
    /// Don't use mwalib - just iterate over the HDUs and add them. The result
    /// might be different because the start/end times of the observation may
    /// not be consistent.
    #[structopt(long)]
    direct: bool,

    /// Path to the metafits file.
    #[structopt(short, long)]
    metafits: Option<String>,

    /// Paths to the gpubox files.
    #[structopt(name = "GPUBOX FILE")]
    files: Vec<String>,

    /// Name of antenna1
    #[structopt(long)]
    ant1_name: String,

    /// Name of antenna2
    #[structopt(long)]
    ant2_name: String,
}

fn sum_direct(files: Vec<String>) -> Result<(), anyhow::Error> {
    println!("Summing directly from HDUs...");
    let mut sum: f64 = 0.0;
    for gpubox in files {
        println!("Reading {}", gpubox);
        let mut hdu_index = 1;
        let mut s: f64 = 0.0;
        let mut fptr = FitsFile::open(&gpubox)?;
        while let Ok(hdu) = fptr.hdu(hdu_index) {
            let buffer: Vec<f32> = hdu.read_image(&mut fptr)?;
            s += buffer.iter().map(|v| *v as f64).sum::<f64>();
            hdu_index += 1;
        }

        println!("Sum: {}", s);
        sum += s;
    }

    println!("Total sum: {}", sum);
    Ok(())
}

fn sum_mwalib(
    metafits: String,
    files: Vec<String>,
    ant1_name: String,
    ant2_name: String,
) -> Result<(), anyhow::Error> {
    println!("Summing via mwalib...");
    let mut context = mwalibContext::new(&metafits, &files)?;
    println!("Correlator version: {}", context.corr_version);

    let selected_baseline = misc::get_baseline_from_antenna_names(
        ant1_name.clone(),
        ant2_name.clone(),
        &context.antennas,
    )
    .unwrap();

    let mut sum: f64 = 0.0;
    let mut count: u64 = 0;
    for t in 0..context.num_timesteps {
        for c in 0..context.num_coarse_channels {
            print!("t {}, c {}...", t, c);
            let data = context.read_one_timestep_coarse_channel_bfp(t, c)?;

            for b in 0..context.num_baselines {
                let baseline_index =
                    b * (context.num_fine_channels * context.num_visibility_pols * 2);

                if b == selected_baseline {
                    let mut this_sum: f64 = 0.;
                    let mut this_count: u64 = 0;

                    for f in 0..context.num_fine_channels {
                        let fine_chan_index = f * (context.num_visibility_pols * 2);

                        for v in 0..8 {
                            this_sum += data[baseline_index + fine_chan_index + v] as f64;
                        }
                        this_count += 8;
                    }

                    println!("Sum: {}; Count: {}", this_sum, this_count);
                    sum += this_sum;
                    count += this_count;
                }
            }
        }
    }

    println!(
        "{} v {} (baseline: {}) Sum: {}; Count: {}",
        ant1_name, ant2_name, selected_baseline, sum, count
    );

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let opts = Opt::from_args();
    if opts.direct {
        sum_direct(opts.files)?;
    } else {
        // Ensure we have a metafits file.
        if let Some(m) = opts.metafits {
            sum_mwalib(m, opts.files, opts.ant1_name, opts.ant2_name)?;
        } else {
            bail!("A metafits file is required when using mwalib.")
        }
    }

    Ok(())
}
