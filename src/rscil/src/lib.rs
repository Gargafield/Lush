mod metadata;

pub use metadata::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let image = PeParser::open("tests/HelloWorld.exe")
            .and_then(|mut parser| parser.read())
            .unwrap();

        let assembly = image.get_assembly().unwrap();
        
        dbg!(image.get_string(assembly.name));
    }
}
