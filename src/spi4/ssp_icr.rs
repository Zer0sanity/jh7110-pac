#[doc = "Register `ssp_icr` reader"]
pub type R = crate::R<SspIcrSpec>;
#[doc = "Register `ssp_icr` writer"]
pub type W = crate::W<SspIcrSpec>;
#[doc = "Field `roric` reader - Clears the SSPRORINTR interrupt"]
pub type RoricR = crate::BitReader;
#[doc = "Field `roric` writer - Clears the SSPRORINTR interrupt"]
pub type RoricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rtic` reader - Clears the SSPRTINTR interrupt"]
pub type RticR = crate::BitReader;
#[doc = "Field `rtic` writer - Clears the SSPRTINTR interrupt"]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RoricR {
        RoricR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RticR {
        RticR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RoricW<SspIcrSpec> {
        RoricW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<SspIcrSpec> {
        RticW::new(self, 1)
    }
}
#[doc = "The SSPICR register is the interrupt clear register and is write-only. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspIcrSpec;
impl crate::RegisterSpec for SspIcrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_icr::R`](R) reader structure"]
impl crate::Readable for SspIcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_icr::W`](W) writer structure"]
impl crate::Writable for SspIcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_icr to value 0"]
impl crate::Resettable for SspIcrSpec {
    const RESET_VALUE: u16 = 0;
}
