#[doc = "Register `uhs_reg_ext` reader"]
pub type R = crate::R<UhsRegExtSpec>;
#[doc = "Register `uhs_reg_ext` writer"]
pub type W = crate::W<UhsRegExtSpec>;
#[doc = "Field `smpl_phase` reader - MMC drive and sample phase"]
pub type SmplPhaseR = crate::FieldReader;
#[doc = "Field `smpl_phase` writer - MMC drive and sample phase"]
pub type SmplPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 16:20 - MMC drive and sample phase"]
    #[inline(always)]
    pub fn smpl_phase(&self) -> SmplPhaseR {
        SmplPhaseR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20 - MMC drive and sample phase"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_phase(&mut self) -> SmplPhaseW<UhsRegExtSpec> {
        SmplPhaseW::new(self, 16)
    }
}
#[doc = "MMC UHS regulator extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhs_reg_ext::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhs_reg_ext::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhsRegExtSpec;
impl crate::RegisterSpec for UhsRegExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs_reg_ext::R`](R) reader structure"]
impl crate::Readable for UhsRegExtSpec {}
#[doc = "`write(|w| ..)` method takes [`uhs_reg_ext::W`](W) writer structure"]
impl crate::Writable for UhsRegExtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets uhs_reg_ext to value 0"]
impl crate::Resettable for UhsRegExtSpec {
    const RESET_VALUE: u32 = 0;
}
