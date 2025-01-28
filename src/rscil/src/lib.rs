mod metadata;

pub use metadata::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let image = PeParser::open("tests/HelloWorld.exe")
            .and_then(|parser| parser.read())
            .unwrap();

        let assembly = image.get_assembly().unwrap();
        dbg!(image.get_string(assembly.name));

        let entry_point_index = match image.cli_header.entry_point_token {
            MetadataToken::Table(_, index) => index,
            _ => panic!("Invalid entry point token"),
        };
        dbg!(entry_point_index);

        let entry_point = image.get_method_body(entry_point_index).unwrap();

        dbg!(entry_point);
    }
}
