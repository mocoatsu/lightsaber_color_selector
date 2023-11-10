use std::io;

pub fn get_user_input() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    Ok(buf.trim().to_string())
}
