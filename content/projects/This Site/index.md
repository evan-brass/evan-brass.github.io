+++
title = "Building this Site"
date = 2020-07-30
weight = 1
draft = true

+++
* Github Site evolution:
	* First version: Portfolio
		* Cards
		* Excited when I learned about GitHub Pages: "I can build websites that I can share with people!"
			* Started as a portfolio: Mandelbrot, City thing, Dotty, Chandler, etc.
	* Free Code Camp: Portfolio
		* Cards
		* Horizontal Scroll
		* More color
	* Current: Blog
		* Static Site (Zola, but tried Hugo and others)
		* Long form content: Matches my switch from building cool things to look at -> cool things to program
			* Stories
			* What did I learn - even if other people already know it.
		* Really, really wanted to make a horizontal-scrolling, column-based, layout work
			* Horizontal scrolling is hard on desktop (easy on mobile)
			* CSS column widths are quite variable - column-width is only a hint to the browser
			* Reminds me of books
			* Having column breaks and wrapping give the user pause to think - unlike bottomless infinite scroll of doom
			* Nostalgic: Newspapers, Magazines, etc.
				* Takes effort and craft - but hard on variable devices.
			* I intend to keep attempting it.  Maybe one day this site will have a horizontal layout.
		* Not trying to get a job with it
		* Advocate for things I don't think will be popular
		* Communicate what I'm working on to make it worthwhile
		* Document myself / my life
		* Heavily inspired by Medium
			* Text Focused - instead of icons and images
				* Small-caps for dates and nav links
			* Few colors and lots of negative space
			* This also uses simple markup which I hope is screen reader accessible since I can't focus on that at the moment.
				* I've been down the screen reader hole and learned that I need to learn how to use a screen reader better in order to understand if my site is accessible.
		* Fewer barriers to writing:
			* Lower quality than I would put into a Medium article
				* Since I have controversial beliefs, I feel like I have to put extra effort in - It's easy for me to swim against the flow on my own but if I want to talk to other people then swimming against the flow becomes really difficult
					* Most Medium readers probably wouldn't be interested in what I have to say and I wouldn't want to disappoint them by using up one of their free reads.  And if I don't enable curration then there's little point in using Medium for distribution.
			* Static site generator
			* Minimal javascript (I'm picky about Javascript and get sucked in)
			* Learn to be content with imperfection
		* Respectful: 
			* Dark mode support
			* Testing for vision deficiencies
			* Need to work on keyboard usage experience
			* Be privacy preserving (only privacy preserving analytics and comments)
			* Focus on efficient size
				* Responsive image support with layout reservation
			* Microdata for crawlability
		* Chose Zola because I'm learning Rust and am more likely to be able to modify it than Hugo
			* Need to make the Syntax highlighting support css styling
		* Learn about professional website design / administration
			* Verified site owner with Google and Bing
				* Just messing around for now.
				* Currious about how to make syndication between Medium and this blog
					* How do "link juices" flow?
					* Medium has income, curation / distribution,  and commenting but also has a paywall
						* I'd like to get people here so that they can read for free and perhaps I can someday build a community around my interests.
				* While I learn: Hoping that being genuine and content focused (simple) will be good enough
					* I'm a real noob
				* Tried to get rich results but that requires an image for each article and I'm probably not going to do that, so I'll just use microdata as much as fits my content.
			* I want to setup this blog so that it's easy enough to maintain that I could comfortably hand it off to a customer.
				* I played with Forestry.io with the Hugo version, but am having trouble setting it up for this version.  Maybe the dodgy git has something to do with it.
				* Intend to play with Netlify (Maybe forms and stuff?)
