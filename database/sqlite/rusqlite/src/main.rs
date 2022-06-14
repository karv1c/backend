use rand::Rng;
use rusqlite::{params, Connection};
use std::io;

fn main() {
    let mut db_path = String::new();
    println!("Enter a database name:");
    io::stdin()
        .read_line(&mut db_path)
        .expect("Failed to read line");
    let connection = Connection::open(db_path.trim()).unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS facts (id INTEGER PRIMARY KEY, fact TEXT);",
            [],
        )
        .unwrap();
    println!("Openning {}...", db_path.trim());

    loop {
        println!(
            "Enter 1 to create new fact,
      2 to read all database,
      3 to read random fact,
      4 to drop table,
      other to quit"
        );
        let choice = read_input_action();
        match choice {
            Some(1) => create(&connection),
            Some(2) => read_all(&connection),
            Some(3) => read_random(&connection),
            Some(4) => drop(&connection),
            _ => break,
        }
    }
}

fn read_input_action() -> Option<u32> {
    let mut action = String::new();

    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read line");
    let choice = action.trim().parse().ok()?;
    Some(choice)
}

fn create(connection: &Connection) {
    let mut new_fact = String::new();
    println!("Enter a new fact:");
    io::stdin()
        .read_line(&mut new_fact)
        .expect("Failed to read line");
    connection
        .execute("INSERT INTO facts (fact) VALUES (?1);", params![new_fact])
        .unwrap();
}

fn read_all(connection: &Connection) {
    let facts = read(connection);
    if !facts.is_empty() {
        for fact in facts {
            print!("{}", fact);
        }
    } else {
        println!("Empty");
    }
}

fn read_random(connection: &Connection) {
    let facts = read(connection);
    if !facts.is_empty() {
        let mut rng = rand::thread_rng();
        print!("{}", facts[rng.gen_range(0..=facts.len() - 1)]);
    } else {
        println!("Empty");
    }
}

fn read(connection: &Connection) -> Vec<String> {
    let mut stmt = connection.prepare("SELECT * FROM facts").unwrap();
    let rows = stmt.query_map([], |row| row.get(1)).unwrap();
    let mut facts: Vec<String> = vec![];
    for fact_result in rows {
        facts.push(fact_result.unwrap());
    }
    facts
}

fn drop(connection: &Connection) {
    connection.execute("DELETE FROM facts;", []).unwrap();
}
