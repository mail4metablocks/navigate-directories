use std::env;
use std::path::{Path, PathBuf};
use std::fs;

struct DirectoryNavigator {
    current_dir: String,
    previous_dir: Option<String>,
}

impl DirectoryNavigator {
    fn new() -> Self {
        let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
        Self { current_dir, previous_dir: None }
    }

    fn go_to_dir(&mut self, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(&self.current_dir).join(dir);
        if !path.exists() {
            return Err(format!("{}: No such file or directory", path.display()).into());
        }
        if !path.is_dir() {
            return Err(format!("{}: Not a directory", path.display()).into());
        }
        self.previous_dir = Some(self.current_dir.clone());
        self.current_dir = path.to_string_lossy().to_string();
        Ok(())
    }

    fn go_to_parent_dir(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(prev_dir) = self.previous_dir.take() {
            self.current_dir = prev_dir;
            Ok(())
        } else {
            Err("Already at root directory".into())
        }
    }

    fn display_directory_structure(&self) {
        let path = Path::new(&self.current_dir);
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let name = path.file_name().unwrap().to_string_lossy();
            if path.is_dir() {
                println!("{}/", name);
            } else {
                println!("{}", name);
            }
        }
    }
    
    use prettytable::{Table, Cell, Row};

fn display_directory_structure(path: &Path) -> Table {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Name"), Cell::new("Type")]));
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        let row = if path.is_dir() {
            Row::new(vec![Cell::new(&name), Cell::new("Directory")])
        } else {
            Row::new(vec![Cell::new(&name), Cell::new("File")])
        };
        table.add_row(row);
    }
    table
}

   
   fn display_directory_structure(&self) {
    let path = Path::new(&self.current_dir);
    let table = display_directory_structure(path);
    table.printstd();
}


}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nav = DirectoryNavigator::new();
    nav.go_to_dir("src")?;
    nav.display_directory_structure();
    nav.go_to_parent_dir()?;
    nav.display_directory_structure();
    Ok(())
}
