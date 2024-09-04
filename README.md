# LibraryProgram_
# Library Smart Contract

This project implements a smart contract on the Solana blockchain for managing a library system. The smart contract handles operations such as creating users, adding books, borrowing and returning books, and managing authority configurations.

## Project Overview

The smart contract is designed to manage a decentralized library where users can borrow and return books, and administrators (authorities) can manage the inventory. Each user can borrow only one book at a time, and multiple authorities are responsible for managing the system.

### Key Components

- **Configuration**: Manages the list of authorities who can add or remove books.
- **User**: Represents a user in the system who can borrow a book.
- **Book**: Represents a book in the library, including details such as the number of copies available and those currently borrowed.
- **Counter**: Keeps track of the number of users and books.
- **BorrowRecord**: Records the borrowing details of a book by a user.

## Steps

### Prerequisites

- Rust and Cargo installed
- Solana CLI installed and configured

### Build and Deploy

1. **Clone the Repository:**

   ```bash
   git clone https://github.com/yourusername/LibrarySmartContract.git
   cd LibrarySmartContract

