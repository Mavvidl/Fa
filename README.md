FaFs — Fractal File System

<p align="center">
<img src="docs/images/fafs-logo.png" alt="FaFs Logo" width="200">
</p>

<h3 align="center">A Cross-Platform Fractal-Based File System</h3>

<p align="center">
<img src="https://img.shields.io/badge/Rust-2024-orange?logo=rust" alt="Rust">
<img src="https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue" alt="Platforms">
<img src="https://img.shields.io/badge/Architecture-Modular-success" alt="Architecture">
<img src="https://img.shields.io/badge/Status-Under%20Development-yellow" alt="Status">
<img src="https://img.shields.io/badge/License-MIT-green" alt="License">
</p>

1. Overview

FaFs (Fractal File System) is an experimental cross-platform file system written in Rust, designed around fractal geometry and spatial data organization.

Its objective is to explore how mathematical structures such as Hilbert curves and Sierpinski geometries can improve:

Storage space utilization.
Block allocation efficiency.
File organization and classification.
Sequential and random read performance.
File indexing and retrieval.
Storage fragmentation management.

Unlike conventional file systems, FaFs explores the use of fractal structures as part of its storage organization and allocation strategy.

The project aims to combine fractal mathematics, efficient storage structures, persistent indexing and modern Rust engineering.

Project status: FaFs is currently under development. Its architecture and algorithms are being progressively implemented and validated.
2. Core Concepts
2.1 Hilbert Curve — Spatial Block Organization

<p align="center">
<img src="https://upload.wikimedia.org/wikipedia/commons/2/21/Hilbert_curve.gif" alt="Hilbert Curve" width="420">
</p>

The Hilbert curve is a space-filling curve that maps multidimensional coordinates into a one-dimensional sequence while preserving spatial locality to a useful degree.

FaFs explores its use for:

Mapping logical storage positions.
Grouping related data blocks.
Improving spatial locality.
Reducing unnecessary random accesses.
2.2 Sierpinski Geometry — Hierarchical Organization

<p align="center">
<img src="https://upload.wikimedia.org/wikipedia/commons/0/05/Sierpinski_triangle.svg" alt="Sierpinski Triangle" width="350">
</p>

The Sierpinski structure provides a recursive geometric model for hierarchical organization.

FaFs uses this concept as a basis for exploring:

Recursive storage regions.
Hierarchical directory organization.
Free-space subdivision.
Spatial allocation strategies.
2.3 Fractal Block Allocation

<p align="center">
<img src="docs/images/fractal-allocation.png" alt="FaFs Fractal Block Allocation" width="650">
</p>

The allocator is designed to manage storage regions using fractal-aware placement and extent-based allocation.

Its objectives include:

Minimizing fragmentation.
Reusing released blocks.
Grouping contiguous storage regions.
Reducing allocation metadata overhead.
2.4 Intelligent File Indexing

FaFs introduces a dedicated indexing layer to accelerate file retrieval.

Planned indexing structures:

Index	Responsibility
Name Index	File lookup by name
Inode Index	Lookup by inode identifier
Path Index	Full path resolution
Metadata Index	Search by size, date and type
Spatial Index	Fractal storage location mapping

Persistent B+ trees and in-memory hash indexes are being considered for efficient lookup operations.

3. Architecture

FaFs follows a modular architecture designed to separate filesystem logic, fractal algorithms and platform-specific storage operations.

<p align="center">
<img src="docs/images/fafs-architecture.png" alt="FaFs Architecture" width="800">
</p>

Architectural Layers
Layer	Responsibility
Application	CLI and demonstration interface
VFS	Files, directories, inodes and filesystem operations
Fractal Engine	Hilbert curves and Sierpinski structures
Allocator	Block allocation and extent management
Indexing	File and metadata retrieval
Cache	Block, inode and directory caching
Storage	Abstract block device interface
Backend	Platform-specific storage implementation
Design Principles
Separation of concerns.
Platform independence.
Modular Rust architecture.
Testability without physical disks.
Persistent and recoverable metadata.
Performance measured through benchmarks.
4. Project Structure
text
fafs/
│
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── tests.yml
│       └── security.yml
│
├── docs/
│   └── images/
│       ├── fafs-logo.png
│       ├── fafs-architecture.png
│       ├── fractal-allocation.png
│       └── storage-layout.png
│
├── benches/
│   ├── allocation.rs
│   ├── file_search.rs
│   ├── sequential_read.rs
│   └── random_read.rs
│
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   │
│   ├── fractal/
│   │   ├── mod.rs
│   │   ├── hilbert.rs
│   │   ├── sierpinski.rs
│   │   └── spatial_mapping.rs
│   │
│   ├── allocator/
│   │   ├── mod.rs
│   │   ├── block_alloc.rs
│   │   ├── extent_alloc.rs
│   │   ├── free_space.rs
│   │   └── fragmentation.rs
│   │
│   ├── index/
│   │   ├── mod.rs
│   │   ├── name_index.rs
│   │   ├── inode_index.rs
│   │   ├── path_index.rs
│   │   └── metadata_index.rs
│   │
│   ├── cache/
│   │   ├── mod.rs
│   │   ├── block_cache.rs
│   │   ├── inode_cache.rs
│   │   └── directory_cache.rs
│   │
│   ├── vfs/
│   │   ├── mod.rs
│   │   ├── inode.rs
│   │   ├── directory.rs
│   │   ├── file_ops.rs
│   │   └── filesystem.rs
│   │
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── block_device.rs
│   │   └── image.rs
│   │
│   ├── metadata/
│   │   ├── mod.rs
│   │   ├── superblock.rs
│   │   ├── journal.rs
│   │   └── serialization.rs
│   │
│   └── backend/
│       ├── mod.rs
│       ├── windows/
│       ├── linux/
│       └── macos/
│
├── tests/
│   ├── test_fractal.rs
│   ├── test_allocator.rs
│   ├── test_index.rs
│   ├── test_cache.rs
│   ├── test_vfs.rs
│   └── test_persistence.rs
│
├── Cargo.toml
├── Cargo.lock
└── README.md


