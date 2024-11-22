pub enum Command {
    WrN,
    SwLDAC,
    WrUpdAll,
    WrUpdN,
    Pwr,
    Rst,
    LDAC,
    Ref,
}

pub enum Addr {
    CHA = 0,
    CHB = 1,
    Gain = 2,
    CHAB = 7,
}

pub enum ChAddr {
    CHA = 0,
    CHB = 1,
    CHAB = 7,
}
impl Into<Addr> for ChAddr {
    fn into(self) -> Addr {
        match self {
            ChAddr::CHA => Addr::CHA,
            ChAddr::CHB => Addr::CHB,
            ChAddr::CHAB => Addr::CHAB,
        }
    }
}

pub enum GainMode {
    A2B2,
    A1B2,
    A2B1,
    A1B1,
}

pub enum PowerMode {
    Up,
    Down1k,
    Down100k,
    DownHighZ,
}

pub enum PowerSelector {
    A = 1,
    B = 2,
    AB = 3,
}

pub enum ResetMode {
    InputOnly,
    All,
}

pub enum LDACMode {
    All,
    ChB,
    ChA,
    None,
}
pub enum RefMode {
    Disable,
    Enable,
}

pub type Packet = [u8; 3];

pub struct CmdBuilder {
    val: Packet,
}

impl CmdBuilder {
    pub fn new() -> Self {
        Self { val: [0; 3] }
    }
    pub fn build(&self) -> Packet {
        self.val
    }
    pub fn cmd(&mut self, cmd: Command) -> &mut Self {
        self.val[0] |= ((cmd as u8) << 3) & 0b0011_1000;
        self
    }
    pub fn addr(&mut self, addr: Addr) -> &mut Self {
        self.val[0] |= addr as u8;
        self
    }
    pub fn data(&mut self, data: u16) -> &mut Self {
        self.val[1] = (data >> 8) as u8;
        self.val[2] = data as u8;
        self
    }
    pub fn gain(&mut self, gain: GainMode) -> &mut Self {
        self.val[2] = gain as u8;
        self
    }
    pub fn power(&mut self, pwr_mode: PowerMode, selector: PowerSelector) -> &mut Self {
        self.val[2] = (pwr_mode as u8) << 4 | selector as u8;
        self
    }

    pub fn reset(&mut self, rst: ResetMode) -> &mut Self {
        self.val[2] = rst as u8;
        self
    }
    pub fn ldac(&mut self, ldac_mode: LDACMode) -> &mut Self {
        self.val[2] = ldac_mode as u8;
        self
    }

    pub fn reference(&mut self, ref_mode: RefMode) -> &mut Self {
        self.val[2] = ref_mode as u8;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn test_cmd_builder() {
        let pkt = CmdBuilder::new()
            .cmd(Command::WrN)
            .addr(Addr::CHAB)
            .data(0x1234)
            .build();
        assert_eq!(pkt, [0b0000_0111, 0x12, 0x34]);
    }
    #[test]
    fn test_sw_ldac() {
        let pkt = CmdBuilder::new()
            .cmd(Command::SwLDAC)
            .addr(Addr::CHAB)
            .build();
        assert_eq!(pkt, [0b0000_1111, 0, 0]);
    }

    #[test]
    fn test_gain() {
        let pkt = CmdBuilder::new()
            .cmd(Command::WrN)
            .addr(Addr::Gain)
            .gain(GainMode::A1B2)
            .build();
        assert_eq!(pkt, [0b00_000_010, 0, 0b01]);
    }

    #[test]
    fn test_power() {
        let pkt = CmdBuilder::new()
            .cmd(Command::Pwr)
            .power(PowerMode::Up, PowerSelector::AB)
            .build();
        assert_eq!(pkt, [0b00_100_000, 0, 0b0000_0011]);

        let pkt = CmdBuilder::new()
            .cmd(Command::Pwr)
            .power(PowerMode::Down100k, PowerSelector::AB)
            .build();
        assert_eq!(pkt, [0b00_100_000, 0, 0b0010_0011]);
    }

    #[test]
    fn test_ldac() {
        let pkt = CmdBuilder::new()
            .cmd(Command::LDAC)
            .ldac(LDACMode::All)
            .build();
        assert_eq!(pkt, [0b00_110_000, 0, 0b0000_0000]);
        let pkt = CmdBuilder::new()
            .cmd(Command::LDAC)
            .ldac(LDACMode::None)
            .build();
        assert_eq!(pkt, [0b00_110_000, 0, 0b0000_0011]);
    }

    #[test]
    fn test_ref() {
        let pkt = CmdBuilder::new()
            .cmd(Command::Ref)
            .reference(RefMode::Enable)
            .build();
        assert_eq!(pkt, [0b00_111_000, 0, 0b0000_0001]);
        let pkt = CmdBuilder::new()
            .cmd(Command::Ref)
            .reference(RefMode::Disable)
            .build();
        assert_eq!(pkt, [0b00_111_000, 0, 0b0000_0000]);
    }
}
