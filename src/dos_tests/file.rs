use rust_dos::{*, dos::error_code::ErrorCode};

#[allow(dead_code)]
pub(crate) fn file_read_test() {
    let test_file = dos::file::File::open("C:\\AUTOEXEC.BAT\0");
    let test_file = test_file.unwrap_or(dos::file::File::open("README.md\0").unwrap());
    let mut buffer = [0; 100];
    let bytes_read = test_file.read(&mut buffer).unwrap();
    println!("{} bytes read", bytes_read);
    println!("{}", core::str::from_utf8(&buffer).unwrap());
    match test_file.close() {
        Ok(_) => println!("File closed"),
        Err(_) => println!("Error closing file")
    }
}

#[allow(dead_code)]
pub(crate) fn file_attribute_test() {
    let attributes = dos::file::File::attributes("C:\\AUTOEXEC.BAT\0");
    let attributes = attributes.unwrap_or(dos::file::File::attributes("README.md\0").unwrap());

    // Attributes aren't supported in DOSBox so expect this to have no info
    println!("Attributes: {:?}", attributes);

    println!("Long filename {:?}", dos::file::File::attributes("Really long name or something\0"));

    let test_file = dos::file::File::open("C:\\AUTOEXEC.BAT\0");
    let test_file = test_file.unwrap_or(dos::file::File::open("README.md\0").unwrap());
    let (date, time) = test_file.last_write().unwrap();

    let test_file = dos::file::File::open("BAD FILE\0");
    assert_eq!(test_file.err().unwrap(), ErrorCode::FileNotFound);

    println!("File modified on {:?} at {:?}", date, time);
}

#[allow(dead_code)]
pub(crate) fn directory_test() {
    let old_path = "C:\\\0";
    let new_path = "C:\\1A2B3C4D\0";

    print!("Creating folder {new_path}... ");
    dos::file::Directory::make(new_path).unwrap();
    println!("Done");

    print!("Changing to folder {new_path}... ");
    dos::file::Directory::change_current(new_path).unwrap();
    println!("Done");

    print!("Changing to folder {old_path}... ");
    dos::file::Directory::change_current(old_path).unwrap();
    println!("Done");

    print!("Deleting folder {new_path}... ");
    dos::file::Directory::remove(new_path).unwrap();
    println!("Done");
}