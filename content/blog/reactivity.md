+++
title = "Reactivity"
date = 2020-07-31
draft = true

+++
Everything I know (or think I know) about reactivity.

# Reactivity
Reactivity is two things: detecting where data is mutated and plumbing those mutations to where that data was used.  Without a reactive system the burden of updating the view is present at every mutation.  I've written a lot of code without a reactive system.  Most often with those projects, I would write some half-baked reactive code to make life easier.  A little forthought and knowledge at the beginning of a project can save a lot of time down the line.

# What does code without reactivity look like:
All update work done in events.  Mutation's happen directly in event handlers rather than being propagated by an external system.

Sticky situations:
* When you want something to happen most of the time except for a brief time.  Do you remove the event listener? Do you gate the mutation based on an external variable that get's updated?
	* Modeling both of these is very difficult.  I think state machines are better, but state machines are hard to build UI's with.

# Detecting Change
Doing that plumbing requires something called a dependency graph which is a Directed Asyclic Graph (for our purposes) where a set of 'root' values can be changed and everything else is computed off of them.  What those base nodes of the graph look like can be different and is often a matter of personal preference.  The ways that I know of are:
* It's all setters and getters:
	* React hook style: `const [count, set_count] = use_state()`
		* Because count here is not a getter, this approach doesn't work with dynamic dependency graph construction or at least it's "context" is scoped to the call to `use_state`.  This is fine for React because it only tracks one context and one state anyway.
	* Proxies or `setState(key, newvalue)`
		* Vue
	* Getter and setter: (My prefered method)
		```javascript
		const state = { 
			get value() {
				// Use
			},
			set value() {
				// Mutate
			}
		};
		```
		The nice thing about the setter/getter method is that they share the same name so you can do stuff like this: `count.value += 1` instead of `set_count(count + 1)`.  I think setter/getter is about as close to the Svelte destiny operator as you can get in something that doesn't have a compiler.
	* Probably don't do this:
		```javascript
		const state = {
			get name() {}, set name() {},
			get count() {}, set count() {},
		};
		with (state) {
			return html`
				<h2>${name}</h2>
				<p>The number is ${count}</p>
			`;
		}
		```
		This wouldn't even work:
		```javascript
		const new_scope = new Proxy({
			has: function(target, prop_key) {
				console.log("inner was looking for a ", prop_key);
				return Reflect.has(target, prop_key);
			}
		});
		let test = (function() {
			let a = 5;
			with (scope) {
				let b = 6;
				return function (c) {
					let d = "A string";
					console.log(a, b, c, d);
				};
			}
		})();
		test("This is c");
		```
		Even though it's pretty interesting.
	* Compilers / Static Analysis:
		* I believe that compilers are the way to go, but I think they should be an optional optimization step.  Ideally, we should create a runtime framework with enough information encoded so that something like Prepack could optimize away most of need for the dependency graph.
		* Svelte

