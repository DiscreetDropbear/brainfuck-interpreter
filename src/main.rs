use std::fs::File;
use std::io::{self, Read, Write, BufRead, Error, ErrorKind};
use std::env;

static DATA_INCREMENTS: usize= 30_000; 

fn main() {

    let file_contents =  match get_instructions(){
        Ok(contents) => contents,
        Err(_) => return ()
    };

    //setup instruction array
    let mut inst_idx: usize = 0;   
    let inst = file_contents.as_bytes();
    let inst_len = inst.len();

    // settup the data array and index
    let mut data_idx = 0;
    let mut data: Vec<u8> = vec![0; DATA_INCREMENTS]; 

    loop {
        
        match inst[inst_idx] as char{
            /* increment the data index by one, effectively moving the data 'pointer' one 'cell
             * to the right
             */
            '>' => {
                data_idx += 1;

                if data_idx >= data.len(){
                    data.reserve(DATA_INCREMENTS);
                    unsafe{data.set_len(data.len() + DATA_INCREMENTS);}
                    for i in data.len()-DATA_INCREMENTS..DATA_INCREMENTS{
                        data[i] = 0;
                    }

                }
            },
            /* decrement the data index by one, effectively moving the data 'pointer' one 'cell
             * to the left 
             */
            '<' => {
                if data_idx != 0{
                    data_idx -= 1;
                }
            },
            /* add one to the data in the current 'cell' refered to by the data index
             * if an overflow happens it is just wrapped around
             */
            '+' => {
                data[data_idx] = data[data_idx].wrapping_add(1);                
            },
            /* subtract one from the data in the current 'cell' refered to by the data index
             * if an overflow happens it is just wrapped around
             */
            '-' => {
                data[data_idx] = data[data_idx].wrapping_sub(1);               
            },
            /* prints the character from the current 'cell' refered to by the data index */
            '.' => {
                print!("{}", data[data_idx] as char);
                io::stdout().flush().unwrap();
            },
            /* gets a byte from stdin, and places it in the current 'cell' refered to by the data
             * index
             */
            ',' => {
                let input = get_input();
                match input{
                    Ok(input) => {
                        data[data_idx] = input;
                    },
                    Err(_) => {}
                }
           },
            '[' => {

                /* when '[' is encountered and data[data_idx] equals 0 then  
                 * jump to the matching ']', to do this we must deal with nested brackets
                 *
                 * This is done by incrementing count for  every subsequent '[' encountered
                 * and decrementing count for every sub sequent ']' ecountered untill a ']' 
                 * is encountered and count equals 0 in which case we have found the matching ']'
                 *
                 * we then fall out of the loop and let the instruction be incrmented to the one 
                 * after the ']' at the bottom of the main loop as normal
                 */

                if data[data_idx] == 0{
                    
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
 
                /* when ']' is encountered and data[data_idx] doesn't equal 0 then  
                 * jump backwards  to the matching '[', to do this we must deal with nested brackets
                 *
                 * This is done by incrementing count for  every subsequent ']' encountered
                 * and decrementing count for every sub sequent '[' ecountered untill a '[' 
                 * is encountered and count equals 0 in which case we have found the matching '['
                 *
                 * we then fall out of the loop and let the instruction be incrmented to the one 
                 * after the ']' at the bottom of the main loop as normal
                 */
           
                if data[data_idx] != 0{

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
                            count -= 1;
                        }
                        else if inst[inst_idx] as char == ']'{
                            count += 1;
                        }
                    }
                }

            },
            _ =>{} // ignore any characters that arne't commands
        }

        inst_idx += 1; 
        if inst_idx >= inst_len{
            break;
        }
    }
}

// returns a string containing the brainfuck instructions from a file specified in
// the first command line argument provided
fn get_instructions() -> Result<String, io::Error> {

    // get the input file from command line arguments
    let mut input_file: String = String::new();
    let mut arg_iter = env::args();
    let prog_name = arg_iter.next().unwrap();

    match arg_iter.next(){
        Some(val) => {
            input_file = val;
        }
        None => {
            println!("Usage: {} Path", prog_name);
            return Err(Error::new(ErrorKind::Other, ""));
        }
    }
    
    // read input file contents
    let mut file_contents: String = String::new();
    File::open(&input_file)?
        .read_to_string(&mut file_contents)?;

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

