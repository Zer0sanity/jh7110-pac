#[doc = "Register `rx_debug` reader"]
pub type R = crate::R<RxDebugSpec>;
#[doc = "Register `rx_debug` writer"]
pub type W = crate::W<RxDebugSpec>;
#[doc = "Field `rwcsts` reader - MTL RX Rreceive Write Controller Status"]
pub type RwcstsR = crate::BitReader;
#[doc = "Field `rwcsts` writer - MTL RX Rreceive Write Controller Status"]
pub type RwcstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MTL Debug RX FIFO Read Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rrcsts {
    #[doc = "0: MTL RX Controller Idle"]
    Idle = 0,
    #[doc = "1: MTL RX Controller Read Data"]
    Rdata = 1,
    #[doc = "2: MTL RX Controller Read Status"]
    Rstat = 2,
    #[doc = "3: MTL RX Controller Flush"]
    Flush = 3,
}
impl From<Rrcsts> for u8 {
    #[inline(always)]
    fn from(variant: Rrcsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rrcsts {
    type Ux = u8;
}
#[doc = "Field `rrcsts` reader - MTL Debug RX FIFO Read Controller Status"]
pub type RrcstsR = crate::FieldReader<Rrcsts>;
impl RrcstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrcsts {
        match self.bits {
            0 => Rrcsts::Idle,
            1 => Rrcsts::Rdata,
            2 => Rrcsts::Rstat,
            3 => Rrcsts::Flush,
            _ => unreachable!(),
        }
    }
    #[doc = "MTL RX Controller Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Rrcsts::Idle
    }
    #[doc = "MTL RX Controller Read Data"]
    #[inline(always)]
    pub fn is_rdata(&self) -> bool {
        *self == Rrcsts::Rdata
    }
    #[doc = "MTL RX Controller Read Status"]
    #[inline(always)]
    pub fn is_rstat(&self) -> bool {
        *self == Rrcsts::Rstat
    }
    #[doc = "MTL RX Controller Flush"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == Rrcsts::Flush
    }
}
#[doc = "Field `rrcsts` writer - MTL Debug RX FIFO Read Controller Status"]
pub type RrcstsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rrcsts>;
impl<'a, REG> RrcstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MTL RX Controller Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Rrcsts::Idle)
    }
    #[doc = "MTL RX Controller Read Data"]
    #[inline(always)]
    pub fn rdata(self) -> &'a mut crate::W<REG> {
        self.variant(Rrcsts::Rdata)
    }
    #[doc = "MTL RX Controller Read Status"]
    #[inline(always)]
    pub fn rstat(self) -> &'a mut crate::W<REG> {
        self.variant(Rrcsts::Rstat)
    }
    #[doc = "MTL RX Controller Flush"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut crate::W<REG> {
        self.variant(Rrcsts::Flush)
    }
}
#[doc = "MTL Debug RX FIFO Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxfsts {
    #[doc = "0: MTL RX FIFO Empty"]
    Empty = 0,
    #[doc = "1: MTL RX FIFO Below Threshold"]
    Bt = 1,
    #[doc = "2: MTL RX FIFO At/Above Threshold"]
    At = 2,
    #[doc = "3: MTL RX FIFO Full"]
    Full = 3,
}
impl From<Rxfsts> for u8 {
    #[inline(always)]
    fn from(variant: Rxfsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxfsts {
    type Ux = u8;
}
#[doc = "Field `rxfsts` reader - MTL Debug RX FIFO Status"]
pub type RxfstsR = crate::FieldReader<Rxfsts>;
impl RxfstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfsts {
        match self.bits {
            0 => Rxfsts::Empty,
            1 => Rxfsts::Bt,
            2 => Rxfsts::At,
            3 => Rxfsts::Full,
            _ => unreachable!(),
        }
    }
    #[doc = "MTL RX FIFO Empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rxfsts::Empty
    }
    #[doc = "MTL RX FIFO Below Threshold"]
    #[inline(always)]
    pub fn is_bt(&self) -> bool {
        *self == Rxfsts::Bt
    }
    #[doc = "MTL RX FIFO At/Above Threshold"]
    #[inline(always)]
    pub fn is_at(&self) -> bool {
        *self == Rxfsts::At
    }
    #[doc = "MTL RX FIFO Full"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Rxfsts::Full
    }
}
#[doc = "Field `rxfsts` writer - MTL Debug RX FIFO Status"]
pub type RxfstsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rxfsts>;
impl<'a, REG> RxfstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MTL RX FIFO Empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfsts::Empty)
    }
    #[doc = "MTL RX FIFO Below Threshold"]
    #[inline(always)]
    pub fn bt(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfsts::Bt)
    }
    #[doc = "MTL RX FIFO At/Above Threshold"]
    #[inline(always)]
    pub fn at(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfsts::At)
    }
    #[doc = "MTL RX FIFO Full"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfsts::Full)
    }
}
impl R {
    #[doc = "Bit 0 - MTL RX Rreceive Write Controller Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RwcstsR {
        RwcstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Debug RX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RrcstsR {
        RrcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Debug RX FIFO Status"]
    #[inline(always)]
    pub fn rxfsts(&self) -> RxfstsR {
        RxfstsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MTL RX Rreceive Write Controller Status"]
    #[inline(always)]
    #[must_use]
    pub fn rwcsts(&mut self) -> RwcstsW<RxDebugSpec> {
        RwcstsW::new(self, 0)
    }
    #[doc = "Bits 1:2 - MTL Debug RX FIFO Read Controller Status"]
    #[inline(always)]
    #[must_use]
    pub fn rrcsts(&mut self) -> RrcstsW<RxDebugSpec> {
        RrcstsW::new(self, 1)
    }
    #[doc = "Bits 4:5 - MTL Debug RX FIFO Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxfsts(&mut self) -> RxfstsW<RxDebugSpec> {
        RxfstsW::new(self, 4)
    }
}
#[doc = "MTL RX Debug - GMII or MII Transmit Protocol Engine Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDebugSpec;
impl crate::RegisterSpec for RxDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_debug::R`](R) reader structure"]
impl crate::Readable for RxDebugSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_debug::W`](W) writer structure"]
impl crate::Writable for RxDebugSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_debug to value 0"]
impl crate::Resettable for RxDebugSpec {
    const RESET_VALUE: u32 = 0;
}
