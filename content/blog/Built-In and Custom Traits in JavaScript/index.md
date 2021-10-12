= Built-In and Custom Traits in JavaScript
How to use Symbols to implement a trait pattern.
JavaScript, Patterns, Traits
:date: 2020-06-15

# The Problem
Polymorphism is important in any language and while it is very easy to write polymorphic code in JavaScript, it is relatively hard to manage that code. Without explicit types you must either assume the structure of an object, or explicitly test its structure before using any functionality. Explicitly testing slows down your code, while making unsound assumptions usually results in bugs. This might sound like a trade off, but it’s just another problem for which fast and safe solutions exist.

A great solution is type annotations. You probably know about TypeScript and Flow already, so I’ll just be mentioning pieces that relate to what I want to discuss.

Flow and TypeScript don’t solve naming conflicts. If you were trying to implement two interfaces that both required a method named `.foo()` but with different behavior then chances are that you’ll need multiple objects and switch between them. This is probably a very rare case depending on your coding style and the libraries you interact with. Even so, naming conflicts can cause bugs and hinder innovation. #smooshgate is an example of the worst case. A best case is Fantasy Land, which uses method names like `fantasy-land/equals` and `fantasy-land/empty` which have a low chance of colliding.

Web standard designers can’t have any collisions and they want to innovate quickly. What can they do? They can use symbols - well-known symbols, to be specific.

# Changing Language Behavior
By using well-known symbols we can do all kinds of things. Most of them don’t seem useful — like `Symbol.toStringTag` which lets you minutely change the output of `.toString` — while others are very useful — like `Symbol.iterator` which lets you create objects that work with for-of loops, the spread operator, and elsewhere. Whether powerful or not, the existence of well-known symbols increases the consistency and introspective ability of JavaScript. Later, we’ll use some well-known symbols to build a custom trait class, but first let’s use `Symbol.iterator` (usually abbreviated `@@iterator`) to build an object that iterates over the Fibonacci sequence to understand how they work.

## Fibonacci Iterator
In the real world, it would be better to use a generator instead of the following class and I’m confident that you could build a more concise version than this example. Anyway, here’s the code:

```javascript
class Fib {
	a = 0;
	b = 0;
	next() {
		if (this.a == 0) {
			this.a = 1;
			return { value: 1, done: false };
		}
		if (this.b == 0) {
			this.b = 1;
			return { value: 1, done: false };
		}
		const value = this.a + this.b;
		this.a = this.b;
		this.b = value;
		return { value, done: false };
	}
	[Symbol.iterator]() {
		return this;
	}
}
for (let n of new Fib()) {
	console.log(n);
	if (n > 1000) break;
}
/* Output from the above loop:
1
1
2
3
..
1597
*/
```

When you try to iterate over an object with a for-of loop, the runtime checks if the object has a `@@iterator` method. If it does it calls it which should return an object that follows the [iterator protocol](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols). The iterator protocol requires having a `.next()` method that returns objects with `value` and `done` properties. In our case, we’ve implemented `next` on the class itself and so our `@@iterator` method just returns `this`. If there had been a conflict, then we could have returned a different object — one that probably would have some access to the root object. Here’s an example of that:

```javascript
class Foo {
	a = 5;
	next() {
		return false; // Doesn't implement the iterator protocol
	}
	[Symbol.iterator]() {
		const self = this;
		return {
			i: 0,
			next() {
				return { value: self.a + this.i++, done: false };
			}
		}
	}
}
```

We can thus always get around naming conflicts, because we can return any object from our `@@iterator` function. If you weren’t using symbols you would probably still have two objects, but might not have a clear way of providing the right one in the right places.

# Runtime and Static Analysis
This is a runtime solution which means it has runtime overhead. In TypeScript, we could have just called `.next()` on an instance of our Fib class because TypeScript’s extra information tells us that Fib has a next method and that its signature matches what we want. With well-known symbols, we have a level of indirection. Not just a pointer de-reference indirection, but a function run indirection — not something to be ignored performance wise. Indirection is usually a requirement for polymorphism unless you have monomorphization. [Also, technically it’s two levels of indirection because we already have one level due to only having references to objects in JavaScript, but let’s move on.]

You may be wondering how using a symbol is any better than just testing if the instance has a next method. If we did that (called duck-typing) we would need to test a lot of stuff. We would need to check if there was a next function, then see if calling next returned an object (if it didn’t, then hopefully we didn’t trigger any side effects), and then check if that object has value and done properties, and that the done property is a boolean, etc. We have to do all these checks because we aren’t certain that the object having a next method that resembles our iterator protocol isn’t just a coincidence. Someone could accidentally implement the iterator protocol when they never intended it to be iterated. Unlikely but possible.

