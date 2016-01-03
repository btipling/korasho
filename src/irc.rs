use std::str;

pub fn handle_connection(mut connection: ::connection::Connection, config: ::config::Config) {
    let _ = connection.write(&[1]);
    let mut buf = [0; 128];
    loop {
        let result = connection.read(&mut buf).unwrap(); // ignore here too
        let result_str = str::from_utf8(&buf).unwrap();
        process_data(result, &result_str);
    }
}

fn process_data(result_size: usize, result_str: &str) {
    println!("got {size} bytes: {str}", size=result_size, str=result_str);
}

