use std::io::{self, Read, Write};

/// Minimal Factorio PropertyTree codec — enough to read/write mod-settings.dat.
///
/// Format: 8-byte version (4x u16 LE) + 1-byte has_quality + PropertyTree root.
/// PropertyTree node: 1-byte type tag + 1-byte any_type flag + payload.
/// ImmutableString: 1-byte is_none; if not none: 1-byte len (0xFF = extended 4-byte len) + bytes.

#[derive(Debug, Clone, PartialEq)]
pub struct ModSettings {
    pub version: (u16, u16, u16, u16),
    pub has_quality: bool,
    pub data: PropertyTree,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyTree {
    Null,
    Bool(bool),
    Number(f64),
    String(Option<Vec<u8>>),
    List(Vec<(Vec<u8>, PropertyTree)>),
    Dictionary(Vec<(Vec<u8>, PropertyTree)>),
    SignedInt(i64),
    UnsignedInt(u64),
}

impl ModSettings {
    /// Produce a minimal empty mod-settings.dat suitable for a fresh run.
    pub fn empty(version: (u16, u16, u16, u16)) -> Self {
        ModSettings {
            version,
            has_quality: true,
            data: PropertyTree::Dictionary(vec![
                (b"startup".to_vec(),          PropertyTree::Dictionary(vec![])),
                (b"runtime-global".to_vec(),   PropertyTree::Dictionary(vec![])),
                (b"runtime-per-user".to_vec(), PropertyTree::Dictionary(vec![])),
            ]),
        }
    }

    pub fn read(r: &mut impl Read) -> io::Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        let version = (
            u16::from_le_bytes([buf[0], buf[1]]),
            u16::from_le_bytes([buf[2], buf[3]]),
            u16::from_le_bytes([buf[4], buf[5]]),
            u16::from_le_bytes([buf[6], buf[7]]),
        );
        let mut flag = [0u8; 1];
        r.read_exact(&mut flag)?;
        let has_quality = flag[0] != 0;
        let data = read_tree(r)?;
        Ok(ModSettings { version, has_quality, data })
    }

    pub fn write(&self, w: &mut impl Write) -> io::Result<()> {
        let (a, b, c, d) = self.version;
        w.write_all(&a.to_le_bytes())?;
        w.write_all(&b.to_le_bytes())?;
        w.write_all(&c.to_le_bytes())?;
        w.write_all(&d.to_le_bytes())?;
        w.write_all(&[self.has_quality as u8])?;
        write_tree(w, &self.data)
    }

    pub fn to_bytes(&self) -> io::Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.write(&mut buf)?;
        Ok(buf)
    }
}

fn read_immutable_string(r: &mut impl Read) -> io::Result<Option<Vec<u8>>> {
    let mut b = [0u8; 1];
    r.read_exact(&mut b)?;
    if b[0] != 0 {
        return Ok(None);
    }
    r.read_exact(&mut b)?;
    let len = if b[0] == 0xff {
        let mut lb = [0u8; 4];
        r.read_exact(&mut lb)?;
        u32::from_le_bytes(lb) as usize
    } else {
        b[0] as usize
    };
    let mut s = vec![0u8; len];
    r.read_exact(&mut s)?;
    Ok(Some(s))
}

fn write_immutable_string(w: &mut impl Write, s: Option<&[u8]>) -> io::Result<()> {
    match s {
        None => w.write_all(&[1u8]),
        Some(bytes) => {
            w.write_all(&[0u8])?;
            if bytes.len() >= 0xff {
                w.write_all(&[0xff])?;
                w.write_all(&(bytes.len() as u32).to_le_bytes())?;
            } else {
                w.write_all(&[bytes.len() as u8])?;
            }
            w.write_all(bytes)
        }
    }
}