With symbols we can be certain that the author intended to implement the iterator protocol. They must have acquired our symbol from `Symbol.iterator` — a unique symbol that cannot be acquired any other way (for the most part). Even if you have methods with the right names and even the right signatures, you must also have the `@@iterator` method for it to be iterable.

Since we know that the implementer intentionally made the object iterable, we don’t have to check if the returned object has a next method because it must (or else it’s a bug). We may still do the duck-typing checks, but perhaps only in a debug build and we could remove them in release. Additionally, if you’re using TypeScript, it will help make sure that a manual iterator implementation has a next method with a valid signature.

Lastly, the iterator protocol plays nicely with others. Anything iterable can be iterated either with a for-of loop, spread, `Array.from()`, or manually calling `.next()` no mater what library it comes from. Similarly, if we create our own symbols — well-known in that they are only obtainable by importing an es6 module — with a clear protocol, then all code using it _should_ be interoperable.

# The Pattern: Imitate and Extend Well-Known Symbols
To begin exploring what having our own well-known symbols (which I will now also call traits) is like, let’s first introduce a helper that will make working with them easier. It’s a simple class that takes advantage of a few well-known symbols to adjust JS behavior.

```javascript
export default class Trait {
	constructor(description) {
		this.symbol = Symbol(description);
	}
	[Symbol.toPrimitive]() {
		return this.symbol;
	}
	[Symbol.hasInstance](target) {
		return typeof target == 'object' && target[this.symbol] !== undefined;
	}
}
```

We use `@@toPrimitive` so that it turns into the underlying symbol when we use it in a method / property name expression and we use `@@hasInstance` to override the instanceof behavior. It looks like this when it is used:

```javascript
const Sayable = new Trait("Can be made to say something.");
const Drivable = new Trait("Can be driven.");

class Cow {
	[Sayable]() {
		return "Eat Mor ChiKin";
	}
}
class Horse {
	[Sayable]() {
		return "Nay."
	}
	get [Drivable]() {
		const horse = this;
		return {
			steer(direction, throttle) {
				// ...
			}
		}
	}
}
class Car {
	[Sayable]() {
		return "Vroom."
	}
	steer(direction, throttle) {
		// ...
	}
	get [Drivable]() {
		return this;
	}
}
console.assert(new Cow() instanceof Sayable);
console.assert(new Horse() instanceof Drivable && new Horse() instanceof Sayable);
new Car()[Drivable].steer(45.0, 0.34);
console.assert(new Car()[Sayable]() == "Vroom.");
```

This illustrates how easy it is to test if something implements a trait and that the protocol a trait represents can be anything (functions, objects, just a property, etc.) except undefined.

## State in Trait Protocols and Implementations
Something to note is that the protocol for Drivable is different from the iterator protocol. In the iterator protocol, the object returned from `@@iterator` can have state which is why when you use a for-of loop to iterate over an array and then later use another for-of loop on that array, the second iteration will start from the beginning not where you last stopped — the iterations are independent. This happens even if you break out of the first loop before reading to the end of the array. In our Drivable trait, we get the object that implements Drivable for our Car, but we don’t store it before calling `.steer()`. This means if we needed to call `.steer()` again, we would have to fetch the Drivable implementation again. In the case of Horse, the returned implementation would be a **brand new object** instead of the one previously returned, but it would **function identically** to the first value it returned because it has no state / applies all changes to the Horse directly.

For many traits, it makes sense for the protocol to require statelessness in the implementation. If you had an array of objects, and your protocol wasn’t stateless, you might need a second array to hold the trait implementation objects doubling the memory used. Here’s an example of that happening:

```javascript
class Car {
	get [Drivable]() {
		return {
			x: 0,
			y: 0,
			steer(direction, throttle) {
				this.x += throttle * direction.x;
				this.y += throttle * direction.y;
			}
		}
	}
}

const cars = [new Car(), new Car(), new Car()];
const cars_drivables = cars.map(c => c[Drivable]);

while (true) {
	const riding_index = cars.indexOf(player.get_riding_car());
	cars_drivables[riding_index].steer(direction, throttle);
}
```

See, because we needed repeated access to the trait implementation, but it was stateful, and lastly because getting the trait implementation for car returned a new implementation every time, we needed two arrays. This problem only comes up when these three things come together. I’ve done it accidentally before so I wanted to mention it. A weak-map would probably work better in this case (either instead of the second array or to return the same implementation every time), but it still illustrates the pitfall.

