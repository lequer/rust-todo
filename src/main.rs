use docopt::Docopt;
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};

#[macro_use]
mod macros;

const TODO_FILE: &str = "/home/michel/.todos";

const USAGE: &str = "
todos.

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
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_type: String,
    flag_lf: String,
    arg_name: String,
    arg_index: usize,
    arg_stat: String,
    cmd_new: bool,
    cmd_rm: bool,
    cmd_edit: bool,
    cmd_list: bool,
    cmd_report: bool,
    cmd_status: bool,
}
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
enum Status {
    Possible,
    Open,
    Achieved,
    Obsolete,
}
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum Lifespan {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Life,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todos {
    todos: Vec<Todo>,
}
impl Todos {
    fn new() -> Self {
        Self { todos: vec![] }
    }
    fn load(&mut self) {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(TODO_FILE)
            .unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        if data.is_empty() {
            data = String::from("[]");
        }
        self.todos = serde_json::from_str(&data).unwrap();
    }
    fn get_all(&self) -> &Vec<Todo> {
        &self.todos
    }
    //fn get(&self, index: usize) -> &todo {
    //	&self.todos[index]
    //}
    fn get_by_lifespan(&mut self, lf: Lifespan) -> Vec<Todo> {
        let mut v = self.todos.clone();
        v.retain(|o| o.lifespan == lf);
        //  let mut v = &self.todos.into_iter().filter(|&o| o.lifespan == lf).collect::<Vec<todo>>();
        v
    }
    fn add(&mut self, todo: Todo) {
        &self.todos.push(todo);
    }
    fn set_status(&mut self, index: usize, status: Status) {
        &self.todos[index - 1].status.set(status);
        println!("new status {:?}", &self.todos[index - 1].status);
    }
    fn rm(&mut self, index: usize) {
        let mut i = index;
        if &index >= &self.todos.len() {
            i = *&self.todos.len()
        }
        &self.todos.remove(i - 1);
    }
    fn save(&mut self) -> Result<(), io::Error> {
        let encoded = serde_json::to_string(&self.todos).unwrap();
        let mut file = File::create(TODO_FILE)?;
        file.write_all(encoded.as_bytes())?;
        Ok(())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    title: String,
    description: String,
    status: Cell<Status>,
    lifespan: Lifespan,
}

impl Todo {
    pub fn new(title: String, description: String, lifespan: Lifespan) -> Self {
        Self {
            title: title,
            description: description,
            lifespan: lifespan,
            status: Cell::new(Status::Possible),
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    //                          println!("{:?}", args);
    let mut todos: Todos = Todos::new();
    let mut t = term::stdout().unwrap();
    todos.load();

    if args.cmd_list {
        match args.flag_lf.as_ref() {
            "daily" => list_todos(&mut t, todos.get_by_lifespan(Lifespan::Daily)),
            "weekly" => list_todos(&mut t, todos.get_by_lifespan(Lifespan::Weekly)),
            "monthly" => list_todos(&mut t, todos.get_by_lifespan(Lifespan::Monthly)),
            "yearly" => list_todos(&mut t, todos.get_by_lifespan(Lifespan::Yearly)),
            "life" => list_todos(&mut t, todos.get_by_lifespan(Lifespan::Life)),
            _ => list_todos(&mut t, todos.get_all().clone()),
        }
    } else if args.cmd_rm {
        println!("{}", args.arg_index);
        todos.rm(args.arg_index);
    } else if args.cmd_status {
        match args.arg_stat.as_ref() {
            "possible" => todos.set_status(args.arg_index, Status::Possible),
            "open" => todos.set_status(args.arg_index, Status::Open),
            "achieved" => todos.set_status(args.arg_index, Status::Achieved),
            "obsolete" => todos.set_status(args.arg_index, Status::Obsolete),
            _ => println!("Something went wront with status {}", args.arg_stat),
        }
        println!(
            "Status for {} modified to: {:?}",
            args.arg_index, args.arg_stat
        );
    } else if args.cmd_edit {
    } else if args.cmd_report {
        let mut i: usize = 1;
        if args.flag_type == "html" {
            println!("<ul>");
        }
        for o in todos.get_all() {
            match args.flag_type.as_ref() {
                "html" => show_todo_as_html(&o, &i),
                _ => show_todo_as_markdown(&o, &i),
            }
            i += 1;
        }
        if args.flag_type == "html" {
            println!("</ul>");
        }
    } else if args.cmd_new {
        println!("Creating new todo: {}", args.arg_name);
        println!("Enter description: ");

        let mut description = String::new();
        io::stdin()
            .read_line(&mut description)
            .ok()
            .expect("failed to read line");

        println!("Enter lifespan(daily): ");
        let mut lifespan = String::new();
        io::stdin()
            .read_line(&mut lifespan)
            .ok()
            .expect("failed to read lifespan");
        let lifespan: Lifespan = match lifespan.trim().as_ref() {
            "daily" => Lifespan::Daily,
            "weekly" => Lifespan::Weekly,
            "monthly" => Lifespan::Monthly,
            "yearly" => Lifespan::Yearly,
            "life" => Lifespan::Life,
            _ => Lifespan::Daily,
        };
        let todo: Todo = Todo::new(args.arg_name, description, lifespan);
        todos.add(todo);
    }
    todos.save().unwrap();
}
fn list_todos(t: &mut Box<term::StdoutTerminal>, todos: Vec<Todo>) {
    let mut i = 1;
    for o in todos {
        show_todo(t, &o, &i);
        i += 1;
    }
}
fn show_todo(t: &mut Box<term::StdoutTerminal>, todo: &Todo, index: &usize) {
    p_green!(t, "{}- {}", index, todo.title);
    p_red!(t, "({:?}, {:?})\n", todo.lifespan, todo.status.get());
    p_white!(t, "\t{}\n", &todo.description.clone());
}
fn show_todo_as_html(todo: &Todo, index: &usize) {
    println!("<li><h2>{}- {}</h2>", index, todo.title);
    println!("({:?})<br/>", todo.lifespan);
    println!("\t{}</li>", &todo.description.clone());
}
fn show_todo_as_markdown(todo: &Todo, index: &usize) {
    println!("##{}- {}", index, todo.title);
    println!("({:?})", todo.lifespan);
    println!("* {}", &todo.description.clone());
}