# Side Stepping the problem
What I'd like to talk about is called fine-grained reactivity where we detect changes at the root and then perform ~exactly the computation required to sync things up, but you could also detect changes at other points in the cycle.  Most well known would be detecting changes at the view level with VDOM.
* Diff at the state level: (Angular? Doesn't angular have like global listeners and then it checks the state when any of them are triggered?)
* Diff at the DOM level: React and any other VDOM library
* Diff somewhere in between: Lit-HTML (Still view level but not quite at the DOM level), Vue (ish), (Hyper?)

# Propogating Change
Once you have a set of root state we can start deriving views and additional state from them.  We need to build up that dependency graph though and there's a few ways that we can do that.
* Manual graph construction: Observables, async iterators / streams
	* My first ~reactive system had string keys that represented properties and I had to declare my dependencies manually.
		```javascript
		const state = {
			a: 2, b: "John", c: "Doe"
		};
		context('a', () => {
			// Use state.a somehow
			let d = 'Hi '.repeat(state.a);

			context(['b', 'c'], () => {
				// Use state.b and state.c and d
				greeting.innerText = `${d}${state.b} ${state.c}!`;
			});
		});
		```
	* Manual graph construction usually has trouble with changing dependencies.  (See Problems Affecting change propagation)
* Automatic graph construction:
	* Compiler: Svelte, Solid.js?,
	* Single threaded systems: Auto user collection with a virtual stack (begin / end calls).

## Problems Affecting change propagation
For several reasons, it is helpful to be able to propagate multiple changed root variables at the same time or as a single operation.
1. This lets us update multiple values during an event.
2. It also lets us "suspend" propagating updated while the user isn't looking at the page and then perform one big update when they return rather than needing to replay every change that happened while they were away.

Take this graph for example:
```
A - - - - - - - - - \
                     - > F
B - \               /
     - > D - > E - /
C - /
```
Three root properties: A, B, and C.
Three computed properties: D, E, and F.
D depends on B and C.
E depends on D.
F depends on A and E.

* Double updates (priority / update sorting)
	* If A and B are both changed, we should update D's value before updating F.  If D's value doesn't change then recomputing F can reuse the existing value for E.  If D's value does change though, then we need to update E too before computing F.
	* Computing F using an old value of E before computing D / E likely won't be visible to the user if all update happen during a single task before any DOM changes are rendered.
* Back pressure / user visability
	* How do you maintain a "dirty list" so that you can resync when the user returns?
		* Observables don't have backpressure but async iterables do.
	* Fast producer slow consumer problem:
		* Eg. What if a "progress" or "ETA" is changing faster than the user's screen refresh rate?  We should throttle updating the dom to match their refresh rate (RequestAnimationFrame).
* Optional dependence / changing or dynamic graph
	* Some amount of graph reconstruction would be required to manage a perfectly minimal update hierarchy.
		```javascript
		function calc_d(a, b, c) {
			if (a <= 5) {
				return a + b; // Becomes stale if: a or b change (c doesn't matter)
			} else {
				return b + c; // Becomes stale if: a goes above 5, or if b or c change
			}
		}
		```

# The JSON observation
The JSON observation is that all data can be represented using bools, numbers, strings, arrays, and objects.  We should consider each of these (primitives, arrays, and objects) when building our a reactive library.  Up till know I've assumed that all values were either primitive or immutable (that way a change to an item in an array or a property within an object required resetting the whole object).
## A motivating example
Doing this is really inefficient:
```javascript
function selectable_list(items) {
	const active_index = reactive(0);
	return html`
		<ul>
			${items.map((item, ind) => html`
				<li ${use(el => {
					if (ind == active_index.value) {
						el.classList.add('selected');
					} else {
						el.classList.remove('selected');
					}
				})}>
					${item}
				</li>
			`)}
		</ul>
		<button ${on('click', _ => active_ind += 1)}>+</button>
		<button ${on('click', _ => active_ind -= 1)}>-</button>
	`;
}
```
What this does is it creates a closure for every single element in the list and that closure runs every time the active index changes.  Each item is checking to see if it is the active index.
We look at this and immediately notice that all we need to do is access the previously selected item and remove it's selection and access the currently selected item and select it (assuming they are different items - though it would work if they were the same as long as remove comes first).
From [Adapton](http://adapton.org/) "A program P is incremental if repeating P with a changed input is faster than from-scratch recomputation."
Examples: 
* Sorting an element into an already sorted array is O(n) while adding the element and resorting could be O(nlog(n))
* If you already know the max of a list, the max after adding an item is the max of the old list and the new value.
* Adding or removing elements to a list and mapping it is easy, but only if the map doesn't rely on the index because addition / removal affects neiboring indexes.
Sets, maps, and objects can be thought of as lists in most cases I think.  A set is just a list without duplicates.  A map is a list of key -> value pairs without duplicate keys, and an objects has `keys`, `values`, and `entries` lists.
My conclusion is that working with lists is hard.  I think there's a reason that the DOM apis are like they are and so working with lists should probably look similiar to a child node's interface where it can remove itself / insert relative to itself, but it doesn't really know where it is.
I also think that there is a place for diffing / reconciliation algorithms as a good balance between memory usage and performance for when you're dealing with complicated lists with difficult transitions.
I don't really have an answer for this.

# Solutions for Parts of Change Propagation:
* How to group multiple changes that result from a single event into a propagation flow?
	* Using microtasks or requestAnimationFrame (or more likely both to automatically get "suspend" when the user isn't looking at the page).
* How to construct the dependency graph?
	* Virtual stack of functions:
	Use and Single:
	```javascript
	const context_stack = [];

	export function context(user) {
		context_stack.push(user);
		user();
		if (user !== context_stack.pop()) throw new Error("Context corruption");
	}
	export function single(initial_value) {
		let value = initial_value;
		let downstream = new Set();
		return {
			get value() {
				// Aquire the top context as a dependent so that we can queue it for updates when our value changes:
				downstream.add(context_stack[context_stack.length - 1]);

				return value;
			},
			set value(newValue) {
				value = newValue;
				// Queue our downstream updates.
				queue_downstream(downstream);

				return true;
			}
		};
	}
	```
	View:
	```javascript
	const state_1 = single("initial value");
	context(() => {
		let intermediate = "Something" + state_1.value;

		context(() => {

		});
	});

	```




# (WIP:) Incremental algorithms:
* Sort:
	
* First use duplication: You write code to construct the view that looks like - but isn't identical to - the code that you write to update the UI when the value changes.
---

* Reactivity is two things: detecting where data is mutated and plumbing it to where it is used.
	* Mono state - Only one thing can change: React, Redux
		* Not fine-grained: DOM diffing required
		* Often implemented with immutability - sad implementations use unneccessary copying.
	* Dirty checking - Check everything that could change on each event: Angular
		* Not fine-grained: All handlers run
	* Observables - Construct a graph and flow changes: Rxjs
		* Fine-grained
		* Changing the graph can be difficult
		* Back pressure
	* Compiled graphs: Svelte, Solid.js?
		* What if the graph changes (data is not shown for a while and then comes back)?
	* Automatic graph:
* JSON observation: Any data can be represented as a composition of primitives, objects, and arrays.
	* Serde is similiar
	* To be complete, a reactive system needs to handle each of these cases.
* Why is it hard?
	* Conditional usage (unsubscribing): How do you handle a computation which sometimes requires a value and somtimes doesn't?
		```javascript
		function calc_d(a, b, c) {
			if (a <= 5) {
				return a + b; // Becomes stale if: a or b change (c doesn't matter)
			} else {
				return b + c; // Becomes stale if: a goes above 5, or if b or c change
			}
		}
		```
		* With observables / other manual dependency management, you'll probably still be subscribed to c even if you don't need to be.
		* The only way to know when a value is needed in a computation or not is to either recompute the graph during runtime or to use a compiler.
	* Preventing double updates: What happens when two things that a computation depends on change simultaneously?
		* What about batching multiple updates while the page isn't visible and getting back up to date when they return?
	* Fine-grained: Recompute the smallest amount possible
	* Dependency Depth: Are your dependencies a Directed Acyclic Graph (DAG)?
		```
		A - \
		     - > D - \
		B - /         \
		               - > E
		C - - - - - - /
		```
		D depends on A and B
		E depends on D and C
		How do you make sure that D is computed before E gets computed?
		A system could accidentaly update E twice or leave E with a wrong value if it doesn't handle this properly.
	* User visibility / back-pressure: What happens when something isn't user visible anymore (permanantly or for a short time)?  What if the updates are happening faster than the user can see them (progress bar for example)?
* Solutions:
	* Microtask / animation frame: Accumulate changes and update after main script runs.
	* Async iterators: Like observables but with back-pressure.
	* Single values: Sepperate setter and getter.  React Hooks, LiveData, etc.
	* The functional JS Array observation: map, filter, reduce, flatMap, etc cover the majority of use cases.
	* Arrays: People seem to like the functional array methods: map, filter, reduce, flatMap, etc.  Generally, I think that not creating temporary arrays is a good idea, but in terms of a functional system I think we could say that implementing these (and a few other) array functions would satisfy most use cases for arrays.  Take map for instance, to update a mapped array from a change to the source array you just apply the map function to any new items and delete any removed items.  Similiar with filter, you just check any new items to see if they should be included and delete any removed items.  One more case against reduce, there's not really a way of incrementally computing a reduce.  You can take the output of reduce and un-reduce an item from it.  Any change to an array would require a complete re-reduce.  You could build your own incremental reduce for some operations, but let's ignore that.  Some other array methods that work out in a nice incremental way are sort, and count / length.
* What am I actually going to talk about?
	* You can get started with something reactive pretty quickly.
* Incremental computation:
	* Work well: Map, Filter, Sort (though removal might be hard), Avg, Count / Length
	* Work ok: Min / Max (Could keep a buffer of previous Mins/Maxs in case the current Min/Max is removed), 
	* Don't work well: Reduce (Have to recompute from scratch unless it's reversible), Zip?, Anything with multiple arrays?,
	* Not just arrays:
		```javascript
		function selectable_list(items) {
			const active_index = reactive(0);
			return html`
				<ul>
					${items.map((item, ind) => html`
						<li ${use(el => {
							if (ind == active_index.value) {
								el.classList.add('selected');
							} else {
								el.classList.remove('selected');
							}
						})}>
							${item}
						</li>
					`)}
				</ul>
				<button ${on('click', _ => active_ind += 1)}>+</button>
				<button ${on('click', _ => active_ind -= 1)}>-</button>
			`;
		}
		```
		* This should be incremental but it's hard to write and I'm not sure if this case could be factored out into something worth putting in a library.
		* Maybe just give up on incremental arrays?  They're hard.
	* What about objects?
		* Treat individual properties as live-data
			* Might not be expressive enough?
		* Treat `.values()`, `.entries()`, and `.keys()` as live arrays
		* For a lot of use cases I think you could even get away without the ability to add / remove properties from live objects
	* Warning: For map to be incremental, we can't use the index - because the index can change without recomputation.  This means you can't do something like (I've tried.):
	```javascript
		const list = live_list();
		const buttons = list.map((num, i) -> html`Item ${num} <button ${on('click', () => list.splice(i, 1))}>delete</button>`);
	```
	* While writing an incremental map and filter, I've found that they have common duplication:
		* On construction, map / filter all elements, then do the incremental approach on each update.
		* This duplication makes writing incremental algorithms difficult / harder to maintain or ensure are correct.
		* Similiarly with sort: The first sort is nlog(n), but when you splice, you have to remove old items + sort the new items + place the new items into the old array.  This is a completely different thing than the first sort.
		* Using incremental algorithms is "easy" (if you don't need indexes) but writing them is hard.
			* Build a library of common invremental datatypes?  Lots of trade offs - how to pick the right ones?
* Conclusion:
	* Hopefully there's not much code to build this out
	* I'm not saying you should use this
		* My goal has been to use simple, small code to implement interesting developer experiences.  I think this is an interesting developer experience and I hope you enjoyed reading about it.

---

# The JSON observation
Let's talk about the JSON observation.  It states that, "Any data can be represented as a composition of primitives, objects, and arrays."  You can see this observation in Serde, Rust's primary serialization and deserialization library.  Now, your data might not fit JSON very well, but it should fit.

If we want to talk about generalized theories of reactivity over arbitrary data, we need solutions for all three types: single values (primitives), associative arrays (objects), and sequential arrays.

# Why so hard??
So, how is reactivity hard?  Isn't a view just a function of data?

Well, that's one way of implementing it - and I would say that functional approaches are fairly elegant.  However there's this big problem of push and pull.  If you're on the side of observables then you are a pusher.  Data is pushed from source to destination.  If you're a React user, you're more pull.  Something changes, the whole thing get's re-rendered and the render function pulls the data it needs.

The case that tests - benchmarks per say - all reactive setups is the fast producer + slow consumer case.  It's this case that shows we don't just need push or pull but in many cases both.  We can't (maybe shouldn't) just use push because we could end up computing a value that the user interface isn't interested in anymore.  And we can't just do pull 
