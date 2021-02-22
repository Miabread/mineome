use crate::internal_prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Selector {
    pub base: SelectorBase,
    pub arguments: Vec<SelectorArgument>,
}

#[derive(Debug, Clone)]
pub enum SelectorBase {
    This,
    Player,
    Random,
    AllPlayers,
    Everything,
}

#[derive(Debug, Clone)]
pub struct SelectorArgument {
    pub fragment: SelectorFragment,
    pub inverted: bool,
}

#[derive(Debug, Clone)]
pub enum SelectorFragment {
    X(f64),
    Y(f64),
    Z(f64),
    Distance(SelectorRange<i32>),
    DifferenceX(f64),
    DifferenceY(f64),
    DifferenceZ(f64),
    Scores(HashMap<String, SelectorRange<i32>>),
    Tag(String),
    Team(String),
    Limit(i32),
    Sort(SelectorSort),
    Level(SelectorRange<i32>),
    Gamemode(Gamemode),
    Name(String),
    XRotation(SelectorRange<f64>),
    YRotation(SelectorRange<f64>),
    Type(NamespacedId),
    Nbt,
    Advancements(HashMap<NamespacedId, SelectorAdvancements>),
    Predicate(NamespacedId),
}

#[derive(Debug, Clone, Copy)]
pub enum SelectorRange<T> {
    Number(T),
    Range { start: Option<T>, end: Option<T> },
}

#[derive(Debug, Clone, Copy)]
pub enum SelectorSort {
    Arbitrary,
    Nearest,
    Furthest,
    Random,
}

#[derive(Debug, Clone, Copy)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

#[derive(Debug, Clone)]
pub enum SelectorAdvancements {
    General(bool),
    Criteria(HashMap<NamespacedId, bool>),
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}[", self.base)?;
        for arg in &self.arguments {
            write!(f, "{},", arg)?;
        }
        write!(f, "]")
    }
}

impl Display for SelectorBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::This => write!(f, "@s"),
            Self::Player => write!(f, "@p"),
            Self::Random => write!(f, "@r"),
            Self::AllPlayers => write!(f, "@a"),
            Self::Everything => write!(f, "@e"),
        }
    }
}

impl Display for SelectorArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        use SelectorFragment::*;

        let mut name = |name| {
            if self.inverted {
                write!(f, "{}=!", name)
            } else {
                write!(f, "{}=", name)
            }
        };

        match &self.fragment {
            X(num) => {
                name("x")?;
                write!(f, "{}", num)
            }
            Y(num) => {
                name("y")?;
                write!(f, "{}", num)
            }
            Z(num) => {
                name("z")?;
                write!(f, "{}", num)
            }
            Distance(ref range) => {
                name("distance")?;
                write!(f, "{}", range)
            }
            DifferenceX(num) => {
                name("dx")?;
                write!(f, "{}", num)
            }
            DifferenceY(num) => {
                name("dy")?;
                write!(f, "{}", num)
            }
            DifferenceZ(num) => {
                name("dz")?;
                write!(f, "{}", num)
            }
            Scores(scores) => {
                write!(f, "{{")?;
                for (name, range) in scores {
                    write!(f, "{}={},", name, range)?;
                }
                write!(f, "}}")
            }
            Tag(tag) => {
                name("dz")?;
                write!(f, "{}", tag)
            }
            Team(team) => {
                name("dz")?;
                write!(f, "{}", team)
            }
            Limit(num) => {
                name("limit")?;
                write!(f, "{}", num)
            }
            Sort(sort) => {
                name("sort")?;
                write!(f, "{}", sort)
            }
            Level(range) => {
                name("level")?;
                write!(f, "{}", range)
            }
            Gamemode(mode) => {
                name("level")?;
                write!(f, "{}", mode)
            }
            Name(text) => {
                name("level")?;
                write!(f, "{}", text)
            }
            XRotation(num) => {
                name("level")?;
                write!(f, "{}", num)
            }
            YRotation(num) => {
                name("level")?;
                write!(f, "{}", num)
            }
            Type(text) => {
                name("level")?;
                write!(f, "{}", text)
            }
            Nbt => todo!(),
            Advancements(goals) => {
                use SelectorAdvancements::*;

                write!(f, "{{")?;
                for (name, goal) in goals {
                    match goal {
                        General(flag) => write!(f, "{}={},", name, flag)?,
                        Criteria(map) => {
                            write!(f, "{}={{", name)?;
                            for (name, flag) in map {
                                write!(f, "{}={},", name, flag)?;
                            }
                            write!(f, "}},")?;
                        }
                    }
                }
                write!(f, "}}")
            }
            Predicate(pred) => {
                name("level")?;
                write!(f, "{}", pred)
            }
        }
    }
}

impl<T: Display> Display for SelectorRange<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Number(num) => num.fmt(f)?,
            Self::Range { start, end } => {
                if let Some(start) = start {
                    start.fmt(f)?
                }
                write!(f, "..")?;
                if let Some(end) = end {
                    end.fmt(f)?
                }
            }
        };
        Ok(())
    }
}

impl Display for SelectorSort {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Arbitrary => write!(f, "arbitrary"),
            Self::Nearest => write!(f, "nearest"),
            Self::Furthest => write!(f, "furthest"),
            Self::Random => write!(f, "random"),
        }
    }
}

impl Display for Gamemode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Survival => write!(f, "survival"),
            Self::Creative => write!(f, "creative"),
            Self::Adventure => write!(f, "adventure"),
            Self::Spectator => write!(f, "spectator"),
        }
    }
}
