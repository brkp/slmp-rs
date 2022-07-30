#[derive(Debug, Clone, Copy)]
pub enum Command {
    // Device commands
    DeviceRead         = 0x0401,
    DeviceWrite        = 0x1401,
    DeviceReadRandom   = 0x0403,
    DeviceWriteRandom  = 0x1402,
    EntryMonitorDevice = 0x0801,
    ExecuteMonitor     = 0x802,
    ReadBlock          = 0x406,
    WriteBlock         = 0x1406,

    // Label commands
    ArrayLabelRead   = 0x041a,
    ArrayLabelWrite  = 0x141a,
    LabelReadRandom  = 0x041c,
    LabelWriteRandom = 0x141b,

    // Memory commands
    MemoryRead  = 0x0613,
    MemoryWrite = 0x1613,

    // Extend Unit commands
    ExtendUnitRead  = 0x0601,
    ExtendUnitWrite = 0x1601,

    // Remote Control commands
    RemoteRun        = 0x1001,
    RemoteStop       = 0x1002,
    RemotePause      = 0x1003,
    RemoteLatchClear = 1005,
    RemoteReset      = 0x1006,
    ReadTypeName     = 0x0101,

    // Remote Password commands
    Lock   = 0x1631,
    Unlock = 0x1630,

    // File commands
    ReadFileInfo    = 0x1810,
    SearchFile      = 0x1811,
    NewFile         = 0x1820,
    DeleteFile      = 0x1822,
    CopyFile        = 0x1824,
    ChangeFileState = 0x1825,
    ChangeFileDate  = 0x1826,
    OpenFile        = 0x1827,
    ReadFile        = 0x1828,
    WriteFile       = 0x1829,
    CloseFile       = 0x182a,

    // Stray ones
    SelfTest   = 0x0619,
    ClearError = 0x1617,
    OnDemand   = 0x2101,
}
