#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tape<Data>
where
    Data: Default,
{
    index: usize,
    mem: Vec<Data>,
}

impl<Data> Default for Tape<Data>
where
    Data: Default,
{
    fn default() -> Self {
        Self {
            index: 0,
            mem: vec![Data::default()],
        }
    }
}

impl<Data> Tape<Data>
where
    Data: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_data(mem: Vec<Data>) -> Self {
        Self::new_with_data_index(mem, 0)
    }

    pub fn new_with_data_index(mem: Vec<Data>, index: usize) -> Self {
        Self { index, mem }
    }

    pub fn write(&mut self, data: Data) {
        self.mem[self.index] = data;
    }

    pub fn read(&self) -> &Data {
        &self.mem[self.index]
    }

    pub fn move_right(&mut self) {
        self.index += 1;
        if self.index >= self.mem.len() {
            self.mem.push(Data::default())
        }
    }

    pub fn move_left(&mut self) {
        if self.index == 0 {
            self.mem.insert(0, Data::default())
        } else {
            self.index -= 1;
        }
    }

    pub const fn display<'a, State>(&'a self, state: &'a State) -> TapeDisplay<'a, Data, State>
    where
        Data: std::fmt::Display,
        State: std::fmt::Display,
    {
        TapeDisplay { tape: self, state }
    }
}

impl<Data: Default + std::fmt::Display> std::fmt::Display for Tape<Data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(&'>'))
    }
}

pub struct TapeDisplay<'a, Data, State>
where
    Data: Default + std::fmt::Display,
    State: std::fmt::Display,
{
    tape: &'a Tape<Data>,
    state: &'a State,
}

impl<'a, Data, State> std::fmt::Display for TapeDisplay<'a, Data, State>
where
    Data: Default + std::fmt::Display,
    State: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, elem) in self.tape.mem.iter().enumerate() {
            if idx == self.tape.index {
                write!(f, " {}", self.state)?;
            }
            write!(f, " {}", elem)?;
        }
        Ok(())
    }
}
