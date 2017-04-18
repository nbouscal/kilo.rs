use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Flag {
    HighlightNumbers,
}

pub type Flags = HashSet<Flag>;

pub struct Syntax {
    pub filetype: &'static str,
    pub filematch: Vec<&'static str>,
    pub flags: Flags,
}

impl Syntax {
    fn database() -> Vec<Self> {
        let mut db = Vec::new();
        db.push(Syntax {
            filetype: "c",
            filematch: vec![".c", ".h", ".cpp"],
            flags: [Flag::HighlightNumbers].iter().cloned().collect(),
        });
        db
    }

    pub fn for_filename(filename: &str) -> Option<Self> {
        for s in Self::database().into_iter() {
            let res = s.filematch.iter()
                .map(|ext| filename.rfind(ext))
                .enumerate()
                .find(|&(_, opt)| opt.is_some());
            match res {
                Some((match_idx, Some(name_idx))) => {
                    let matched = s.filematch[match_idx];
                    if matched.chars().next().unwrap() == '.' ||
                        name_idx + matched.len() == filename.len() {
                        return Some(s)
                    }
                },
                Some((_, None)) => unreachable!(),
                None => continue,
            }
        }
        None
    }
}

