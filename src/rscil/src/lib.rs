mod metadata;

pub use metadata::*;

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
