use ::{
    anyhow::Result,
    clap::Parser,
    indicatif::{ProgressBar, ProgressStyle},
    regex::Regex,
    ssh_key::{rand_core::OsRng, Algorithm, Fingerprint, HashAlg, LineEnding, PrivateKey},
    std::{
        path::Path,
        sync::atomic::{AtomicBool, Ordering},
    },
};

/// A simple tool to generate SSH vanity keys whose fingerprint matches a given
/// regular expression.
#[derive(Parser)]
struct Args {
    /// A regular expression that should match an SSH key fingerprint.
    /// Regular expressions that cannot match a base64 string will result
    /// in the tool running indefinitely without finding a match. Avoid
    /// attempting to match more than ~7-8 characters as it will take a
    /// very long time to complete.
    regex: Regex,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let regex = &args.regex;

    let style = ProgressStyle::with_template("[{elapsed}/{duration}] [{bar}] ({per_sec})")
        .expect("bad progress bar style");
    let pb = ProgressBar::no_length().with_style(style);

    let chunks = std::thread::available_parallelism()?.get();
    let found = AtomicBool::new(false);
    std::thread::scope(|s| {
        let mut joins = vec![];
        for _ in 0..chunks {
            joins.push(s.spawn(|| -> Option<(PrivateKey, Fingerprint)> {
                let mut rng = OsRng;
                let mut count = 0;
                loop {
                    let private_key = PrivateKey::random(&mut rng, Algorithm::Ed25519).unwrap();
                    let fingerprint = private_key.fingerprint(HashAlg::Sha256);
                    let fingerprint_str = fingerprint.to_string();
                    if regex.is_match(&fingerprint_str.strip_prefix("SHA256:").unwrap()) {
                        found.store(true, Ordering::Relaxed);
                        return Some((private_key, fingerprint));
                    }
                    count += 1;
                    const PB_INCR: u64 = 10_000;
                    if count % PB_INCR == 0 {
                        if found.load(Ordering::Relaxed) {
                            return None;
                        }
                        pb.inc(PB_INCR);
                    }
                }
            }));
        }
        for j in joins {
            if let Some((private_key, fingerprint)) = j.join().unwrap() {
                private_key
                    .write_openssh_file(Path::new("id_ed25519"), LineEnding::LF)
                    .unwrap();
                private_key
                    .public_key()
                    .write_openssh_file(Path::new("id_ed25519.pub"))
                    .unwrap();
                println!(
                    "Found: {fingerprint}. Private key written to id_ed25519, \
                    public key written to id_ed25519.pub."
                );
                return;
            }
        }
    });
    Ok(())
}
