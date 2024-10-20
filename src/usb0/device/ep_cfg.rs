#[doc = "Register `ep_cfg` reader"]
pub type R = crate::R<EpCfgSpec>;
#[doc = "Register `ep_cfg` writer"]
pub type W = crate::W<EpCfgSpec>;
#[doc = "Field `enable` reader - Endpoint enable."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Endpoint enable."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "1: Isochronous endpoint"]
    Isochronous = 1,
    #[doc = "2: Bulk endpoint"]
    Bulk = 2,
    #[doc = "3: Interrupt endpoint"]
    Interrupt = 3,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
#[doc = "Field `eptype` reader - Endpoint type."]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eptype> {
        match self.bits {
            1 => Some(Eptype::Isochronous),
            2 => Some(Eptype::Bulk),
            3 => Some(Eptype::Interrupt),
            _ => None,
        }
    }
    #[doc = "Isochronous endpoint"]
    #[inline(always)]
    pub fn is_isochronous(&self) -> bool {
        *self == Eptype::Isochronous
    }
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn is_bulk(&self) -> bool {
        *self == Eptype::Bulk
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn is_interrupt(&self) -> bool {
        *self == Eptype::Interrupt
    }
}
#[doc = "Field `eptype` writer - Endpoint type."]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Eptype>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Isochronous endpoint"]
    #[inline(always)]
    pub fn isochronous(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Isochronous)
    }
    #[doc = "Bulk endpoint"]
    #[inline(always)]
    pub fn bulk(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Bulk)
    }
    #[doc = "Interrupt endpoint"]
    #[inline(always)]
    pub fn interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Interrupt)
    }
}
#[doc = "Field `stream_en` reader - Stream support enable - only in SS mode."]
pub type StreamEnR = crate::BitReader;
#[doc = "Field `stream_en` writer - Stream support enable - only in SS mode."]
pub type StreamEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tdl_chk` reader - TDL check - only in SS mode for BULK EP."]
pub type TdlChkR = crate::BitReader;
#[doc = "Field `tdl_chk` writer - TDL check - only in SS mode for BULK EP."]
pub type TdlChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sid_chk` reader - SID check - only in SS mode for BULK OUT EP."]
pub type SidChkR = crate::BitReader;
#[doc = "Field `sid_chk` writer - SID check - only in SS mode for BULK OUT EP."]
pub type SidChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ependian` reader - DMA transfer endianness."]
pub type EpendianR = crate::BitReader;
#[doc = "Field `ependian` writer - DMA transfer endianness."]
pub type EpendianW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `maxburst` reader - Max burst size - used only in SS mode."]
pub type MaxburstR = crate::FieldReader;
#[doc = "Field `maxburst` writer - Max burst size - used only in SS mode."]
pub type MaxburstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "ISO max burst size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mult {
    #[doc = "0: ISO burst: 0"]
    Mult0 = 0,
    #[doc = "1: ISO burst: 1"]
    Mult1 = 1,
    #[doc = "2: ISO burst: 2"]
    Mult2 = 2,
}
impl From<Mult> for u8 {
    #[inline(always)]
    fn from(variant: Mult) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mult {
    type Ux = u8;
}
#[doc = "Field `mult` reader - ISO max burst size."]
pub type MultR = crate::FieldReader<Mult>;
impl MultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mult> {
        match self.bits {
            0 => Some(Mult::Mult0),
            1 => Some(Mult::Mult1),
            2 => Some(Mult::Mult2),
            _ => None,
        }
    }
    #[doc = "ISO burst: 0"]
    #[inline(always)]
    pub fn is_mult0(&self) -> bool {
        *self == Mult::Mult0
    }
    #[doc = "ISO burst: 1"]
    #[inline(always)]
    pub fn is_mult1(&self) -> bool {
        *self == Mult::Mult1
    }
    #[doc = "ISO burst: 2"]
    #[inline(always)]
    pub fn is_mult2(&self) -> bool {
        *self == Mult::Mult2
    }
}
#[doc = "Field `mult` writer - ISO max burst size."]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mult>;
impl<'a, REG> MultW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ISO burst: 0"]
    #[inline(always)]
    pub fn mult0(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Mult0)
    }
    #[doc = "ISO burst: 1"]
    #[inline(always)]
    pub fn mult1(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Mult1)
    }
    #[doc = "ISO burst: 2"]
    #[inline(always)]
    pub fn mult2(self) -> &'a mut crate::W<REG> {
        self.variant(Mult::Mult2)
    }
}
#[doc = "Field `maxpktsize` reader - ISO max packet size."]
pub type MaxpktsizeR = crate::FieldReader<u16>;
#[doc = "Field `maxpktsize` writer - ISO max packet size."]
pub type MaxpktsizeW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `buffering` reader - Max number of buffered packets."]
pub type BufferingR = crate::FieldReader;
#[doc = "Field `buffering` writer - Max number of buffered packets."]
pub type BufferingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Endpoint enable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Endpoint type."]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Stream support enable - only in SS mode."]
    #[inline(always)]
    pub fn stream_en(&self) -> StreamEnR {
        StreamEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TDL check - only in SS mode for BULK EP."]
    #[inline(always)]
    pub fn tdl_chk(&self) -> TdlChkR {
        TdlChkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SID check - only in SS mode for BULK OUT EP."]
    #[inline(always)]
    pub fn sid_chk(&self) -> SidChkR {
        SidChkR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA transfer endianness."]
    #[inline(always)]
    pub fn ependian(&self) -> EpendianR {
        EpendianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Max burst size - used only in SS mode."]
    #[inline(always)]
    pub fn maxburst(&self) -> MaxburstR {
        MaxburstR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - ISO max burst size."]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:26 - ISO max packet size."]
    #[inline(always)]
    pub fn maxpktsize(&self) -> MaxpktsizeR {
        MaxpktsizeR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:30 - Max number of buffered packets."]
    #[inline(always)]
    pub fn buffering(&self) -> BufferingR {
        BufferingR::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint enable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<EpCfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Endpoint type."]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<EpCfgSpec> {
        EptypeW::new(self, 1)
    }
    #[doc = "Bit 3 - Stream support enable - only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn stream_en(&mut self) -> StreamEnW<EpCfgSpec> {
        StreamEnW::new(self, 3)
    }
    #[doc = "Bit 4 - TDL check - only in SS mode for BULK EP."]
    #[inline(always)]
    #[must_use]
    pub fn tdl_chk(&mut self) -> TdlChkW<EpCfgSpec> {
        TdlChkW::new(self, 4)
    }
    #[doc = "Bit 5 - SID check - only in SS mode for BULK OUT EP."]
    #[inline(always)]
    #[must_use]
    pub fn sid_chk(&mut self) -> SidChkW<EpCfgSpec> {
        SidChkW::new(self, 5)
    }
    #[doc = "Bit 7 - DMA transfer endianness."]
    #[inline(always)]
    #[must_use]
    pub fn ependian(&mut self) -> EpendianW<EpCfgSpec> {
        EpendianW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Max burst size - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn maxburst(&mut self) -> MaxburstW<EpCfgSpec> {
        MaxburstW::new(self, 8)
    }
    #[doc = "Bits 14:15 - ISO max burst size."]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MultW<EpCfgSpec> {
        MultW::new(self, 14)
    }
    #[doc = "Bits 16:26 - ISO max packet size."]
    #[inline(always)]
    #[must_use]
    pub fn maxpktsize(&mut self) -> MaxpktsizeW<EpCfgSpec> {
        MaxpktsizeW::new(self, 16)
    }
    #[doc = "Bits 27:30 - Max number of buffered packets."]
    #[inline(always)]
    #[must_use]
    pub fn buffering(&mut self) -> BufferingW<EpCfgSpec> {
        BufferingW::new(self, 27)
    }
}
#[doc = "USB3 Endpoint configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpCfgSpec;
impl crate::RegisterSpec for EpCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_cfg::R`](R) reader structure"]
impl crate::Readable for EpCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_cfg::W`](W) writer structure"]
impl crate::Writable for EpCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ep_cfg to value 0"]
impl crate::Resettable for EpCfgSpec {
    const RESET_VALUE: u32 = 0;
}
