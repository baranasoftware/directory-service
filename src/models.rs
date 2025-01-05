use chrono::{NaiveDateTime};

pub struct Worker {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    pub supervisor: String,
    pub team: String,
    pub coworkers: Vec<String>,
    pub department: String,
    pub email: String,
    pub skills: Vec<String>,
    pub notifications: Vec<String>,
    pub contacts: Vec<String>,
    pub connections: Vec<String>,
    pub projects: Vec<Project>,
}

pub struct Project {
    pub id: i32,
    pub name: String,
    pub started_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

pub struct Unit {
    pub id: i32,
    pub name: String,
    pub workers: Vec<Box<Worker>>,
    pub parent: Box<Unit>,
    pub children: Vec<Box<Unit>>,
}

pub struct Organization {
    pub id: i32,
    pub name:String,
    pub children:Vec<Unit>
}
