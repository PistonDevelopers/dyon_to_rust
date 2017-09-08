use std::io;
use std::sync::Arc;

use dyon::{Object, Variable};

use Secret;

pub trait PrintLn {
    fn println(&self) {
        self.print();
        println!("");
    }
    fn print(&self);
}

impl<T: WriteLn> PrintLn for T {
    default fn print(&self) {
        let mut stdout = io::stdout();
        self.write(&mut stdout).unwrap();
    }
}

impl<'a> PrintLn for &'a str {
    fn print(&self) {
        print!("{}", self);
    }
}

pub fn println<T: PrintLn>(item: &T) {
    item.println();
}

pub fn print<T: PrintLn>(item: &T) {
    item.print();
}

pub trait WriteLn {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()>;
    fn writeln<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.write(w)?;
        write!(w, "")
    }
}

impl<T: WriteLn> WriteLn for Arc<T> {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        (**self).writeln(w)
    }
}

impl<'a> WriteLn for &'a str {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        use piston_meta::json;

        json::write_string(w, self)
    }
}

impl<T: WriteLn> WriteLn for Vec<T> {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "[")?;
        let n = self.len();
        for (i, it) in self.iter().enumerate() {
            it.write(w)?;
            if (i + 1) != n {
                write!(w, ", ")?;
            }
        }
        write!(w, "]")?;
        Ok(())
    }
}

impl WriteLn for bool {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self)
    }
}

impl WriteLn for f64 {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "{}", self)
    }
}

impl WriteLn for [f32; 4] {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write!(w, "(")?;
        match (self[2] == 0.0, self[3] == 0.0) {
            (true, true) => {
                write!(w, "{}, {}", self[0], self[1])?;
            }
            (false, true) => {
                write!(w, "{}, {}, {}", self[0], self[1], self[2])?;
            }
            (_, _) => {
                write!(w, "{}, {}, {}, {}", self[0], self[1], self[2], self[3])?;
            }
        }
        write!(w, ")")
    }
}

impl<'a, T: WriteLn> WriteLn for &'a T {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        (*self).write(w)
    }
}

impl<T: WriteLn, A> WriteLn for Secret<T, A> {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.val.write(w)
    }
}

impl WriteLn for Object {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        use dyon::write::{write_variable, EscapeString};
        use dyon::Runtime;
        use piston_meta::json;

        let obj = self;
        let ref rt = Runtime::new();
        let tabs = 0;

        try!(write!(w, "{{"));
        let n = obj.len();
        for (i, (k, v)) in obj.iter().enumerate() {
            if k.chars().all(|c| c.is_alphanumeric()) {
                try!(write!(w, "{}: ", k));
            } else {
                try!(json::write_string(w, &k));
                try!(write!(w, ": "));
            }
            try!(write_variable(w, rt, v, EscapeString::Json, tabs));
            if i + 1 < n {
                try!(write!(w, ", "));
            }
        }
        try!(write!(w, "}}"));
        Ok(())
    }
}

impl WriteLn for Variable {
    fn write<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        use dyon::write::{write_variable, EscapeString};
        use dyon::Runtime;

        let ref rt = Runtime::new();
        let tabs = 0;
        write_variable(w, rt, self, EscapeString::Json, tabs)
    }
}
