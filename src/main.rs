use colored::Colorize;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::str::FromStr;

struct ToDo {
    finished: bool,
    content: String,
    idx: usize,
}

struct Save {
    num: usize,
    list: Vec<ToDo>,
}

impl Save {
    fn save_to_file(&self, path: &str) -> Result<(), io::Error> {
        let mut file = File::create(path)?;
        for todo in &self.list {
            writeln!(
                file,
                "{},{},{}",
                todo.idx,
                todo.finished,
                todo.content.replace("\n", "\\n")
            )?;
        }
        Ok(())
    }

    // Loads the list from a file
    fn load_from_file(path: &str) -> Result<Self, io::Error> {
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut list = Vec::new();
        let mut num = 0;

        for line in buf_reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 3 {
                continue; // Malformed line
            }
            let idx = parts[0].parse().unwrap_or(0);
            let finished = parts[1] == "true";
            let content = parts[2].replace("\\n", "\n");

            list.push(ToDo {
                finished,
                content,
                idx,
            });
            num = num.max(idx);
        }

        Ok(Self { num, list })
    }

    fn add(&mut self, s: String, file_path: &str) -> std::io::Result<()> {
        // Check if the `ToDo` with the same content already exists.
        if self.list.iter().any(|todo| todo.content == s) {
            // If the content exists, we return early without adding.
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "ToDo with this content already exists",
            ));
        }

        let new_idx = self.num;
        self.num += 1;

        let new_todo = ToDo {
            finished: false,
            content: s,
            idx: new_idx,
        };

        // Add the new ToDo to the list in memory
        self.list.push(new_todo);

        // Now, let's append this new ToDo to a file.
        let mut file = OpenOptions::new()
            .append(true) // Open the file in append mode.
            .create(true) // If the file does not exist, create it.
            .open(file_path)?; // Open the file, return the error if there is one.

        // Write the new ToDo to the file.
        writeln!(
            file,
            "{},{},{}",
            new_idx,
            false,
            self.list.last().unwrap().content
        )?;

        Ok(())
    }

    fn finish(&mut self, identifier: &str) -> Result<(), io::Error> {
        if let Ok(idx) = identifier.parse::<usize>() {
            // If `identifier` can be parsed into a number, it's an index.
            for tmp_todo in &mut self.list {
                if tmp_todo.idx == idx {
                    tmp_todo.finished = true;
                    break;
                }
            }
        } else {
            // If `identifier` cannot be parsed into a number, treat it as content.
            for tmp_todo in &mut self.list {
                if tmp_todo.content == identifier {
                    tmp_todo.finished = true;
                    break;
                }
            }
        }
        self.save_to_file("./todo_list.txt")
    }

    fn showtodo(&self) {
        let mut show: Vec<String> = Vec::new();
        for tmp_todo in &self.list {
            if !tmp_todo.finished {
                show.push(tmp_todo.idx.to_string() + ". " + &tmp_todo.content.clone());
            } else {
                show.push(
                    tmp_todo.idx.to_string()
                        + ". "
                        + &(tmp_todo.content.strikethrough().to_string()),
                );
            }
        }
        for show_string in show.iter() {
            println!("{}", show_string);
        }
    }

    fn delete(&mut self, delete_idx: usize) {
        let result = (self.list.iter()).position(|x| x.idx == delete_idx);
        match result {
            Some(index) => {
                self.list.remove(index);
                self.num -= 1;
                for todo in &mut self.list {
                    if todo.idx > delete_idx {
                        todo.idx -= 1;
                    }
                }
            }
            None => {
                panic!("The index you try to delete does not exist.");
            }
        }
        self.save_to_file("./todo_list.txt");
    }
}

fn main() -> io::Result<()> {
    let mut loaded_save = match Save::load_from_file("todo_list.txt") {
        Ok(todo) => todo,
        Err(_) => {
            let mut file = File::create("./todo_list.txt")?;
            Save {
                num: 1,
                list: Vec::new(),
            }
        }
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => {
                println!("method success!");
                loaded_save.showtodo();
            }
            "add" => loaded_save.add(args[2].clone(), "todo_list.txt")?,
            "rm" => {
                let n: usize = FromStr::from_str(&args[2]).unwrap();
                loaded_save.delete(n);
            }
            "done" => loaded_save.finish(&args[2])?,
            &_ => loaded_save.showtodo(),
        }
    } else {
        loaded_save.showtodo();
    }

    // let my_input: Vec<String> = vec![
    //     String::from("coding"),
    //     String::from("sports"),
    //     String::from("gaming"),
    // ];
    // for ss in my_input {
    //     loaded_save.add(ss, "todo_list.txt")?;
    // }
    // loaded_save.showtodo();
    //
    // // loaded_save.save_to_file("todo_list.txt")?;
    // loaded_save.finish("1")?;
    // loaded_save.showtodo();
    // println!("************");
    // begin.delete(1);
    // begin.showtodo();
    // Save the state to a file

    // loaded_save.save_to_file("todo_list.txt")?;
    // Load the state from a file
    // let mut loaded_save = Save::load_from_file("todo_list.txt")?;
    // loaded_save.showtodo();

    // You can now continue to use loaded_save as your Save object

    Ok(())
}
