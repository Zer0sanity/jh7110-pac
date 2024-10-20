#[doc = "Register `usb_conf2` reader"]
pub type R = crate::R<UsbConf2Spec>;
#[doc = "Register `usb_conf2` writer"]
pub type W = crate::W<UsbConf2Spec>;
#[doc = "Field `tdl_trb(_dis,_en)` reader - TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
pub type TdlTrbR = crate::BitReader;
#[doc = "Field `tdl_trb(_dis,_en)` writer - TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
pub type TdlTrbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `tdl_trb_dis` field"]
    #[inline(always)]
    pub fn tdl_trb(&self, n: u8) -> TdlTrbR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TdlTrbR::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[inline(always)]
    pub fn tdl_trb_iter(&self) -> impl Iterator<Item = TdlTrbR> + '_ {
        (0..2).map(move |n| TdlTrbR::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[inline(always)]
    pub fn tdl_trb_dis(&self) -> TdlTrbR {
        TdlTrbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[inline(always)]
    pub fn tdl_trb_en(&self) -> TdlTrbR {
        TdlTrbR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `tdl_trb_dis` field"]
    #[inline(always)]
    #[must_use]
    pub fn tdl_trb(&mut self, n: u8) -> TdlTrbW<UsbConf2Spec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TdlTrbW::new(self, n + 1)
    }
    #[doc = "Bit 1 - TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn tdl_trb_dis(&mut self) -> TdlTrbW<UsbConf2Spec> {
        TdlTrbW::new(self, 1)
    }
    #[doc = "Bit 2 - TDL calculation based on TRB feature in controller for DMULT mode, only supported in DEV_VER_V2 version - tdl_trb0: disable, tdl_trb1: enable."]
    #[inline(always)]
    #[must_use]
    pub fn tdl_trb_en(&mut self) -> TdlTrbW<UsbConf2Spec> {
        TdlTrbW::new(self, 2)
    }
}
#[doc = "USB3 Global configurartion 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbConf2Spec;
impl crate::RegisterSpec for UsbConf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_conf2::R`](R) reader structure"]
impl crate::Readable for UsbConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`usb_conf2::W`](W) writer structure"]
impl crate::Writable for UsbConf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_conf2 to value 0"]
impl crate::Resettable for UsbConf2Spec {
    const RESET_VALUE: u32 = 0;
}
