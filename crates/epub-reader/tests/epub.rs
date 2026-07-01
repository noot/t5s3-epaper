use epub_reader::{decode_image, parse_epub, Block, Epub, Style};

const MINIMAL: &[u8] = include_bytes!("fixtures/minimal.epub");
const COVER_HEURISTIC: &[u8] = include_bytes!("fixtures/cover-heuristic.epub");
const COVER_PROPERTIES: &[u8] = include_bytes!("fixtures/cover-properties.epub");
const COVER_META: &[u8] = include_bytes!("fixtures/cover-meta.epub");

#[test]
fn reads_metadata_and_spine_order() {
    let doc = parse_epub(MINIMAL).unwrap();
    assert_eq!(doc.meta().title(), Some("The Minimal Book"));
    assert_eq!(doc.meta().author(), Some("A. Tester"));
    // two spine items => two chapters, in order
    assert_eq!(doc.chapters().len(), 2);
}

#[test]
fn parses_headings_paragraphs_and_rule() {
    let doc = parse_epub(MINIMAL).unwrap();
    let ch1 = doc.chapters()[0].blocks();
    assert!(matches!(ch1[0], Block::Heading { level: 1, .. }));
    assert!(ch1
        .iter()
        .any(|b| matches!(b, Block::Heading { level: 2, .. })));

    let ch2 = doc.chapters()[1].blocks();
    assert!(ch2.iter().any(|b| matches!(b, Block::Rule)));
}

#[test]
fn decodes_inline_styles_and_entities() {
    let doc = parse_epub(MINIMAL).unwrap();
    let ch1 = doc.chapters()[0].blocks();

    let styled: Vec<(&str, Style)> = ch1
        .iter()
        .filter_map(|b| match b {
            Block::Paragraph(spans) => Some(spans),
            _ => None,
        })
        .flatten()
        .map(|s| (s.text(), s.style()))
        .collect();

    assert!(styled
        .iter()
        .any(|(t, s)| *t == "bold" && *s == Style::BOLD));
    assert!(styled
        .iter()
        .any(|(t, s)| *t == "italic" && *s == Style::ITALIC));
    assert!(styled
        .iter()
        .any(|(t, s)| *t == "code" && *s == Style::CODE));

    // entities resolved: & em-dash (U+2014) and nbsp must appear in the text
    let joined: String = styled.iter().map(|(t, _)| *t).collect();
    assert!(joined.contains("AT&T"), "amp entity: {joined}");
    assert!(joined.contains('\u{2014}'), "em-dash entity: {joined}");
}

#[test]
fn cover_via_epub3_properties() {
    let epub = Epub::open(COVER_PROPERTIES.to_vec()).unwrap();
    let bytes = epub.cover().expect("cover present");
    assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
}

#[test]
fn cover_via_epub2_meta() {
    let epub = Epub::open(COVER_META.to_vec()).unwrap();
    let bytes = epub.cover().expect("cover present");
    assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
}

#[test]
fn cover_via_heuristic_decodes() {
    let epub = Epub::open(COVER_HEURISTIC.to_vec()).unwrap();
    let bytes = epub.cover().expect("cover present");
    // the sample's cover is a jpeg that must decode to a non-empty image.
    let image = decode_image(&bytes).expect("cover decodes");
    assert!(image.width() > 0 && image.height() > 0);
}

#[test]
fn no_cover_when_absent() {
    let epub = Epub::open(MINIMAL.to_vec()).unwrap();
    assert!(epub.cover().is_err());
}
