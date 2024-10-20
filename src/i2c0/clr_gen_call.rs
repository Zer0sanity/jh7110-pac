#[doc = "Register `clr_gen_call` reader"]
pub type R = crate::R<ClrGenCallSpec>;
#[doc = "Register `clr_gen_call` writer"]
pub type W = crate::W<ClrGenCallSpec>;
#[doc = "Field `clr_gen_call` reader - "]
pub type ClrGenCallR = crate::FieldReader<u32>;
#[doc = "Field `clr_gen_call` writer - "]
pub type ClrGenCallW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_gen_call(&self) -> ClrGenCallR {
        ClrGenCallR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_gen_call(&mut self) -> ClrGenCallW<ClrGenCallSpec> {
        ClrGenCallW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear General Call\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_gen_call::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_gen_call::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrGenCallSpec;
impl crate::RegisterSpec for ClrGenCallSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_gen_call::R`](R) reader structure"]
impl crate::Readable for ClrGenCallSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_gen_call::W`](W) writer structure"]
impl crate::Writable for ClrGenCallSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_gen_call to value 0"]
impl crate::Resettable for ClrGenCallSpec {
    const RESET_VALUE: u32 = 0;
}
