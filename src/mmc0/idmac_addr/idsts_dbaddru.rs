#[doc = "Register `idsts_dbaddru` reader"]
pub type R = crate::R<IdstsDbaddruSpec>;
#[doc = "Register `idsts_dbaddru` writer"]
pub type W = crate::W<IdstsDbaddruSpec>;
#[doc = "Field `idsts_dbaddru` reader - MMC Internal DMAC status / DB address"]
pub type IdstsDbaddruR = crate::FieldReader<u32>;
#[doc = "Field `idsts_dbaddru` writer - MMC Internal DMAC status / DB address"]
pub type IdstsDbaddruW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Internal DMAC status / DB address"]
    #[inline(always)]
    pub fn idsts_dbaddru(&self) -> IdstsDbaddruR {
        IdstsDbaddruR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Internal DMAC status / DB address"]
    #[inline(always)]
    #[must_use]
    pub fn idsts_dbaddru(&mut self) -> IdstsDbaddruW<IdstsDbaddruSpec> {
        IdstsDbaddruW::new(self, 0)
    }
}
#[doc = "MMC internal DMAC status / DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): status, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address upper 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idsts_dbaddru::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idsts_dbaddru::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdstsDbaddruSpec;
impl crate::RegisterSpec for IdstsDbaddruSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idsts_dbaddru::R`](R) reader structure"]
impl crate::Readable for IdstsDbaddruSpec {}
#[doc = "`write(|w| ..)` method takes [`idsts_dbaddru::W`](W) writer structure"]
impl crate::Writable for IdstsDbaddruSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets idsts_dbaddru to value 0"]
impl crate::Resettable for IdstsDbaddruSpec {
    const RESET_VALUE: u32 = 0;
}
