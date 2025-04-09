use std::error::Error;
use bincode;

pub trait Communication {
    fn send(&mut self, buf: &mut [u8]) -> Result<(), Box<dyn Error>>;
    fn receive(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>>;

    fn encode <T:bincode::Encode> (object: T, buf: &mut[u8]) -> Result<usize, Box<dyn Error>> {
        let mut bytes = bincode::encode_into_slice(
            object,
            buf,
            bincode::config::standard()
        )?;
        Ok(bytes)
    }
    
    fn decode <T> (length:usize, buf: &mut[u8]) -> Result<T, Box<dyn Error>> 
    where T: bincode::Decode<()>
    {
        let buffer = &buf[..length];
        let deserialized: T = bincode::decode_from_slice(buffer, bincode::config::standard())?.0;
        Ok(deserialized)
    }
}

    
