fn main() {
    let r = nekosbest::get(nekosbest::Category::Neko).unwrap();
    println!("URL: {}", r.url);
}
