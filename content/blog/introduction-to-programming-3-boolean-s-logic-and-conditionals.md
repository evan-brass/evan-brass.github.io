+++
title = "Introduction to Programming 3: Boolean’s, Logic, and Conditionals"
date = 2018-07-19T19:46:27Z

[taxonomies]
tags = [
	"JavaScript",
	"Tutorial"
]

[extra]
canonical = ""
alternate = []
+++
Till now, I’ve given you no method of telling the computer how to make decisions. You currently don’t have a Turing Complete understanding of JavaScript. Turing Complete means that a machine or language has the functionality required to compute anything that a Turing Machine can compute. We still won’t have a Turing Complete knowledge at the end of this section but we’ll be pretty close.

The Boolean is a data type with two values, True and False. Boolean’s are a core component of logic. Logic spreads over both math and philosophy. Logic — implemented by electrical components — is what engineers use to build computers. The circuits that they design combine little logic gates together into circuits that can be represented by Boolean Expressions.

Anyway, there are three fundamental logical operators: And, Or, and Not. We use something called a truth table to describe them. Each operator has two inputs and one output except the Not which has only one input. Look at the tables here:
```
|          AND           |
|   A   |   B   | Output |
| True  | True  |  True  |
| True  | False |  False |
| False | True  |  False |
| False | False |  False |

|           OR           |
|   A   |   B   | Output |
| True  | True  |  True  |
| True  | False |  True  |
| False | True  |  True  |
| False | False |  False |

|      Not       |
|   A   | Output |
| True  | False  |
| False | True   |
```

The notation that we use in JavaScript (and most languages) for these operators are: && (AND), || (OR), ! (Not). Once again, we can use parenthesis to indicate the order that we want these expressions to be evaluated. Ok, enough talk, time for some examples. Open your console and try these expressions:
```javascript
true;
false;
true && false; // This will be false
true || false; // This will be true
!true;
!(true && true) // NOT (true AND true)
```

Now let’s talk about equality. JavaScript has many comparing operators — less than, greater then, less than or equal to, greater than or equal to, equal to, and not equal to. Each of these operators will take numbers and return a boolean, but the last two will also take other data types. Try these:
```javascript
4 < 5; // Less Than. True
5 <= 9; // Less Than or Equal to. False
11 > 7; // Greater Than. True
(7 + 3) >= 10; // Greater Than or Equal to. True
6 == 6; // Equal to. True
Math.pow(5, 2) != 25; // Not Equal to. False

"Hello World" == 'Hello World'; // True
`Hello World` == 'Hello World'; // True
'cat' != 'dog'; // True
```

We can combine functions, comparison, variables, and boolean operators for a more interesting example. Also I should mention that the // indicates a comment. Everything after the two forward slashes until the end of that line of code will be ignored by the computer. It’s a way of annotating your code for yourself and others.

```javascript
function is_divisible_by_5(x) {
	return ((x % 5) == 0);
}
```

This function takes the remainder of it’s parameter (x) when divided by 5 and tests to see if it is 0. If it is then the number is divisible by 5 and the equality test (==) will return true. Go ahead and call it with a few numbers.

The last new information for this lesson is to perform operations based on whether or not a boolean expression is true or false. This is done using the if and else clauses. An if looks a bit like a function definition:

```
if (<boolean expression>) {
	<statements>
}
```

The statements inside the if’s block (blocks are statements inside of the squiggle braces: {…}) will only be executed if the if’s boolean expression is true. If you want to do one thing when is true and another when it’s false you use an if-else clause:

```
if (<boolean expression>) {
	<statements when true>
} else {
	<statements when false>
}
```

Let’s use this in a function. This one is for those people who’s favorite number is 8:

```javascript
function guess_favorite(number) {
	if (number == 8) {
	console.log('You guessed correctly.');
	alert('Congratulations!');
	} else {
	console.log('Sorry, ' + number + ' is not my favorite number. Guess again.');
	}
}
```

This function combines a lot of different things that we’ve learned. It’s got printing, an if-else clause, and an equality test. Note that the else is always optional.

Ok, usually when you’re programming you want to get something simple working and then start tinkering with it. Let’s add a way of telling people how close they are:

```javascript
function guess_favorite(number) {
	// By storing our favorite number in a variable we can change it in all the places quite easily... If that were to ever happen.
	const favorite = 8;
	// Check if they guessed correctly
	if (number == favorite) {
	console.log('You guessed correctly.');
	alert('Congratulations!');
	} else {
	// If they didn't then get the absolute value of the difference between their guess and our favorite
	let difference = Math.abs(number - favorite);
	// If none of the following if/else if statments are true, then the default will be way off.
	let closeness = 'way off';
	if (difference < 3) {
		closeness = 'really close';
	} else if (difference < 10) {
		closeness = 'pretty close';
	} else if (difference < 20) {
		closeness = 'not very close';
	}
	// Give them a nice message
	console.log('Sorry, ' + number + " is not my favorite number. You were " + closeness + ". Guess again.");
	}
}
```

That’s a lot of code. Hopefully it is understandable. There’s lots of other features that we could use to make it look cleaner but this works for now.

***

Time for the truthiness talk. JavaScript has very weak truthiness checks. This means that things like strings, numbers other than zero, and object references all “count” as a true value in both the boolean expressions we wrote earlier and in an if clause. The few values that count as a false include false, 0, null, and undefined (I’ll explain later what null and undefined are). Worse yet, if you asked “0 == false” then JavaScript would say true. This is another place where JavaScript let’s us hang ourselves. That said, from what I’ve seen that doesn’t happen very often. When it does, you can replace equals (==) or not equal (!=) with a strict equals (===) or strict not equal (!==). Like I said, I don’t think you’ll find a need to do that during these examples and I think it is cleaner to use non-strict equality tests in all the places that you don’t actually need one, but it’s something you should know.

***

Next up will likely be looping/recursion. Once you have those, you’ll be a Turing Complete JavaScript programmer. Everything after that is quality of life improvements. Probably after that we’d talk about objects, arrays and references.
