#[doc = "Register `func_sel5` reader"]
pub type R = crate::R<FuncSel5Spec>;
#[doc = "Register `func_sel5` writer"]
pub type W = crate::W<FuncSel5Spec>;
#[doc = "Field `pad_gpio6` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio6R = crate::FieldReader;
#[doc = "Field `pad_gpio6` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pad_gpio7` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio7R = crate::FieldReader;
#[doc = "Field `pad_gpio7` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pad_gpio8` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio8R = crate::FieldReader;
#[doc = "Field `pad_gpio8` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pad_gpio9` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio9R = crate::FieldReader;
#[doc = "Field `pad_gpio9` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `pad_gpio0` reader - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio0R = crate::FieldReader;
#[doc = "Field `pad_gpio0` writer - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
pub type PadGpio0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data10` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData10R = crate::FieldReader;
#[doc = "Field `vin_dvp_data10` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data11` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData11R = crate::FieldReader;
#[doc = "Field `vin_dvp_data11` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data1` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData1R = crate::FieldReader;
#[doc = "Field `vin_dvp_data1` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data2` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData2R = crate::FieldReader;
#[doc = "Field `vin_dvp_data2` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data3` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData3R = crate::FieldReader;
#[doc = "Field `vin_dvp_data3` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `vin_dvp_data4` reader - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData4R = crate::FieldReader;
#[doc = "Field `vin_dvp_data4` writer - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
pub type VinDvpData4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio6(&self) -> PadGpio6R {
        PadGpio6R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio7(&self) -> PadGpio7R {
        PadGpio7R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 4:6 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio8(&self) -> PadGpio8R {
        PadGpio8R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 6:8 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio9(&self) -> PadGpio9R {
        PadGpio9R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 8:10 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    pub fn pad_gpio0(&self) -> PadGpio0R {
        PadGpio0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data10(&self) -> VinDvpData10R {
        VinDvpData10R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data11(&self) -> VinDvpData11R {
        VinDvpData11R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data1(&self) -> VinDvpData1R {
        VinDvpData1R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data2(&self) -> VinDvpData2R {
        VinDvpData2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data3(&self) -> VinDvpData3R {
        VinDvpData3R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    pub fn vin_dvp_data4(&self) -> VinDvpData4R {
        VinDvpData4R::new(((self.bits >> 26) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio6(&mut self) -> PadGpio6W<FuncSel5Spec> {
        PadGpio6W::new(self, 0)
    }
    #[doc = "Bits 2:4 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio7(&mut self) -> PadGpio7W<FuncSel5Spec> {
        PadGpio7W::new(self, 2)
    }
    #[doc = "Bits 4:6 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio8(&mut self) -> PadGpio8W<FuncSel5Spec> {
        PadGpio8W::new(self, 4)
    }
    #[doc = "Bits 6:8 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio9(&mut self) -> PadGpio9W<FuncSel5Spec> {
        PadGpio9W::new(self, 6)
    }
    #[doc = "Bits 8:10 - GPIO function selector: * Function 0: See Function Description no page 84 for more information, * Function 1: See Full Multiplexing for more information, * Function 2: See Function 2 for more information, * Function 3: See Function 3 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn pad_gpio0(&mut self) -> PadGpio0W<FuncSel5Spec> {
        PadGpio0W::new(self, 8)
    }
    #[doc = "Bits 11:13 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data10(&mut self) -> VinDvpData10W<FuncSel5Spec> {
        VinDvpData10W::new(self, 11)
    }
    #[doc = "Bits 14:16 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data11(&mut self) -> VinDvpData11W<FuncSel5Spec> {
        VinDvpData11W::new(self, 14)
    }
    #[doc = "Bits 17:19 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data1(&mut self) -> VinDvpData1W<FuncSel5Spec> {
        VinDvpData1W::new(self, 17)
    }
    #[doc = "Bits 20:22 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data2(&mut self) -> VinDvpData2W<FuncSel5Spec> {
        VinDvpData2W::new(self, 20)
    }
    #[doc = "Bits 23:25 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data3(&mut self) -> VinDvpData3W<FuncSel5Spec> {
        VinDvpData3W::new(self, 23)
    }
    #[doc = "Bits 26:28 - Function Selector of DVP_DATA\\[idx\\], see Function 2 for more information"]
    #[inline(always)]
    #[must_use]
    pub fn vin_dvp_data4(&mut self) -> VinDvpData4W<FuncSel5Spec> {
        VinDvpData4W::new(self, 26)
    }
}
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FuncSel5Spec;
impl crate::RegisterSpec for FuncSel5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_sel5::R`](R) reader structure"]
impl crate::Readable for FuncSel5Spec {}
#[doc = "`write(|w| ..)` method takes [`func_sel5::W`](W) writer structure"]
impl crate::Writable for FuncSel5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets func_sel5 to value 0"]
impl crate::Resettable for FuncSel5Spec {
    const RESET_VALUE: u32 = 0;
}
