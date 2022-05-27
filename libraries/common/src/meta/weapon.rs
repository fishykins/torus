pub enum WeaponClass {
    Sniper,
    Assault,
    Carbine,
    Pistol,
    Shotgun,
    Launcher,
    Smg,
    Hmg,
    Pdw,
    Hybrid,
}

pub enum WeaponPropulsion {
    Powder,
    Plasma,
    Laser,
    Magnetic,
    Hybrid,
}

pub enum WeaponSlot {
    Receiver,
    Barrel,
    Stock,
    Grip,
}

pub struct WeaponSystem {
    pub nickname: String,
    pub maker: u16,
    pub model: u16,
    pub variant: Option<char>,
    pub class: WeaponClass,
    pub propulsion: WeaponPropulsion,
    pub description: String,
}