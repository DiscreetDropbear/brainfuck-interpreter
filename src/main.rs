use std::fs::File;
use std::io::{self, Read};
use std::env;

static MAX_DATA: usize = 30_000;

fn main() -> Result<(), io::Error>{

    // get the input file
    let mut input_file: String = String::new();
    let mut arg_iter = env::args();
    arg_iter.next();

    match arg_iter.next(){
        Some(val) => {
            input_file = val;
        }
        None => {
            println!("no input file specified!");
        }
    }
    
    // read input file contents
    let mut file_contents: String = String::new();
    File::open(&input_file)?
        .read_to_string(&mut file_contents)?;
    
    //setup instrucion array
    let mut inst_idx: usize = 0;   
    let inst = file_contents.as_bytes();
    let inst_len = inst.len();

    // settup the data array and index
    let mut data: Vec<u8> = Vec::with_capacity(MAX_DATA); 
    unsafe{data.set_len(MAX_DATA);}
    let mut data_pointer = 0;


    loop {
       
        match inst[inst_idx] as char{
            '>' => {
                data_pointer += 1;
            },

            '<' => {
                if data_pointer != 0{
                    data_pointer -= 1;
                }
            },

            '+' => {
                data[data_pointer] = data[data_pointer].wrapping_add(1);                
            },

            '-' => {
                data[data_pointer] = data[data_pointer].wrapping_sub(1);               
            },

            '.' => {
                print!("{}", data[data_pointer] as char);
            },
            ',' => {
                // get input from stdin, only reads the first byte and the rest are discarded
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
               
                data[data_pointer] = buffer.as_bytes()[0]; 

            },
            '[' => {
                if data[data_pointer] == 0{

                    loop{
                        inst_idx += 1;
                        if inst_idx >= inst_len{
                            break;
                        }

                        if inst[inst_idx] as char == ']'{
                            inst_idx += 1;
                            break;
                        }

                    }    
                }
                

            },
            ']' => {
            
                if data[data_pointer] != 0{
                    loop {
                        let (new_inst_idx, overflow) = inst_idx.overflowing_sub(1);
                        if overflow{
                            break;
                        }

                        inst_idx = new_inst_idx;

                        if inst[inst_idx] as char == '['{
                            break;
                        }

                    }
                }

            },
            _ =>{}
        }

        
        inst_idx += 1; 
 
        if inst_idx >= inst_len{
            break;
        }

    }

    Ok(())
}
