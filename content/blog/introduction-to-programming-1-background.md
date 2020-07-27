+++
title = 'Introduction to Programming 1: Background'
date = 2018-07-17T19:31:29.000+00:00

[taxonomies]
tags = [
	"JavaScript",
	"Tutorial"
]

[extra]
canonical = ""
alternate = []
+++
Programming has its roots in mathematics. Every ([sane](https://esolangs.org/wiki/Three_Star_Programmer)) programming language has an underlying model of computation. Those underlying models have been proven to have equal power, that is that anything which can be done in one can be done in another.

Computation can be described by different models and performed by different machines. For example, your brain right now, as it reads these words is performing computation. Each individual neuron is comprehensible. It has dendrites which are its inputs, and an axon which is its output. The spaces between axons and dendrites are called synapses and different neurotransmitters are passed across these synapses. The output of each neuron computes a weighted sum of the inputs on its dendrites. If their sum is higher then a threshold then the neuron “fires” and it sends a signal out its axon (to other dendrites). There’s no magic here (though I’ve grossly over simplified). But if you attempt to understand even a moderate portion of the brain or one of its functions (emotion, object recognition, the thalamus, memory, sleep, etc.) then there is more magic to unravel then perhaps there is time left for our sun. A similarly simple-component-but-complex-system is that of the Turing Machine which is the primary model of computing behind modern Central Processing Units (CPU).

Back to programming. The programming paradigm layered on top of the Turing Machine is that of imperative programming. Since imperative programming languages use the same model of computation that is used by the CPU, they almost always host faster and more efficient programs. Functional programming utilizes lambda calculus as it’s underlying model of computation. Lambda calculus and Turing Machines have equal power and in many situations a program written functionally makes more sense then a program written imperatively. But functional programming languages must eventually convert to something that the CPU (a Turing Machine) can understand, so how a functional program is _run_ is often harder to understand than an imperative one. The CPU efficiency vs. programmer’s time efficiency is one of the core balancing acts that programming languages must do.

While programming languages do their best to trade performance, memory efficiency, programmer’s time, and cross-platform targeting it may not be enough for whatever project you intend to create. Your magical mind, as it hones it’s skills ind different languages and deepens its understanding of the computational models will become more equipped to determine what languages (general or domain specific) are right for your project. There’s plenty of dogmatism surrounding different languages, but do your best to ignore it while you’re starting out. There’s plenty of time to learn new languages and explore later, but I think you’ll find it more useful to pick one and stick with it while you get a handle on your programming theory. _Experiencing_ one language’s strengths and weaknesses is better then being able to _recite_ the same for twelve languages.

Why do I recommend Javascript? It’s easy to get started (I’m planning on writing that next). It is at a high enough level that you’ll be able to build fun and interesting things quickly which will hopefully keep you from becoming burned out. It’s a desirable skill right now as it is the language of the dynamic web (though with WebAssembly it may gain valuable competition). Lastly, it’s the language that I know best and I want to present accurate and useful information.

So then, a programmer is an architect of computation. We (including you) take a problem we wish to solve and design a solution to it. Not a one time solution like that of a math problem, but a reusable solution that you can quickly stamp as many times as you need. You’ll find it’s a very creative process — frustrating at times, but rewarding.

Bonus: There is a current hypothesis that everything that can be physically computed (by all material machines — brains included) can be computed using a Turing Machine. Yet, there are useful problems which have been proven to be [impossible to compute using a Turing Machine](https://plato.stanford.edu/entries/turing-machine/#Uncomputability). Some of those problems (the Halting Problem for example) is really useful. What important questions does your brain encounter which it may not be to compute an answer for? What is your strategy to handle those problems? Lastly, how crazy is it that there are an infinite number of problems which our computers (and maybe our minds) cannot solve? I think that’s amazing!
