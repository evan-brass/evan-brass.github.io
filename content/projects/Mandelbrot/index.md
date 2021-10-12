= The Mandelbrot Set
:date: 2020-11-26
:draft:

A while back I made a simple mandelbrot set visulizer after my calculus professor gave a talk about it.  It only took me one (late) night to get it working which I believe is a testament to the power of webbrowsers.  I used canvas, a math library, and maybe a worker.  Since I rebuilt this website, it's no longer accessible and I want to change that.

I've decided to recreate it here.  Partly I just want a simple project to work on.  Partly I want to learn how to write interactive educational documents and this seems like a good stress test for me.  When I was in school I occasionally wrote little programs to help me with my homework or to understand things.  I remember writing a little program while solving CPU cache problems.  It just broke memory addresses into parts for cache line, cache affinity, etc.  I've wanted to revisit that and create something which shows the cache lines and could even run simple programs to visualize their memory access behavior.

# The Mandelbrot Set
The mandelbrot set is the set of complex numbers that are "bounded" (don't approach infinity) in the following sequence {{ math(body="{[Z_n = {Z_{n - 1}}^2 + C], [Z_0 = C] :}") }} 

with the initial Z of C

## Tracking a point's evolution:
<canvas id="evolution">
	Canvas isn't supported in your browser.
</canvas>

## Renderer:
<canvas id="mandelbrot">
	Canvas isn't supported by your browser.
</canvas>