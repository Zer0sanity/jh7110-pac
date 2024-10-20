#[doc = "Register `rstn` reader"]
pub type R = crate::R<RstnSpec>;
#[doc = "Register `rstn` writer"]
pub type W = crate::W<RstnSpec>;
#[doc = "Field `smt` reader - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
pub type SmtR = crate::BitReader;
#[doc = "Field `smt` writer - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
pub type SmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pos` reader - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
pub type PosR = crate::BitReader;
#[doc = "Field `pos` writer - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
pub type PosW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
    #[inline(always)]
    pub fn smt(&self) -> SmtR {
        SmtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active high Schmitt (SMT) trigger selector - 0: No hysteresis, 1: Schmitt trigger enabled"]
    #[inline(always)]
    #[must_use]
    pub fn smt(&mut self) -> SmtW<RstnSpec> {
        SmtW::new(self, 0)
    }
    #[doc = "Bit 1 - Power-on-Start (POS) enabler - 1: Enable active pull-down for loss of core power, 0: Active pull-down capability disabled"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> PosW<RstnSpec> {
        PosW::new(self, 1)
    }
}
#[doc = "AON IOMUX CFG SAIF SYSCFG 68 - RSTN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstnSpec;
impl crate::RegisterSpec for RstnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstn::R`](R) reader structure"]
impl crate::Readable for RstnSpec {}
#[doc = "`write(|w| ..)` method takes [`rstn::W`](W) writer structure"]
impl crate::Writable for RstnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rstn to value 0x01"]
impl crate::Resettable for RstnSpec {
    const RESET_VALUE: u32 = 0x01;
}
