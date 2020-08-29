Difference structures
=====================

This crate implements a macro and a trait
that can be used to detect differences between
two structs of the same type.

Example:

```
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
assert!(diff.detailed().unwrap().surname.is_same());
assert!(!diff.detailed().unwrap().name.is_same());
```


