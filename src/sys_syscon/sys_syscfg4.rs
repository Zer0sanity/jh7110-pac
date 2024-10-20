#[doc = "Register `sys_syscfg4` reader"]
pub type R = crate::R<SysSyscfg4Spec>;
#[doc = "Register `sys_syscfg4` writer"]
pub type W = crate::W<SysSyscfg4Spec>;
#[doc = "Field `coda12_cur_inst` reader - Tie 0 in JPU internal, do not care"]
pub type Coda12CurInstR = crate::FieldReader;
#[doc = "Field `wave511_vpu_idle` reader - VPU monitoring signal"]
pub type Wave511VpuIdleR = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_0` reader - "]
pub type CanCtrlFdEnable0R = crate::BitReader;
#[doc = "Field `can_ctrl_fd_enable_0` writer - "]
pub type CanCtrlFdEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ctrl_host_ecc_disable_0` reader - "]
pub type CanCtrlHostEccDisable0R = crate::BitReader;
#[doc = "Field `can_ctrl_host_ecc_disable_0` writer - "]
pub type CanCtrlHostEccDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ctrl_host_if_0` reader - "]
pub type CanCtrlHostIf0R = crate::FieldReader<u32>;
#[doc = "Field `qspi_sclk_dlychain_sel` reader - des_qspi_sclk_dla: clock delay"]
pub type QspiSclkDlychainSelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Tie 0 in JPU internal, do not care"]
    #[inline(always)]
    pub fn coda12_cur_inst(&self) -> Coda12CurInstR {
        Coda12CurInstR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - VPU monitoring signal"]
    #[inline(always)]
    pub fn wave511_vpu_idle(&self) -> Wave511VpuIdleR {
        Wave511VpuIdleR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn can_ctrl_fd_enable_0(&self) -> CanCtrlFdEnable0R {
        CanCtrlFdEnable0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn can_ctrl_host_ecc_disable_0(&self) -> CanCtrlHostEccDisable0R {
        CanCtrlHostEccDisable0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:23"]
    #[inline(always)]
    pub fn can_ctrl_host_if_0(&self) -> CanCtrlHostIf0R {
        CanCtrlHostIf0R::new((self.bits >> 5) & 0x0007_ffff)
    }
    #[doc = "Bits 24:28 - des_qspi_sclk_dla: clock delay"]
    #[inline(always)]
    pub fn qspi_sclk_dlychain_sel(&self) -> QspiSclkDlychainSelR {
        QspiSclkDlychainSelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_fd_enable_0(&mut self) -> CanCtrlFdEnable0W<SysSyscfg4Spec> {
        CanCtrlFdEnable0W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn can_ctrl_host_ecc_disable_0(&mut self) -> CanCtrlHostEccDisable0W<SysSyscfg4Spec> {
        CanCtrlHostEccDisable0W::new(self, 4)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg4Spec;
impl crate::RegisterSpec for SysSyscfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg4::R`](R) reader structure"]
impl crate::Readable for SysSyscfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg4::W`](W) writer structure"]
impl crate::Writable for SysSyscfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg4 to value 0"]
impl crate::Resettable for SysSyscfg4Spec {
    const RESET_VALUE: u32 = 0;
}