Note: This tree represents the planned architecture. Some modules are still under development.

5. Storage Optimization Strategy

FaFs explores multiple complementary optimization techniques.

5.1 Extent-Based Allocation

Instead of allocating individual blocks independently, FaFs uses the concept of extents: contiguous ranges of storage blocks.

<p align="center">
<img src="docs/images/storage-layout.png" alt="Storage Layout" width="700">
</p>

Benefits:

Reduced allocation metadata.
Improved sequential access.
Better contiguous storage management.
More efficient block lookup.
5.2 Adaptive Block Management

The initial design uses a 4 KiB logical block size, with extent-based allocation for larger files.

The allocation strategy will be evaluated using benchmarks before being finalized.

5.3 Multi-Level Caching

FaFs plans three main cache layers:

Cache	Purpose
Block Cache	Frequently accessed data blocks
Inode Cache	Frequently used file metadata
Directory Cache	Frequently accessed directory entries

Cache replacement strategies such as LRU and 2Q will be evaluated.

6. File Retrieval and Indexing

FaFs aims to support efficient file lookup through dedicated indexes.

Expected operations include:

text
find_by_name("document.txt")
find_by_inode(1024)
find_by_path("/documents/report.pdf")
find_by_metadata(size, date, type)


The indexing architecture is designed to avoid repeatedly traversing the entire directory hierarchy.

7. Cross-Platform Support

FaFs is designed to support multiple operating systems.

Platform	Planned Backend	Status
Windows	Windows storage APIs	Planned
Linux	Linux block/storage interfaces	Planned
macOS	macOS storage interfaces	Planned
Memory	In-memory block device	Initial development

The core filesystem logic is designed to remain independent of platform-specific APIs.

8. Continuous Integration and Testing

FaFs uses GitHub Actions to automate code validation and testing.

<p align="center">
<img src="https://img.shields.io/badge/CI-GitHub%20Actions-blue?logo=githubactions" alt="GitHub Actions">
<img src="https://img.shields.io/badge/Tests-Rust-orange?logo=rust" alt="Rust Tests">
<img src="https://img.shields.io/badge/Quality-Clippy-success" alt="Clippy">
</p>

Automated Checks
Rust compilation.
Rustfmt formatting validation.
Clippy static analysis.
Unit tests.
Integration tests.
Cross-platform builds.
Dependency security auditing.
Code coverage.
Functional Testing

The test suite aims to validate:

Test	Objective
Hilbert	Encoding and decoding
Sierpinski	Recursive structure correctness
Allocator	Allocation and release
Storage	Block read/write
Inodes	Metadata management
Directory	Entry management
VFS	File operations
Persistence	Recovery after remount
Integrity	Detection of data corruption
Running Tests Locally
bash
cargo check
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo build --release

9. Benchmarking

Performance is an important objective of FaFs.

The benchmark suite will evaluate:

Sequential read throughput.
Random read latency.
File lookup performance.
Block allocation throughput.
Storage fragmentation.
Metadata overhead.
Cache hit ratio.

Comparisons will be performed between fractal-aware allocation and conventional allocation strategies.

No performance improvements will be claimed without reproducible benchmark results.

10. Development Roadmap
Initial project architecture
Core Rust implementation
Hilbert encoding and decoding
Sierpinski hierarchy
Fractal block allocator
Extent-based allocation
Persistent indexing
Multi-level caching
Virtual File System
Filesystem formatting and mounting
Persistent metadata
Journaling and recovery
Windows backend
Linux backend
macOS backend
Performance benchmarks
Filesystem integrity testing
Production-readiness evaluation
11. Technology Stack
Technology	Purpose
Rust	Core implementation
Cargo	Build and dependency management
GitHub Actions	Continuous integration
Criterion.rs	Benchmarking
Proptest	Property-based testing
Thiserror	Error handling
LLVM Coverage	Code coverage
12. Building FaFs

Clone the repository:

bash
git clone https://github.com/YOUR_USERNAME/fafs.git
cd fafs


Build:

bash
cargo build


Run tests:

bash
cargo test


Run the demonstration:

bash
cargo run


Build an optimized binary:

bash
cargo build --release

13. Project Vision

FaFs explores a different approach to filesystem design by combining:

Fractal geometry.
Spatial data organization.
Efficient block allocation.
Persistent indexing.
Intelligent caching.
Cross-platform storage abstraction.

Its long-term objective is to investigate whether fractal-based storage organization can provide measurable advantages over conventional filesystem allocation and indexing approaches.

14. Contributing

Contributions, ideas, bug reports and performance experiments are welcome.

Before submitting a pull request:

Format the code with cargo fmt.
Run cargo clippy.
Run the complete test suite.
Include tests for new functionality.
Document architectural changes.
15. License

FaFs is distributed under the MIT License.

<p align="center">
<strong>FaFs — Rethinking File Systems Through Fractal Geometry.</strong>
</p>

<p align="center">
Built with Rust.
</p>
