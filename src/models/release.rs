use strum_macros::EnumString;

#[derive(EnumString)]
#[allow(non_camel_case_types)]
pub enum Format {
    #[strum(serialize = "FLAC")]
    Flac,
    #[strum(serialize = "V0")]
    Mp3_V0,
    #[strum(serialize = "V2")]
    Mp3_V2,
    #[strum(serialize = "320")]
    Mp3_320,
}

#[derive(EnumString)]
pub enum Source {
    #[strum(serialize = "WEB")]
    Web,
    #[strum(serialize = "Vinyl")]
    Vinyl,
    #[strum(serialize = "CD")]
    Cd,
}

pub struct Release {
    pub artist: String,
    pub release_name: String,
    pub release_id: String,
    pub year: usize,
    pub format: Format,
    pub source: Source,
}
