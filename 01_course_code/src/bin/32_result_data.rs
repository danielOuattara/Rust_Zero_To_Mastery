enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new("alert"))
    } else {
        Err("unable to find sound data".to_owned())
    }
}

fn main() {
    let sound = get_sound("alert");

    match sound {
        Ok(_) => println!("Sound data located"),
        Err(err) => println!("error: {:?}", err),
    }
}
