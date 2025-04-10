use std::error::Error;
use bincode;

// The Communication trait groups related functions
pub trait Communication {

    // send contents of the buffer over the stream
    fn send(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn Error>>;
    
    // receive contents from the stream and place them into the buffer
    fn receive(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>>;

    // The functions below are Default implementations - Any struct that implements 
    // the Communication trait can use these functions

    // encode the contents of the object as bytes into the buffer
    fn encode <T:bincode::Encode> (object: T, buf: &mut[u8]) -> Result<usize, Box<dyn Error>> {
        let mut bytes = bincode::encode_into_slice(
            object,
            buf,
            bincode::config::standard()
        )?;
        Ok(bytes)
    }
    
    // decode a buffer of bytes into an object
    fn decode <T> (length:usize, buf: &mut[u8]) -> Result<T, Box<dyn Error>> 
    where T: bincode::Decode<()>
    {
        let buffer = &buf[..length];
        let deserialized: T = bincode::decode_from_slice(buffer, bincode::config::standard())?.0;
        Ok(deserialized)
    }
}

    
