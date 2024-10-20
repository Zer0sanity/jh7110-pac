#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `setup` reader - Setup transfer complete."]
pub type SetupR = crate::BitReader;
#[doc = "Field `setup` writer - Setup transfer complete."]
pub type SetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stall` reader - Endpoint STALL status."]
pub type StallR = crate::BitReader;
#[doc = "Field `stall` writer - Endpoint STALL status."]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ioc` reader - Interrupt On Complete."]
pub type IocR = crate::BitReader;
#[doc = "Field `ioc` writer - Interrupt On Complete."]
pub type IocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isp` reader - Interrupt on Short Packet."]
pub type IspR = crate::BitReader;
#[doc = "Field `isp` writer - Interrupt on Short Packet."]
pub type IspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `descmis` reader - Transfer desccriptor missing."]
pub type DescmisR = crate::BitReader;
#[doc = "Field `descmis` writer - Transfer desccriptor missing."]
pub type DescmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `streamr` reader - Stream Rejected - used only in SS mode."]
pub type StreamrR = crate::BitReader;
#[doc = "Field `streamr` writer - Stream Rejected - used only in SS mode."]
pub type StreamrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `md_exit` reader - EXIT from MOVE DATA State - used only for stream transfers in SS mode."]
pub type MdExitR = crate::BitReader;
#[doc = "Field `md_exit` writer - EXIT from MOVE DATA State - used only for stream transfers in SS mode."]
pub type MdExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `trberr` reader - TRB error."]
pub type TrberrR = crate::BitReader;
#[doc = "Field `trberr` writer - TRB error."]
pub type TrberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nrdy` reader - Not Ready - used only in SS mode."]
pub type NrdyR = crate::BitReader;
#[doc = "Field `nrdy` writer - Not Ready - used only in SS mode."]
pub type NrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dbusy` reader - DMA busy."]
pub type DbusyR = crate::BitReader;
#[doc = "Field `dbusy` writer - DMA busy."]
pub type DbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `buffempty` reader - Endpoint Buffer Empty."]
pub type BuffemptyR = crate::BitReader;
#[doc = "Field `buffempty` writer - Endpoint Buffer Empty."]
pub type BuffemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ccs` reader - Current Cycle Status."]
pub type CcsR = crate::BitReader;
#[doc = "Field `ccs` writer - Current Cycle Status."]
pub type CcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prime` reader - Prime - used only in SS mode."]
pub type PrimeR = crate::BitReader;
#[doc = "Field `prime` writer - Prime - used only in SS mode."]
pub type PrimeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `siderr` reader - Stream Error - used only in SS mode."]
pub type SiderrR = crate::BitReader;
#[doc = "Field `siderr` writer - Stream Error - used only in SS mode."]
pub type SiderrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `outsmm` reader - OUT size mismatch."]
pub type OutsmmR = crate::BitReader;
#[doc = "Field `outsmm` writer - OUT size mismatch."]
pub type OutsmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `isoerr` reader - ISO transmission error."]
pub type IsoerrR = crate::BitReader;
#[doc = "Field `isoerr` writer - ISO transmission error."]
pub type IsoerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hostpp` reader - Host Packet Pending - used only in SS mode."]
pub type HostppR = crate::BitReader;
#[doc = "Field `hostpp` writer - Host Packet Pending - used only in SS mode."]
pub type HostppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Stream Protocol State Machine State - used only for BULK stream endpoints.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spsmst {
    #[doc = "0: Stream State Machine: DISABLED."]
    Disabled = 0,
    #[doc = "1: Stream State Machine: IDLE."]
    Idle = 1,
    #[doc = "2: Stream State Machine: START STREAM."]
    StartStream = 2,
    #[doc = "3: Stream State Machine: MOVE DATA."]
    MoveData = 3,
}
impl From<Spsmst> for u8 {
    #[inline(always)]
    fn from(variant: Spsmst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spsmst {
    type Ux = u8;
}
#[doc = "Field `spsmst` reader - Stream Protocol State Machine State - used only for BULK stream endpoints."]
pub type SpsmstR = crate::FieldReader<Spsmst>;
impl SpsmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spsmst {
        match self.bits {
            0 => Spsmst::Disabled,
            1 => Spsmst::Idle,
            2 => Spsmst::StartStream,
            3 => Spsmst::MoveData,
            _ => unreachable!(),
        }
    }
    #[doc = "Stream State Machine: DISABLED."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Spsmst::Disabled
    }
    #[doc = "Stream State Machine: IDLE."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Spsmst::Idle
    }
    #[doc = "Stream State Machine: START STREAM."]
    #[inline(always)]
    pub fn is_start_stream(&self) -> bool {
        *self == Spsmst::StartStream
    }
    #[doc = "Stream State Machine: MOVE DATA."]
    #[inline(always)]
    pub fn is_move_data(&self) -> bool {
        *self == Spsmst::MoveData
    }
}
#[doc = "Field `spsmst` writer - Stream Protocol State Machine State - used only for BULK stream endpoints."]
pub type SpsmstW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Spsmst>;
impl<'a, REG> SpsmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stream State Machine: DISABLED."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Spsmst::Disabled)
    }
    #[doc = "Stream State Machine: IDLE."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Spsmst::Idle)
    }
    #[doc = "Stream State Machine: START STREAM."]
    #[inline(always)]
    pub fn start_stream(self) -> &'a mut crate::W<REG> {
        self.variant(Spsmst::StartStream)
    }
    #[doc = "Stream State Machine: MOVE DATA."]
    #[inline(always)]
    pub fn move_data(self) -> &'a mut crate::W<REG> {
        self.variant(Spsmst::MoveData)
    }
}
#[doc = "Field `iot` reader - Interrupt On Transfer complete."]
pub type IotR = crate::BitReader;
#[doc = "Field `iot` writer - Interrupt On Transfer complete."]
pub type IotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `outq_no` reader - OUT queue endpoint number."]
pub type OutqNoR = crate::FieldReader;
#[doc = "Field `outq_no` writer - OUT queue endpoint number."]
pub type OutqNoW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "OUT queue valid flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutqVal {
    #[doc = "0: OUT queue invalid."]
    Invalid = 0,
    #[doc = "1: OUT queue valid."]
    Valid = 1,
}
impl From<OutqVal> for bool {
    #[inline(always)]
    fn from(variant: OutqVal) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `outq_val` reader - OUT queue valid flag."]
pub type OutqValR = crate::BitReader<OutqVal>;
impl OutqValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutqVal {
        match self.bits {
            false => OutqVal::Invalid,
            true => OutqVal::Valid,
        }
    }
    #[doc = "OUT queue invalid."]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == OutqVal::Invalid
    }
    #[doc = "OUT queue valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == OutqVal::Valid
    }
}
#[doc = "Field `outq_val` writer - OUT queue valid flag."]
pub type OutqValW<'a, REG> = crate::BitWriter<'a, REG, OutqVal>;
impl<'a, REG> OutqValW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OUT queue invalid."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(OutqVal::Invalid)
    }
    #[doc = "OUT queue valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(OutqVal::Valid)
    }
}
#[doc = "Field `stpwait` reader - SETUP WAIT."]
pub type StpwaitR = crate::BitReader;
#[doc = "Field `stpwait` writer - SETUP WAIT."]
pub type StpwaitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setup transfer complete."]
    #[inline(always)]
    pub fn setup(&self) -> SetupR {
        SetupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint STALL status."]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt On Complete."]
    #[inline(always)]
    pub fn ioc(&self) -> IocR {
        IocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on Short Packet."]
    #[inline(always)]
    pub fn isp(&self) -> IspR {
        IspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer desccriptor missing."]
    #[inline(always)]
    pub fn descmis(&self) -> DescmisR {
        DescmisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream Rejected - used only in SS mode."]
    #[inline(always)]
    pub fn streamr(&self) -> StreamrR {
        StreamrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EXIT from MOVE DATA State - used only for stream transfers in SS mode."]
    #[inline(always)]
    pub fn md_exit(&self) -> MdExitR {
        MdExitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TRB error."]
    #[inline(always)]
    pub fn trberr(&self) -> TrberrR {
        TrberrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Ready - used only in SS mode."]
    #[inline(always)]
    pub fn nrdy(&self) -> NrdyR {
        NrdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA busy."]
    #[inline(always)]
    pub fn dbusy(&self) -> DbusyR {
        DbusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint Buffer Empty."]
    #[inline(always)]
    pub fn buffempty(&self) -> BuffemptyR {
        BuffemptyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Current Cycle Status."]
    #[inline(always)]
    pub fn ccs(&self) -> CcsR {
        CcsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Prime - used only in SS mode."]
    #[inline(always)]
    pub fn prime(&self) -> PrimeR {
        PrimeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Stream Error - used only in SS mode."]
    #[inline(always)]
    pub fn siderr(&self) -> SiderrR {
        SiderrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OUT size mismatch."]
    #[inline(always)]
    pub fn outsmm(&self) -> OutsmmR {
        OutsmmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ISO transmission error."]
    #[inline(always)]
    pub fn isoerr(&self) -> IsoerrR {
        IsoerrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Host Packet Pending - used only in SS mode."]
    #[inline(always)]
    pub fn hostpp(&self) -> HostppR {
        HostppR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Stream Protocol State Machine State - used only for BULK stream endpoints."]
    #[inline(always)]
    pub fn spsmst(&self) -> SpsmstR {
        SpsmstR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Interrupt On Transfer complete."]
    #[inline(always)]
    pub fn iot(&self) -> IotR {
        IotR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:27 - OUT queue endpoint number."]
    #[inline(always)]
    pub fn outq_no(&self) -> OutqNoR {
        OutqNoR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - OUT queue valid flag."]
    #[inline(always)]
    pub fn outq_val(&self) -> OutqValR {
        OutqValR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - SETUP WAIT."]
    #[inline(always)]
    pub fn stpwait(&self) -> StpwaitR {
        StpwaitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setup transfer complete."]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SetupW<StatusSpec> {
        SetupW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint STALL status."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<StatusSpec> {
        StallW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt On Complete."]
    #[inline(always)]
    #[must_use]
    pub fn ioc(&mut self) -> IocW<StatusSpec> {
        IocW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt on Short Packet."]
    #[inline(always)]
    #[must_use]
    pub fn isp(&mut self) -> IspW<StatusSpec> {
        IspW::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer desccriptor missing."]
    #[inline(always)]
    #[must_use]
    pub fn descmis(&mut self) -> DescmisW<StatusSpec> {
        DescmisW::new(self, 4)
    }
    #[doc = "Bit 5 - Stream Rejected - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn streamr(&mut self) -> StreamrW<StatusSpec> {
        StreamrW::new(self, 5)
    }
    #[doc = "Bit 6 - EXIT from MOVE DATA State - used only for stream transfers in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn md_exit(&mut self) -> MdExitW<StatusSpec> {
        MdExitW::new(self, 6)
    }
    #[doc = "Bit 7 - TRB error."]
    #[inline(always)]
    #[must_use]
    pub fn trberr(&mut self) -> TrberrW<StatusSpec> {
        TrberrW::new(self, 7)
    }
    #[doc = "Bit 8 - Not Ready - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn nrdy(&mut self) -> NrdyW<StatusSpec> {
        NrdyW::new(self, 8)
    }
    #[doc = "Bit 9 - DMA busy."]
    #[inline(always)]
    #[must_use]
    pub fn dbusy(&mut self) -> DbusyW<StatusSpec> {
        DbusyW::new(self, 9)
    }
    #[doc = "Bit 10 - Endpoint Buffer Empty."]
    #[inline(always)]
    #[must_use]
    pub fn buffempty(&mut self) -> BuffemptyW<StatusSpec> {
        BuffemptyW::new(self, 10)
    }
    #[doc = "Bit 11 - Current Cycle Status."]
    #[inline(always)]
    #[must_use]
    pub fn ccs(&mut self) -> CcsW<StatusSpec> {
        CcsW::new(self, 11)
    }
    #[doc = "Bit 12 - Prime - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn prime(&mut self) -> PrimeW<StatusSpec> {
        PrimeW::new(self, 12)
    }
    #[doc = "Bit 13 - Stream Error - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn siderr(&mut self) -> SiderrW<StatusSpec> {
        SiderrW::new(self, 13)
    }
    #[doc = "Bit 14 - OUT size mismatch."]
    #[inline(always)]
    #[must_use]
    pub fn outsmm(&mut self) -> OutsmmW<StatusSpec> {
        OutsmmW::new(self, 14)
    }
    #[doc = "Bit 15 - ISO transmission error."]
    #[inline(always)]
    #[must_use]
    pub fn isoerr(&mut self) -> IsoerrW<StatusSpec> {
        IsoerrW::new(self, 15)
    }
    #[doc = "Bit 16 - Host Packet Pending - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn hostpp(&mut self) -> HostppW<StatusSpec> {
        HostppW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Stream Protocol State Machine State - used only for BULK stream endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn spsmst(&mut self) -> SpsmstW<StatusSpec> {
        SpsmstW::new(self, 17)
    }
    #[doc = "Bit 19 - Interrupt On Transfer complete."]
    #[inline(always)]
    #[must_use]
    pub fn iot(&mut self) -> IotW<StatusSpec> {
        IotW::new(self, 19)
    }
    #[doc = "Bits 24:27 - OUT queue endpoint number."]
    #[inline(always)]
    #[must_use]
    pub fn outq_no(&mut self) -> OutqNoW<StatusSpec> {
        OutqNoW::new(self, 24)
    }
    #[doc = "Bit 28 - OUT queue valid flag."]
    #[inline(always)]
    #[must_use]
    pub fn outq_val(&mut self) -> OutqValW<StatusSpec> {
        OutqValW::new(self, 28)
    }
    #[doc = "Bit 31 - SETUP WAIT."]
    #[inline(always)]
    #[must_use]
    pub fn stpwait(&mut self) -> StpwaitW<StatusSpec> {
        StpwaitW::new(self, 31)
    }
}
#[doc = "USB3 Endpoint status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets status to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
