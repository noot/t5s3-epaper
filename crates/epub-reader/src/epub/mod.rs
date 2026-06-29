mod container;
mod opf;
mod xhtml;
mod zip;

use alloc::{format, string::String, vec::Vec};

use crate::{
    document::{Chapter, Document, Meta},
    error::Error,
};

/// Parse the bytes of an `.epub` file into a [`Document`], one chapter per
/// spine item in reading order.
///
/// # Errors
///
/// Returns an [`Error`] if the archive, its container, or its OPF package
/// cannot be read, or [`Error::Empty`] if no spine item yielded any text.
/// Individual spine items that are missing or fail to parse are skipped.
pub fn parse(bytes: &[u8]) -> Result<Document, Error> {
    let archive = zip::Archive::open(bytes)?;
    let container = archive.read("META-INF/container.xml")?;
    let opf_path = container::opf_path(&container)?;
    let package = opf::parse(&archive.read(&opf_path)?)?;
    let base = dir_of(&opf_path);

    let mut chapters = Vec::new();
    for href in &package.spine_hrefs {
        let path = resolve(base, href);
        let Ok(data) = archive.read(&path) else {
            continue;
        };
        let Ok(blocks) = xhtml::parse(&data) else {
            continue;
        };
        if !blocks.is_empty() {
            chapters.push(Chapter::new(None, blocks));
        }
    }

    if chapters.is_empty() {
        return Err(Error::Empty);
    }
    Ok(Document::new(
        Meta::new(package.title, package.author),
        chapters,
    ))
}

fn dir_of(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(dir, _)| dir)
}

// resolve a manifest href (relative to the OPF directory) to a normalized,
// percent-decoded archive path, dropping any fragment.
fn resolve(base: &str, href: &str) -> String {
    let href = href.split('#').next().unwrap_or(href);
    let combined = if let Some(rest) = href.strip_prefix('/') {
        String::from(rest)
    } else if base.is_empty() {
        String::from(href)
    } else {
        format!("{base}/{href}")
    };
    normalize(&percent_decode(&combined))
}

fn normalize(path: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                out.pop();
            }
            segment => out.push(segment),
        }
    }
    out.join("/")
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}
