#[repr(C)]
#[doc = "L2PM Event Control registers."]
#[doc(alias = "event_control")]
pub struct EventControl {
    event_select: [EventSelect; 6],
    _reserved1: [u8; 0x07d0],
    client_filter: ClientFilter,
}
impl EventControl {
    #[doc = "0x00..0x30 - L2PM Event Control Event Select configuration."]
    #[inline(always)]
    pub const fn event_select(&self, n: usize) -> &EventSelect {
        &self.event_select[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - L2PM Event Control Event Select configuration."]
    #[inline(always)]
    pub fn event_select_iter(&self) -> impl Iterator<Item = &EventSelect> {
        self.event_select.iter()
    }
    #[doc = "0x800..0x808 - L2PM Event Control Event Select configuration."]
    #[inline(always)]
    pub const fn client_filter(&self) -> &ClientFilter {
        &self.client_filter
    }
}
#[doc = "event_select (rw) register accessor: L2PM Event Control Event Select configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`event_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_select`]
module"]
#[doc(alias = "event_select")]
pub type EventSelect = crate::Reg<event_select::EventSelectSpec>;
#[doc = "L2PM Event Control Event Select configuration."]
pub mod event_select;
#[doc = "client_filter (rw) register accessor: L2PM Event Control Event Select configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`client_filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`client_filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@client_filter`]
module"]
#[doc(alias = "client_filter")]
pub type ClientFilter = crate::Reg<client_filter::ClientFilterSpec>;
#[doc = "L2PM Event Control Event Select configuration."]
pub mod client_filter;
