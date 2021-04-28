
pub mod classfi
{    
    use std::{io,fs};
    use std::path as P;
    use std::ffi::OsStr;
    use crate::stdwrap::*;
    use std::collections::HashMap as H;
    
    fn create_dir_from_hashmap(h:&H<String,String>,p:&P::PathBuf)
    {
        //通过hashmap在根目录p下建立对应的文件夹
        let mut v0:Vec<String>                      =vec![];
        for i0 in h.values()
        {
            let s0                                  =i0.clone();
            if v0.contains(&s0){}
            else {v0.push(s0)}//剔除重复
        }
        for i0 in v0
        {
            let mut path=p.clone();
            path.push(&i0);
            create_dir(&path)
        }
    }
    pub fn scan(infolder:&P::PathBuf,target:&H<String,String>)
    {
        fn file_job(ipath:&P::PathBuf,infolder:&P::PathBuf,h:&H<String,String>)
        {
            match ipath.extension()
            {
                Some(rs)=>match rs.to_str()
                    {
                        Some(s)=>
                        {
                            match h.get(&s.to_lowercase().as_str().to_string())
                            {
                                Some(s)=>
                                {
                                    let mut target             =infolder.clone();
                                    let filename               =match ipath.file_name()
                                        {
                                            Some(s)=>s,
                                            None=>{println!("Err:get filename\"{:?}\"",ipath);OsStr::new("")}                                                
                                        };
                                    target.push(s);
                                    target.push(filename);
                                    rename(ipath, &target);
                                },
                                None=>()
                            }
                        },
                        None=>()
                    },
                None=>()
            }            
        }
        fn recursion(v0:&Vec<P::PathBuf>,infolder:&P::PathBuf,h:&H<String,String>)
        {
            for i0 in v0.iter()
            {
                if i0.is_file()                 {file_job(i0,infolder,h)}
                else if i0.is_dir()
                {
                    let v0                      =ls(i0);
                    recursion(&v0,infolder,h);
                }
            }            
        }
        let v0                                  =ls(infolder);
        create_dir_from_hashmap(target, infolder);
        recursion(&v0, infolder,target);
    }
}

pub mod preset
{
    use std::collections::HashMap as H;
    pub fn get_videos_hashmap()->H<String,String>
    {
        let mut output                          =H::new();
    
        for i0 in ["mp4","mkv","rmvb","wmv","avi"].iter()
        {
            output.insert(i0.to_string(),"videos".to_string());
        }
        output
    }
    
    pub fn get_audios_hashmap()->H<String,String>
    {
        let mut output                          =H::new();
    
        for i0 in ["mp3","ape","flac","wma","aac","m4a"].iter()
        {
            output.insert(i0.to_string(),"audios".to_string());
        }
        output
    }    

    pub fn get_iso_hashmap()->H<String,String>
    {
        let mut output                          =H::new();
    
        for i0 in ["iso"].iter()
        {
            output.insert(i0.to_string(),"iso".to_string());
        }
        output
    }

    pub fn get_photos_hashmap()->H<String,String>
    {
        let mut output                          =H::new();
    
        for i0 in ["jpg","jpeg","png","bmp","tif","tiff"].iter()
        {
            output.insert(i0.to_string(),"photos".to_string());
        }
        output
    }  
    
}