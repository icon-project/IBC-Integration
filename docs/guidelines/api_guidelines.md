 ## Summary

 In API documentation, the first line should be a single-line short sentence providing a summary of the code. This line is used as a summary description throughout Rustdoc’s output, so it’s a good idea to keep it short.

The summary line should be written in third person singular present indicative form. Basically, this means write ‘Returns’ instead of ‘Return’.

### Motivation
[Motivation]:#motivation

Documentation is an extremely important part of any project. It's important that we have consistency in 
our project. When you're documenting an API, provide a complete API reference, typically generated from source code using doc comments that describe all public classes, methods, constants, and other members.

### Detailed Design Guidelines
[Destailed-Design-Guidelines]:#detailed-design-guidelines

- Put all API names, classes, methods, constants, etc. in code font, and link each name to the corresponding reference page
- Put parameter names in italic. For example, when you refer to the parameters of a method like  ` doSometing(Uri data, int count) ` , italicize the name *data* and *count*.
- Make sure that the spelling of a class name in documentation matches the spelling in code, with capital letters and no spaces. (for example, **ActionBar**)
  
### Interfaces and structs
[Interfaces-and-structs]:#interfaces-and-structs

In the first sentence of class description, breifly state the intended purpose or function or class or interface with infomation that can't be deducted from the classname and signature. In additional documentation, elaborate on how to use the API, including how to invoke or instantiate it, what some of the key features are, and any best practices or pitfalls.

The following example is the first sentence of the description - [Rectangle](https://doc.rust-lang.org/book/ch05-02-example-structs.html)

### Members
[Members]:#members

Make descriptions for members (constants and fields) as brief as possible. Be sure to link to relevant methods that use the constant or field. For example here is the description for Struct [**Rectangle**](https://doc.rust-lang.org/book/ch05-02-example-structs.html) where it has fields like *width* and *height*.

### Methods
[Methods]:#methods

In the first sentence for a method description, briefly state what action the method performs. In subsequent sentences, explain why and how to use the method, state any prerequisites that must be met before calling it, give details about exceptions that may occur, and specify any related APIs.

For example, here's the description for [**fn area(&self) -> u32 {}**](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) method.

Use present tense for all the descriptions - for example
- Calculates the area
- Returns an area

### Descriptions
[Descriptions]: #descriptions

- If a method performs an operation and returns some data, start the description with a verb describing the operation
-for example:
  - Multiplies the height and width of the Rectangle and returns the area.
- If it's a "getter" method and it returns a boolean, start with "Checks whether ...."
- If it's a "getter" method and it returns something other than a boolean, start with "Gets the ...."
- If it has no return value, start with a verb like one of the following:
  - Turning on an ability or setting: "Sets the ...."
  - Updating a property: "Updates the ...."
  - Deleting something: "Deletes the ...."

### Use line comments
[use-line-comments]: #use-line-comments

Avoid block comments. Use line comments instead:

```rust
// Wait for the main task to return, and set the process error code
// appropriately.
```

Instead of:

```rust
/*
 * Wait for the main task to return, and set the process error code
 * appropriately.
 */
```

Only use inner doc comments `//!` to write crate and module-level documentation,
nothing else. When using `mod` blocks, prefer `///` outside of the block:

```rust
/// This module contains tests
mod tests {
    // ...
}
```

over

```rust
mod tests {
    //! This module contains tests

    // ...
}
```

### Using Markdown
[using-markdown]: #using-markdown

Within doc comments, use Markdown to format your documentation.

Use top level headings (`#`) to indicate sections within your comment. Common headings:

* Examples
* Errors

An example:

```rust
/// # Examples
```

Even if you only include one example, use the plural form: ‘Examples’ rather than ‘Example’. Future tooling is easier this way.

Use backticks (`) to denote a code fragment within a sentence.

Use triple backticks (```) to write longer examples, like this:

    This code does something cool.

    ```rust
    let x = foo();

    x.bar();
    ```

In API documentation, feel free to rely on the default being ‘rust’:

    /// For example:
    ///
    /// ```
    /// let x = 5;
    /// ```

In long-form documentation, always be explicit:

    For example:

    ```rust
    let x = 5;
    ```

This will highlight syntax in places that do not default to ‘rust’, like GitHub.

Rustdoc is able to test all Rust examples embedded inside of documentation, so
it’s important to mark what is not Rust so your tests don’t fail.

References and citation should be linked ‘reference style.’ Prefer

```
[Rust website]

[Rust website]: http://www.rust-lang.org
```

to

```
[Rust website](http://www.rust-lang.org)
```

If the text is very long, feel free to use the shortened form:

```
This link [is very long and links to the Rust website][website].

[website]: http://www.rust-lang.org
```

### Examples in API docs
[examples-in-api-docs]: #examples-in-api-docs

Everything should have examples. Here is an example:

```
/// # Examples
///
/// ```
/// use op;
///
/// let s = "foo";
/// let answer = op::compare(s, "bar");
/// ```
///
/// Passing a closure to compare with, rather than a string:
///
/// ```
/// use op;
///
/// let s = "foo";
/// let answer = op::compare(s, |a| a.chars().is_whitespace().all());
/// ```
```
### Refering to types

When talking about the types, use its full name. In other words, if the type is generic, say `Option<T>` not `Option`. An exception to this is bounds. Write `Cow<'a, B>`rather than `Cow<'a, B> where B: 'a + ToOwned + ?Sized`.

Another possibility is to write in lower case using a more generic term. In other words,
‘string’ can refer to a `String` or an `&str`, and ‘an option’ can be ‘an `Option<T>`’.

### Link all the things
[link-all-the-things]: #link-all-the-things

A major drawback of Markdown is that it cannot automatically link types in API documentation.
Do this yourself with the reference-style syntax, for ease of reading:

```
/// The [`String`] passed in lorum ipsum...
///
/// [`String`]: ../string/struct.String.html
```










