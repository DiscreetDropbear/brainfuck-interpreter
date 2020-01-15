use std::fs::File;
use std::io::{self, Read, Write, BufRead};
use std::env;

static DATA_INCREMENTS: usize= 30_000; 

fn get_instructions() -> Result<String, io::Error> {

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

    // remove all white space, newline and carriage return characters
    file_contents = file_contents.replace(" ", "")
        .replace("\n", "")
        .replace("\r", "");

    return Ok(file_contents);
}

fn get_input() -> Result<u8, ()> {
    
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let buffer = stdin.fill_buf().unwrap(); 
    
    if buffer.len() > 0{

        let input = buffer[0];
        stdin.consume(1);

        return Ok(input);
    }

    return Err(());
}

fn main() -> Result<(), io::Error> {

    let file_contents = get_instructions()?; 

    //setup instruction array
    let mut inst_idx: usize = 0;   
    let inst = file_contents.as_bytes();
    let inst_len = inst.len();

    // settup the data array and index
    let mut data: Vec<u8> = vec![0; DATA_INCREMENTS]; 
    let mut data_pointer = 0;

    loop {
        
        match inst[inst_idx] as char{
            '>' => {
                data_pointer += 1;

                if data_pointer >= data.len(){
                    data.reserve(DATA_INCREMENTS);
                    unsafe{data.set_len(data.len() + DATA_INCREMENTS);}
                    for i in data.len()-DATA_INCREMENTS..DATA_INCREMENTS{
                        data[i] = 0;
                    }

                }
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
                io::stdout().flush().unwrap();
            },
            ',' => {
                let input = get_input();
                match input{
                    Ok(input) => {
                        data[data_pointer] = input;
                    },
                    Err(_) => {}
                }
           },
            '[' => {
                if data[data_pointer] == 0{
                    let mut count = 0; 
                    loop{

                        inst_idx += 1;
                        if inst_idx >= inst_len{
                            panic!("inst_idx is larger than the amount of instructions available.");
                        }

                        if inst[inst_idx] as char == ']'{
                            if count == 0{
                                break;
                            }
                            count -= 1;
                        }
                        else if inst[inst_idx] as char == '['{
                            count += 1; 
                        }
                        
                    }    
                }
            },
            ']' => {
            
                if data[data_pointer] != 0{
                    let mut count = 0;
                    loop {

                        let (new_inst_idx, overflow) = inst_idx.overflowing_sub(1);
                        if overflow{
                            panic!("inst_idx was negative.");
                        }

                        inst_idx = new_inst_idx;
                        
                        if inst[inst_idx] as char == '['{
                            if count == 0{
                                break;
                            }
                            else{
                                count -= 1;
                            }
                        }
                        else if inst[inst_idx] as char == ']'{
                            count += 1;
                        }


                    }
                }

            },
            _ =>{
//                panic!("incorrect instruction");
            }
        }

        
        inst_idx += 1; 
 
        if inst_idx >= inst_len{
            break;
        }
        
        //println!("{}", inst_idx);

    }

    Ok(())
}
