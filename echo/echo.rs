fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 0 || args.len() > 2 {
        println!("Usage is ./echo \"Soome text\"");
        return;
    }
    println!("{}", args[1]);
}
