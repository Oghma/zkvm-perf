#!/usr/bin/env sh
set -e

echo "Running eval script"

TARGET="programs/$4/target/delendum-unknown-baremetal-gnu/release/$4"
VALIDA_EXECUTABLE="/valida-toolchain/bin/valida"
BENCHMARKS_DIR="benchmarks"
LITA_PROOF="lita_proof"

# Get the current git commit hash
if ! COMMIT_HASH=$(git rev-parse --short HEAD 2>/dev/null); then
    echo "Error: Not a git repository or git is not installed."
    exit 1
fi

# CSV files
CSV_FILE_LATEST="$BENCHMARKS_DIR/benchmarks_latest.csv"
CSV_FILE_COMMIT="$BENCHMARKS_DIR/benchmark_${COMMIT_HASH}.csv"
# CSV header
HEADER="program,prover,hashfn,shard_size,shards,cycles,speed,execution_duration,prove_duration,core_prove_duration,core_verify_duration,core_proof_size,compress_prove_duration,compress_verify_duration,compress_proof_size"

# Ensure the benchmarks directory exists
mkdir -p "$BENCHMARKS_DIR"

# Function to create CSV file with header if it doesn't exist
create_csv_if_not_exists() {
    if [ ! -f "$1" ]; then
        echo "$HEADER" > "$1"
        echo "$HEADER" > "$2"
    fi
}

# Create CSV files if they don't exist
create_csv_if_not_exists "$CSV_FILE_COMMIT" "$CSV_FILE_LATEST"

# Measure the execution time of the 'valida prove' command
start_time_prove=$(date +%s.%N)


"$VALIDA_EXECUTABLE" prove "$TARGET" "$LITA_PROOF" $5

end_time_prove=$(date +%s.%N)

# Calculate elapsed time in seconds
elapsed_time_prove=$(echo "$end_time_prove - $start_time_prove" | bc -l)

# Get the proof size
proof_size=$(stat -c%s "$LITA_PROOF")

# Measure the executiuon time of the 'valida verify' command
start_time_verify=$(date +%s.%N)

"$VALIDA_EXECUTABLE" verify "$TARGET" "$LITA_PROOF"

end_time_verify=$(date +%s.%N)
# Calculate elapsed time in seconds with high precision
elapsed_time_verify=$(echo "$end_time_verify - $start_time_verify" | bc -l)

# Prepare the line to append to CSV files
# Fields are:
# program,prover,hashfn,shard_size,shards,cycles,speed,execution_duration,prove_duration,core_prove_duration,core_verify_duration,core_proof_size,compress_prove_duration,compress_verify_duration,compress_proof_size

# We need to fill:
# - 'program': $1
# - 'prover': $2
# - 'hashfn': $3
# - 'core_prove_duration': $elapsed_time_prove (field 10)
# - 'core_verify_duration': $elapsed_time_verify (field 11)
# - 'core_proof_size': $proof_size (field 12)
# Other fields remain empty

line="$1,$2,poseidon,,,,,,,,$elapsed_time_prove,$elapsed_time_verify,$proof_size,,,"

# Append the line to each CSV file
echo "$line" >> "$CSV_FILE_LATEST"
echo "$line" >> "$CSV_FILE_COMMIT"

echo "$2 Core Prove Duration: $elapsed_time_prove seconds"
echo "$2 Core Verify Duration: $elapsed_time_verify seconds"
echo "$2 Core Proof Size: $proof_size bytes"
