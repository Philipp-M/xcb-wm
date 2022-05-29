use crate::icccm::atoms::Atoms;
use crate::icccm::traits::{
    IcccmPropertyCookieChecked, IcccmPropertyCookieUnchecked, IcccmPropertyRequestUnchecked,
    IcccmRequest, IcccmVoidRequestChecked,
};

/// The main `icccm` entry point
///
/// `Connection` is a thin wrapper around [`xcb::Connection`]. It provides a
/// subset of the [`xcb::Connection`] API covering the functionality needed for
/// `icccm`.
///
/// The provided subset works the same as the corresponding API in
/// [`xcb::Connection`]. That is, methods with the same name do the same thing.
/// The general usage pattern is the same for both crates.
///
/// This wrapper is not strictly needed for `icccm` (ICCCM Atoms are pre-defined
/// in the core protocol [1]). However, to align with `ewmh` we provide the same
/// usage pattern/API as for `ewmh`.
///
/// [1] <https://www.x.org/releases/current/doc/xproto/x11protocol.html#Predefined_Atoms>
///
pub struct Connection<'a> {
    pub(crate) con: &'a xcb::Connection,

    /// Interned [`Atoms`] for the `icccm` protocol
    pub atoms: Atoms,
}

#[allow(dead_code)]
impl<'a> Connection<'a> {
    pub fn connect(xcb_con: &'a xcb::Connection) -> Connection<'a> {
        Connection {
            con: xcb_con,
            atoms: Atoms::intern(xcb_con),
        }
    }

    pub fn send_request<'b, R>(&self, request: &'b R) -> R::IcccmCookie
    where
        R: IcccmRequest<'b>,
    {
        request.send(self)
    }

    pub fn send_request_checked<'b, R>(&self, request: &'b R) -> xcb::VoidCookieChecked
    where
        R: IcccmVoidRequestChecked<'b>,
    {
        request.send(self)
    }

    pub fn send_request_unchecked<R>(&self, request: &R) -> R::IcccmCookie
    where
        R: IcccmPropertyRequestUnchecked,
    {
        request.send(self)
    }

    pub fn wait_for_reply<C>(&self, cookie: C) -> C::Reply
    where
        C: IcccmPropertyCookieChecked,
    {
        let xcb_reply = self.con.wait_for_reply(cookie.inner());
        xcb_reply.unwrap().into()
    }

    pub fn wait_for_reply_unchecked<C>(&self, cookie: C) -> C::Reply
    where
        C: IcccmPropertyCookieUnchecked,
    {
        let xcb_reply = self.con.wait_for_reply_unchecked(cookie.inner());
        xcb_reply.unwrap().unwrap().into()
    }
}

#[cfg(test)]
mod tests {
    use crate::icccm::proto::SetWmName;

    #[test]
    fn number_of_desktops() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let icccm_con = crate::icccm::Connection::connect(&xcb_con);

        use xcb::XidNew;

        let window = unsafe { xcb::x::Window::new(0x2e0013e) };

        let request = crate::icccm::proto::GetWmName::new(window);
        let cookie = icccm_con.send_request(&request);
        let reply = icccm_con.wait_for_reply(cookie);
        println!("{:?}", reply);
    }

    #[test]
    fn set_number_of_desktops() {
        let xcb_con = xcb::Connection::connect(Option::None).unwrap().0;
        let icccm_con = crate::icccm::Connection::connect(&xcb_con);

        use xcb::XidNew;

        let window = unsafe { xcb::x::Window::new(0x4600003) };
        let name = String::from("NEW NAME").into_bytes();

        let request = SetWmName::new(window, xcb::x::ATOM_STRING, name);
        let cookie = icccm_con.send_request_checked(&request);
        let reply = xcb_con.check_request(cookie);
        println!("{:?}", reply);
    }
}
