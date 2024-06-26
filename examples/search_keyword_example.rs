use line_history::history::History;

fn main() {
    let history = History::read_from_file("./history.txt").unwrap();
    let result = history.search_by_keyword("本");
    for elem in result {
        println!("{}", elem);
    }
}
