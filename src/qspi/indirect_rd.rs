#[doc = "Register `indirect_rd` reader"]
pub type R = crate::R<IndirectRdSpec>;
#[doc = "Register `indirect_rd` writer"]
pub type W = crate::W<IndirectRdSpec>;
#[doc = "Field `start` reader - Start indirect read"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - Start indirect read"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cancel` reader - Cancel indirect read"]
pub type CancelR = crate::BitReader;
#[doc = "Field `cancel` writer - Cancel indirect read"]
pub type CancelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `done` reader - Indirect read done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `done` writer - Indirect read done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start indirect read"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancel indirect read"]
    #[inline(always)]
    pub fn cancel(&self) -> CancelR {
        CancelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect read done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start indirect read"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IndirectRdSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Cancel indirect read"]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CancelW<IndirectRdSpec> {
        CancelW::new(self, 1)
    }
    #[doc = "Bit 5 - Indirect read done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IndirectRdSpec> {
        DoneW::new(self, 5)
    }
}
#[doc = "Cadence QSPI Indirect Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectRdSpec;
impl crate::RegisterSpec for IndirectRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_rd::R`](R) reader structure"]
impl crate::Readable for IndirectRdSpec {}
#[doc = "`write(|w| ..)` method takes [`indirect_rd::W`](W) writer structure"]
impl crate::Writable for IndirectRdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets indirect_rd to value 0"]
impl crate::Resettable for IndirectRdSpec {
    const RESET_VALUE: u32 = 0;
}
