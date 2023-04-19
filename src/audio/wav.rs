// temporary location for wav file implementation. will be used with cpal implementation of audio engine

pub struct WAVFile {
    path: String,
    data: Block,
    channels: u32,
    bitrate: u32,
    bps: u32,
}

impl WAVFile {
    pub fn from(path: String) -> Result<WAVFile, Box<dyn std::error::Error>> {
        let mut ret: WAVFile = WAVFile {
            path: String::new(),
            data: Block::empty(),
            channels: 0,
            bitrate: 0,
            bps: 0,
        };
        ret.path = path;
        ret.data = Block::from_file(&mut File::open(Path::new(&ret.path))?)?;

        // NOTE: .wav files are little endian
        // TODO: big endian support

        // Checks:
        // bytes 1-4 MUST spell out string "RIFF"
        unsafe {
            if ret.data.get::<[u8;4]>(0)? != &[0x52, 0x49, 0x46, 0x46] {
                return Err(Box::new(ResourceError::new("Invalid WAVFile".to_owned())))
            }
        }
        // bytes 9-12 MUST spell out "WAVE"
        unsafe {
            if ret.data.get::<[u8;4]>(8)? != &[0x57, 0x41, 0x56, 0x45] {
                return Err(Box::new(ResourceError::new("Invalid WAVFile".to_owned())))
            }
        }

        // Data:
        // bytes 23-24 determine the number of channels
        ret.channels = unsafe { *ret.data.get::<u16>(22)? as u32 };
        // bytes 25-28 determine the bitrate
        ret.bitrate = unsafe { *ret.data.get::<u32>(24)? };
        // bytes 35-36 determine the bits per sample
        ret.bps = unsafe { *ret.data.get::<u16>(34)? as u32 };
        // bytes 45 and on are the audio data: truncate ret.data
        ret.data.remove_bytes(0, 44)?;

        Ok(ret)
    }
}
