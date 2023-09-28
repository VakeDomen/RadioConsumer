pub enum Radio<'a> {
    Shoutcast(ShoutcastConfig<'a>),
    WssMetadataRadio(WSSConfig<'a>)
}

pub type WSSConfig<'a> = (&'a str, &'a str);
pub type ShoutcastConfig<'a> = (&'a str, &'a str, u16, &'a str);


