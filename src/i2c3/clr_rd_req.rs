#[doc = "Register `clr_rd_req` reader"]
pub type R = crate::R<ClrRdReqSpec>;
#[doc = "Register `clr_rd_req` writer"]
pub type W = crate::W<ClrRdReqSpec>;
#[doc = "Field `clr_rd_req` reader - "]
pub type ClrRdReqR = crate::FieldReader<u32>;
#[doc = "Field `clr_rd_req` writer - "]
pub type ClrRdReqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_rd_req(&self) -> ClrRdReqR {
        ClrRdReqR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_rd_req(&mut self) -> ClrRdReqW<ClrRdReqSpec> {
        ClrRdReqW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear Read Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rd_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rd_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRdReqSpec;
impl crate::RegisterSpec for ClrRdReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_rd_req::R`](R) reader structure"]
impl crate::Readable for ClrRdReqSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_rd_req::W`](W) writer structure"]
impl crate::Writable for ClrRdReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_rd_req to value 0"]
impl crate::Resettable for ClrRdReqSpec {
    const RESET_VALUE: u32 = 0;
}
