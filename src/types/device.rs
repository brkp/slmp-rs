/// This enum is used to denote what kind of device
/// we're dealing with in terms of memory size.
#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Bit,
    Word,
    DoubleWord,
}

/// All possible devices that can be interacted with over SLMP.
#[derive(Debug, Clone, Copy)]
pub enum Device {
    /// Special relay
    SM   = 0x0091,
    /// Special register
    SD   = 0x00a9,
    /// Input
    X    = 0x009c,
    /// Output
    Y    = 0x009d,
    /// Internet relay
    M    = 0x0090,
    /// Latch relay
    L    = 0x0092,
    /// Annunciator
    F    = 0x0093,
    /// Edge relay
    V    = 0x0094,
    /// Link relay
    B    = 0x00a0,
    /// Data register
    D    = 0x00a8,
    /// Link register
    W    = 0x00b4,
    /// Timer (Contact)
    TS   = 0x00c1,
    /// Timer (Coil)
    TC   = 0x00c0,
    /// Timer (Current value)
    TN   = 0x00c2,
    /// Long timer (Contact)
    LTS  = 0x0051,
    /// Long timer (Coil)
    LTC  = 0x0050,
    /// Long timer (Current value)
    LTN  = 0x0052,
    /// Retentive timer (Contact)
    STS  = 0x00c7,
    /// Retentive timer (Coil)
    STC  = 0x00c6,
    /// Retentive timer (Current value)
    STN  = 0x00c8,
    /// Long retentive timer (Contact)
    LSTS = 0x0059,
    /// Long retentive timer (Coil)
    LSTC = 0x0058,
    /// Long retentive timer (Current value)
    LSTN = 0x005a,
    /// Counter (Contact)
    CS   = 0x00c4,
    /// Counter (Coil)
    CC   = 0x00c3,
    /// Counter (Current value)
    CN   = 0x00c5,
    /// Long counter (Contact)
    LCS  = 0x0055,
    /// Long counter (Coil)
    LCC  = 0x0054,
    /// Long counter (Current value)
    LCN  = 0x0056,
    /// Link special relay
    SB   = 0x00a1,
    /// Link special register 
    SW   = 0x00b5,
    /// Direct access input
    DX   = 0x00a2,
    /// Direct access output 
    DY   = 0x00a3,
    /// Index register
    Z    = 0x00cc,
    /// Long index register 
    LZ   = 0x0062,
    // R    = 0x00af,
    // ZR   = 0x00b0,
    // ExtD = 0x00a8,
    // ExtW = 0x00b4,
    /// Module refresh register
    RD   = 0x002c,
}

impl Device {
    /// Returns the `DeviceType` of `self`.
    pub fn get_type(&self) -> DeviceType {
        use Device::*;
        use DeviceType::*;

        match self {
            X    | Y    | M   | L   | F   | V   | B       => Bit,
            SM   | TS   | TC  | SB  | DX  | DY  | CS | CC => Bit,
            LTS  | LTC  | STS | STC | LCS | LCC           => Bit,
            LSTS | LSTC                                   => Bit, 

            D  | W  | Z                  => Word,
            SD | TN | CN | SW | RD | STN => Word,

            LZ | LTN | LSTN | LCN  => DoubleWord,
        }
    }
} 