Looking back at the iterator protocol, iteration is usually short-lived reducing the likelihood that you’d need to hold a reference to the implementation object. Iterating doesn’t affect the container — unlike a filter or sort for example. These two properties make storing state in the implementation object a good choice for iterator. Choosing whether to couple or decouple state in a trait implementation is a technical decision and being familiar with other implementations / protocols similar to your own can help. There are many places to learn about good protocols. Look at the life-cycle in React or other frameworks. Look at async iterators. My favorite source is the Rust standard library which has many traits and good documentation.

Iteration can be made to affect the container as we did in our Fibonacci example. It wasn’t very useful for Fibonacci, but it is very useful if you are building something like a queue where every item needs to be seen once even though there might be pauses between iteration. Building a consuming iterator is a good way to transform events / callbacks into an async “stream” — something I’ve done a lot of.

## Generic Implementations
Well-known symbols help avoid / work around collisions. They give us confidence that our protocol was intentionally implemented (or at least attempted to be implemented). That alone isn’t enough to want to use this pattern, though. It becomes more attractive when you see that you can write functions to implement traits for a class based on other traits it implements — generic implementations.

Here’s an example where the AI of an entity is derived from whether the entity is Undead or not:

```javascript
const Undead = new Trait("Neither fully alive nor fully dead.");
const AI = new Trait("Has an AI script");

function implement_ai(base) {
	let ai_impl;
	if (base instanceof Undead) {
		ai_impl = {
			ai_tick() {
				for (const creature of this.source.surroundings()) {
					// Find living things in the surounding and eat them with a preference for brains...
				}
			}
		};
	} else {
		ai_impl = {
			ai_tick() {
				for (const creature of this.source.surroundings()) {
					// Find food and avoid the undead...
				}
			}
		};
	}
	Object.defineProperty(base, AI, {
		get() {
			const ret = Object.create(ai_impl);
			ret.source = this;
			return ret;
		}
	});
}

class Squirrel {
	chase_other_squirrel() { /* ... */ }
	// ...
}
implement_ai(Squirrel);
class Zombie {
	groan() { /* ... */ }
	get [Undead]() {
		// Undead things...
	}
}
implement_ai(Zombie);
```

This puts all of the AI logic in one place and keeps the prototype chains short. Any changes to the AI need only be made in one place and will be visible across all the entities. This is the power of derivation.

It’s a little inconvenient to have to call `implement_ai()` on each class, but once we get decorators this kind of thing will be easier and more common.

## Interaction with TypeScript
The last thing I want to mention is that this pattern is hard to work with in TypeScript. I’ve briefly tried to add types for some code that used it heavily and it didn’t work. If you know how to do it, I’m eager to learn how.

The problem is mostly because we’re adding a method / property with a dynamic name. The Trait class is a constant expression that evaluates to the symbol, but TypeScript can’t see it — not without partial evaluation perhaps. If I’m not mistaken, even the standardized well-known symbols require special support within TypeScript — which can’t be extended to our symbols.

Something I haven’t tried is using a symbol registered with a string using `Symbol.for()`. This is an alternative to having it be in an es6 module and importing that module into all the code that implements that trait. Personally, I don’t like the registering approach because it gets back to using strings to avoid collisions. It’s like using `fantasy-land/empty` as the function name — unlikely to collide, but possible.

# Conclusion
I hope you learned something. Maybe you hadn’t overridden default behavior using well-known symbols before. Maybe you have an idea for a better `instanceof` than the one I’ve shown you. Maybe you take issue with everything I’ve said. That’s fair and I’d love to talk with you about what you think.

I think this pattern has precedence in the standard and is therefore worthwhile exploring. We may see more well-known symbols in the future and working with them should be natural. There’s a lot that goes into designing traits and building good protocols — some of which we can learn from watching JavaScript be developed. Sadly, this pattern doesn’t seem typed-super-set friendly.

Thanks for reading. Have a good day, and happy coding.

## Resources
* [TypeScript Symbols](https://www.typescriptlang.org/docs/handbook/symbols.html)
* [Fantasy Land](https://github.com/fantasyland/fantasy-land)
* [The Iterator Protocol](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Iteration_protocols)
* [Code that uses this pattern heavily](https://github.com/evan-brass/js-min/tree/1bad89fcf41c6746d9b3d429a6e154f20c564e8f/src/templating/users)
* [#Smooshgate](https://developers.google.com/web/updates/2018/03/smooshgate)
