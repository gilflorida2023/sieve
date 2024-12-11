use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;
use clap::Parser;

const PRIMES_CSV: &str = "primes.csv";
const PRIMES_BIN: &str = "primes.bin";

// Default values for command line options
const DEFAULT_WINDOW_SIZE: u32 = 100_000;
const DEFAULT_UPPER_LIMIT: u64 = 1_000_000;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'w', long, default_value_t = DEFAULT_WINDOW_SIZE)]
    window_size: u32,

    #[arg(short = 'u', long, default_value_t = DEFAULT_UPPER_LIMIT)]
    upper_limit: u64,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    fast: bool,
}

#[derive(Clone, Copy)]
struct Prime {
    p: u64,
    nextval: u64,
}

impl Prime {
    const SIZE: usize = std::mem::size_of::<Prime>();
}

fn prime_open(filename: &str) -> io::Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
}

fn prime_read(file: &mut File, prime: &mut Prime) -> io::Result<bool> {
    let mut buf = [0u8; std::mem::size_of::<Prime>()];
    match file.read_exact(&mut buf) {
        Ok(_) => {
            let p = u64::from_ne_bytes(buf[0..8].try_into().unwrap());
            let nextval = u64::from_ne_bytes(buf[8..16].try_into().unwrap());
            *prime = Prime { p, nextval };
            Ok(true)
        }
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(false),
        Err(e) => Err(e),
    }
}

fn prime_write(file: &mut File, prime: &Prime) -> io::Result<()> {
    let mut buf = Vec::with_capacity(Prime::SIZE);
    buf.extend_from_slice(&prime.p.to_ne_bytes());
    buf.extend_from_slice(&prime.nextval.to_ne_bytes());
    file.write_all(&buf)
}

fn prime_unread(file: &mut File) -> io::Result<()> {
    file.seek(SeekFrom::Current(-(Prime::SIZE as i64)))?;
    Ok(())
}

fn prime_bin2csv(input_name: &str, output_name: &str, fast: bool) -> io::Result<usize> {
    let mut input = prime_open(input_name)?;
    let output = File::create(output_name)?;
    let mut writer = BufWriter::new(output);
    let mut count = 0;
    let mut prime = Prime { p: 0, nextval: 0 };

    while prime_read(&mut input, &mut prime)? {
        count += 1;
        writeln!(writer, "{},{}", prime.p, prime.nextval)?;
        if !fast && count % 10_000 == 0 {
            thread::sleep(Duration::from_millis(250));
        }
    }
    writer.flush()?;
    Ok(count)
}

fn sieve(buffer_size: u32, upper_limit: u64, fast: bool, verbose: bool) -> io::Result<()> {
    let mut current_window: u64 = 0;
    let mut is_prime = vec![true; buffer_size as usize];
    let mut fp = prime_open(PRIMES_BIN)?;
    
    while current_window < upper_limit {
        if verbose {
            eprintln!("current_window: {}", current_window);
        }
        
        is_prime.fill(true);
        let mut cp = Prime { p: 0, nextval: 0 };
        
        // Mark composites from known primes
        while prime_read(&mut fp, &mut cp)? {
            let mut entered_loop = false;
            
            while cp.nextval < current_window + u64::from(buffer_size) {
                entered_loop = true;
                let val = (cp.nextval - current_window) as usize;
                if val < buffer_size as usize {
                    if !fast && cp.nextval % 1_000_000 == 0 {
                        thread::sleep(Duration::from_millis(150));
                    }
                    is_prime[val] = false;
                }
                cp.nextval += cp.p;
            }
            
            if entered_loop {
                prime_unread(&mut fp)?;
                prime_write(&mut fp, &cp)?;
            }
        }
        
        // Discover new primes
        let start_p = if current_window == 0 { 2 } else { current_window };
        for potential_prime in start_p..current_window + u64::from(buffer_size) {
            let val = (potential_prime - current_window) as usize;
            if val < buffer_size as usize && is_prime[val] {
                cp.p = potential_prime;
                cp.nextval = cp.p + cp.p;
                
                // Mark multiples as not prime
                while cp.nextval < current_window + u64::from(buffer_size) {
                    let val = (cp.nextval - current_window) as usize;
                    if val < buffer_size as usize {
                        if !fast && cp.nextval % 100_000 == 0 {
                            thread::sleep(Duration::from_millis(150));
                        }
                        is_prime[val] = false;
                    }
                    cp.nextval += cp.p;
                }
                prime_write(&mut fp, &cp)?;
            }
        }
        
        fp.seek(SeekFrom::Start(0))?;
        current_window += u64::from(buffer_size);
    }
    
    fp.flush()?;
    Ok(())
}

fn files_remove() -> io::Result<()> {
    for file in &[PRIMES_BIN, PRIMES_CSV] {
        if Path::new(file).exists() {
            std::fs::remove_file(file)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    println!("created primes.csv and primes.bin");
    println!("Window size: {}", args.window_size);
    println!("Upper limit: {}", args.upper_limit);

    files_remove()?;
    sieve(args.window_size, args.upper_limit, args.fast, args.verbose)?;
    let count = prime_bin2csv(PRIMES_BIN, PRIMES_CSV, args.fast)?;
    println!("Found {} primes", count);

    Ok(())
}
