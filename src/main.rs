use std::convert::{TryFrom, TryInto};
use std::fs;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let mut dir = fs::read_dir("files/cali")?;
    while let Some(i) = dir.next() {
        let path = i?.path();
        let metadata = fs::metadata(&path)?;

        if let Ok(time) = metadata.modified() {
            let duration =
                time::Duration::try_from(time.duration_since(SystemTime::UNIX_EPOCH).unwrap())
                    .unwrap();

            let time = time::PrimitiveDateTime::unix_epoch() + duration;
            let date = time.date();

            let file_path = path.as_path();
            let extension = match path.extension() {
                Some(x) => x.to_str().unwrap(),
                None => continue,
            };
            let date = date.lazy_format("%d.%m.%Y");

            let mut new_filename = format!("files/cali/{}.{}", date, extension);
            let mut count = 0;

            while fs::File::open(&new_filename).is_ok() {
                count += 1;
                new_filename = format!("files/cali/{}_{}.{}", date, count, extension);
            }

            // println!("{:?}, {:?}", file_path, new_filename);
            fs::rename(file_path, new_filename);
        } else {
            println!("Not supported on this platform");
        }
    }
    Ok(())
}
