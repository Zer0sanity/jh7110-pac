#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    syscfg0: Syscfg0,
    syscfg1: Syscfg1,
    syscfg2: Syscfg2,
    syscfg3: Syscfg3,
    syscfg4: Syscfg4,
    syscfg5: Syscfg5,
    syscfg6: Syscfg6,
    syscfg7: Syscfg7,
    syscfg8: Syscfg8,
    syscfg9: Syscfg9,
    syscfg10: Syscfg10,
    syscfg11: Syscfg11,
    syscfg12: Syscfg12,
    syscfg_xcfgi: [SyscfgXcfgi; 12],
    syscfg25: Syscfg25,
}
impl RegisterBlock {
    #[doc = "0x00 - MIPITX DPHY SYSCFG 0: mipitx_apbifsaif_syscfg_0"]
    #[inline(always)]
    pub const fn syscfg0(&self) -> &Syscfg0 {
        &self.syscfg0
    }
    #[doc = "0x04 - MIPITX DPHY SYSCFG 1: mipitx_apbifsaif_syscfg_4"]
    #[inline(always)]
    pub const fn syscfg1(&self) -> &Syscfg1 {
        &self.syscfg1
    }
    #[doc = "0x08 - MIPITX DPHY SYSCFG 2: mipitx_apbifsaif_syscfg_8"]
    #[inline(always)]
    pub const fn syscfg2(&self) -> &Syscfg2 {
        &self.syscfg2
    }
    #[doc = "0x0c - MIPITX DPHY SYSCFG 3: mipitx_apbifsaif_syscfg_12"]
    #[inline(always)]
    pub const fn syscfg3(&self) -> &Syscfg3 {
        &self.syscfg3
    }
    #[doc = "0x10 - MIPITX DPHY SYSCFG 4: mipitx_apbifsaif_syscfg_16"]
    #[inline(always)]
    pub const fn syscfg4(&self) -> &Syscfg4 {
        &self.syscfg4
    }
    #[doc = "0x14 - MIPITX DPHY SYSCFG 5: mipitx_apbifsaif_syscfg_20"]
    #[inline(always)]
    pub const fn syscfg5(&self) -> &Syscfg5 {
        &self.syscfg5
    }
    #[doc = "0x18 - MIPITX DPHY SYSCFG 6: mipitx_apbifsaif_syscfg_24"]
    #[inline(always)]
    pub const fn syscfg6(&self) -> &Syscfg6 {
        &self.syscfg6
    }
    #[doc = "0x1c - MIPITX DPHY SYSCFG 7: mipitx_apbifsaif_syscfg_28"]
    #[inline(always)]
    pub const fn syscfg7(&self) -> &Syscfg7 {
        &self.syscfg7
    }
    #[doc = "0x20 - MIPITX DPHY CLANE: mipitx_apbifsaif_syscfg_32"]
    #[inline(always)]
    pub const fn syscfg8(&self) -> &Syscfg8 {
        &self.syscfg8
    }
    #[doc = "0x24 - MIPITX DPHY CLANE and DLANE: mipitx_apbifsaif_syscfg_36"]
    #[inline(always)]
    pub const fn syscfg9(&self) -> &Syscfg9 {
        &self.syscfg9
    }
    #[doc = "0x28 - MIPITX DPHY SYSCFG 10: mipitx_apbifsaif_syscfg_40"]
    #[inline(always)]
    pub const fn syscfg10(&self) -> &Syscfg10 {
        &self.syscfg10
    }
    #[doc = "0x2c - MIPITX DPHY SYSCFG 11: mipitx_apbifsaif_syscfg_44"]
    #[inline(always)]
    pub const fn syscfg11(&self) -> &Syscfg11 {
        &self.syscfg11
    }
    #[doc = "0x30 - MIPITX DPHY SYSCFG 12: mipitx_apbifsaif_syscfg_48"]
    #[inline(always)]
    pub const fn syscfg12(&self) -> &Syscfg12 {
        &self.syscfg12
    }
    #[doc = "0x34..0x64 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi(&self, n: usize) -> &SyscfgXcfgi {
        &self.syscfg_xcfgi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x64 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub fn syscfg_xcfgi_iter(&self) -> impl Iterator<Item = &SyscfgXcfgi> {
        self.syscfg_xcfgi.iter()
    }
    #[doc = "0x34 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi00(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(0)
    }
    #[doc = "0x38 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi01(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(1)
    }
    #[doc = "0x3c - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi02(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(2)
    }
    #[doc = "0x40 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi03(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(3)
    }
    #[doc = "0x44 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi04(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(4)
    }
    #[doc = "0x48 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi05(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(5)
    }
    #[doc = "0x4c - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi06(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(6)
    }
    #[doc = "0x50 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi07(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(7)
    }
    #[doc = "0x54 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi08(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(8)
    }
    #[doc = "0x58 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi09(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(9)
    }
    #[doc = "0x5c - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi0a(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(10)
    }
    #[doc = "0x60 - MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
    #[inline(always)]
    pub const fn syscfg_xcfgi0b(&self) -> &SyscfgXcfgi {
        self.syscfg_xcfgi(11)
    }
    #[doc = "0x64 - MIPITX DPHY SYSCFG 25: mipitx_apbifsaif_syscfg_100"]
    #[inline(always)]
    pub const fn syscfg25(&self) -> &Syscfg25 {
        &self.syscfg25
    }
}
#[doc = "syscfg0 (rw) register accessor: MIPITX DPHY SYSCFG 0: mipitx_apbifsaif_syscfg_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg0`]
module"]
#[doc(alias = "syscfg0")]
pub type Syscfg0 = crate::Reg<syscfg0::Syscfg0Spec>;
#[doc = "MIPITX DPHY SYSCFG 0: mipitx_apbifsaif_syscfg_0"]
pub mod syscfg0;
#[doc = "syscfg1 (r) register accessor: MIPITX DPHY SYSCFG 1: mipitx_apbifsaif_syscfg_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg1`]
module"]
#[doc(alias = "syscfg1")]
pub type Syscfg1 = crate::Reg<syscfg1::Syscfg1Spec>;
#[doc = "MIPITX DPHY SYSCFG 1: mipitx_apbifsaif_syscfg_4"]
pub mod syscfg1;
#[doc = "syscfg2 (rw) register accessor: MIPITX DPHY SYSCFG 2: mipitx_apbifsaif_syscfg_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg2`]
module"]
#[doc(alias = "syscfg2")]
pub type Syscfg2 = crate::Reg<syscfg2::Syscfg2Spec>;
#[doc = "MIPITX DPHY SYSCFG 2: mipitx_apbifsaif_syscfg_8"]
pub mod syscfg2;
#[doc = "syscfg3 (rw) register accessor: MIPITX DPHY SYSCFG 3: mipitx_apbifsaif_syscfg_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg3`]
module"]
#[doc(alias = "syscfg3")]
pub type Syscfg3 = crate::Reg<syscfg3::Syscfg3Spec>;
#[doc = "MIPITX DPHY SYSCFG 3: mipitx_apbifsaif_syscfg_12"]
pub mod syscfg3;
#[doc = "syscfg4 (rw) register accessor: MIPITX DPHY SYSCFG 4: mipitx_apbifsaif_syscfg_16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg4`]
module"]
#[doc(alias = "syscfg4")]
pub type Syscfg4 = crate::Reg<syscfg4::Syscfg4Spec>;
#[doc = "MIPITX DPHY SYSCFG 4: mipitx_apbifsaif_syscfg_16"]
pub mod syscfg4;
#[doc = "syscfg5 (rw) register accessor: MIPITX DPHY SYSCFG 5: mipitx_apbifsaif_syscfg_20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg5`]
module"]
#[doc(alias = "syscfg5")]
pub type Syscfg5 = crate::Reg<syscfg5::Syscfg5Spec>;
#[doc = "MIPITX DPHY SYSCFG 5: mipitx_apbifsaif_syscfg_20"]
pub mod syscfg5;
#[doc = "syscfg6 (rw) register accessor: MIPITX DPHY SYSCFG 6: mipitx_apbifsaif_syscfg_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg6`]
module"]
#[doc(alias = "syscfg6")]
pub type Syscfg6 = crate::Reg<syscfg6::Syscfg6Spec>;
#[doc = "MIPITX DPHY SYSCFG 6: mipitx_apbifsaif_syscfg_24"]
pub mod syscfg6;
#[doc = "syscfg7 (rw) register accessor: MIPITX DPHY SYSCFG 7: mipitx_apbifsaif_syscfg_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg7`]
module"]
#[doc(alias = "syscfg7")]
pub type Syscfg7 = crate::Reg<syscfg7::Syscfg7Spec>;
#[doc = "MIPITX DPHY SYSCFG 7: mipitx_apbifsaif_syscfg_28"]
pub mod syscfg7;
#[doc = "syscfg8 (rw) register accessor: MIPITX DPHY CLANE: mipitx_apbifsaif_syscfg_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg8`]
module"]
#[doc(alias = "syscfg8")]
pub type Syscfg8 = crate::Reg<syscfg8::Syscfg8Spec>;
#[doc = "MIPITX DPHY CLANE: mipitx_apbifsaif_syscfg_32"]
pub mod syscfg8;
#[doc = "syscfg9 (rw) register accessor: MIPITX DPHY CLANE and DLANE: mipitx_apbifsaif_syscfg_36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg9`]
module"]
#[doc(alias = "syscfg9")]
pub type Syscfg9 = crate::Reg<syscfg9::Syscfg9Spec>;
#[doc = "MIPITX DPHY CLANE and DLANE: mipitx_apbifsaif_syscfg_36"]
pub mod syscfg9;
#[doc = "syscfg10 (rw) register accessor: MIPITX DPHY SYSCFG 10: mipitx_apbifsaif_syscfg_40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg10`]
module"]
#[doc(alias = "syscfg10")]
pub type Syscfg10 = crate::Reg<syscfg10::Syscfg10Spec>;
#[doc = "MIPITX DPHY SYSCFG 10: mipitx_apbifsaif_syscfg_40"]
pub mod syscfg10;
#[doc = "syscfg11 (rw) register accessor: MIPITX DPHY SYSCFG 11: mipitx_apbifsaif_syscfg_44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg11`]
module"]
#[doc(alias = "syscfg11")]
pub type Syscfg11 = crate::Reg<syscfg11::Syscfg11Spec>;
#[doc = "MIPITX DPHY SYSCFG 11: mipitx_apbifsaif_syscfg_44"]
pub mod syscfg11;
#[doc = "syscfg12 (rw) register accessor: MIPITX DPHY SYSCFG 12: mipitx_apbifsaif_syscfg_48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg12`]
module"]
#[doc(alias = "syscfg12")]
pub type Syscfg12 = crate::Reg<syscfg12::Syscfg12Spec>;
#[doc = "MIPITX DPHY SYSCFG 12: mipitx_apbifsaif_syscfg_48"]
pub mod syscfg12;
#[doc = "syscfg_xcfgi (rw) register accessor: MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_xcfgi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg_xcfgi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg_xcfgi`]
module"]
#[doc(alias = "syscfg_xcfgi")]
pub type SyscfgXcfgi = crate::Reg<syscfg_xcfgi::SyscfgXcfgiSpec>;
#[doc = "MIPITX DPHY SYSCFG 13-24: mipitx_apbifsaif_syscfg_52-96"]
pub mod syscfg_xcfgi;
#[doc = "syscfg25 (rw) register accessor: MIPITX DPHY SYSCFG 25: mipitx_apbifsaif_syscfg_100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syscfg25`]
module"]
#[doc(alias = "syscfg25")]
pub type Syscfg25 = crate::Reg<syscfg25::Syscfg25Spec>;
#[doc = "MIPITX DPHY SYSCFG 25: mipitx_apbifsaif_syscfg_100"]
pub mod syscfg25;
