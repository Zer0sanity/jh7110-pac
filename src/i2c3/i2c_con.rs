#[doc = "Register `i2c_con` reader"]
pub type R = crate::R<I2cConSpec>;
#[doc = "Register `i2c_con` writer"]
pub type W = crate::W<I2cConSpec>;
#[doc = "Field `master` reader - I2C Master Connection - 0: Slave, 1: Master"]
pub type MasterR = crate::BitReader;
#[doc = "Field `master` writer - I2C Master Connection - 0: Slave, 1: Master"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `speed` reader - I2C Speed - 01: Standard, 10: Fast, 11: High"]
pub type SpeedR = crate::FieldReader;
#[doc = "Field `speed` writer - I2C Speed - 01: Standard, 10: Fast, 11: High"]
pub type SpeedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `slave_10bitaddr` reader - I2C Slave 10-bit Address - 0: False, 1: True"]
pub type Slave10bitaddrR = crate::BitReader;
#[doc = "Field `slave_10bitaddr` writer - I2C Slave 10-bit Address - 0: False, 1: True"]
pub type Slave10bitaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `master_10bitaddr` reader - I2C Master 10-bit Address - 0: False, 1: True"]
pub type Master10bitaddrR = crate::BitReader;
#[doc = "Field `master_10bitaddr` writer - I2C Master 10-bit Address - 0: False, 1: True"]
pub type Master10bitaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `restart_en` reader - I2C Restart Enable - 0: False, 1: True"]
pub type RestartEnR = crate::BitReader;
#[doc = "Field `restart_en` writer - I2C Restart Enable - 0: False, 1: True"]
pub type RestartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `slave_disable` reader - I2C Slave Disable - 0: False, 1: True"]
pub type SlaveDisableR = crate::BitReader;
#[doc = "Field `slave_disable` writer - I2C Slave Disable - 0: False, 1: True"]
pub type SlaveDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stop_det_ifaddressed` reader - I2C Stop DET If Addressed - 0: False, 1: True"]
pub type StopDetIfaddressedR = crate::BitReader;
#[doc = "Field `stop_det_ifaddressed` writer - I2C Stop DET If Addressed - 0: False, 1: True"]
pub type StopDetIfaddressedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tx_empty_ctrl` reader - I2C TX Empty Control - 0: False, 1: True"]
pub type TxEmptyCtrlR = crate::BitReader;
#[doc = "Field `tx_empty_ctrl` writer - I2C TX Empty Control - 0: False, 1: True"]
pub type TxEmptyCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_fifo_full_hld_ctrl` reader - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
pub type RxFifoFullHldCtrlR = crate::BitReader;
#[doc = "Field `rx_fifo_full_hld_ctrl` writer - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
pub type RxFifoFullHldCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bus_clear_ctrl` reader - I2C Bus Clear Control - 0: False, 1: True"]
pub type BusClearCtrlR = crate::BitReader;
#[doc = "Field `bus_clear_ctrl` writer - I2C Bus Clear Control - 0: False, 1: True"]
pub type BusClearCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C Master Connection - 0: Slave, 1: Master"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - I2C Speed - 01: Standard, 10: Fast, 11: High"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - I2C Slave 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    pub fn slave_10bitaddr(&self) -> Slave10bitaddrR {
        Slave10bitaddrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C Master 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    pub fn master_10bitaddr(&self) -> Master10bitaddrR {
        Master10bitaddrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Restart Enable - 0: False, 1: True"]
    #[inline(always)]
    pub fn restart_en(&self) -> RestartEnR {
        RestartEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Slave Disable - 0: False, 1: True"]
    #[inline(always)]
    pub fn slave_disable(&self) -> SlaveDisableR {
        SlaveDisableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Stop DET If Addressed - 0: False, 1: True"]
    #[inline(always)]
    pub fn stop_det_ifaddressed(&self) -> StopDetIfaddressedR {
        StopDetIfaddressedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C TX Empty Control - 0: False, 1: True"]
    #[inline(always)]
    pub fn tx_empty_ctrl(&self) -> TxEmptyCtrlR {
        TxEmptyCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
    #[inline(always)]
    pub fn rx_fifo_full_hld_ctrl(&self) -> RxFifoFullHldCtrlR {
        RxFifoFullHldCtrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C Bus Clear Control - 0: False, 1: True"]
    #[inline(always)]
    pub fn bus_clear_ctrl(&self) -> BusClearCtrlR {
        BusClearCtrlR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C Master Connection - 0: Slave, 1: Master"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<I2cConSpec> {
        MasterW::new(self, 0)
    }
    #[doc = "Bits 1:2 - I2C Speed - 01: Standard, 10: Fast, 11: High"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SpeedW<I2cConSpec> {
        SpeedW::new(self, 1)
    }
    #[doc = "Bit 3 - I2C Slave 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn slave_10bitaddr(&mut self) -> Slave10bitaddrW<I2cConSpec> {
        Slave10bitaddrW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C Master 10-bit Address - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn master_10bitaddr(&mut self) -> Master10bitaddrW<I2cConSpec> {
        Master10bitaddrW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Restart Enable - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn restart_en(&mut self) -> RestartEnW<I2cConSpec> {
        RestartEnW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Slave Disable - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn slave_disable(&mut self) -> SlaveDisableW<I2cConSpec> {
        SlaveDisableW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Stop DET If Addressed - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn stop_det_ifaddressed(&mut self) -> StopDetIfaddressedW<I2cConSpec> {
        StopDetIfaddressedW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C TX Empty Control - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn tx_empty_ctrl(&mut self) -> TxEmptyCtrlW<I2cConSpec> {
        TxEmptyCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - I2C RX FIFO Full Hold Control - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_full_hld_ctrl(&mut self) -> RxFifoFullHldCtrlW<I2cConSpec> {
        RxFifoFullHldCtrlW::new(self, 9)
    }
    #[doc = "Bit 11 - I2C Bus Clear Control - 0: False, 1: True"]
    #[inline(always)]
    #[must_use]
    pub fn bus_clear_ctrl(&mut self) -> BusClearCtrlW<I2cConSpec> {
        BusClearCtrlW::new(self, 11)
    }
}
#[doc = "DesignWare I2C CON\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cConSpec;
impl crate::RegisterSpec for I2cConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_con::R`](R) reader structure"]
impl crate::Readable for I2cConSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_con::W`](W) writer structure"]
impl crate::Writable for I2cConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets i2c_con to value 0"]
impl crate::Resettable for I2cConSpec {
    const RESET_VALUE: u32 = 0;
}
