

/*
 * CLI tool to create 'post-it' notes.
 */
enum Command {

}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}

