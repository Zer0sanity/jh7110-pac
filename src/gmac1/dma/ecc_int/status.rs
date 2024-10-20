#[doc = "Register `status` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `status` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `tceis` reader - MTL ECC TCE Interrupt Status - Write 1 to clear interrupt"]
pub type TceisR = crate::BitReader;
#[doc = "Field `tceis` writer - MTL ECC TCE Interrupt Status - Write 1 to clear interrupt"]
pub type TceisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MTL ECC TCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    pub fn tceis(&self) -> TceisR {
        TceisR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MTL ECC TCE Interrupt Status - Write 1 to clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tceis(&mut self) -> TceisW<StatusSpec> {
        TceisW::new(self, 0)
    }
}
#[doc = "DMA ECC Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
