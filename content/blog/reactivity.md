+++
title = "Reactivity"
date = 2020-07-31
draft = true

+++
# Reactivity
Reactivity is two things: detecting where data is mutated and plumbing those mutations to where that data was used.  Without a reactive system the burden of updating the view is present at every mutation.  I've written a lot of code without a reactive system.  Most often with those projects, I would write some half-baked reactive code to make life easier.  A little forthought and knowledge at the beginning of a project can save a lot of time down the line.

# Detecting Change
* It's all setters and getters:
	* React hook style: `const [count, set_count] = use_state()`
	* Proxies
	* Getter and setter: (My prefered method)
```javascript
const state = { 
	get value() {
		// Use
	},
	set value() {
		/// Mutate
	}
};
```
The nice thing about the setter/getter method is that they share the same name so you can do stuff like this: `count.value += 1` instead of `set_count(count + 1)`  It's not a huge difference and it might not be "pure", but if the point of a reactive system is to abstract change propogation and remove concerns, than I think this does that better.

# Propogating Change
* Manual graph construction: Observables
* Automatic graph construction: 

---

* Reactivity is two things: detecting where data is mutated and plumbing it to where it is used.
	* Mono state - Only one thing can change: React
		* Not fine-grained: DOM diffing required
		* Often implemented with immutability - sad implementations use unneccessary copying.
	* Dirty checking - Check everything that could change on each event: Angular
		* Not fine-grained: All handlers run
	* Observables - Construct a graph and flow changes: Rxjs
		* Fine-grained
		* Changing the graph can be difficult
		* Back pressure
* JSON observation: Any data can be represented as a composition of primitives, objects, and arrays.
	* Serde is similiar
	* To be complete, a reactive system needs to handle each of these cases.
* Why is it hard?
	* Conditional usage (unsubscribing): How do you handle a computation which sometimes requires a value and somtimes doesn't?
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
	* What about objects?
		* Treat individual properties as live-data
		* Treat `.values()`, `.entries()`, and `.keys()` as live arrays
		* For a lot of use cases I think you could even get away without the ability to add / remove properties from live objects
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
