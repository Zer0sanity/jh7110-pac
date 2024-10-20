#[doc = "Register `rxp_ctrl_status` reader"]
pub type R = crate::R<RxpCtrlStatusSpec>;
#[doc = "Register `rxp_ctrl_status` writer"]
pub type W = crate::W<RxpCtrlStatusSpec>;
#[doc = "Field `nve` reader - MTL RXP NVE"]
pub type NveR = crate::FieldReader;
#[doc = "Field `nve` writer - MTL RXP NVE"]
pub type NveW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `npe` reader - MTL RXP NPE"]
pub type NpeR = crate::FieldReader;
#[doc = "Field `npe` writer - MTL RXP NPE"]
pub type NpeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rxpi` reader - MTL RXP Interrupt"]
pub type RxpiR = crate::BitReader;
#[doc = "Field `rxpi` writer - MTL RXP Interrupt"]
pub type RxpiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - MTL RXP NVE"]
    #[inline(always)]
    pub fn nve(&self) -> NveR {
        NveR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MTL RXP NPE"]
    #[inline(always)]
    pub fn npe(&self) -> NpeR {
        NpeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - MTL RXP Interrupt"]
    #[inline(always)]
    pub fn rxpi(&self) -> RxpiR {
        RxpiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - MTL RXP NVE"]
    #[inline(always)]
    #[must_use]
    pub fn nve(&mut self) -> NveW<RxpCtrlStatusSpec> {
        NveW::new(self, 0)
    }
    #[doc = "Bits 16:23 - MTL RXP NPE"]
    #[inline(always)]
    #[must_use]
    pub fn npe(&mut self) -> NpeW<RxpCtrlStatusSpec> {
        NpeW::new(self, 16)
    }
    #[doc = "Bit 31 - MTL RXP Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxpi(&mut self) -> RxpiW<RxpCtrlStatusSpec> {
        RxpiW::new(self, 31)
    }
}
#[doc = "MTL RXP Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxp_ctrl_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxp_ctrl_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxpCtrlStatusSpec;
impl crate::RegisterSpec for RxpCtrlStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxp_ctrl_status::R`](R) reader structure"]
impl crate::Readable for RxpCtrlStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`rxp_ctrl_status::W`](W) writer structure"]
impl crate::Writable for RxpCtrlStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxp_ctrl_status to value 0"]
impl crate::Resettable for RxpCtrlStatusSpec {
    const RESET_VALUE: u32 = 0;
}
