#[doc = "Register `syscfg_xcfgi[%s]` reader"]
pub type R = crate::R<SyscfgXcfgiSpec>;
#[doc = "Register `syscfg_xcfgi[%s]` writer"]
pub type W = crate::W<SyscfgXcfgiSpec>;
#[doc = "Field `dw` reader - XCFGI DW: u0_mipitx_dphy_XCFGI_DW"]
pub type DwR = crate::FieldReader<u32>;
#[doc = "Field `dw` writer - XCFGI DW: u0_mipitx_dphy_XCFGI_DW"]
pub type DwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - XCFGI DW: u0_mipitx_dphy_XCFGI_DW"]
    #[inline(always)]
    pub fn dw(&self) -> DwR {
        DwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - XCFGI DW: u0_mipitx_dphy_XCFGI_DW"]
    #[inline(always)]
    #[must_use]
    pub fn dw(&mut self) -> DwW<SyscfgXcfgiSpec> {
        DwW::new(self, 0)
    }
}
#[doc = "MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_xcfgi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_xcfgi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyscfgXcfgiSpec;
impl crate::RegisterSpec for SyscfgXcfgiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_xcfgi::R`](R) reader structure"]
impl crate::Readable for SyscfgXcfgiSpec {}
#[doc = "`write(|w| ..)` method takes [`syscfg_xcfgi::W`](W) writer structure"]
impl crate::Writable for SyscfgXcfgiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg_xcfgi[%s]
to value 0"]
impl crate::Resettable for SyscfgXcfgiSpec {
    const RESET_VALUE: u32 = 0;
}
