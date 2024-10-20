#[doc = "Register `tx_debug` reader"]
pub type R = crate::R<TxDebugSpec>;
#[doc = "Register `tx_debug` writer"]
pub type W = crate::W<TxDebugSpec>;
#[doc = "Field `paused` reader - MTL TX Paused"]
pub type PausedR = crate::BitReader;
#[doc = "Field `paused` writer - MTL TX Paused"]
pub type PausedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MTL Debug TX FIFO Read Controller Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trcsts {
    #[doc = "0: MTL TX FIFO Idle"]
    Idle = 0,
    #[doc = "1: MTL TX FIFO Read"]
    Read = 1,
    #[doc = "2: MTL TX FIFO Wait"]
    Wait = 2,
    #[doc = "3: MTL TX FIFO Write"]
    Write = 3,
}
impl From<Trcsts> for u8 {
    #[inline(always)]
    fn from(variant: Trcsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trcsts {
    type Ux = u8;
}
#[doc = "Field `trcsts` reader - MTL Debug TX FIFO Read Controller Status"]
pub type TrcstsR = crate::FieldReader<Trcsts>;
impl TrcstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trcsts {
        match self.bits {
            0 => Trcsts::Idle,
            1 => Trcsts::Read,
            2 => Trcsts::Wait,
            3 => Trcsts::Write,
            _ => unreachable!(),
        }
    }
    #[doc = "MTL TX FIFO Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Trcsts::Idle
    }
    #[doc = "MTL TX FIFO Read"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Trcsts::Read
    }
    #[doc = "MTL TX FIFO Wait"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Trcsts::Wait
    }
    #[doc = "MTL TX FIFO Write"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Trcsts::Write
    }
}
#[doc = "Field `trcsts` writer - MTL Debug TX FIFO Read Controller Status"]
pub type TrcstsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Trcsts>;
impl<'a, REG> TrcstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MTL TX FIFO Idle"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Trcsts::Idle)
    }
    #[doc = "MTL TX FIFO Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Trcsts::Read)
    }
    #[doc = "MTL TX FIFO Wait"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Trcsts::Wait)
    }
    #[doc = "MTL TX FIFO Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Trcsts::Write)
    }
}
impl R {
    #[doc = "Bit 0 - MTL TX Paused"]
    #[inline(always)]
    pub fn paused(&self) -> PausedR {
        PausedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Debug TX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn trcsts(&self) -> TrcstsR {
        TrcstsR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MTL TX Paused"]
    #[inline(always)]
    #[must_use]
    pub fn paused(&mut self) -> PausedW<TxDebugSpec> {
        PausedW::new(self, 0)
    }
    #[doc = "Bits 1:2 - MTL Debug TX FIFO Read Controller Status"]
    #[inline(always)]
    #[must_use]
    pub fn trcsts(&mut self) -> TrcstsW<TxDebugSpec> {
        TrcstsW::new(self, 1)
    }
}
#[doc = "MTL TX Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDebugSpec;
impl crate::RegisterSpec for TxDebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_debug::R`](R) reader structure"]
impl crate::Readable for TxDebugSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_debug::W`](W) writer structure"]
impl crate::Writable for TxDebugSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_debug to value 0"]
impl crate::Resettable for TxDebugSpec {
    const RESET_VALUE: u32 = 0;
}
