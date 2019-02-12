use std::io;

pub struct ValueName<'help>(&'help str);

impl<'help> ValueName<'help> {
    fn write_as_required<W: io::Write>(&self, w: W) -> io::Result {
        write!(w, "<{}>", self.0)
    }
    fn write_as_optional<W: io::Write>(&self, w: W) -> io::Result {
        write!(w, "[{}]", self.0)
    }
    fn write<W: io::Write>(&self, w: W) -> io::Result {
        write!(w, "{}", self.0)
    }
}