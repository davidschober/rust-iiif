use crate::iiif::types::*;

pub fn parse_region(s: &str) -> Option<Region> {
    if s == "full" {
        return Some(Region::Full);
    }
    if s == "square" {
        return Some(Region::Square);
    }

    if let Some(rest) = s.strip_prefix("pct:") {
        let parts: Vec<&str> = rest.split(',').collect();
        if parts.len() == 4 {
            let x = parts[0].parse().ok()?;
            let y = parts[1].parse().ok()?;
            let w = parts[2].parse().ok()?;
            let h = parts[3].parse().ok()?;
            return Some(Region::Percentage(x, y, w, h));
        }
    } else {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 4 {
            let x = parts[0].parse().ok()?;
            let y = parts[1].parse().ok()?;
            let w = parts[2].parse().ok()?;
            let h = parts[3].parse().ok()?;
            return Some(Region::Absolute(x, y, w, h));
        }
    }
    None
}

pub fn parse_size(s: &str) -> Option<Size> {
    if s == "max" {
        return Some(Size::Max);
    }
    if s == "^max" {
        return Some(Size::ScaleAsFull);
    }
    if let Some(rest) = s.strip_prefix("pct:") {
        let n: f64 = rest.parse().ok()?;
        return Some(Size::Percentage(n));
    }
    if s.contains(',') {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() == 2 {
            match (parts[0], parts[1]) {
                ("", h) => return Some(Size::Height(h.parse().ok()?)),
                (w, "") => return Some(Size::Width(w.parse().ok()?)),
                (w, h) => {
                    let w_val = w.parse().ok()?;
                    let h_val = h.parse().ok()?;
                    return Some(Size::WidthHeight(w_val, h_val));
                }
            }
        }
    }
    // TODO: Handle !w,h
    None
}

pub fn parse_rotation(s: &str) -> Option<Rotation> {
    if let Some(rest) = s.strip_prefix('!') {
        let degrees = rest.parse().ok()?;
        return Some(Rotation { degrees, mirror: true });
    }
    let degrees = s.parse().ok()?;
    Some(Rotation { degrees, mirror: false })
}

pub fn parse_quality(s: &str) -> Option<Quality> {
    match s {
        "default" => Some(Quality::Default),
        "color" => Some(Quality::Color),
        "gray" => Some(Quality::Gray),
        "bitonal" => Some(Quality::Bitonal),
        _ => None,
    }
}

pub fn parse_format(s: &str) -> Option<Format> {
    match s {
        "jpg" => Some(Format::Jpg),
        "png" => Some(Format::Png),
        "tif" => Some(Format::Tif),
        "webp" => Some(Format::Webp),
        "gif" => Some(Format::Gif),
        "pdf" => Some(Format::Pdf),
        _ => None,
    }
}
