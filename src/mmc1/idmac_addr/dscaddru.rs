#[doc = "Register `dscaddru` reader"]
pub type R = crate::R<DscaddruSpec>;
#[doc = "Register `dscaddru` writer"]
pub type W = crate::W<DscaddruSpec>;
#[doc = "Field `dscaddru` reader - MMC Internal DMAC reserved / DSC address"]
pub type DscaddruR = crate::FieldReader<u32>;
#[doc = "Field `dscaddru` writer - MMC Internal DMAC reserved / DSC address"]
pub type DscaddruW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC reserved / DSC address"]
    #[inline(always)]
    pub fn dscaddru(&self) -> DscaddruR {
        DscaddruR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC reserved / DSC address"]
    #[inline(always)]
    #[must_use]
    pub fn dscaddru(&mut self) -> DscaddruW<DscaddruSpec> {
        DscaddruW::new(self, 0)
    }
}
#[doc = "MMC internal DMAC reserved / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address upper 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddru::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscaddru::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DscaddruSpec;
impl crate::RegisterSpec for DscaddruSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscaddru::R`](R) reader structure"]
impl crate::Readable for DscaddruSpec {}
#[doc = "`write(|w| ..)` method takes [`dscaddru::W`](W) writer structure"]
impl crate::Writable for DscaddruSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dscaddru to value 0"]
impl crate::Resettable for DscaddruSpec {
    const RESET_VALUE: u32 = 0;
}
