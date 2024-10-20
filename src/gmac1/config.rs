#[doc = "Register `config` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `config` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `re` reader - Receive Enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `re` writer - Receive Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `te` reader - Transmit Enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `te` writer - Transmit Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dcrs` reader - DCRS"]
pub type DcrsR = crate::BitReader;
#[doc = "Field `dcrs` writer - DCRS"]
pub type DcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lm` reader - Loopback Mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `lm` writer - Loopback Mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dm` reader - Duplex Mode"]
pub type DmR = crate::BitReader;
#[doc = "Field `dm` writer - Duplex Mode"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Ethernet Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Speed {
    #[doc = "2: Speed 10 Mbits"]
    Speed10 = 2,
    #[doc = "3: Speed 100 Mbits"]
    Speed100 = 3,
    #[doc = "0: Speed 1000 Mbits"]
    Speed1000 = 0,
    #[doc = "1: Speed 2500 Mbits"]
    Speed2500 = 1,
}
impl From<Speed> for u8 {
    #[inline(always)]
    fn from(variant: Speed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Speed {
    type Ux = u8;
}
#[doc = "Field `speed` reader - Ethernet Speed"]
pub type SpeedR = crate::FieldReader<Speed>;
impl SpeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Speed {
        match self.bits {
            2 => Speed::Speed10,
            3 => Speed::Speed100,
            0 => Speed::Speed1000,
            1 => Speed::Speed2500,
            _ => unreachable!(),
        }
    }
    #[doc = "Speed 10 Mbits"]
    #[inline(always)]
    pub fn is_speed10(&self) -> bool {
        *self == Speed::Speed10
    }
    #[doc = "Speed 100 Mbits"]
    #[inline(always)]
    pub fn is_speed100(&self) -> bool {
        *self == Speed::Speed100
    }
    #[doc = "Speed 1000 Mbits"]
    #[inline(always)]
    pub fn is_speed1000(&self) -> bool {
        *self == Speed::Speed1000
    }
    #[doc = "Speed 2500 Mbits"]
    #[inline(always)]
    pub fn is_speed2500(&self) -> bool {
        *self == Speed::Speed2500
    }
}
#[doc = "Field `speed` writer - Ethernet Speed"]
pub type SpeedW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Speed>;
impl<'a, REG> SpeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Speed 10 Mbits"]
    #[inline(always)]
    pub fn speed10(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed10)
    }
    #[doc = "Speed 100 Mbits"]
    #[inline(always)]
    pub fn speed100(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed100)
    }
    #[doc = "Speed 1000 Mbits"]
    #[inline(always)]
    pub fn speed1000(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed1000)
    }
    #[doc = "Speed 2500 Mbits"]
    #[inline(always)]
    pub fn speed2500(self) -> &'a mut crate::W<REG> {
        self.variant(Speed::Speed2500)
    }
}
#[doc = "Field `je` reader - JE"]
pub type JeR = crate::BitReader;
#[doc = "Field `je` writer - JE"]
pub type JeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `jd` reader - JD"]
pub type JdR = crate::BitReader;
#[doc = "Field `jd` writer - JD"]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `be` reader - BE"]
pub type BeR = crate::BitReader;
#[doc = "Field `be` writer - BE"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `acs` reader - ACS"]
pub type AcsR = crate::BitReader;
#[doc = "Field `acs` writer - ACS"]
pub type AcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `p2k` reader - Packet 2KB"]
pub type P2kR = crate::BitReader;
#[doc = "Field `p2k` writer - Packet 2KB"]
pub type P2kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ipg` reader - IPG"]
pub type IpgR = crate::FieldReader;
#[doc = "Field `ipg` writer - IPG"]
pub type IpgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ipc` reader - IPC"]
pub type IpcR = crate::BitReader;
#[doc = "Field `ipc` writer - IPC"]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sarc` reader - SARC"]
pub type SarcR = crate::FieldReader;
#[doc = "Field `sarc` writer - SARC"]
pub type SarcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `arpen` reader - ARP Enable"]
pub type ArpenR = crate::BitReader;
#[doc = "Field `arpen` writer - ARP Enable"]
pub type ArpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - DCRS"]
    #[inline(always)]
    pub fn dcrs(&self) -> DcrsR {
        DcrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Duplex Mode"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Ethernet Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - JE"]
    #[inline(always)]
    pub fn je(&self) -> JeR {
        JeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - JD"]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ACS"]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Packet 2KB"]
    #[inline(always)]
    pub fn p2k(&self) -> P2kR {
        P2kR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - IPG"]
    #[inline(always)]
    pub fn ipg(&self) -> IpgR {
        IpgR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - IPC"]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SARC"]
    #[inline(always)]
    pub fn sarc(&self) -> SarcR {
        SarcR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - ARP Enable"]
    #[inline(always)]
    pub fn arpen(&self) -> ArpenR {
        ArpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<ConfigSpec> {
        ReW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<ConfigSpec> {
        TeW::new(self, 1)
    }
    #[doc = "Bit 9 - DCRS"]
    #[inline(always)]
    #[must_use]
    pub fn dcrs(&mut self) -> DcrsW<ConfigSpec> {
        DcrsW::new(self, 9)
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<ConfigSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - Duplex Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dm(&mut self) -> DmW<ConfigSpec> {
        DmW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Ethernet Speed"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<ConfigSpec> {
        SpeedW::new(self, 14)
    }
    #[doc = "Bit 16 - JE"]
    #[inline(always)]
    #[must_use]
    pub fn je(&mut self) -> JeW<ConfigSpec> {
        JeW::new(self, 16)
    }
    #[doc = "Bit 17 - JD"]
    #[inline(always)]
    #[must_use]
    pub fn jd(&mut self) -> JdW<ConfigSpec> {
        JdW::new(self, 17)
    }
    #[doc = "Bit 18 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<ConfigSpec> {
        BeW::new(self, 18)
    }
    #[doc = "Bit 20 - ACS"]
    #[inline(always)]
    #[must_use]
    pub fn acs(&mut self) -> AcsW<ConfigSpec> {
        AcsW::new(self, 20)
    }
    #[doc = "Bit 22 - Packet 2KB"]
    #[inline(always)]
    #[must_use]
    pub fn p2k(&mut self) -> P2kW<ConfigSpec> {
        P2kW::new(self, 22)
    }
    #[doc = "Bits 24:26 - IPG"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IpgW<ConfigSpec> {
        IpgW::new(self, 24)
    }
    #[doc = "Bit 27 - IPC"]
    #[inline(always)]
    #[must_use]
    pub fn ipc(&mut self) -> IpcW<ConfigSpec> {
        IpcW::new(self, 27)
    }
    #[doc = "Bits 28:30 - SARC"]
    #[inline(always)]
    #[must_use]
    pub fn sarc(&mut self) -> SarcW<ConfigSpec> {
        SarcW::new(self, 28)
    }
    #[doc = "Bit 31 - ARP Enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpen(&mut self) -> ArpenW<ConfigSpec> {
        ArpenW::new(self, 31)
    }
}
#[doc = "MAC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets config to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