fn read_tree(r: &mut impl Read) -> io::Result<PropertyTree> {
    let mut hdr = [0u8; 2];
    r.read_exact(&mut hdr)?;
    let type_tag = hdr[0];
    // hdr[1] is any_type flag — we preserve it implicitly by always writing false on output
    match type_tag {
        0 => Ok(PropertyTree::Null),
        1 => {
            let mut b = [0u8; 1];
            r.read_exact(&mut b)?;
            Ok(PropertyTree::Bool(b[0] != 0))
        }
        2 => {
            let mut b = [0u8; 8];
            r.read_exact(&mut b)?;
            Ok(PropertyTree::Number(f64::from_le_bytes(b)))
        }
        3 => {
            let s = read_immutable_string(r)?;
            Ok(PropertyTree::String(s))
        }
        4 => {
            let count = read_u32(r)? as usize;
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                let key = read_immutable_string(r)?.unwrap_or_default();
                let val = read_tree(r)?;
                items.push((key, val));
            }
            Ok(PropertyTree::List(items))
        }
        5 => {
            let count = read_u32(r)? as usize;
            let mut items = Vec::with_capacity(count);
            for _ in 0..count {
                let key = read_immutable_string(r)?.unwrap_or_default();
                let val = read_tree(r)?;
                items.push((key, val));
            }
            Ok(PropertyTree::Dictionary(items))
        }
        6 => {
            let mut b = [0u8; 8];
            r.read_exact(&mut b)?;
            Ok(PropertyTree::SignedInt(i64::from_le_bytes(b)))
        }
        7 => {
            let mut b = [0u8; 8];
            r.read_exact(&mut b)?;
            Ok(PropertyTree::UnsignedInt(u64::from_le_bytes(b)))
        }
        t => Err(io::Error::new(io::ErrorKind::InvalidData, format!("unknown PropertyTree type {t}"))),
    }
}

fn write_tree(w: &mut impl Write, tree: &PropertyTree) -> io::Result<()> {
    let tag: u8 = match tree {
        PropertyTree::Null       => 0,
        PropertyTree::Bool(_)    => 1,
        PropertyTree::Number(_)  => 2,
        PropertyTree::String(_)  => 3,
        PropertyTree::List(_)    => 4,
        PropertyTree::Dictionary(_) => 5,
        PropertyTree::SignedInt(_)  => 6,
        PropertyTree::UnsignedInt(_) => 7,
    };
    w.write_all(&[tag, 0])?; // any_type always false on write
    match tree {
        PropertyTree::Null => {}
        PropertyTree::Bool(b) => w.write_all(&[*b as u8])?,
        PropertyTree::Number(n) => w.write_all(&n.to_le_bytes())?,
        PropertyTree::String(s) => write_immutable_string(w, s.as_deref())?,
        PropertyTree::List(items) | PropertyTree::Dictionary(items) => {
            w.write_all(&(items.len() as u32).to_le_bytes())?;
            for (key, val) in items {
                write_immutable_string(w, Some(key))?;
                write_tree(w, val)?;
            }
        }
        PropertyTree::SignedInt(i) => w.write_all(&i.to_le_bytes())?,
        PropertyTree::UnsignedInt(u) => w.write_all(&u.to_le_bytes())?,
    }
    Ok(())
}

fn read_u32(r: &mut impl Read) -> io::Result<u32> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)?;
    Ok(u32::from_le_bytes(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_roundtrip() {
        let settings = ModSettings::empty((2, 1, 12, 0));
        let bytes = settings.to_bytes().unwrap();
        let recovered = ModSettings::read(&mut bytes.as_slice()).unwrap();
        assert_eq!(settings, recovered);
    }

    #[test]
    fn example_roundtrip() {
        let original = include_bytes!("../reference/factorio-data-codec/example-mod-settings.dat");
        let settings = ModSettings::read(&mut original.as_slice()).unwrap();
        let reencoded = settings.to_bytes().unwrap();
        assert_eq!(original.as_slice(), reencoded.as_slice());
    }
}
