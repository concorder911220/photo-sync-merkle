# Merkle Tree in Rust

This repository contains a Rust implementation of a **Merkle Tree**, which is a fundamental data structure used in blockchain systems, peer-to-peer networks, and other distributed systems. The implementation supports building a Merkle Tree, retrieving the root hash, and synchronizing it with other Merkle Trees.

---

## Features

- **Tree Construction**: Build a Merkle Tree from a list of string values.
- **Hashing**: Uses SHA-256 for node hashing.
- **Synchronization**: Synchronize two or more Merkle Trees to ensure consistency.
- **Utilities**: Retrieve root hash, tree height, and merge values between trees.

---

## Requirements

To build and run the project, ensure the following are installed:

- **Rust (>= 1.70)**: Install from [rustup.rs](https://rustup.rs/).
- **`sha2` crate**: For SHA-256 hashing.

---

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/concorder911220/photo-sync-merkle.git
   cd photo-sync-merkle
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the program:

   ```bash
   cargo run
   ```

---

## Usage

The program demonstrates:

1. **Building Merkle Trees**:

   - Construct three Merkle Trees using different input values.
   - Automatically computes the root hash for each tree.

2. **Synchronizing Trees**:
   - Synchronize one Merkle Tree with others to merge their differences.
   - Output the final merged array of values.

### Example Input

```rust
let elems = vec![
    "A".to_string(),
    "B".to_string(),
    "C".to_string(),
    "D".to_string(),
    "E".to_string(),
    "For".to_string(),
    "Go".to_string(),
];

let elems2 = vec![
    "A".to_string(),
    "XX".to_string(),
    "C".to_string(),
    "D".to_string(),
    "E".to_string(),
    "For2".to_string(),
    "Rust".to_string(),
];

let elems3 = vec![
    "A".to_string(),
    "B".to_string(),
    "C".to_string(),
    "D".to_string(),
    "E".to_string(),
    "For3".to_string(),
    "Solana".to_string(),
];
```

### Output

The final synchronized array after merging all trees:

```bash
Final Array: ["A", "B", "C", "D", "E", "For", "For2", "For3", "Go", "Rust", "Solana", "XX"]
```

---

## Code Overview

### Core Components

1. **`Node`**:
   Represents a node in the Merkle Tree. Each node stores:

   - `left` and `right` child nodes.
   - SHA-256 `value` of the node.
   - `content`, `height`, and `leave_count` metadata.
   - A flag to identify if the node is a padding copy.

2. **`MerkleTree`**:

   - Builds the tree using a sorted list of values.
   - Supports synchronization between trees to merge differences.
   - Provides helper methods like retrieving the root hash, leftmost node, etc.

3. **Utility Functions**:
   - `merge_sorted_arrays`: Merges two sorted arrays while removing duplicates.

---

## Development

1. Add dependencies:

   ```bash
   cargo add sha2
   ```

2. Run app (if available):

   ```bash
   cargo run
   ```

---

## How It Works

### Tree Construction

1. Each value is hashed using SHA-256.
2. Leaf nodes are created for each hashed value.
3. Internal nodes are created by combining hashes of child nodes.
4. The root node represents the overall hash of the tree.

### Synchronization

1. Trees are synchronized by finding the common prefix of values.
2. Differences between trees are merged into a new array.
3. A new tree is built using the merged array.

---

## Future Enhancements

- Add **proof of inclusion** for verifying if a value exists in the tree.
- Extend functionality to support custom hash functions.
- Optimize tree synchronization for large datasets.
