pub mod derive {
    pub use diff_derive::Diffable;
}

#[derive(Debug)]
pub enum Diff<'t, T, S> {
    /// The value of T has changed, this variant contains both the old and the new value
    Changed(&'t T, &'t T),
    /// The value of T hasn't changed
    Same,
    /// A detailed diff is given by the given type
    Detailed(S),
}

impl<'t, T, S> Diff<'t, T, S> {
    pub fn is_same(&self) -> bool {
        match self {
            Diff::Same => true,
            _ => false
        }
    }

    pub fn detailed(&self) -> Option<&S> {
        match self {
            Diff::Detailed(ref s) => Some(s),
            _ => None
        }
    }
}

pub trait Diffable<'t> {
    type Detailed;
    fn diff(&'t self, other: &'t Self) -> Diff<'t, Self, Self::Detailed>
    where
        Self: Sized;
}

/// Convience implementation for any type that implements Eq.
/// This is a very crude diff as it (for example for a struct) only
/// shows if the struct in its entirety has changed, and not which
/// fields has been changed. In order to generate a `Diffable` struct
/// you can use the derive macro.
impl<'t, T> Diffable<'t> for T
where
    T: Eq,
{
    type Detailed = T;
    fn diff(&'t self, other: &'t T) -> Diff<'t, T, Self::Detailed> {
        if self == other {
            Diff::Same
        } else {
            Diff::Changed(self, other)
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::derive::Diffable;
    use crate::*;

    #[derive(Diffable, Debug)]
    struct Person {
        name: String,
        surname: String,
    }

    #[derive(Diffable, Debug)]
    struct SpecialPerson {
        country: String,
        person: Person,
    }

    #[test]
    fn test_diff_reflexive() {
        let person = Person {
            name: "John".to_owned(),
            surname: "Doe".to_owned(),
        };

        assert!(person.diff(&person).is_same());
    }

    #[test]
    fn test_diff_firstname() {
        let person1 = Person {
            name: "John".to_owned(),
            surname: "Doe".to_owned(),
        };

        let person2 = Person {
            name: "Jane".to_owned(),
            surname: "Doe".to_owned(),
        };

        let diff = person1.diff(&person2);
        assert!(!diff.is_same());
        assert!(!diff.detailed().unwrap().name.is_same());
        assert!(diff.detailed().unwrap().surname.is_same());

    }

    #[test]
    fn test_diff_primitives() {
        let s1 = "hello";
        let s2 = "world";
        assert!(!s1.diff(&s2).is_same());

        let i1  = 42;
        let i2  = 42;
        assert!(i1.diff(&i2).is_same());
    }

    #[test]
    fn test_nested_diff() {
        let person1 = Person {
            name: "John".to_owned(),
            surname: "Doe".to_owned(),
        };

        let person2 = Person {
            name: "Jane".to_owned(),
            surname: "Doe".to_owned(),
        };

        let sp1 = SpecialPerson {
            country: "WonderfulCountry".to_owned(),
            person: person1,
        };

        let sp2 = SpecialPerson {
            country: "WonderfulCountry".to_owned(),
            person: person2,
        };

        let d = sp1.diff(&sp2);
        assert!(!d.is_same());
        let details = d.detailed().unwrap();
        assert!(details.country.is_same());
        assert!(!details.person.is_same());

        let person_details = details.person.detailed().unwrap();
        assert!(!person_details.name.is_same());
        assert!(person_details.surname.is_same());
    }

}
