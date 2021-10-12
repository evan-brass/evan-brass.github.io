= Classed Highlighting Sample
:keywords: Open Source
:date: 2020-12-03

# A sample of Zola PR 1242

```rust, linenostart=0, hl_lines=2 3
fn main() {
	println!("Hello World!");
}
```
--------
```javascript, linenos, linenostart=3, hl_lines=2-3 8-9 26
function* fib() {
	let a = 1;
	let b = 1;
	yield a;
	yield b;
	while (true) {
		let ret = a + b;
		a = b;
		b = ret;
		yield ret;
	}
}

for (const v of fib()) {
	console.log(v);
	if (v > 1000) break;
}
/**
 * Output:
 * 1
 * 1
 * 2
 * 3
 ...
 * 1597
 */
```