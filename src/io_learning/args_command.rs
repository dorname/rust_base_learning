use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
pub fn args_test() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", &args[0]);
    dbg!(args);
}
pub fn read_src_test() {
    let src = fs::read_dir("src\\io_learning\\files");
    src.unwrap().for_each(|x| {
        println!("{:?}",x);
        println!("{:?}",x.unwrap().path());
    });

}
pub fn read_file_test(){
    // let file_path = "src\\io_learning\\file_test.txt";
    let file_path = "src\\io_learning\\Panel.js";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut file = File::create("src\\io_learning\\files\\test.txt").unwrap();
    //读取到文件内容后，按行处理输出行内容和行号 
    let lines = contents.lines();
    let mut line_num = 0;   
    for line in lines {
        line_num += 1;

        if line.contains("Ext.define") {
            
            println!("test{}", line_num);
            // break;
        }
        write!(file, "{},{}\n",line_num, line);
        // println!("{}: {}", line_num, line);
    }

   

    // println!("With text:\n{contents}");
    // println!("With lines:\n{:?}",lines);
}
#[test]
fn test(){
    // args_test();
    read_file_test();
    // read_src_test()
}