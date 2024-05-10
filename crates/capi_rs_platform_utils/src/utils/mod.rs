use std::fs::OpenOptions;
use std::io::Write;

pub fn write_bytes_to_file(byte_data: &[u8], filename: &str) {
    // 使用 OpenOptions 来配置打开或创建文件的方式
    let mut file = OpenOptions::new()
        // 文件路径
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)
        .expect(format!("cannot open file {}", filename).as_str()); // 处理可能的错误
    match file.write_all(byte_data) {
        Ok(_) => println!("write success!"),
        Err(error) => println!("write err: {:?}", error),
    }
}
