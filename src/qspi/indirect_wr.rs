#[doc = "Register `indirect_wr` reader"]
pub type R = crate::R<IndirectWrSpec>;
#[doc = "Register `indirect_wr` writer"]
pub type W = crate::W<IndirectWrSpec>;
#[doc = "Field `start` reader - Start indirect write"]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - Start indirect write"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cancel` reader - Cancel indirect write"]
pub type CancelR = crate::BitReader;
#[doc = "Field `cancel` writer - Cancel indirect write"]
pub type CancelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `done` reader - Indirect write done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `done` writer - Indirect write done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start indirect write"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancel indirect write"]
    #[inline(always)]
    pub fn cancel(&self) -> CancelR {
        CancelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect write done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start indirect write"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<IndirectWrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Cancel indirect write"]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CancelW<IndirectWrSpec> {
        CancelW::new(self, 1)
    }
    #[doc = "Bit 5 - Indirect write done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IndirectWrSpec> {
        DoneW::new(self, 5)
    }
}
#[doc = "Cadence QSPI Indirect Write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectWrSpec;
impl crate::RegisterSpec for IndirectWrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirect_wr::R`](R) reader structure"]
impl crate::Readable for IndirectWrSpec {}
#[doc = "`write(|w| ..)` method takes [`indirect_wr::W`](W) writer structure"]
impl crate::Writable for IndirectWrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets indirect_wr to value 0"]
impl crate::Resettable for IndirectWrSpec {
    const RESET_VALUE: u32 = 0;
}
