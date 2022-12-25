

String literals `(&str)` are used when the value of a string is known at compile time. String literals are a set of characters, which are hardcoded into a variable. String literals are static by default. This means that string literals are guaranteed to be valid for the duration of the entire program.
```rs
let name:&str="Myname";
```

The `String` object type is provided in Standard Library. Unlike string literal, the string object type is not a part of the core language. It is defined as public structure in standard library pub struct String. String is a growable collection. It is mutable and UTF-8 encoded type. The String object type can be used to represent string values that are provided at runtime. String object is allocated in the heap.
```rs
String::new()   // creates an empty string
    name.push_str("");
String::from()  // creates a string with some default value passed as parameter to the from() method.
```


[String Methods](http://web.mit.edu/rust-lang_v1.26.0/arch/amd64_ubuntu1404/share/doc/rust/html/std/string/struct.String.html)