#![warnn(rust_2018_idioms)]
#![allow(elided_lifetimes_in_paths)]

use std::fs;
use std::io;
use std::path::Path;

fn copy_dir_to(src :&Path, dst :&Path) ->io::Result<()>{
    if !dst.is_dir() {
        fs::create_dir(dst)?;
    }

    for entry_result in src.read_dir()?{
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        copy_to(&entry.path(), &file_type, &dst.join(entry.file_name()))?;
    }
    Ok(())
}



