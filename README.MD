# Recoverable Removal

A Safe and Efficient Replacement for the Linux `rm` Command.

## Overview

RR (Recoverable Removal) is a Rust-based alternative to the traditional Linux `rm` command, designed to enhance usability and file recoverability. With RR, you can enjoy the efficiency of the classic `rm` command, now with modern, user-centric safety and performance features tailored for Linux environments.

## Key Features

1. **Optimized Parameter Handling**

    - Simplifies command options for a more efficient and user-friendly experience.

    By acctualy just move the target files to `/var/tmp`, options such as `-r` and `-f` are no longer needed.

2. **Recoverability**

    - Provides built-in recovery options, allowing easy restoration of deleted files, minimizing accidental deletions.

    `rr` restore the last removed files, and you can also check the `/var/tmp/rr_removed` directory to manually restore files.

You can also use `rr --restore` reapetedly to restore files that were removed before the last removal.

Or, you can see `/var/tmp/rr_removed` to manually restore files.

## Usage

### Remove

To remove a file or folder, simply run `rr <file/folder>`.

### Restore

To restore the last removed file, run `rr --restore`.

### Use Binary

To use RR directly, simply run the following command:

```bash
<<TODO: Auto install sh>>
```

### Build from Source

To build RR from source, you just run `cargo build` in the project directory like any other Rust project.

```bash
git clone https://github.com/kites262/Recoverable_Removal.git
cd Recoverable_Removal
cargo build
```

If you find `rr` useful, please consider giving us a star!

Feel free to raise any ideas or issues in the Issues section.
