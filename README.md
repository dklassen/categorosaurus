## 🦖 Categorosaurus

Categorosaurus is a simple library for classifying text based on user-supplied partial patterns and labels. It allows you to define categories using partial string patterns and efficiently match text against them. The library is pretty lightweight at the moment patterns are limited to partial matches and the longest match wins.


### Disclaimer
This at the moment was for run to play with implementing the Aho-Corasick algorithm in Rust. It is not intended for production use. I wanted to test an idea about having to classify short text labels manually.

### 🚀 Features

- Define custom categories for partial text matches
- Lightweight because theres nothing to it
- Might work or might not work?

### 📦 Installation

To use Categorosaurus in your Rust project, add the following to your Cargo.toml:

[dependencies]
categorosaurus = { git = "https://github.com/dklassen/categorosaurus.git" }
To use a specific version (if tagged):

[dependencies]
categorosaurus = { git = "https://github.com/dklassen/categorosaurus.git", tag = "v0.1.0" }

### 📝 Usage

Here's a simple example of how to use Categorosaurus:

```rust
use categorosaurus::LabelMaker;

fn main() {
    let categories: super::PatternMap = HashMap::from([
      ("Tyrannosaur".to_string(), "Therapod".to_string()),
      ("Velociraptor".to_string(), "Therapod".to_string()),
      ("Brachiosaurus".to_string(), "Saurapod".to_string()),
      ("Patagotitan".to_string(), "Saurapod".to_string()),
    ]);

    let categorosaurus = super::LabelMaker::build(categories);

    let result = categorosaurus.categorize("Tyrannosaurus rex").unwrap();
    
    println!("Category: {:?}", result);
}
```

Output:
Category: ["Therapod"]

### 📖 API Reference

```rust
let categories: PatternMap = HashMap::from([
  ("Tyrannosaur".to_string(), "Therapod".to_string()),
  ("Velociraptor".to_string(), "Therapod".to_string()),
  ("Brachiosaurus".to_string(), "Saurapod".to_string()),
  ("Patagotitan".to_string(), "Saurapod".to_string()),
]);

LabelMaker::build(categories: PatternMap) -> LabelMaker
```
Creates a new classifier that is loaded and ready to go.

```rust
LabelMaker::categorize(text: &str) -> Option<String>
```
Returns a category, or not, for a given piece of text.

### 🔧 Development

Clone the repository and test the library locally:

git clone https://github.com/dklassen/categorosaurus.git
cd categorosaurus
cargo test

### 🏗 Roadmap

-  Give up and use a regex library

### 📜 License

This project is licensed under the MIT License.

### 🙌 Contributions

Contributions are welcome! Feel free to submit issues and pull requests.


