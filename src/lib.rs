#![no_std]
pub mod cmd;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::spi::FullDuplex;

pub struct DACxx6x<SPI, PIN> {
    spi: SPI,
    cs: PIN,
}

impl<SPI, PIN> DACxx6x<SPI, PIN>
where
    SPI: FullDuplex<u8>,
    PIN: OutputPin,
{
    pub fn new(spi: SPI, cs: PIN) -> Self {
        Self { spi, cs }
    }

    pub fn exec(&mut self, pkt: cmd::Packet) -> Result<(), SPI::Error> {
        self.cs.set_low();
        self.spi.send(pkt[0]);
        self.spi.send(pkt[1]);
        self.spi.send(pkt[2]);
        // self.cs.set_high().unwrap();
        self.cs.set_high();
        Ok(())
    }
    pub fn init(&mut self) -> Result<(), SPI::Error> {
        let pkt = cmd::CmdBuilder::new()
            .cmd(cmd::Command::Rst)
            .reset(cmd::ResetMode::All)
            .build();
        self.exec(pkt)?;
        Ok(())
    }
    pub fn set_power(
        &mut self,
        pwr_mode: cmd::PowerMode,
        selector: cmd::PowerSelector,
    ) -> Result<(), SPI::Error> {
        let pkt = cmd::CmdBuilder::new()
            .cmd(cmd::Command::Pwr)
            .power(pwr_mode, selector)
            .build();
        self.exec(pkt)?;
        Ok(())
    }
    pub fn write(&mut self, data: u16, target: cmd::ChAddr) -> Result<(), SPI::Error> {
        let pkt = cmd::CmdBuilder::new()
            .cmd(cmd::Command::WrN)
            .addr(target.into())
            .data(data)
            .build();
        self.exec(pkt)?;
        Ok(())
    }
    pub fn sw_ldac(&mut self) -> Result<(), SPI::Error> {
        let pkt = cmd::CmdBuilder::new()
            .cmd(cmd::Command::SwLDAC)
            .addr(cmd::Addr::CHAB)
            .build();
        self.exec(pkt)?;
        Ok(())
    }
}
