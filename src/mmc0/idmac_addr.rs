#[repr(C)]
#[doc = "MMC Internal DMAC Address registers"]
#[doc(alias = "idmac_addr")]
pub struct IdmacAddr {
    dbaddr_dbaddrl: DbaddrDbaddrl,
    idsts_dbaddru: IdstsDbaddru,
    idinten_idsts64: IdintenIdsts64,
    dscaddr_idinten64: DscaddrIdinten64,
    bufaddr_dscaddrl: BufaddrDscaddrl,
    dscaddru: Dscaddru,
    bufaddr: [Bufaddr; 2],
}
impl IdmacAddr {
    #[doc = "0x00 - MMC internal DMAC DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): DB address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address lower 32-bits"]
    #[inline(always)]
    pub const fn dbaddr_dbaddrl(&self) -> &DbaddrDbaddrl {
        &self.dbaddr_dbaddrl
    }
    #[doc = "0x04 - MMC internal DMAC status / DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): status, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address upper 32-bits"]
    #[inline(always)]
    pub const fn idsts_dbaddru(&self) -> &IdstsDbaddru {
        &self.idsts_dbaddru
    }
    #[doc = "0x08 - MMC internal DMAC interrupt enable / status - HCON\\[ADDR_CONFIG\\]
32-bit(0): interrupt enable, HCON\\[ADDR_CONFIG\\]
64-bit(1): status"]
    #[inline(always)]
    pub const fn idinten_idsts64(&self) -> &IdintenIdsts64 {
        &self.idinten_idsts64
    }
    #[doc = "0x0c - MMC internal DMAC DSC address / interrupt enable - HCON\\[ADDR_CONFIG\\]
32-bit(0): DSC address, HCON\\[ADDR_CONFIG\\]
64-bit(1): interrupt enable"]
    #[inline(always)]
    pub const fn dscaddr_idinten64(&self) -> &DscaddrIdinten64 {
        &self.dscaddr_idinten64
    }
    #[doc = "0x10 - MMC internal DMAC buffer address / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): buffer address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address lower 32-bits"]
    #[inline(always)]
    pub const fn bufaddr_dscaddrl(&self) -> &BufaddrDscaddrl {
        &self.bufaddr_dscaddrl
    }
    #[doc = "0x14 - MMC internal DMAC reserved / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address upper 32-bits"]
    #[inline(always)]
    pub const fn dscaddru(&self) -> &Dscaddru {
        &self.dscaddru
    }
    #[doc = "0x18..0x20 - MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits"]
    #[inline(always)]
    pub const fn bufaddr(&self, n: usize) -> &Bufaddr {
        &self.bufaddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits"]
    #[inline(always)]
    pub fn bufaddr_iter(&self) -> impl Iterator<Item = &Bufaddr> {
        self.bufaddr.iter()
    }
    #[doc = "0x18 - MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits"]
    #[inline(always)]
    pub const fn bufaddrl(&self) -> &Bufaddr {
        self.bufaddr(0)
    }
    #[doc = "0x1c - MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits"]
    #[inline(always)]
    pub const fn bufaddru(&self) -> &Bufaddr {
        self.bufaddr(1)
    }
}
#[doc = "dbaddr_dbaddrl (rw) register accessor: MMC internal DMAC DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): DB address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address lower 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbaddr_dbaddrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbaddr_dbaddrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbaddr_dbaddrl`]
module"]
#[doc(alias = "dbaddr_dbaddrl")]
pub type DbaddrDbaddrl = crate::Reg<dbaddr_dbaddrl::DbaddrDbaddrlSpec>;
#[doc = "MMC internal DMAC DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): DB address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address lower 32-bits"]
pub mod dbaddr_dbaddrl;
#[doc = "idsts_dbaddru (rw) register accessor: MMC internal DMAC status / DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): status, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address upper 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idsts_dbaddru::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idsts_dbaddru::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idsts_dbaddru`]
module"]
#[doc(alias = "idsts_dbaddru")]
pub type IdstsDbaddru = crate::Reg<idsts_dbaddru::IdstsDbaddruSpec>;
#[doc = "MMC internal DMAC status / DB address - HCON\\[ADDR_CONFIG\\]
32-bit(0): status, HCON\\[ADDR_CONFIG\\]
64-bit(1): DB address upper 32-bits"]
pub mod idsts_dbaddru;
#[doc = "idinten_idsts64 (rw) register accessor: MMC internal DMAC interrupt enable / status - HCON\\[ADDR_CONFIG\\]
32-bit(0): interrupt enable, HCON\\[ADDR_CONFIG\\]
64-bit(1): status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idinten_idsts64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idinten_idsts64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idinten_idsts64`]
module"]
#[doc(alias = "idinten_idsts64")]
pub type IdintenIdsts64 = crate::Reg<idinten_idsts64::IdintenIdsts64Spec>;
#[doc = "MMC internal DMAC interrupt enable / status - HCON\\[ADDR_CONFIG\\]
32-bit(0): interrupt enable, HCON\\[ADDR_CONFIG\\]
64-bit(1): status"]
pub mod idinten_idsts64;
#[doc = "dscaddr_idinten64 (rw) register accessor: MMC internal DMAC DSC address / interrupt enable - HCON\\[ADDR_CONFIG\\]
32-bit(0): DSC address, HCON\\[ADDR_CONFIG\\]
64-bit(1): interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddr_idinten64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscaddr_idinten64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscaddr_idinten64`]
module"]
#[doc(alias = "dscaddr_idinten64")]
pub type DscaddrIdinten64 = crate::Reg<dscaddr_idinten64::DscaddrIdinten64Spec>;
#[doc = "MMC internal DMAC DSC address / interrupt enable - HCON\\[ADDR_CONFIG\\]
32-bit(0): DSC address, HCON\\[ADDR_CONFIG\\]
64-bit(1): interrupt enable"]
pub mod dscaddr_idinten64;
#[doc = "bufaddr_dscaddrl (rw) register accessor: MMC internal DMAC buffer address / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): buffer address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address lower 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufaddr_dscaddrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufaddr_dscaddrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufaddr_dscaddrl`]
module"]
#[doc(alias = "bufaddr_dscaddrl")]
pub type BufaddrDscaddrl = crate::Reg<bufaddr_dscaddrl::BufaddrDscaddrlSpec>;
#[doc = "MMC internal DMAC buffer address / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): buffer address, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address lower 32-bits"]
pub mod bufaddr_dscaddrl;
#[doc = "dscaddru (rw) register accessor: MMC internal DMAC reserved / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address upper 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddru::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscaddru::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscaddru`]
module"]
#[doc(alias = "dscaddru")]
pub type Dscaddru = crate::Reg<dscaddru::DscaddruSpec>;
#[doc = "MMC internal DMAC reserved / DSC address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): DSC address upper 32-bits"]
pub mod dscaddru;
#[doc = "bufaddr (rw) register accessor: MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bufaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufaddr`]
module"]
#[doc(alias = "bufaddr")]
pub type Bufaddr = crate::Reg<bufaddr::BufaddrSpec>;
#[doc = "MMC internal DMAC reserved / buffer address - HCON\\[ADDR_CONFIG\\]
32-bit(0): reserved, HCON\\[ADDR_CONFIG\\]
64-bit(1): buffer address lower/upper 32-bits"]
pub mod bufaddr;
