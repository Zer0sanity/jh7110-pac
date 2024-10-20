#[doc = "Register `aon_syscfg_0` reader"]
pub type R = crate::R<AonSyscfg0Spec>;
#[doc = "Register `aon_syscfg_0` writer"]
pub type W = crate::W<AonSyscfg0Spec>;
#[doc = "Field `aon_gp_reg` reader - "]
pub type AonGpRegR = crate::FieldReader<u32>;
#[doc = "Field `aon_gp_reg` writer - "]
pub type AonGpRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn aon_gp_reg(&self) -> AonGpRegR {
        AonGpRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn aon_gp_reg(&mut self) -> AonGpRegW<AonSyscfg0Spec> {
        AonGpRegW::new(self, 0)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg0Spec;
impl crate::RegisterSpec for AonSyscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_0::R`](R) reader structure"]
impl crate::Readable for AonSyscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`aon_syscfg_0::W`](W) writer structure"]
impl crate::Writable for AonSyscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets aon_syscfg_0 to value 0"]
impl crate::Resettable for AonSyscfg0Spec {
    const RESET_VALUE: u32 = 0;
}
