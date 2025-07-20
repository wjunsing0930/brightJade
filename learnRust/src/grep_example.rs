use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

fn grep<R>(target: &str, reader:R) -> io::Result<()>
    where R:BufRead
{
    for line_result in reader.lines(){
        let line = line_result?;
        if line.contains(target){
            println!{"grep:{}", line};
        }
    }
    Ok(())
}

// fn grep_main() ->Result<(), Box<dyn Error>>{
//     let mut args = std::env::args().skip(1);
//     let target = match args.next(){
//         Some(s) => s,
//         None => Err("usage , grep patern file..")?
//     };
//     let files:Vec<PathBuf> = args.map(PathBuf::from).collect();
//     if files.is_empty(){
//         let stdin = io::stdin();
//         grep(&target, stdin.lock())?;
//     } else {
//         for file in files{
//             let f = File::open(file)?;
//             grep(&target, BufReader::new(f));
//         }
//     }
//     Ok(())
// }
fn grep_main(target: &str, files: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    if files.is_empty() {
        let stdin = io::stdin();
        grep(target, stdin.lock())?;
    } else {
        for file in files {
            let f = File::open(file)?;
            grep(target, BufReader::new(f))?;
        }
    }
    Ok(())
}


#[test]
fn grep_test(){
    // let result = grep_main();
    // if let Err(err) = result{
    //     eprintln!("{}", err);
    //     std::process::exit(1);
    // }
    let file_path = "test_file.txt";
    let content = b"hello\nworld\ntarget line\nanother target line\n";

    // Write to file
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(content).expect("Unable to write to file");

    // Now wrap the file path in a Vec<PathBuf>
    let target = "target";
    let file_path = vec![PathBuf::from(file_path)];  // Wrap in Vec<PathBuf>
    
    // Call grep_main with the file as input
    let result = grep_main(target, file_path.clone());  // Pass a clone of file_path

    // Ensure result is successful
    assert!(result.is_ok(), "grep_main failed: {:?}", result.err());

    // Cleanup
    std::fs::remove_file(&file_path[0]).expect("Failed to remove file"); 
}