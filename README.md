# dirflat

A fast, reliable, and single-purpose command-line (CLI) and graphical (GTK4) application written in Rust designed to flatten deep, chaotic directory trees into neatly categorized groups.

[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/) [English](README.md) | [Español](README.es.md)
<div id="english">

## Overview

`dirflat` takes a directory with a massive, nested tree of folders, extracts all files, and reorganizes them into distinct, structured categories at a specified destination. It follows the Single Responsibility Principle, SRP.

### Key Categories
- **Images:** Photos, graphics, and vector files.
- **Videos:** Clip recordings, movies, and animations.
- **Documents:** PDFs, text files, spreadsheets, and presentations.
- **Audio/Music:** Sound tracks, voice notes, and audio files.
- **Archives:** Compressed files and packages.
- **Others:** Any file that doesn't fit the main categories.

---

## Technical Roadmap & Blueprint

This is the planned pipeline execution flow for `dirflat`. Currently, the project is under active development.

### 1. Pre-Execution Validation
- [x] **Path Verification:** Check and validate that both source and destination paths exist.
- [x] **Same Partition Check:** Ensure both source and destination reside on the same physical disk drive to allow instant or efficient file operations.
- [ ] **Storage Capacity Check:** Compute total source size and verify if the destination drive has enough free bytes available before starting copy/move processes.

### 2. Scanning & Mapping
- [ ] **Recursive Tree Traversal:** Enumerate every file within the deeply nested directory structures recursively.
- [ ] **Content Classification:** Run an internal classification algorithm to separate files into structured maps (Archives, Images, Videos, Documents, Audio).
- [ ] **Statistical Generation:** Build an execution summary showcasing counts and total sizes per category.

### 3. Collision Resolution (De-duplication)
- [ ] **Filename Collision Check:** Detect matching names in the flattened map.
- [ ] **Size Verification:** If names match, compare file sizes in bytes.
- [ ] **Cryptographic Hashing:** If sizes match, generate cryptographic hashes (e.g., SHA-256) of both files.
  - *If hashes match:* Keep only 1 instance and safely remove the duplicate.
  - *If hashes differ:* Retain both files, preserving the original path via a `rename` metadata attribute to safely assign a unique name.

### 4. Reorganization Phase
- [ ] **Directory Creation:** Generate the root classification folders at the destination target.
- [ ] **File Relocation:** Efficiently move files into their respective folders utilizing the pre-calculated mapped routes.
- [ ] **Verification Sweep:** Post-move analysis to re-scan the source tree and identify any files left behind or skipped due to system locking.
- [ ] **Catch-all Fallback:** Move remaining unclassified data into the `others/` folder.

### 5. Cleanup & State Preservation
- [ ] **Source Tree Pruning:** Verify if the original directory tree size equals 0 bytes; if fully empty, purge the directory tree skeleton.
- [ ] **Idempotent Snapshot & Restore:** Save the internal mapped tree history to a file, allowing users to fully rollback and restore the original deeply-nested directory structure if needed.

---

## Installation & Building

### Prerequisites
Ensure you have the Rust toolchain and GTK4 development libraries installed on your system.

</div>
```bash
# Ubuntu / Debian / Mint dependencies
sudo apt install libgtk-4-dev build-essential pkg-config