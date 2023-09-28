# TextCount

## Introduction

This command-line tool written in Rust helps you count the number of words and characters (excluding spaces) in text files (.txt). Whether you're working on essays, reports, or any textual content, this script simplifies the task of text analysis.

## How to Use

### Prerequisites

Make sure you have Rust installed on your system. You can download it from [the official Rust website](https://www.rust-lang.org/).

### Installation

1. Clone this repository to your local machine or download the script directly.

   ```bash
   git clone https://github.com/yLukas077/TextCount

   Navigate to the project directory.

### Running the Script

2. To count words and characters in a text file, use the following command:

    ```bash
    cargo run -- -i <input_file.txt>

    Replace <input_file.txt> with the path to your text file.

Let's say you have a text file named sample.txt in the same directory as the script. You can count the words and characters by running:

    cargo run -- -i sample.txt


The script will display the word count and the number of characters (excluding spaces) in the specified text file.

#### If you have any suggestions or encounter issues, feel free to open an issue or contribute to this project. Your feedback is valuable!
