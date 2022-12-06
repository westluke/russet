// this is stupid and wrong, it's not actually optimizing at all
pub struct LineUpdate {
    cs: Vec<LayerCell>
}

impl Default for LineUpdate {
    fn default() -> Self {
        Self { cs: Default::default() }
    }
}

impl LineUpdate {
    pub fn new(length: i16) -> Self {
        Self { cs: vec![Transparent; usize::try_from(length).unwrap()] }
    }

    pub fn set(&mut self, i: i16, c: LayerCell) {
        self.cs[usize::try_from(i).unwrap()] = c;
    }

    // Returns number of characters consumed to find start, Termable produced (if any)
    fn first_termable(&mut self) -> (i16, Option<Termable>) {
        let mut term = None;
        let mut cons = 0;

        for i in 0..self.cs.len() {
            let c_opt = self.cs[i];

            match (&mut term, c_opt) {
                (None, Transparent) => {
                    cons += 1;
                },
                (None, Opaque(c)) => {
                    term = Some(Termable::from(c));
                },
                (Some(_), Transparent) => {
                    self.cs.drain(0..i);
                    return (cons, term);
                },
                (Some(t), Opaque(c)) => {
                    if !t.push(c) {
                        self.cs.drain(0..i);
                        return (cons, term);
                    };
                }
            };
        };

        self.cs.drain(0..);
        (cons, term)
    }

    // Outputs a vector of pairs (i, cont) where cont is a StyledContent ready to be Print'd,
    // and i is the column where cont should be printed
    pub fn finalize(mut self) -> Vec<(i16, StyledContent<Termable>)> {
        let mut out = Vec::new();

        let (mut cons, mut term) = self.first_termable();
        let mut last = 0;

        while let Some(t) = term {
            let len = t.len();
            out.push((last + cons, t.finalize()));
            last += cons + len;
            (cons, term) = self.first_termable();
        };

        out
    }
}

