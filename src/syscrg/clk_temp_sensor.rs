#[repr(C)]
#[doc = "Clock Temperature registers"]
#[doc(alias = "clk_temp_sensor")]
pub struct ClkTempSensor {
    apb: Apb,
    temp_sensor: TempSensor,
}
impl ClkTempSensor {
    #[doc = "0x00 - Clock Temperature Sensor APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04 - Clock Temperature Sensor"]
    #[inline(always)]
    pub const fn temp_sensor(&self) -> &TempSensor {
        &self.temp_sensor
    }
}
#[doc = "apb (rw) register accessor: Clock Temperature Sensor APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock Temperature Sensor APB"]
pub mod apb;
#[doc = "temp_sensor (rw) register accessor: Clock Temperature Sensor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp_sensor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`temp_sensor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp_sensor`]
module"]
#[doc(alias = "temp_sensor")]
pub type TempSensor = crate::Reg<temp_sensor::TempSensorSpec>;
#[doc = "Clock Temperature Sensor"]
pub mod temp_sensor;
