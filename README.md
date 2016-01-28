## rust-todo

a todo list in rust.

this is a example application based on scrutch (https://crates.io/crates/scrutch).

## Usage

```
Usage:
  todos new <name>
  todos list [--lf=<lfsp>] [--status=<s>]
  todos edit <index>
  todos status <index> <stat>
  todos rm <index>
  todos report [--type=<t>]
  todos (-h | --help)
  todos --version
  
  Options:
  -h --help    	Show this screen.
  --version     Show version.
  --lf=<lfsp>   define lifespan (daily, weekly, monthly, yearly).
  --type=<t>    report type (html, markdown).
```

## Build

Clone the repo and compile with cargo:
```
$ git clone https://github.com/lequer/rust-todo
$ cd rust-todo
$ cargo build --release
```

rust-todo can then be run via cargo itself or copied in a local path and run as standalone app.

