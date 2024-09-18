# LeetCode Solutions

![LeetCode Solutions Banner](https://avatars.githubusercontent.com/u/56916889?s=100&v=4)

Welcome to **LeetCode Solutions**! This repository hosts a collection of solutions to [LeetCode](https://leetcode.com/) problems, implemented in both **Rust** and **Python**. Leveraging the [LeetGo](https://github.com/j178/leetgo) application, solutions are automatically synced with your LeetCode account.

## Table of Contents

- [LeetCode Solutions](#leetcode-solutions)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Branches](#branches)
  - [Installation](#installation)
    - [Prerequisites](#prerequisites)
    - [Steps](#steps)
  - [Usage](#usage)
    - [Running Solutions Locally](#running-solutions-locally)
      - [Rust](#rust)
      - [Python](#python)
    - [Syncing with LeetCode](#syncing-with-leetcode)
  - [Contact](#contact)

## Features

- **Dual Language Support**: Solutions are available in both Rust and Python.
- **Automated Sync**: Utilizes the [LeetGo](https://github.com/j178/leetgo) app to automatically update your LeetCode account with the latest solutions.
- **Organized Structure**: Each problem is organized by difficulty and category for easy navigation.
- **Community Contributions**: Open to contributions from the community to expand the repository.

## Branches

This repository contains two primary branches:

- **`rust`**: Contains all solutions implemented in [Rust](https://www.rust-lang.org/).
- **`python`**: Contains all solutions implemented in [Python](https://www.python.org/).

Switch between branches using the following commands:

```bash
# Clone the repository
git clone https://github.com/lmenale/letgo_solutions.git
cd leetcode-solutions

# Switch to the Rust branch
git checkout rust

# Switch to the Python branch
git checkout python
```

## Installation

### Prerequisites

- [Git](https://git-scm.com/) installed on your machine.
- [LeetGo](https://github.com/j178/leetgo) app installed and configured.
- Rust and Python environments set up if you plan to run the solutions locally.

### Steps

1. **Clone the Repository**

   ```bash
   git clone https://github.com/lmenale/letgo_solutions.git
   ```

2. **Navigate to the Desired Branch**

   ```bash
   cd leetcode-solutions
   git checkout rust # or python
   ```

3. **Install LeetGo**

   Follow the [LeetGo installation guide](https://github.com/j178/leetgo#installation) to set up the app.

4. **Configure LeetGo**

   Update the configuration file with your LeetCode account details to enable automatic syncing.

## Usage

### Running Solutions Locally

#### Rust

1. Navigate to the Rust branch:

   ```bash
   git checkout rust
   ```

2. Compile and run a solution:

   ```bash
   cd solutions/TwoSum
   cargo run
   ```

#### Python

1. Navigate to the Python branch:

   ```bash
   git checkout python
   ```

2. Run a solution:

   ```bash
   cd solutions/two-sum
   python solution.py
   ```

### Syncing with LeetCode

LeetGo automatically syncs your solutions to your LeetCode account. Ensure that the app is running and properly configured.

```bash
leetgo sync
```

Refer to the [LeetGo documentation](https://github.com/j178/leetgo#usage) for advanced usage and configuration options.


## Contact

- **GitHub**: [@lmenale](https://github.com/lmenale)
- **LeetCode**: [lmenale](https://leetcode.com/lmenale)
- **Email**: lume.dev(at)outlook.com

---

<br>
Happy Coding! 🚀
