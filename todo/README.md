# Todo CLI

A simple command-line todo list application written in Rust. The application stores todos in a JSON file and persists them between runs. It uses serde and serde_json for serialization.

## Usage

```bash
todo list              # Show all todos
todo add <text>        # Add a new todo
todo remove <number>   # Remove a todo by its number
todo do <number>       # Mark a todo as done
```
