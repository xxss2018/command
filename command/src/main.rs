use structopt::StructOpt;
use std;
use std::fmt::{self, Display, Formatter};
use std::process::Command;

#[derive(StructOpt)]
#[structopt(name = "app")]
pub struct GetCommand {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt)]
pub enum Command {
    

    //linux创建文件
    #[structopt(name = "khtouch")]
    KhTouch(Elements),
    // linux删除文件
    #[structopt(name = "khrm")]
    KhRm(Elements),

}

#[derive(StructOpt)]
pub struct Elements {
    pub elements: Vec<i32>,
}
//实现display
impl Display for Elements {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#?}]", self.elements)
    }
}

fn main() {
    let cmd = GetCommand::from_args();
    match cmd.command {
        
        Command::KhTouch(e) =>{
            println!("Operants: {}", e);
            print!("Result: ");
            //迭代器访问
            for i in &e.elements {
		let g = i.to_string();
		let act_str = String::from("touch ");
		let cmd_str = act_str+&g;
		Command::new("sh").arg("-c")arg(cmd_str).output().expect("sh exec error!");



                }
            }
        },
        Command::KhRm(e) =>{
            println!("Operants: {}", e);
            print!("Result: ");
            for i in &e.elements {
                let act_str = String::from("rm -rf ");
		let cmd_str = act_str+&g;
		Command::new("sh").arg("-c")arg(cmd_str).output().expect("sh exec error!");

                }
            }
        },

    }
}
