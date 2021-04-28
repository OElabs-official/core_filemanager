use std::collections::HashMap as H;
use std::io;
pub mod utilset;

pub mod stdwrap
{
    use std::{io,fs};
    use std::path as P;
    use std::ffi::OsStr;

    pub fn ls(path:&P::PathBuf)->Vec<P::PathBuf>
    {
        fn readdir(path:&P::PathBuf,v0:&mut Vec<P::PathBuf>)->io::Result<()>
        {//返回不是重点，是改变输入的vec，在外面再包装一层
            *v0     =
                    std::fs::read_dir(path)?
                    .map
                    (   //Struct std::fs::DirEntry
                        |res|res.map(|e|e.path())
                    )
                    .collect::<Result<Vec<_>, io::Error>>()?//->Vec<std::path::PathBuf>
                    ;
            Ok(())
        }//把目录的文件列表写入输入的v0
        let mut output:Vec<P::PathBuf>          =vec![];
        let info                                =readdir(path, &mut output);
        output
    }

    pub fn create_dir(path:&P::PathBuf)
    {
        match fs::create_dir(path)
        {
            Ok(_)                               =>println!("create dir {:?} sucess",path),
            Err(e)                              =>println!("create{:?},err{}",path,e)
        }
    }

    pub fn rename(path:&P::PathBuf,target:&P::PathBuf)
    {//一样可以move文件，在当前分区下
        let report                              =fs::rename(path, target);
        println!                                ("rename(move) file:\"{:?}\"->{:?}",path,report);
    }

}


fn main() {
    //两种工作模式
    //A:通过CLI启动
    //B:通过CLI指定工作(可供C#使用)
    non_args();
}

const S_funcs:&str="select funcs:\n\t[1]:classfication\n\t[2]:compress\n\t[others *n]:exit";
const S_type:&str="select file type:\n\t[1]:videos\n\t[2]:audios\n\t[3]:photos\n\t[4]:read rules from file\n\t[others *n]:exit";
fn non_args()
{
    let ws                              = getworkspace();
    let n0                              = select(S_funcs);
    match n0
        {
            1=> {
                    let n1              = select(S_type);
                    let hm0                             = 
                    match n1
                        {
                            1=> crate::utilset::preset::get_videos_hashmap(),
                            2=> crate::utilset::preset::get_audios_hashmap(),
                            3=> crate::utilset::preset::get_photos_hashmap(),
                            _=> panic!("exit."),//还要补全读文件自定义规则
                        };                    
                        //通过hashmap指定不同的扩展文件也可以放进同一个文件夹
                        //hashmap: 扩展名:目标文件夹名
                        //扩展名一律小写
                        crate::utilset::classfi::scan(&ws, &hm0)                        
                },
            _=>(),
        }



}

fn getworkspace()->std::path::PathBuf
{
    println!("input work_space Path, (\")is acceptable :");
    let mut input0                      = String::new();
    io::stdin().read_line(& mut input0);
    input0                              = input0.trim().to_string();
    if let Some('\"')=input0.chars().next()
    {
        input0                          = input0.as_str()[1..input0.len()-1].to_string();
    }
    let output                          = std::path::PathBuf::from(input0);
    output
}

fn select(inf:&str)->u32
{
    println!("{}",inf);
    let mut input0                      = String::new();
    io::stdin().read_line(& mut input0);
    input0                              = input0.trim().to_string();    
    let output :u32                     = match input0.parse() 
    {
        Ok(n)=>n,
        Err(e)=>{println!("accept number only!");select(inf)}
    };
    output    
}
