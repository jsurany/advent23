use crate::prelude::*;

fn to_digit(c: char, line: &str, idx: usize, rev: bool) -> Option<u32> {
    if let Some(n) = c.to_digit(10) {
        return Some(n);
    }

    let two_range = if rev {
        let rev_idx = line.len() - idx;
        (rev_idx).checked_sub(2)?..=rev_idx
    } else {
        idx..=idx+2
    };

    match line.get(two_range) {
        Some("one") => return Some(1),
        Some("two") => return Some(2),
        Some("six") => return Some(6),
        None => return None,
        _ => {}
    }

    let three_range = if rev {
        let rev_idx = line.len() - idx;
        (rev_idx).checked_sub(3)?..=rev_idx
    } else {
        idx..=idx+3
    };

    match line.get(three_range) {
        Some("four") => return Some(4),
        Some("five") => return Some(5),
        Some("nine") => return Some(9),
        None => return None,
        _ => {}
    }

    let four_range = if rev {
        let rev_idx = line.len() - idx;
        (rev_idx).checked_sub(4)?..=rev_idx
    } else {
        idx..=idx+4
    };

    match line.get(four_range) {
        Some("three") => Some(3),
        Some("seven") => Some(7),
        Some("eight") => Some(8),
        _ => None,
    }
}

pub(crate) fn run(buf: Buf) -> Result<()> {
    let mut total = 0;

    for line in buf.lines() {
        let line = line?;

        let first = line
            .chars()
            .enumerate()
            .find_map(|(idx, c)| to_digit(c, &line, idx, false))
            .ok_or_else(|| anyhow!("No first"))?;

        let last = line
            .chars()
            .rev()
            .enumerate()
            .find_map(|(idx, c)| to_digit(c, &line, idx, true))
            .unwrap_or(first);

        total += first * 10 + last;
    }

    println!("{total}");
    Ok(())
}
