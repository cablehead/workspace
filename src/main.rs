use liquid;
use serde_json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let parser = liquid::ParserBuilder::with_stdlib()
        .build()
        .expect("should succeed without partials");
    let template = parser.parse_file(filename).unwrap();
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let data: liquid::Object = serde_json::from_reader(stdin).unwrap();
    let output = template.render(&data).unwrap();
    println!("{}", output);
}
