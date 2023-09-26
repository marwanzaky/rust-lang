## Statements and Expressions
Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.
 
Creating a variable and assigning a value to it with the 'let' keyword is a statement. let y = 6; is a statement.
Statements do not return values. Therefore, you can’t assign a let statement to another variable,
as the following code tries to do; you’ll get an error:

``` rust
fn main() {
    let x = (let y = 6);
}
```

The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.
In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust.
Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6.
Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

``` rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

This expression:

``` rust
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement.
Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far.
Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement,
and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

## Functions with Return Values
Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

``` rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```