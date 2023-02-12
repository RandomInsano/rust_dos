use rust_dos::{
    *,
    dos::{
        error_code::ErrorCode,
        file::{
            StorageParameters,
            AccessMode,
            File, FileAttributes
        }
    }
};

#[allow(dead_code)]
pub(crate) fn file_read_write_test() {
    let input_file = File::open("C:\\AUTOEXEC.BAT\0", AccessMode::default());
    let input_file = input_file.unwrap_or(File::open("README.md\0", AccessMode::default()).unwrap());

    let output_file = dos::file::File::create("WRITE.TMP\0", FileAttributes::NORMAL).unwrap();

    let mut total_read: usize = 0;
    let mut total_written: usize = 0;

    let mut buffer = [0; 1024];
    
    loop {
        let read = input_file.read(&mut buffer).unwrap();

        total_read += read;
        total_written += output_file.write(&mut buffer[0..read]).unwrap();

        if read < buffer.len() {
            break;
        }
    }

    input_file.close().unwrap();
    output_file.close().unwrap();

    println!("Total bytes transferred: In: {}, Out: {}", total_read, total_written);
}

#[allow(dead_code)]
pub(crate) fn file_attribute_test() {
    let attributes = dos::file::File::attributes("C:\\AUTOEXEC.BAT\0");
    let attributes = attributes.unwrap_or(dos::file::File::attributes("README.md\0").unwrap());

    // Attributes aren't supported in DOSBox so expect this to have no info
    println!("Attributes: {:?}", attributes);

    println!("Long filename {:?}", dos::file::File::attributes("Really long name or something\0"));

    let test_file = dos::file::File::open("C:\\AUTOEXEC.BAT\0", AccessMode::default());
    let test_file = test_file.unwrap_or(dos::file::File::open("README.md\0", AccessMode::default()).unwrap());
    let (date, time) = test_file.last_write().unwrap();

    let test_file = dos::file::File::open("BAD FILE\0", AccessMode::default());
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

#[allow(dead_code)]
pub(crate) fn disk_space_test() {
    // Grab the free and total storage on drive "C:"
    let parameters = StorageParameters::disk_space(2);

    print!("Storage on drive C: ");

    match parameters {
        Ok(x) => {
            println!("{} total, {} free", x.total_space(), x.free_space());
        },
        Err(_) => {
            println!("Unable to get storage information");
        }
    }
}