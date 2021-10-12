= Introduction to Programming 2: Getting started
:keywords: JavaScript, Tutorial
:date: 2018-07-17T19:34:04Z

How do you get started programming? You press control+shift+i if your on Chrome, Firefox or Edge. To open the developer tools for Safari I Googled it for you: [http://wickedlysmart.com/hfjsconsole/](http://wickedlysmart.com/hfjsconsole/ "http://wickedlysmart.com/hfjsconsole/"). This brings up the developer tools, your new home for the first section of this tutorial. Next you’ll want to navigate to the console. This is where we’ll be typing in expressions.

Hello world in JavaScript is about as easy as could be. At the console type: `console.log('Hello World');`

Then press enter. You should see a little message in the console with the sentence you entered. Try a few more things (one line at a time, pressing enter after each line) before we get started:

```javascript
alert('I get annoying pretty quick');
setTimeout(() => console.log("... And I'm back"), 3000);
let x = 70;
console.log(x);
x = 80;
console.log(x);
x += 10;
console.log(x);
x = 'Good luck Johnny?';
console.log(x);
```

***

**console:** Console is a Global object that has a bunch of useful methods, by far the most useful being log.  
**.:** The dot is an operator. JavaScript has lots of operators, this one let’s you access a property on an object. If you’ve used other programming languages then you might be familiar with the term method. In JavaScript, since functions are objects too, a method is just a property that is a function. We’ll talk about functions in a little bit.  
**‘Hello World’:** In JavaScript speak, this is a string literal. A string is a list of characters. In JavaScript you can use all kinds of characters in a string ranging from regular letters, to numbers, to other characters like emoji or symbols like: ❄ (the snowflake character).  
**log:** log is a function which prints all kinds of things to the console. In this case we are printing a string.  
**():** In this case, this is a function call. The parenthesis are used all over the place to indicate function calls and order of operations (just like in math, except that you use only parenthesis and not brackets(\[\]) or squiggly brackets ({}) as these have different meanings — you just keep nesting parenthesis). Inside the parenthesis are the parameters to the function. Parameters are the inputs to the function and they are separated by a comma.

Syntax is the rules for a language. A programming language’s syntax is defined by a mathematical grammar which is a bit like a regular grammar. The grammar that defines JavaScript determines what letters when combined together are a valid program, just like English’s grammar determines what strings of letters and punctuation are valid sentences. A Syntax Error occurs when we write a program which is not valid JavaScript. As a new programmer, it will doubtless be the most common mistake you make.

Sometimes however, you write a valid JavaScript program but it doesn’t do what you want it to. This is a logical error, or bug. These are much more difficult to find. Debugging is a skill of its own which has lots of techniques we might discuss later.

***

Arithmetic and Logic are the core of programming. A CPU is mostly a supporting shell with an Arithmetic and Logic Unit (ALU) as its heart. This makes sense if your fundamental models of computation come from math.

In JavaScript you can perform any math operation that you would expect. Add with “+” subtract with “-”, divide with “/”, multiply with “*”, and modulus with “%”. Modulus? What’s Modulus? Modulus is just taking the remainder. Go ahead and try a few expressions in the console:
```javascript
6*5
7*1000000000 + 2
3/4 + 10
50 / 0
"Joe Bob" + 9
"3" + 2
```

Why does JavaScript return Infinity for 50 / 0? JavaScript uses a specific format for all of it’s numbers, it’s called 64 bit floating point and I believe it’s standardized by IEEE. What we need to know is that there are a few special “Numbers” that JavaScript knows about: Infinity, -Infinity, and NaN. NaN refers to a Number object that isn’t actually a number (NaN = Not a Number).

What about that Strings plus Numbers thing? Well, JavaScript did something that you might or might not have wanted it to do. If it can it will convert the string to a number and add the two numbers, but when it can’t it reverts to _string addition_ or concatenation — squash the two strings together (hence the “Joe Bob9”).

***

Let’s use something more interesting. Let’s use something to remember the results of our expressions. Let’s introduce variables. Variables are places to store data. They are created using the **let** or **const** keywords (A keyword is a word which has special meaning to JavaScript. We’ll be seeing more of them later.). When you declare a variable you give it a name. That name is an identifier. You can play around with what are valid identifiers here: [https://mothereff.in/js-variables](https://mothereff.in/js-variables "https://mothereff.in/js-variables") but suffice it to say that you’re all clear if you avoid keywords, have no spaces (programmers often substitute underscores or hyphens instead), and only have numbers after the first letter. Variables can be assigned values using the equals operator:
```javascript
let x = 5;
console.log(x);
x = 6;
console.log(x);
```

This is a fundamental difference from math’s notation that is important to understand. When we use the equal sign in a program we are replacing the value that was in the variable with the value that we’re assigning to it. When we create a variable with const we are saying that it can never be assigned a new value (it is constant). JavaScript will bark at us if we try:
```javascript
const c = 10;
c = 7;
```

Every variable has a type. What are types? A type is a format for storing data. For example, Number is a type, String is a type, Date is a type (we’ll use dates later because I really love Javascript’s Date type), and we can even create our own types. JavaScript, however, is not a strictly typed language. That is to say that a variable (or function return value, or function paramter, or many other data spots), can be assigned a value of any type — even one that’s different from what it had previously:
```javascript
let number = 70;
console.log(number);
number = "This is not a number";
console.log(number);
```

This is often a source of bugs, so much so that many people are advocating for alternative languages which are just like JavaScript except with a way of specifying what type a variable is. [Flow](https://flow.org/en/) and [TypeScript](https://www.typescriptlang.org/) are the two most popular of this kind of project. Dynamic Typing (which is what JavaScript has) lets you hang yourself in a lot of ways, but it is easier to get started up with, requires less typing, and makes complex data structures (like unions) easy— though fragile.

There, now you can do some interesting things… You’re right we need… Functions! Programming is largely about creating functions. Functions are created using the function keyword:

`function <identifier>(<parameterList>) {  <statements>}`

Try entering this one into your terminal:
```javascript
function add(a, b) {
	return a + b;
}
```

Then use it like this: `add(50, 56);`

Just like with console.log, we pass the function a list of parameters. Inside the function, “a” and “b” are variables that are assigned with 50 and 56 automatically. We then return the value of their sum.

Now let’s use the function’s returned value in an assignment to another variable:
```javascript
let x = add(50, 58);
console.log(x);
```

See? You can use a called function just like you would any other value:
```javascript
let y = add(40, 20) + 34;
```

This is called evaluation. The function was evaluated with 40 and 20 or whatever else you pass in for “a” and “b”.

Now let’s try mixing the uses for parenthesis:
```javascript
console.log(add(50, 60) * (7+6));
```

Two of those pairs of parenthesis are for function calls (the console.log(…) and add(…)) while the other pair is for order of operations. Operators (+, -, and lots of others) have a priority in which they are applied. This is usually easy to understand in JavaScript math expressions but can become more difficult when you’re working with the other operators. Just like in math, there’s no issue in using too many parenthesis. While you could memorize all the order of operations I would recommend being explicit when ever you’re slightly unsure. Future you, or a younger programmer, or I will thank you for it.

One more example, let’s use a variable inside our function to show off a more realistic use case:
```javascript
function circle_area(diameter) {
	let radius = diameter / 2;
	return Math.PI * Math.pow(radius, 2);
}
```

I know, I threw a bunch of stuff at you when you thought you were almost done. Sorry. Math.pow is a function which raises parameter 1 to the power of parameter 2 and returns it. JavaScript is probably going to get a better way of doing that in the future but for now that’s what we have to do. Math.PI is just a variable (ok, property on the Math variable) that holds the value of pi. Other than that, it should look pretty familier. Give it a try: `circle_area(2);`

So now you know how to use basic arithmetic, and put that arithmetic into a function, and call that function, and use that function’s result in more arithmetic. That’s a lot of information! A triumphant shout and bubble bath is the appropriate response for you’re accomplishment. After you dry off, consider writing these functions:
```javascript
celsius_to_fahrenheit(fahrenheit)
cups_to_pints(cups)
```

Bonus: I’d like to introduce you to what is almost certainly the greatest JavaScript reference on the internet: The [Mozilla Developer Network](https://developer.mozilla.org/en-US/docs/Web/JavaScript). They have a better tutorial than this one too, so you might want to go check it out. There is also a list of all of the functions in the Math object: [https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math")
