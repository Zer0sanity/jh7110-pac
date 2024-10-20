#[doc = "Register `func_sel6` reader"]
pub type R = crate::R<FuncSel6Spec>;
#[doc = "Register `func_sel6` writer"]
pub type W = crate::W<FuncSel6Spec>;
#[doc = "Field `vin_dvp_data5` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData5R = crate::FieldReader;
#[doc = "Field `vin_dvp_data5` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data6` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData6R = crate::FieldReader;
#[doc = "Field `vin_dvp_data6` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data7` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData7R = crate::FieldReader;
#[doc = "Field `vin_dvp_data7` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data8` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData8R = crate::FieldReader;
#[doc = "Field `vin_dvp_data8` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data9` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData9R = crate::FieldReader;
#[doc = "Field `vin_dvp_data9` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data10` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData10R = crate::FieldReader;
#[doc = "Field `vin_dvp_data10` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_hvalid` reader - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type VinDvpHvalidR = crate::FieldReader;
#[doc = "Field `vin_dvp_hvalid` writer - Function Selector of DVP_HSYNC, see Function 2 for more information"]
pub type VinDvpHvalidW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_vvalid` reader - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type VinDvpVvalidR = crate::FieldReader;
#[doc = "Field `vin_dvp_vvalid` writer - Function Selector of DVP_VSYNC, see Function 2 for more information"]
pub type VinDvpVvalidW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data5(&self) -> VinDvpData5R {
        VinDvpData5R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data6(&self) -> VinDvpData6R {
        VinDvpData6R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data7(&self) -> VinDvpData7R {
        VinDvpData7R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data8(&self) -> VinDvpData8R {
        VinDvpData8R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data9(&self) -> VinDvpData9R {
        VinDvpData9R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data10(&self) -> VinDvpData10R {
        VinDvpData10R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_hvalid(&self) -> VinDvpHvalidR {
        VinDvpHvalidR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_vvalid(&self) -> VinDvpVvalidR {
        VinDvpVvalidR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data5(&mut self) -> VinDvpData5W<FuncSel6Spec> {
        VinDvpData5W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data6(&mut self) -> VinDvpData6W<FuncSel6Spec> {
        VinDvpData6W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data7(&mut self) -> VinDvpData7W<FuncSel6Spec> {
        VinDvpData7W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data8(&mut self) -> VinDvpData8W<FuncSel6Spec> {
        VinDvpData8W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data9(&mut self) -> VinDvpData9W<FuncSel6Spec> {
        VinDvpData9W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data10(&mut self) -> VinDvpData10W<FuncSel6Spec> {
        VinDvpData10W::new(self, 15)
    }
    #[doc = "Bits 15:17 - Function Selector of DVP_HSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_hvalid(&mut self) -> VinDvpHvalidW<FuncSel6Spec> {
        VinDvpHvalidW::new(self, 15)
    }
    #[doc = "Bits 18:20 - Function Selector of DVP_VSYNC, see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_vvalid(&mut self) -> VinDvpVvalidW<FuncSel6Spec> {
        VinDvpVvalidW::new(self, 18)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncSel6Spec;
impl crate::RegisterSpec for FuncSel6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_sel6::R`](R) reader structure"]
impl crate::Readable for FuncSel6Spec {}
#[doc = "`write(|w| ..)` method takes [`func_sel6::W`](W) writer structure"]
impl crate::Writable for FuncSel6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets func_sel6 to value 0"]
impl crate::Resettable for FuncSel6Spec {
    const RESET_VALUE: u32 = 0;
}
