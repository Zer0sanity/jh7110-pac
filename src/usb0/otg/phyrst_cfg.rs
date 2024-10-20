#[doc = "Register `phyrst_cfg` reader"]
pub type R = crate::R<PhyrstCfgSpec>;
#[doc = "Register `phyrst_cfg` writer"]
pub type W = crate::W<PhyrstCfgSpec>;
#[doc = "Field `a_enable` reader - USB3 OTG PHY A-device enable."]
pub type AEnableR = crate::BitReader;
#[doc = "Field `a_enable` writer - USB3 OTG PHY A-device enable."]
pub type AEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB3 OTG PHY A-device enable."]
    #[inline(always)]
    pub fn a_enable(&self) -> AEnableR {
        AEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB3 OTG PHY A-device enable."]
    #[inline(always)]
    #[must_use]
    pub fn a_enable(&mut self) -> AEnableW<PhyrstCfgSpec> {
        AEnableW::new(self, 0)
    }
}
#[doc = "USB3 OTG PHY reset configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyrst_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phyrst_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyrstCfgSpec;
impl crate::RegisterSpec for PhyrstCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phyrst_cfg::R`](R) reader structure"]
impl crate::Readable for PhyrstCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`phyrst_cfg::W`](W) writer structure"]
impl crate::Writable for PhyrstCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets phyrst_cfg to value 0"]
impl crate::Resettable for PhyrstCfgSpec {
    const RESET_VALUE: u32 = 0;
}
