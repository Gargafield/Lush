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

        let member = image.get_member_ref(2).unwrap();
        dbg!(member);

        let _type = image.get_type_ref(3).unwrap();
        dbg!(_type);
        dbg!(image.get_string(_type.type_name));
        dbg!(image.get_string(_type.type_namespace));

        let assembly_ref = image.get_assembly_ref(0).unwrap();
        dbg!(image.get_string(assembly_ref.name));
    }
}
