#[derive(Debug)]
pub enum Flags {
    Checkered = 0x00000001,
    White = 0x00000002,
    Green = 0x00000004,
    Yellow = 0x00000008,
    Red = 0x00000010,
    Blue = 0x00000020,
    Debris = 0x00000040,
    Crossed = 0x00000080,
    YellowWaving = 0x00000100,
    OneLapToGreen = 0x00000200,
    GreenHeld = 0x00000400,
    TenToGo = 0x00000800,
    FiveToGo = 0x00001000,
    RandomWaving = 0x00002000,
    Caution = 0x00004000,
    CautionWaving = 0x00008000,

    // drivers black flags
    Black = 0x00010000,
    Disqualify = 0x00020000,
    Servicible = 0x00040000, // car is allowed service (not a flag)
    Furled = 0x00080000,
    Repair = 0x00100000,

    // start lights
    StartHidden = 0x10000000,
    StartReady = 0x20000000,
    StartSet = 0x40000000,
    StartGo = 0x80000000,
}

#[derive(Debug)]
pub enum EngineWarnings {
    WaterTempWarning = 0x01,
    FuelPressureWarning = 0x02,
    OilPressureWarning = 0x04,
    EngineStalled = 0x08,
    PitSpeedLimiter = 0x10,
    RevLimiterActive = 0x20,
    OilTempWarning = 0x40,
}

#[derive(Debug)]
pub enum TrackLocation {
    NotInWorld = -1,
    OffTrack = 0,
    InPitStall = 1,
    AproachingPits = 2,
    OnTrack = 3,
}

#[derive(Debug)]
pub enum TrackSurface {
    SurfaceNotInWorld = -1,
    UndefinedMaterial = 0,

    Asphalt1Material = 1,
    Asphalt2Material = 2,
    Asphalt3Material = 3,
    Asphalt4Material = 4,
    Concrete1Material = 5,
    Concrete2Material = 6,
    RacingDirt1Material = 7,
    RacingDirt2Material = 8,
    Paint1Material = 9,
    Paint2Material = 10,
    Rumble1Material = 11,
    Rumble2Material = 12,
    Rumble3Material = 13,
    Rumble4Material = 14,

    Grass1Material = 15,
    Grass2Material = 16,
    Grass3Material = 17,
    Grass4Material = 18,
    Dirt1Material = 19,
    Dirt2Material = 20,
    Dirt3Material = 21,
    Dirt4Material = 22,
    SandMaterial = 23,
    Gravel1Material = 24,
    Gravel2Material = 25,
    GrasscreteMaterial = 26,
    AstroturfMaterial = 27,
}

#[derive(Debug)]
pub enum SessionState {
    StateInvalid = 0,
    StateGetInCar = 1,
    StateWarmup = 2,
    StateParadeLaps = 3,
    StateRacing = 4,
    StateCheckered = 5,
    StateCoolDown = 6,
}

#[derive(Debug)]
pub enum CarLeftRight {
    LROff = 0,
    LRClear = 1,        // no cars around us
    LRCarLeft = 2,      // there is a car to our left
    LRCarRight = 3,     // there is a car to our right
    LRCarLeftRight = 4, // there are cars on each side
    LR2CarsLeft = 5,    // there are two cars to our left
    LR2CarsRight = 6,   // there are two cars to our right
}

#[derive(Debug)]
pub enum CameraState {
    IsSessionScreen = 0x0001, // the camera tool can only be activated if viewing the session screen (out of car)
    IsScenicActive = 0x0002,  // the scenic camera is active (no focus car)

    // these can be changed with a broadcast message
    CamToolActive = 0x0004,
    UIHidden = 0x0008,
    UseAutoShotSelection = 0x0010,
    UseTemporaryEdits = 0x0020,
    UseKeyAcceleration = 0x0040,
    UseKey10xAcceleration = 0x0080,
    UseMouseAimMode = 0x0100,
}

#[derive(Debug)]
pub enum PitFlags {
    LFTireChange = 0x0001,
    RFTireChange = 0x0002,
    LRTireChange = 0x0004,
    RRTireChange = 0x0008,

    FuelFill = 0x0010,
    WindshieldTearoff = 0x0020,
    FastRepair = 0x0040,
}

#[derive(Debug)]
pub enum PitStatus {
    PitSvNone = 0,
    PitSvInProgress = 1,
    PitSvComplete = 2,

    // errors
    PitSvTooFarLeft = 100,
    PitSvTooFarRight = 101,
    PitSvTooFarForward = 102,
    PitSvTooFarBack = 103,
    PitSvBadAngle = 104,
    PitSvCantFixThat = 105,
}

#[derive(Debug)]
pub enum PaceMode {
    PaceModeSingleFileStart = 0,
    PaceModeDoubleFileStart = 1,
    PaceModeSingleFileRestart = 2,
    PaceModeDoubleFileRestart = 3,
    PaceModeNotPacing = 4,
}

#[derive(Debug)]
pub enum PaceFlags {
    PaceFlagsEndOfLine = 0x01,
    PaceFlagsFreePass = 0x02,
    PaceFlagsWavedAround = 0x04,
}
