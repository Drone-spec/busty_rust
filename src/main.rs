use std::{fs::{OpenOptions}, io::{Write}};
use rocket::serde::{Deserialize, json::Json, Serialize};
use rocket::http::CookieJar;
use std::io::*;
#[macro_use] extern crate rocket;


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str
}

#[post("/addtask", data="<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");   
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("unable to write to tasks.txt");
    "Task added succesfully"
}

#[get("/readtasks")]
fn read_tasks() -> Json<Vec<String>> {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("unable to access tasks.txt");
    let reader = BufReader::new(tasks);
    Json(reader.lines()
         .map(|line| {
             let line_string: String = line.expect("could not read line");
             let line_pieces: Vec<&str> = line_string.split(",").collect();
             line_pieces[1].to_string()
         })
         .collect())
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskUpdate<'r> {
    id: u8,
    item: &'r str
}

#[put("/edittask", data="<task_update>")]
fn edit_task(task_update: Json<TaskUpdate<'_>>) -> &'static str {
    let tasks = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open("tasks.txt")
                    .expect("unable to access tasks.txt");  
    let mut temp = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open("temp.txt")
                    .expect("unable to access temp.txt");
                    
    let reader = BufReader::new(tasks);
    for line in reader.lines() {
        let line_string: String = line.expect("could not read line");
        let line_pieces: Vec<&str> = line_string.split(",").collect();
        
        if line_pieces[0].parse::<u8>().expect("unable to parse id as u8") == task_update.id {
            let task_items: [&str; 2] = [line_pieces[0], task_update.item];
            let task = format!("{}\n", task_items.join(","));
            temp.write(task.as_bytes()).expect("could not write to temp file");
        }
        else {
            let task = format!("{}\n", line_string);
            temp.write(task.as_bytes()).expect("could not write to temp file");
        }
    }
    
    std::fs::remove_file("tasks.txt").expect("unable to remove tasks.txt");
    std::fs::rename("temp.txt", "tasks.txt").expect("unable to rename temp.txt");
    "Task updated succesfully"
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
fn handler(jar: &CookieJar<'_>) {

    let cookie = jar.get("name");
    match cookie {
        Some(_) => println!("Yo you got cookie crumbs on the beat, Let me free style that for you. Uh Yeah yeah "),

        None => println!("Yo you aint got shit little homie. :( Rip to a real one. That cookie jar empty as my bank account."),

    }
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task, edit_task, read_tasks])

}


