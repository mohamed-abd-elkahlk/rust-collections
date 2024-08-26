# Rust Collections

This repository provides explanations and examples for common collections in Rust. The goal is to help developers understand how to use Rust's built-in collections effectively.

## Table of Contents

- [Introduction](#introduction)
- [Collections Covered](#collections-covered)
- [Examples](#examples)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Rust offers a powerful set of collections that allow developers to store and manipulate data efficiently. This repository provides detailed explanations and code examples for each of the common collections in Rust.

## Collections Covered

This repository covers the following Rust collections:

- **Vec**: A growable, heap-allocated array.
- **String**: A UTF-8 encoded, growable string.
- **HashMap**: A hash map implemented with quadratic probing and SIMD lookup.
- **BTreeMap**: A map based on a B-tree.
- **HashSet**: A hash set, implemented as a `HashMap` where the value is `()`.
- **BTreeSet**: A set based on a B-tree.
- **LinkedList**: A doubly linked list.
- **BinaryHeap**: A priority queue implemented with a binary heap.

## Examples

Each collection comes with code examples that demonstrate common operations:

- Creating and initializing the collection.
- Adding, removing, and accessing elements.
- Iterating over elements.
- Common use cases and patterns.

## Documentation

The full documentation for this project is available [here](https://your-username.github.io/your-repository-name/).

## Contributing

Thank you for considering contributing to this project! Here are a few guidelines to help you get started.

### How to Contribute

1. **Fork the repository**  
   Fork the repository to your own GitHub account.

2. **Clone the repository**  
   Clone the forked repository to your local machine.

   ```bash
   git clone https://github.com/mohamed-abd-elkahlk/rust-collections.git

3. **Create a branch**
    Create a new branch for your feature or bugfix.

   ```bash
    git checkout -b feature/your-feature-name

4. **Make your changes**
    Implement your changes or additions. Ensure that your code follows the project's coding standards.

5. **Test your changes**
    If the project has tests, make sure to run them to verify your changes.

   ```bash
    cargo test

6. **Commit your changes**
    Write clear and concise commit messages.

   ```bash
   git add .
   git commit -m "Add feature X"

7. **Push to your fork**
    Write clear and concise commit messages.

   ```bash
    git push origin feature/your-feature-name

8. **Create a Pull Request**
    Open a pull request (PR) to the main repository. Provide a detailed description of your changes and the reason for them.

    - Navigate to the original repository on GitHub.
    - Click on the "Pull Requests" tab and then on the "New Pull Request" button.
    - Select the branch you pushed your changes to and submit the PR.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
