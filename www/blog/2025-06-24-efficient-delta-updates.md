---
slug: efficient-delta-updates
title: Efficient Delta Updates
authors: koehlma
draft: true
tags: [rugix,delta updates]
---

Rugix and other OTA update solutions generally promote *full system updates*, where the system is updated as a whole rather than patching individual components. This ensures that all parts can be tested together and are updated in one atomic step, significantly improving robustness and reducing the risk of bricking devices—a risk that can quickly lead to substantial financial and reputational damage.

The main drawback of full system updates is their size. They are costly in terms of bandwidth and download size. This is where *delta updates* become relevant. Delta updates retain the benefits of full system updates while reducing data transfer by reusing parts of the existing, old version.

In this article, we survey different delta update techniques, examine and discuss their tradeoffs, and present benchmarks that showcase their efficiency. We also introduce `rugix-delta`, a new Rugix tool for benchmarking delta update techniques and exploring the effects of their parameters.

<!-- truncate -->

While delta updates are often discussed, there is surprisingly little concrete evidence on how effective different techniques are in practice. Benchmarks are scarce, and comparisons are often anecdotal. With this article, we aim to shed light on the real-world efficiency of delta updates and contribute to a clearer, more practical understanding of their tradeoffs and performance.


## Delta Updates 101

The core idea behind delta updates is simple: instead of downloading a full system image, reuse what is already there and only fetch what has changed. If we look at existing tooling in the space, we find that there are two main approaches to delta updates.

### Adaptive Delta Updates

With *adaptive delta updates*, the update tool running on a device identifies parts of the existing system that can be reused, and only fetches the missing blocks, files, or directories. This approach:

- Does not require a read-only root filesystem.
- Allows updates from any version to any other version.
- Is easy to implement.

A typical implementation may divide a root filesystem image into blocks, compute a hash for each block, and then use a list of blocks (a *block index*) of the new version to reconstruct the new version from locally available blocks and remotely fetched blocks. Instead of operating on the level of a filesystem image, adaptive delta updates may also directly operate on files and directories. For instance, [*OSTree*](https://ostreedev.github.io/ostree/) implements a form of adaptive delta updates where individual files and directories are synchronized, fetching only those files and directories that are not already locally available in an object store.

Adaptive delta updates *adapt* to what is currently on the device on-demand.

### Static Delta Updates

In contrast to adaptive delta updates, *static delta updates* rely on precomputed *patches* that describe how to go from one specific version to another. This approach:

- Is generally more efficient than the adaptive approach in terms of size and compression.
- Requires a read-only root filesystem to ensure the base version is unmodified.
- Involves more complexity, including precomputing and managing version-to-version patches.

Static delta updates can utilize highly efficient [delta encoding and compression techniques](https://en.wikipedia.org/wiki/Delta_encoding).


## Tools and Techniques

For the purposes of this article, we will consider the following popular OTA update tools for embedded Linux in addition to Rugix: [RAUC](https://rauc.io/), [SWUpdate](https://sbabic.github.io/swupdate/swupdate.html), [Mender](https://mender.io/), and [OSTree](https://ostreedev.github.io/ostree/). Note that these tools, in part, overlap in the underlying techniques that they use for implementing delta updates.

### Block-Based Techniques

As already sketched above, *block-based techniques* split filesystem images into blocks and fetch individual blocks as needed based on an index. When it comes to block-based techniques, implementations vary in how they split images into blocks. Generally, there are two approaches to split images into blocks: Fixed-size blocks, usually aligned with the block size of the filesystem, or variable-size blocks. RAUC supports both fixed-size blocks (natively) and variable-size blocks through an integration with [Casync](https://github.com/systemd/casync). SWUpdate supports delta updates by integrating with external tools. It supports block-based adaptive delta updates by integrating with Casync and [ZChunk](https://github.com/zchunk/zchunk), which both can use variable-sized blocks. Rugix natively supports fixed-size blocks and variable-sized blocks. 

### Delta Compression

### File-Based Techniques


## Benchmarks


## Implementation in Rugix

Rugix offers best-in-class support for adaptive and static delta updates such that you can make the efficiency vs. complexity tradeoff based on your needs.

### Adaptive Delta Updates

### Static Delta Updates

```shell
rugix-bundler delta <old bundle> <new bundle> <patch bundle>
```


## Conclusion


