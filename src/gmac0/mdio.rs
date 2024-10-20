#[repr(C)]
#[doc = "MDIO registers"]
#[doc(alias = "mdio")]
pub struct Mdio {
    data: Data,
    addr: Addr,
}
impl Mdio {
    #[doc = "0x00 - MDIO Data"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - MDIO Address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
}
#[doc = "data (rw) register accessor: MDIO Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "data")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "MDIO Data"]
pub mod data;
#[doc = "addr (rw) register accessor: MDIO Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "addr")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "MDIO Address"]
pub mod addr;
