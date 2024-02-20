use std::fs;
use term_grid::{Grid, GridOptions, Direction, Filling, Cell};
trait Color{
    fn green(&self)-> String;
    fn cyan(&self)-> String;
    fn blue(&self)-> String;
    fn red(&self)-> String;
}
impl Color for String{
    fn green(&self)->String{
        format!("\x1b[32m{}\x1b[0m",self)
    }
    fn cyan(&self)->String{
        format!("\x1b[36m{}\x1b[0m",self)
    }
    fn blue(&self)->String{
        format!("\x1b[34m{}\x1b[0m",self)
    }
    fn red(&self)->String{
        format!("\x1b[31m{}\x1b[0m",self)
    }
}
fn main() {
    let mut grid = Grid::new(GridOptions {
        filling:    Filling::Spaces(4),
        direction:  Direction::LeftToRight,
    });
    for file in fs::read_dir(".").unwrap(){

        let check=file.unwrap();
        if let Some(valid_str)=check.file_name().to_str(){
            let trim=valid_str.trim_matches('"');
            if check.path().is_dir(){
                grid.add(Cell::from(format!("{} {}","\u{f07b}",trim).blue()))
            }
            else if trim[0..1]!='.'.to_string(){
                grid.add(Cell::from(format!("{} {}","\u{f07b}",trim).green()))
            }
            else{
                grid.add(Cell::from(format!("{} {}","\u{f07b}",trim).red()))
            }
        }
    }
    println!("{}", grid.fit_into_width(800).unwrap());
}
