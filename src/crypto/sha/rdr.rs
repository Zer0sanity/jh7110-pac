#[doc = "Register `rdr` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Register `rdr` writer"]
pub type W = crate::W<RdrSpec>;
#[doc = "Field `rdr` reader - SHA RDR"]
pub type RdrR = crate::FieldReader<u32>;
#[doc = "Field `rdr` writer - SHA RDR"]
pub type RdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA RDR"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA RDR"]
    #[inline(always)]
    #[must_use]
    pub fn rdr(&mut self) -> RdrW<RdrSpec> {
        RdrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto SHA RDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`write(|w| ..)` method takes [`rdr::W`](W) writer structure"]
impl crate::Writable for RdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rdr to value 0"]
impl crate::Resettable for RdrSpec {
    const RESET_VALUE: u32 = 0;
}
