pub mod pe_image;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut image = pe_image::PeImage::open("tests/HelloWorld.exe").unwrap();
        image.read().unwrap();
    }
}
