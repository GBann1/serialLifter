// Data stream is ☻+       0 lGM
// <STX\><POL\><wwwwwww\><UNIT\><G/N\><S\><TERM\>

// This struct is to hold the parsed data
pub struct RSData {
    pub polarity: u8,       // <POL\>       char
    pub weight: [u8],       // <wwwwwww\>   int
    pub unit: u8,           // <UNIT\>      char
    pub weight_type: u8,    // <GROSS-NET\> char
    pub scale_status: u8,   // <Z/M/D/' '\> char
  }

// Takes in the serialbuffer slice and returns the struct
fn decode_data(data: &[u8]) -> RSData {
    // Define the struct defaults
    let mut rsdata = RSData {
      // b'_' means the binary expression (I think)
      polarity: b'+',
      weight: [0; 8],
      unit: b'l',
      weight_type: b'G',
      scale_status: b'D',
    };
    rsdata.polarity = data[1];
    rsdata.weight = data[2..10];
    rsdata.unit = data[11];
    rsdata.weight_type = data[12];
    rsdata.scale_status = data[13];
    // It would be nice to convert these to their end data types here
    rsdata
  }

// Function to print the data after it has been parsed
fn print_data(data: RSData) {
    println!("{}", data.weight);
    // emit.to_frontend(data)
}