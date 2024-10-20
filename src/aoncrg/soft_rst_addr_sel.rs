#[doc = "Register `soft_rst_addr_sel` reader"]
pub type R = crate::R<SoftRstAddrSelSpec>;
#[doc = "Register `soft_rst_addr_sel` writer"]
pub type W = crate::W<SoftRstAddrSelSpec>;
#[doc = "GMAC5 AXI64 AXI reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gmac5Axi64Axi {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<Gmac5Axi64Axi> for bool {
    #[inline(always)]
    fn from(variant: Gmac5Axi64Axi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `gmac5_axi64_axi` reader - GMAC5 AXI64 AXI reset."]
pub type Gmac5Axi64AxiR = crate::BitReader<Gmac5Axi64Axi>;
impl Gmac5Axi64AxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gmac5Axi64Axi {
        match self.bits {
            false => Gmac5Axi64Axi::None,
            true => Gmac5Axi64Axi::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Gmac5Axi64Axi::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gmac5Axi64Axi::Reset
    }
}
#[doc = "Field `gmac5_axi64_axi` writer - GMAC5 AXI64 AXI reset."]
pub type Gmac5Axi64AxiW<'a, REG> = crate::BitWriter<'a, REG, Gmac5Axi64Axi>;
impl<'a, REG> Gmac5Axi64AxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Gmac5Axi64Axi::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Gmac5Axi64Axi::Reset)
    }
}
#[doc = "GMAC5 AXI64 AHB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gmac5Axi64Ahb {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<Gmac5Axi64Ahb> for bool {
    #[inline(always)]
    fn from(variant: Gmac5Axi64Ahb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `gmac5_axi64_ahb` reader - GMAC5 AXI64 AHB reset."]
pub type Gmac5Axi64AhbR = crate::BitReader<Gmac5Axi64Ahb>;
impl Gmac5Axi64AhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gmac5Axi64Ahb {
        match self.bits {
            false => Gmac5Axi64Ahb::None,
            true => Gmac5Axi64Ahb::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Gmac5Axi64Ahb::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Gmac5Axi64Ahb::Reset
    }
}
#[doc = "Field `gmac5_axi64_ahb` writer - GMAC5 AXI64 AHB reset."]
pub type Gmac5Axi64AhbW<'a, REG> = crate::BitWriter<'a, REG, Gmac5Axi64Ahb>;
impl<'a, REG> Gmac5Axi64AhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Gmac5Axi64Ahb::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Gmac5Axi64Ahb::Reset)
    }
}
#[doc = "AON IOMUX Presetn reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AonIomuxPresetn {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<AonIomuxPresetn> for bool {
    #[inline(always)]
    fn from(variant: AonIomuxPresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `aon_iomux_presetn` reader - AON IOMUX Presetn reset."]
pub type AonIomuxPresetnR = crate::BitReader<AonIomuxPresetn>;
impl AonIomuxPresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AonIomuxPresetn {
        match self.bits {
            false => AonIomuxPresetn::None,
            true => AonIomuxPresetn::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AonIomuxPresetn::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AonIomuxPresetn::Reset
    }
}
#[doc = "Field `aon_iomux_presetn` writer - AON IOMUX Presetn reset."]
pub type AonIomuxPresetnW<'a, REG> = crate::BitWriter<'a, REG, AonIomuxPresetn>;
impl<'a, REG> AonIomuxPresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(AonIomuxPresetn::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(AonIomuxPresetn::Reset)
    }
}
#[doc = "PMU APB reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuApb {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<PmuApb> for bool {
    #[inline(always)]
    fn from(variant: PmuApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pmu_apb` reader - PMU APB reset."]
pub type PmuApbR = crate::BitReader<PmuApb>;
impl PmuApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuApb {
        match self.bits {
            false => PmuApb::None,
            true => PmuApb::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PmuApb::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PmuApb::Reset
    }
}
#[doc = "Field `pmu_apb` writer - PMU APB reset."]
pub type PmuApbW<'a, REG> = crate::BitWriter<'a, REG, PmuApb>;
impl<'a, REG> PmuApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PmuApb::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PmuApb::Reset)
    }
}
#[doc = "PMU Wake-up reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmuWkup {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<PmuWkup> for bool {
    #[inline(always)]
    fn from(variant: PmuWkup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `pmu_wkup` reader - PMU Wake-up reset."]
pub type PmuWkupR = crate::BitReader<PmuWkup>;
impl PmuWkupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PmuWkup {
        match self.bits {
            false => PmuWkup::None,
            true => PmuWkup::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PmuWkup::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PmuWkup::Reset
    }
}
#[doc = "Field `pmu_wkup` writer - PMU Wake-up reset."]
pub type PmuWkupW<'a, REG> = crate::BitWriter<'a, REG, PmuWkup>;
impl<'a, REG> PmuWkupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PmuWkup::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PmuWkup::Reset)
    }
}
#[doc = "RTC HMS APB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcHmsApb {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<RtcHmsApb> for bool {
    #[inline(always)]
    fn from(variant: RtcHmsApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rtc_hms_apb` reader - RTC HMS APB reset."]
pub type RtcHmsApbR = crate::BitReader<RtcHmsApb>;
impl RtcHmsApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcHmsApb {
        match self.bits {
            false => RtcHmsApb::None,
            true => RtcHmsApb::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RtcHmsApb::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RtcHmsApb::Reset
    }
}
#[doc = "Field `rtc_hms_apb` writer - RTC HMS APB reset."]
pub type RtcHmsApbW<'a, REG> = crate::BitWriter<'a, REG, RtcHmsApb>;
impl<'a, REG> RtcHmsApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHmsApb::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHmsApb::Reset)
    }
}
#[doc = "RTC HMS CAL reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcHmsCal {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<RtcHmsCal> for bool {
    #[inline(always)]
    fn from(variant: RtcHmsCal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rtc_hms_cal` reader - RTC HMS CAL reset."]
pub type RtcHmsCalR = crate::BitReader<RtcHmsCal>;
impl RtcHmsCalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcHmsCal {
        match self.bits {
            false => RtcHmsCal::None,
            true => RtcHmsCal::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RtcHmsCal::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RtcHmsCal::Reset
    }
}
#[doc = "Field `rtc_hms_cal` writer - RTC HMS CAL reset."]
pub type RtcHmsCalW<'a, REG> = crate::BitWriter<'a, REG, RtcHmsCal>;
impl<'a, REG> RtcHmsCalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHmsCal::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHmsCal::Reset)
    }
}
#[doc = "RTC HMS Oscillator 32k reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtcHmsOsc32k {
    #[doc = "0: De-assert the reset."]
    None = 0,
    #[doc = "1: Assert the reset."]
    Reset = 1,
}
impl From<RtcHmsOsc32k> for bool {
    #[inline(always)]
    fn from(variant: RtcHmsOsc32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rtc_hms_osc32k` reader - RTC HMS Oscillator 32k reset."]
pub type RtcHmsOsc32kR = crate::BitReader<RtcHmsOsc32k>;
impl RtcHmsOsc32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RtcHmsOsc32k {
        match self.bits {
            false => RtcHmsOsc32k::None,
            true => RtcHmsOsc32k::Reset,
        }
    }
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RtcHmsOsc32k::None
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RtcHmsOsc32k::Reset
    }
}
#[doc = "Field `rtc_hms_osc32k` writer - RTC HMS Oscillator 32k reset."]
pub type RtcHmsOsc32kW<'a, REG> = crate::BitWriter<'a, REG, RtcHmsOsc32k>;
impl<'a, REG> RtcHmsOsc32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert the reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHmsOsc32k::None)
    }
    #[doc = "Assert the reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(RtcHmsOsc32k::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - GMAC5 AXI64 AXI reset."]
    #[inline(always)]
    pub fn gmac5_axi64_axi(&self) -> Gmac5Axi64AxiR {
        Gmac5Axi64AxiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GMAC5 AXI64 AHB reset."]
    #[inline(always)]
    pub fn gmac5_axi64_ahb(&self) -> Gmac5Axi64AhbR {
        Gmac5Axi64AhbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AON IOMUX Presetn reset."]
    #[inline(always)]
    pub fn aon_iomux_presetn(&self) -> AonIomuxPresetnR {
        AonIomuxPresetnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMU APB reset."]
    #[inline(always)]
    pub fn pmu_apb(&self) -> PmuApbR {
        PmuApbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMU Wake-up reset."]
    #[inline(always)]
    pub fn pmu_wkup(&self) -> PmuWkupR {
        PmuWkupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC HMS APB reset."]
    #[inline(always)]
    pub fn rtc_hms_apb(&self) -> RtcHmsApbR {
        RtcHmsApbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTC HMS CAL reset."]
    #[inline(always)]
    pub fn rtc_hms_cal(&self) -> RtcHmsCalR {
        RtcHmsCalR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTC HMS Oscillator 32k reset."]
    #[inline(always)]
    pub fn rtc_hms_osc32k(&self) -> RtcHmsOsc32kR {
        RtcHmsOsc32kR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GMAC5 AXI64 AXI reset."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_axi(&mut self) -> Gmac5Axi64AxiW<SoftRstAddrSelSpec> {
        Gmac5Axi64AxiW::new(self, 0)
    }
    #[doc = "Bit 1 - GMAC5 AXI64 AHB reset."]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_ahb(&mut self) -> Gmac5Axi64AhbW<SoftRstAddrSelSpec> {
        Gmac5Axi64AhbW::new(self, 1)
    }
    #[doc = "Bit 2 - AON IOMUX Presetn reset."]
    #[inline(always)]
    #[must_use]
    pub fn aon_iomux_presetn(&mut self) -> AonIomuxPresetnW<SoftRstAddrSelSpec> {
        AonIomuxPresetnW::new(self, 2)
    }
    #[doc = "Bit 3 - PMU APB reset."]
    #[inline(always)]
    #[must_use]
    pub fn pmu_apb(&mut self) -> PmuApbW<SoftRstAddrSelSpec> {
        PmuApbW::new(self, 3)
    }
    #[doc = "Bit 4 - PMU Wake-up reset."]
    #[inline(always)]
    #[must_use]
    pub fn pmu_wkup(&mut self) -> PmuWkupW<SoftRstAddrSelSpec> {
        PmuWkupW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC HMS APB reset."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_apb(&mut self) -> RtcHmsApbW<SoftRstAddrSelSpec> {
        RtcHmsApbW::new(self, 5)
    }
    #[doc = "Bit 6 - RTC HMS CAL reset."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_cal(&mut self) -> RtcHmsCalW<SoftRstAddrSelSpec> {
        RtcHmsCalW::new(self, 6)
    }
    #[doc = "Bit 7 - RTC HMS Oscillator 32k reset."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hms_osc32k(&mut self) -> RtcHmsOsc32kW<SoftRstAddrSelSpec> {
        RtcHmsOsc32kW::new(self, 7)
    }
}
#[doc = "Software RESET Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftRstAddrSelSpec;
impl crate::RegisterSpec for SoftRstAddrSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soft_rst_addr_sel::R`](R) reader structure"]
impl crate::Readable for SoftRstAddrSelSpec {}
#[doc = "`write(|w| ..)` method takes [`soft_rst_addr_sel::W`](W) writer structure"]
impl crate::Writable for SoftRstAddrSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets soft_rst_addr_sel to value 0xe3"]
impl crate::Resettable for SoftRstAddrSelSpec {
    const RESET_VALUE: u32 = 0xe3;
}
