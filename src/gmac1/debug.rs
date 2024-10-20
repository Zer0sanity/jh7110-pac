#[doc = "Register `debug` reader"]
pub type R = crate::R<DebugSpec>;
#[doc = "Register `debug` writer"]
pub type W = crate::W<DebugSpec>;
#[doc = "Field `rpests` reader - Receive PE Status"]
pub type RpestsR = crate::BitReader;
#[doc = "Field `rpests` writer - Receive PE Status"]
pub type RpestsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rfcfcsts` reader - RFCFC Status"]
pub type RfcfcstsR = crate::FieldReader;
#[doc = "Field `rfcfcsts` writer - RFCFC Status"]
pub type RfcfcstsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `tpests` reader - Transmission PE Status"]
pub type TpestsR = crate::BitReader;
#[doc = "Field `tpests` writer - Transmission PE Status"]
pub type TpestsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmission Flow Control Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tfcsts {
    #[doc = "0: Transmission Flow Control Idle Status"]
    Idle = 0,
    #[doc = "1: Transmission Flow Control Wait Status"]
    Wait = 1,
    #[doc = "2: Transmission Flow Control Gen Pause Status"]
    GenPause = 2,
    #[doc = "3: Transmission Flow Control Transfer Status"]
    Xfer = 3,
}
impl From<Tfcsts> for u8 {
    #[inline(always)]
    fn from(variant: Tfcsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tfcsts {
    type Ux = u8;
}
#[doc = "Field `tfcsts` reader - Transmission Flow Control Status"]
pub type TfcstsR = crate::FieldReader<Tfcsts>;
impl TfcstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfcsts {
        match self.bits {
            0 => Tfcsts::Idle,
            1 => Tfcsts::Wait,
            2 => Tfcsts::GenPause,
            3 => Tfcsts::Xfer,
            _ => unreachable!(),
        }
    }
    #[doc = "Transmission Flow Control Idle Status"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Tfcsts::Idle
    }
    #[doc = "Transmission Flow Control Wait Status"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Tfcsts::Wait
    }
    #[doc = "Transmission Flow Control Gen Pause Status"]
    #[inline(always)]
    pub fn is_gen_pause(&self) -> bool {
        *self == Tfcsts::GenPause
    }
    #[doc = "Transmission Flow Control Transfer Status"]
    #[inline(always)]
    pub fn is_xfer(&self) -> bool {
        *self == Tfcsts::Xfer
    }
}
#[doc = "Field `tfcsts` writer - Transmission Flow Control Status"]
pub type TfcstsW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tfcsts>;
impl<'a, REG> TfcstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmission Flow Control Idle Status"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Tfcsts::Idle)
    }
    #[doc = "Transmission Flow Control Wait Status"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Tfcsts::Wait)
    }
    #[doc = "Transmission Flow Control Gen Pause Status"]
    #[inline(always)]
    pub fn gen_pause(self) -> &'a mut crate::W<REG> {
        self.variant(Tfcsts::GenPause)
    }
    #[doc = "Transmission Flow Control Transfer Status"]
    #[inline(always)]
    pub fn xfer(self) -> &'a mut crate::W<REG> {
        self.variant(Tfcsts::Xfer)
    }
}
impl R {
    #[doc = "Bit 0 - Receive PE Status"]
    #[inline(always)]
    pub fn rpests(&self) -> RpestsR {
        RpestsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RFCFC Status"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RfcfcstsR {
        RfcfcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - Transmission PE Status"]
    #[inline(always)]
    pub fn tpests(&self) -> TpestsR {
        TpestsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Transmission Flow Control Status"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TfcstsR {
        TfcstsR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Receive PE Status"]
    #[inline(always)]
    #[must_use]
    pub fn rpests(&mut self) -> RpestsW<DebugSpec> {
        RpestsW::new(self, 0)
    }
    #[doc = "Bits 1:2 - RFCFC Status"]
    #[inline(always)]
    #[must_use]
    pub fn rfcfcsts(&mut self) -> RfcfcstsW<DebugSpec> {
        RfcfcstsW::new(self, 1)
    }
    #[doc = "Bit 16 - Transmission PE Status"]
    #[inline(always)]
    #[must_use]
    pub fn tpests(&mut self) -> TpestsW<DebugSpec> {
        TpestsW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Transmission Flow Control Status"]
    #[inline(always)]
    #[must_use]
    pub fn tfcsts(&mut self) -> TfcstsW<DebugSpec> {
        TfcstsW::new(self, 17)
    }
}
#[doc = "MAC Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugSpec;
impl crate::RegisterSpec for DebugSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug::R`](R) reader structure"]
impl crate::Readable for DebugSpec {}
#[doc = "`write(|w| ..)` method takes [`debug::W`](W) writer structure"]
impl crate::Writable for DebugSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets debug to value 0"]
impl crate::Resettable for DebugSpec {
    const RESET_VALUE: u32 = 0;
}
