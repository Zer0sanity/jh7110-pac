#[doc = "Register `sys_syscfg36` reader"]
pub type R = crate::R<SysSyscfg36Spec>;
#[doc = "Register `sys_syscfg36` writer"]
pub type W = crate::W<SysSyscfg36Spec>;
#[doc = "Field `gmac5_axi64_mac_speed` reader - "]
pub type Gmac5Axi64MacSpeedR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel` reader - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII, None)?, 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
pub type Gmac5Axi64PhyIntfSelR = crate::FieldReader;
#[doc = "Field `gmac5_axi64_phy_intf_sel` writer - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII, None)?, 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
pub type Gmac5Axi64PhyIntfSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn gmac5_axi64_mac_speed(&self) -> Gmac5Axi64MacSpeedR {
        Gmac5Axi64MacSpeedR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII, None)?, 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
    #[inline(always)]
    pub fn gmac5_axi64_phy_intf_sel(&self) -> Gmac5Axi64PhyIntfSelR {
        Gmac5Axi64PhyIntfSelR::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:4 - Active PHY Selected | When you have multiple GMAC PHY interfaces in your configuration, this field indicates the sampled value of the PHY selector during reset de-assertion. | Values: 0x0:(GMII or MII, None)?, 0x01:RGMII, 0x2:SGMII, 0x3:TBI, 0x4:RMII, 0x5:RTBI, 0x6:SMII, 0x7:REVMII"]
    #[inline(always)]
    #[must_use]
    pub fn gmac5_axi64_phy_intf_sel(&mut self) -> Gmac5Axi64PhyIntfSelW<SysSyscfg36Spec> {
        Gmac5Axi64PhyIntfSelW::new(self, 2)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg36Spec;
impl crate::RegisterSpec for SysSyscfg36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg36::R`](R) reader structure"]
impl crate::Readable for SysSyscfg36Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg36::W`](W) writer structure"]
impl crate::Writable for SysSyscfg36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg36 to value 0x04"]
impl crate::Resettable for SysSyscfg36Spec {
    const RESET_VALUE: u32 = 0x04;
}
