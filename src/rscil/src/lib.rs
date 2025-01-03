mod metadata;

pub use metadata::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut image = PeParser::open("tests/HelloWorld.exe")
            .and_then(|parser| parser.read())
            .unwrap();

        let assembly = image.get_assembly().unwrap();
        dbg!(image.get_string(assembly.name));

        let methods = cast_table!(MethodDef, image.streams.metadata.get_table(TableKind::MethodDef)).len();
        for i in 0..methods {
            let method = image.get_method_def(i as u32).unwrap();
            dbg!(image.get_string(method.name));
            dbg!(image.get_method_body(i as u32).unwrap());
        }
    }
}
