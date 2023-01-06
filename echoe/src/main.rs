#![warn(clippy::all, clippy::pedantic)]

// Copyright 2023 by Jim Lawless
// X11/MIT license
// See Github repo for full license

// This is a new port of an old C program I had written called echof. echof
// had the ability to embed hex-encoded bytes in the output from an echo command
// workalike.  I thought I'd try an alternate approach with a few escape sequences
// that would permit the inclusion of some common escape sequences for tabs,
// carriage-returns, and newlines.  
// 
// Todo: Support Unicode sequences in hex.
// 

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    for val in args[1..].iter() {
        let mut esc = false;
        for c in val.chars() {
            if esc {
                if c == 'n' {
                    print!("\n");
                }
                else if c== 'r' {
                    print!("\r");
                }
                else if c=='t' {
                    print!("\t");
                }
                else if c=='\\' {
                    print!("\\");
                }
                else {
                    print!("\\{}",c);
                }
                esc=false;
            }
            else {
                if c=='\\' {
                    esc=true;
                }
                else {
                    print!("{}",c);
                }
            }
        }
    }
}
