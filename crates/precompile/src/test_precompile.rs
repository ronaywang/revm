use crate::{Error, Precompile, PrecompileResult, PrecompileWithAddress};
use revm_primitives::{ B256, U256 , Bytes};

pub const FUN: PrecompileWithAddress =
    PrecompileWithAddress(crate::u64_to_address(800), Precompile::Standard(test_run));

pub fn test_run(bytes: &Bytes, gas_limit: u64) -> PrecompileResult {
    //println!("{}",bytes);

    let a: U256 = <B256>::try_from(&bytes[0..32]).unwrap().into(); // a = first 32 bytes
    let b: U256 = <B256>::try_from(&bytes[32..64]).unwrap().into(); // b = last 32 bytes
    //println!("a is {}", a);
    //println!("b is {}", b);                           
    let gas_used: u64 = 0;
    if gas_used > gas_limit { //This should never happen 
        return Err(Error::OutOfGas);
    }
    let result = a.checked_add(b);
    let bytes_result: Bytes = match result {
        Some(value) => {
            println!("Sum result is {value}");
            // Convert the byte array to Bytes
            Bytes::from(value.to_be_bytes_vec())
        },
        None => {
            Bytes::new()
        },
    };
    Ok((gas_used, bytes_result))
}
