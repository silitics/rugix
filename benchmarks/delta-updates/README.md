# Efficient Delta Updates for Embedded Linux

**The following is work in progress. Please take with a grain of salt.**

**Abstract:** Naive full system updates in embedded devices require downloading an entire system image, resulting in large data transfers—even for minor changes. This poses a significant challenge in bandwidth-constrained or metered environments, where data usage directly impacts cost and feasibility. Delta update techniques address this challenge by reusing existing data on the device to avoid the transmission of unchanged data. In this article, we explore the landscape of delta update methods, evaluate their effectiveness in realistic scenarios, and present a novel approach for adaptive delta updates that outperforms existing adaptive solutions in our benchmarks.

## Scope and Background

This article focuses on *full system updates*, updates that, at least conceptually, replace the entire root filesystem or system image rather than updating individual packages or components. This approach is common in embedded systems due to its robustness. A key advantage of full system updates is that the entire software stack can be tested and validated as a whole, ensuring compatibility across components. It also prevents devices from drifting over time by accumulating slightly different combinations of software, which can happen with incremental or package-based updates, and can lead to issues in production that are hard to reproduce, debug, and fix. Therefore, full system updates should generally be preferred.

In contrast to incremental updates, a naive implementation of full system updates has the disadvantage that the entire system needs to be downloaded to the device every time an update is performed. This poses a significant challenge in bandwidth-constrained or metered environments, where data usage directly impacts cost and feasibility. To reduce bandwidth requirements, full system updates can be optimized using *delta update* techniques, which aim to avoid transmitting data that the device already has. These techniques either rely on precomputed differences between known system versions or adaptively identify reusable data based on the device’s current state.

### Static vs. Adaptive

We distinguish two main classes of delta update techniques:

- **Static delta update techniques** rely on precomputed deltas for specific pairs of system versions. For each such pair, the update server generates a compact representation of the differences between both versions, which can then be efficiently applied on the device.

- **Adaptive delta update techniques** identify on-the-fly which parts of an update are already locally available and which need to be downloaded. This process happens on the device using some sort of *precomputed index* which is part of the update and describes the new version.

Static delta updates require storing deltas for all relevant version pairs and assume that the device's exact state is known in advance. As a result, they often require special server support and are only feasible when the root filesystem has not been modified in the field. In contrast, adaptive delta updates are feasible even if the root filesystem has been modified in the field. They also do not require special support from the server, beyond the ability to fetch individual parts of the update at a defined level of granularity. In practice, this is usually achieved using HTTP range queries which are supported by almost all HTTP servers.

### Blob-Based vs. File-Based

Delta update techniques can further be categorized into *blob-based* and *file-based* techniques, based on the level of abstraction at which they operate. This classification is orthogonal to whether a technique is static or adaptive—both static and adaptive techniques can be implemented at either level.

- **Blob-based techniques** operate on raw binary data—typically full disk or filesystem images—without interpreting their internal structure. These techniques treat the update as a flat, opaque sequence of bytes.

- **File-based techniques** operate on the structured representation of a filesystem. They compare individual files, directories, and associated metadata to identify reusable data.

Blob-based techniques are sensitive to incidental changes in the layout of the filesystem. Because files and metadata are interleaved and scattered across the filesystem image, even minor modifications—such as updated timestamps or filesystem-level reordering—can shift unrelated data and lead to poor delta efficiency, even when file contents remain unchanged. These effects can be mitigated to some degree through reproducible filesystem builds that minimize such incidental differences. As file-based techniques operate on files and directories directly, instead of the raw filesystem image, they are not sensitive to such incidental changes in the layout of the filesystem. However, they face similar challenges when it comes to the contents of individual files, which are typically treated as flat, opaque byte sequences as well.

## Tools and Techniques

Of course, delta updates are nothing new and most commonly used solutions for system updates support some variant of them. We will now have a look at several of those existing update solutions as well as at the underlying techniques they use for delta updates. For this article, we are mostly interested in the underlying techniques themselves and their properties rather than their concrete implementations in any given solution.

### Update Solutions

For the purposes of this article, we consider the following commonly used update solutions for embedded Linux: *SWUpdate*, *RAUC*, *Mender*, *Rugix*, and *OSTree*. These solutions differ in their supported update strategies and delta capabilities:

- SWUpdate can be flexibly configured through *update handlers*, enabling both static and adaptive delta updates by integrating with external tools. For example, it supports static delta updates via Xdelta and adaptive delta updates via Casync.
- RAUC provides built-in support for adaptive delta updates, using either Casync or a custom block index mechanism.
- Rugix includes native support for adaptive delta updates based on a custom block index mechanism taking inspiration from Casync.
- Mender, as part of its commercial offering, supports static delta updates based on Xdelta.

Note that all these solutions can be extended through custom update handlers or modules and thus may be configured to use other external delta update tools in one way or another.

All of the aforementioned solutions, SWUpdate, RAUC, Mender, and Rugix, primarily use blob-based delta update techniques, operating on raw filesystem images instead of individual files. Casync and Xdelta are standalone tools commonly used in this context to enable adaptive and static delta updates, respectively. We will return to these tools in more detail soon.

OSTree, in contrast to the other solutions, provides a file-based framework for managing versioned operating system trees. It combines concepts from Git with content-addressed storage and supports both static and adaptive delta updates at the file level. OSTree maintains a local object store of deduplicated file content and computes deltas based on structural changes to files and directories.

### Blob-Based Techniques

As we have seen, most tools in this space rely on blob-based techniques. While implementations may vary in detail, the underlying methods are conceptually similar and can be grouped into two main categories:

- **Block-based deltas**, used for adaptive delta updates, where the device identifies reusable data at a block or chunk level based on its local state.
- **Delta compression**, used for static delta updates, where the server computes and compresses the differences between a known source and target image.

#### Block-Based Deltas

For block-based deltas, a filesystem image or other binary blob is first cut into blocks. Those blocks can be of fixed size, e.g., `4 KB`, or of variable size using content-defined chunking. For each of the blocks, a hash is computed and stored within a block index. The block index is then sent to the device as part of an update. The device uses the same block cutting technique to compute blocks that it already has locally available. It then uses the block index of the update to determine which block needs to be downloaded and which are already locally available.

This fundamental technique is used by Casync as well as RAUC’s and Rugix’s built-in block index mechanisms. Casync uses content-defined chunking, which has the advantage that it works even when bytes are inserted or deleted. RAUC’s built-in block index mechanism uses fixed block sizes of `4 KB`, aligning with the low-level block sizes of typical filesystems. Rugix supports both, content-defined chunking and fixed block sizes. For content-defined chunking it implements the same algorithm as Casync.

The size of blocks presents an inherent trade-off between granularity and efficiency. Smaller blocks improve the chance of reusing data, but they also increase the size of the index and the number of requests needed. In a typical HTTP-based implementation, each block may require a separate HTTP request, incurring a certain fixed overhead per block downloaded. Furthermore, blocks can be compressed only individually (such that they can be decompressed individually by the client). A larger block size typically leads to a better compression ratio for individual blocks.

#### Delta Compression

Delta compression techniques, typically used for static delta updates, aim to compute and encode the exact binary differences between two known versions of a system image. A classic example is Xdelta, which generates a compact delta file that can transform a known source image into a desired target image.

The main advantage of this approach is efficiency: the delta is tightly compressed and tailored to the specific source and target versions, often resulting in very small update payloads. Since the transformation is deterministic and precomputed, the update can be applied quickly and reliably on the device, assuming it is in the expected state.

## Benchmarks