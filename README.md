[![Rust](https://github.com/gilflorida2023/sieve/actions/workflows/rust.yml/badge.svg)](https://github.com/gilflorida2023/sieve/actions/workflows/rust.yml)
# sieve

## Overview
Sieve is a memory-efficient implementation of the Sieve of Eratosthenes algorithm for finding prime numbers. Unlike traditional implementations that require memory proportional to the upper limit, this program uses a sliding window approach to find primes up to large numbers while maintaining a constant memory footprint.

## How It Works

### Core Algorithm
1. **Sliding Window Approach**
   - Instead of allocating memory for the entire range (0 to upper_limit), the program works with fixed-size windows
   - Default window size is 100,000 numbers
   - Each window represents a consecutive range of numbers in the full sequence

2. **Two-Phase Processing**
   - **Phase 1 - Marking Composites**: For each known prime, mark its multiples as composite within the current window
   - **Phase 2 - Prime Discovery**: Scan the window to find new primes and store them for future windows

### Data Structures
1. **Binary File (primes.bin)**
   - Persistent storage for discovered primes
   - Each record contains:
     - `p`: The prime number
     - `nextval`: Next multiple of this prime to be marked as composite

2. **Window Buffer**
   - Boolean array representing primality of numbers in current window
   - Size is fixed regardless of the upper limit
   - Reused for each window segment

### Implementation Details

#### Window Processing
```
For each window:
1. Clear the is_prime[] buffer (set all to true)
2. Read each prime (p) from primes.bin
3. Mark composites: For each prime p
   - Start from p.nextval
   - Mark all its multiples as composite within current window
   - Update p.nextval for next window
4. Discover new primes in current window
   - For each unmarked number n
   - Confirm n is prime
   - Add n to primes.bin
   - Mark multiples of n as composite
5. Move to next window
```

#### File Handling
- Uses binary file for efficient storage and retrieval
- Maintains prime numbers and their next composite values
- Converts binary data to CSV format after completion

## Features

### Command Line Options
- `-w, --window_size <size>`: Set window size (default: 100,000)
- `-u, --upper_limit <limit>`: Set upper limit (default: 1,000,000)
- `-v, --verbose`: Enable verbose output
- `-f, --fast`: Disable processing delays
- `-h, --help`: Display help message

### Output Files
1. **primes.bin**: Binary file storing prime numbers and their next composite values
2. **primes.csv**: Human-readable CSV format of discovered primes

## Performance
- Memory Usage: O(window_size) regardless of upper_limit
- Time Complexity: O(n log log n) where n is upper_limit
- For default parameters (upper_limit = 1,000,000):
  - Finds 78,498 prime numbers
  - Uses constant memory based on window size

## Implementation Notes
- Uses u64 integers for number representation
- Implements efficient file I/O with buffering
- Includes optional processing delays for monitoring long runs
- Error handling for file operations and memory allocation

## execution
- cargo run -- -f -v
