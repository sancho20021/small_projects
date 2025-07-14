#!/bin/bash

# Fail fast on errors
set -e

# Navigate to the rust directory
cd ~/Documents/sci/rust || {
    echo "Failed to cd into ~/Documents/sci/rust"
    exit 1
}

# Run the Python script
echo "Updating rust repo..."
python3 copy_repos.py --source small_projects/ rust_code

# Copy to remote cluster
echo "Copying to cluster..."
scp -r ./rust_code cluster1:/home/remote/u7688652/

echo "✅ Sync complete."
