#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Bit,
    Word,
    DoubleWord,
}

#[derive(Debug, Clone, Copy)]
pub enum Device {
    SM   = 0x0091,
    SD   = 0x00a9,
    X    = 0x009c,
    Y    = 0x009d,
    M    = 0x0090,
    L    = 0x0092,
    F    = 0x0093,
    V    = 0x0094,
    B    = 0x00a0,
    D    = 0x00a8,
    W    = 0x00b4,
    TS   = 0x00c1,
    TC   = 0x00c0,
    TN   = 0x00c2,
    LTS  = 0x0051,
    LTC  = 0x0050,
    LTN  = 0x0052,
    STS  = 0x00c7,
    STC  = 0x00c6,
    STN  = 0x00c8,
    LSTS = 0x0059,
    LSTC = 0x0058,
    LSTN = 0x005a,
    CS   = 0x00c4,
    CC   = 0x00c3,
    CN   = 0x00c5,
    LCS  = 0x0055,
    LCC  = 0x0054,
    LCN  = 0x0056,
    SB   = 0x00a1,
    SW   = 0x00b5,
    DX   = 0x00a2,
    DY   = 0x00a3,
    Z    = 0x00cc,
    LZ   = 0x0062,
    R    = 0x00af,
    ZR   = 0x00b0,
    // D    = 0x00a8,
    // W    = 0x00b4,
    RD   = 0x002c,
}

impl Device {
    pub fn get_type(&self) -> DeviceType {
        use Device::*;
        use DeviceType::*;

        match self {
            X    | Y    | M   | L   | F   | V   | B       => Bit,
            SM   | TS   | TC  | SB  | DX  | DY  | CS | CC => Bit,
            LTS  | LTC  | STS | STC | LCS | LCC           => Bit,
            LSTS | LSTC                                   => Bit, 

            D  | W  | Z  | R                  => Word,
            SD | TN | CN | SW | ZR | RD | STN => Word,

            LZ | LTN | LSTN | LCN  => DoubleWord,
        }
    }
} 
