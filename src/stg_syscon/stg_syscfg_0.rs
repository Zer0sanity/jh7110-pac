#[doc = "Register `stg_syscfg_0` reader"]
pub type R = crate::R<StgSyscfg0Spec>;
#[doc = "Register `stg_syscfg_0` writer"]
pub type W = crate::W<StgSyscfg0Spec>;
#[doc = "Field `scfg_hprot_sd0` reader - "]
pub type ScfgHprotSd0R = crate::FieldReader;
#[doc = "Field `scfg_hprot_sd0` writer - "]
pub type ScfgHprotSd0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `scfg_hprot_sd1` reader - "]
pub type ScfgHprotSd1R = crate::FieldReader;
#[doc = "Field `scfg_hprot_sd1` writer - "]
pub type ScfgHprotSd1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `u0_usb_adp_en` reader - "]
pub type U0UsbAdpEnR = crate::BitReader;
#[doc = "Field `u0_usb_adp_probe_ana` reader - "]
pub type U0UsbAdpProbeAnaR = crate::BitReader;
#[doc = "Field `u0_usb_adp_probe_ana` writer - "]
pub type U0UsbAdpProbeAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_adp_probe_en` reader - "]
pub type U0UsbAdpProbeEnR = crate::BitReader;
#[doc = "Field `u0_usb_adp_sense_ana` reader - "]
pub type U0UsbAdpSenseAnaR = crate::BitReader;
#[doc = "Field `u0_usb_adp_sense_ana` writer - "]
pub type U0UsbAdpSenseAnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_adp_sense_en` reader - "]
pub type U0UsbAdpSenseEnR = crate::BitReader;
#[doc = "Field `u0_usb_adp_sink_current_en` reader - "]
pub type U0UsbAdpSinkCurrentEnR = crate::BitReader;
#[doc = "Field `u0_usb_adp_source_current_en` reader - "]
pub type U0UsbAdpSourceCurrentEnR = crate::BitReader;
#[doc = "Field `u0_usb_bc_en` reader - "]
pub type U0UsbBcEnR = crate::BitReader;
#[doc = "Field `u0_usb_chrg_vbus` reader - "]
pub type U0UsbChrgVbusR = crate::BitReader;
#[doc = "Field `u0_usb_chrg_vbus` writer - "]
pub type U0UsbChrgVbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_dcd_comp_sts` reader - "]
pub type U0UsbDcdCompStsR = crate::BitReader;
#[doc = "Field `u0_usb_dcd_comp_sts` writer - "]
pub type U0UsbDcdCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_dischrg_vbus` reader - "]
pub type U0UsbDischrgVbusR = crate::BitReader;
#[doc = "Field `u0_usb_dischrg_vbus` writer - "]
pub type U0UsbDischrgVbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_dm_vdat_ref_comp_en` reader - "]
pub type U0UsbDmVdatRefCompEnR = crate::BitReader;
#[doc = "Field `u0_usb_dm_vdat_ref_comp_sts` reader - "]
pub type U0UsbDmVdatRefCompStsR = crate::BitReader;
#[doc = "Field `u0_usb_dm_vdat_ref_comp_sts` writer - "]
pub type U0UsbDmVdatRefCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_dm_vlgc_comp_en` reader - "]
pub type U0UsbDmVlgcCompEnR = crate::BitReader;
#[doc = "Field `u0_usb_dm_vlgc_comp_sts` reader - "]
pub type U0UsbDmVlgcCompStsR = crate::BitReader;
#[doc = "Field `u0_usb_dm_vlgc_comp_sts` writer - "]
pub type U0UsbDmVlgcCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_dp_vdat_ref_comp_en` reader - "]
pub type U0UsbDpVdatRefCompEnR = crate::BitReader;
#[doc = "Field `u0_usb_dp_vdat_ref_comp_sts` reader - "]
pub type U0UsbDpVdatRefCompStsR = crate::BitReader;
#[doc = "Field `u0_usb_dp_vdat_ref_comp_sts` writer - "]
pub type U0UsbDpVdatRefCompStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_host_system_err` reader - "]
pub type U0UsbHostSystemErrR = crate::BitReader;
#[doc = "Field `u0_usb_host_system_err` writer - "]
pub type U0UsbHostSystemErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_hsystem_err_ext` reader - "]
pub type U0UsbHsystemErrExtR = crate::BitReader;
#[doc = "Field `u0_usb_idm_sink_en` reader - "]
pub type U0UsbIdmSinkEnR = crate::BitReader;
#[doc = "Field `u0_usb_idp_sink_en` reader - "]
pub type U0UsbIdpSinkEnR = crate::BitReader;
#[doc = "Field `u0_usb_idp_src_en` reader - "]
pub type U0UsbIdpSrcEnR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn scfg_hprot_sd0(&self) -> ScfgHprotSd0R {
        ScfgHprotSd0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn scfg_hprot_sd1(&self) -> ScfgHprotSd1R {
        ScfgHprotSd1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u0_usb_adp_en(&self) -> U0UsbAdpEnR {
        U0UsbAdpEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u0_usb_adp_probe_ana(&self) -> U0UsbAdpProbeAnaR {
        U0UsbAdpProbeAnaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn u0_usb_adp_probe_en(&self) -> U0UsbAdpProbeEnR {
        U0UsbAdpProbeEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn u0_usb_adp_sense_ana(&self) -> U0UsbAdpSenseAnaR {
        U0UsbAdpSenseAnaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn u0_usb_adp_sense_en(&self) -> U0UsbAdpSenseEnR {
        U0UsbAdpSenseEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn u0_usb_adp_sink_current_en(&self) -> U0UsbAdpSinkCurrentEnR {
        U0UsbAdpSinkCurrentEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn u0_usb_adp_source_current_en(&self) -> U0UsbAdpSourceCurrentEnR {
        U0UsbAdpSourceCurrentEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn u0_usb_bc_en(&self) -> U0UsbBcEnR {
        U0UsbBcEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn u0_usb_chrg_vbus(&self) -> U0UsbChrgVbusR {
        U0UsbChrgVbusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn u0_usb_dcd_comp_sts(&self) -> U0UsbDcdCompStsR {
        U0UsbDcdCompStsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn u0_usb_dischrg_vbus(&self) -> U0UsbDischrgVbusR {
        U0UsbDischrgVbusR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn u0_usb_dm_vdat_ref_comp_en(&self) -> U0UsbDmVdatRefCompEnR {
        U0UsbDmVdatRefCompEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn u0_usb_dm_vdat_ref_comp_sts(&self) -> U0UsbDmVdatRefCompStsR {
        U0UsbDmVdatRefCompStsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u0_usb_dm_vlgc_comp_en(&self) -> U0UsbDmVlgcCompEnR {
        U0UsbDmVlgcCompEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u0_usb_dm_vlgc_comp_sts(&self) -> U0UsbDmVlgcCompStsR {
        U0UsbDmVlgcCompStsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u0_usb_dp_vdat_ref_comp_en(&self) -> U0UsbDpVdatRefCompEnR {
        U0UsbDpVdatRefCompEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn u0_usb_dp_vdat_ref_comp_sts(&self) -> U0UsbDpVdatRefCompStsR {
        U0UsbDpVdatRefCompStsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn u0_usb_host_system_err(&self) -> U0UsbHostSystemErrR {
        U0UsbHostSystemErrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn u0_usb_hsystem_err_ext(&self) -> U0UsbHsystemErrExtR {
        U0UsbHsystemErrExtR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn u0_usb_idm_sink_en(&self) -> U0UsbIdmSinkEnR {
        U0UsbIdmSinkEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn u0_usb_idp_sink_en(&self) -> U0UsbIdpSinkEnR {
        U0UsbIdpSinkEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn u0_usb_idp_src_en(&self) -> U0UsbIdpSrcEnR {
        U0UsbIdpSrcEnR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hprot_sd0(&mut self) -> ScfgHprotSd0W<StgSyscfg0Spec> {
        ScfgHprotSd0W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_hprot_sd1(&mut self) -> ScfgHprotSd1W<StgSyscfg0Spec> {
        ScfgHprotSd1W::new(self, 4)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_adp_probe_ana(&mut self) -> U0UsbAdpProbeAnaW<StgSyscfg0Spec> {
        U0UsbAdpProbeAnaW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_adp_sense_ana(&mut self) -> U0UsbAdpSenseAnaW<StgSyscfg0Spec> {
        U0UsbAdpSenseAnaW::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_chrg_vbus(&mut self) -> U0UsbChrgVbusW<StgSyscfg0Spec> {
        U0UsbChrgVbusW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_dcd_comp_sts(&mut self) -> U0UsbDcdCompStsW<StgSyscfg0Spec> {
        U0UsbDcdCompStsW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_dischrg_vbus(&mut self) -> U0UsbDischrgVbusW<StgSyscfg0Spec> {
        U0UsbDischrgVbusW::new(self, 18)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_dm_vdat_ref_comp_sts(&mut self) -> U0UsbDmVdatRefCompStsW<StgSyscfg0Spec> {
        U0UsbDmVdatRefCompStsW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_dm_vlgc_comp_sts(&mut self) -> U0UsbDmVlgcCompStsW<StgSyscfg0Spec> {
        U0UsbDmVlgcCompStsW::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_dp_vdat_ref_comp_sts(&mut self) -> U0UsbDpVdatRefCompStsW<StgSyscfg0Spec> {
        U0UsbDpVdatRefCompStsW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_host_system_err(&mut self) -> U0UsbHostSystemErrW<StgSyscfg0Spec> {
        U0UsbHostSystemErrW::new(self, 25)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg0Spec;
impl crate::RegisterSpec for StgSyscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_0::R`](R) reader structure"]
impl crate::Readable for StgSyscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_0::W`](W) writer structure"]
impl crate::Writable for StgSyscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_0 to value 0"]
impl crate::Resettable for StgSyscfg0Spec {
    const RESET_VALUE: u32 = 0;
}
