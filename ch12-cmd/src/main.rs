use std::env;
use std::fs;

fn main(){
    let args:Vec<String>=env::args().collect();
    let file_name=parse_param(&args).unwrap();

    println!("{}",file_name);

    let content=read_content(file_name);
    println!("{}",content);
}

fn parse_param(arr:&Vec<String>)->Result<&str,&str>{
    let ans=arr.get(1).unwrap();
    Ok(ans.as_str())
}

fn read_content(file_name:&str)->String{
    let content=fs::read_to_string(file_name).unwrap_or_else(|err|{
        println!("error:{}",err);
        "error".to_string()
    });
    content
}

#[cfg(test)]
mod tests{
    use crate::parse_param;

    #[test]
    fn test_parse_param(){
        let mut arr=Vec::new();
        arr.push("other".to_string());
        arr.push("res".to_string());
        let str=parse_param(&arr).unwrap();
        assert_eq!(str,"res");
    }
}