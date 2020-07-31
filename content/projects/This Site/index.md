+++
title = "Building this Site"
date = 2020-07-29
draft = true

+++
# snippets:
* You can see that this blog is very different from my previous portfolio designs.  As my coding has moved less graphical (into templating and state management) I feel it's natural to try and shift my site to more long form content.  I want to tell stories more than build polished things.  I want to write more Rust and work on proving concepts for distributed web applications.  Topics like these aren't easy to just find.  To make my coding worthwhile I need to communicate it and that's going to mean blogging about the code.

The change in design direction represents my changing hopes for this blog.  I'm less interested in it beind a piece of my resume, but rather something that accurately reflects me.  My resume accurately reflects the light I think employers want.  I'm interested in longer form writing.  I want to share what I learn - even if people already know it.  It's documentation and a habit for introspection.  I want to write more which means I need fewer barriers.  I needed to stop writing straight HTML.  I needed to mostly not use JavaScript because I'm picky about JavaScript.  I intend to use JavaScript to extend this site with privacy preserving analytics, not neccessarily offline support (my content isn't that helpful) but perhaps to optimize network usage, etc.

Some of my goals for this site were simple dark mode support.  The syntax highlighting problem is not solved yet but I'm hopeful that I'll be able to get it eventually.

I wanted to build something that has quality: I'm trying to add microdata, I've been testing the sight with Chrome's new vision deficiencies feature, I haven't gotten to it with this Zola site yet but with the hugo version I was experimenting with how to do images, etc.  I would like to use webp although it seems that the static site generators are still working on support for it.  I'd also need to provide a png backup to support Safari and IE anyway.  I doubt that the people I'm interested in reaching are using laggy browsers so I'm not super concerned, but I feel that I should at least try - if only for good practice.

In keeping with using this site as an experimentation into what building a professional quality website would be, I want to get hooked up with Forestry and netlify and whatever else so that the editing / maintenance experience is something I would be satisfied putting a customer in charge of.  I also became the verified owner of this sight and I'm trying to figure out Google's search console.  Thus far it hasn't pulled my sitemap and found these posts yet.  I'm not sure what's wrong yet.

Part of the whole Google thing is testing to see how syndicating my writting between this blog and Medium would work.  I see Medium as a way of earning money on writting, distribution, facilitating discussion (my blog won't have comments unless I can find a free, privacy-preserving solution like Disqus) but the paywall is inconvenient.  As such, if I can double post and use Medium to bring people here then that would be great.  I feel that Medium requires a level of quality that is difficult though not impossible for me to achieve.  But I want to write content that I don't think most Medium readers would be interested in.  Content that shares my experiences and stuborn opinions on web development - which I feel would be a let-down if they used up someone's free articles.  Yet, if you don't enable content curation on Medium (and you don't have a follwer base like me) then there's little chance that what I write will be seen.

On that subject, I was very inspired by medium for this site's design.  I've been playing with just text and seeing how small changes - like using small-caps which I do for the dates - can convey information structure.  Where as my old style would have used lots of font-awesome icons, I'm trying to just use the data and keep it clean.  There's nothing wrong with font-awesome - it's really cool - but I want to eventually make this site as accessible as I can and in the interim I want as much text and semantic HTML as possible.

Back to SEO, I don't know much about interacting with search engines.  I'm hoping that just being content focused and genuine will work out.  As for "link juices" and proper meta tags, I have no idea, I'm a real noob.  Also, I'm wondering how I should update Google when things change.  I've had the impression that Google just checks the sitemap every now and then and re-indexes things.  Maybe I need to be submitting fine grained urls?  It's not important right now - I'll get there if I get there.

I tried to get rich result support but apparently articles need an image.  I'm probably not going to 

I really, really wanted to be able to make a column layout work.  It was fun to learn about CSS orphans and widows and break-inside, and stuff.  And I like the way that it get's layed out.  It's very book like which is nice.  I feel like a lot of problems could be solved with horizontal layout.  For one, infinite scroll.  When the web got infinite scroll, we lost a lot of the experience of hitting natural segmentation.  Our minds don't have that pause to think.  As with much design, the choice and mental exercise is removed.  Adding columns could bring that back because content has column breaks.  I also feel like it's a good use of screen realistate.  you can have text from top to bottom, left to right without lines growing too long and unweildy.  It adds a ton of layout complexity for sure.  I just think about the effort that goes into magazine and newspaper layout and think about how it would be compounded by supporting multiple device form factors.  Even so, I have a very idealistic view of a horizontal layout.  It's something I intend to continue attempting - perhaps with layout worklets - in the future.



# Original
I was so excited when I found out about Github Pages.  "Yay! I can build little websites and then actually show them to people!"  I realized that it should become my portfolio and I had a few things to add.

One was a CodePen art piece.  It was supposed to be an abstract city skyline with a day / night cycle.  I had fun designing a randomly generated skyscraper and then setting probabilities for the lights turning on / off based on the time of day.  I enjoyed watching it and seeing lights go on and off in the middle thinking, "Look! Someone's going home!" "That lights been on for awhile, I wonder what they're working on." "That person got to work before daylight broke, I wonder if they had a really good idea that couldn't wait."  I realize now that this reflects my fantastical INFP tendencies.

I added a few more projects over time: A website template for photographers where the images in the background faded in and out, Chandler which was my response to reading a book about the failed software project by the same name, a screenshot of the React + Electron based calendar I worked on, a Mandelbrot set viewer, and an HTMLCanvas based image pixelator. As time went on, less and less of my code ended up on the website.  I didn't like writing the HTML by hand for each project and my interests roamed into less _flashy_ or aesthetically impressive websites.  Let's look at what it looked like before I started over:

![portfolio-original.png](portfolio-original.png)

# Free Code Camp
I wanted my portfolio to be different and I used the Free Code Camp portfolio as an opportunity to explore that.  One thought was to do horizontal scrolling and I ran with it.  I wish horizontal layout was easier but there's no real support.  First off, the scroll wheel doesn't scroll horizontally so you have to use javascript to change that.  This isn't simple because the value you get in a scroll event doesn't match across browsers.  I've used scroll and wheel a few times, and I don't currently know how to do it right - there might not be a right way.  Let's look at that page:

![free-code-camp-version.png](free-code-camp-version.png)

I've still loved the horizontal view and I wanted to bring it back into this website so I tried to use columns for a while but the same issues always come back.  Columns in css aren't much fun in my opinion.  Maybe with layout worklets we can make something more enjoyable.

# Static Site Generators
I'd intended to make the portfolio I made for Free Code Camp the same as my GitHub Pages site but never did.  By the time I went back around to it I had gained the conviction that most sites should be using site compilers and be staticly hosted.  So I started looking at static site generators.  I started with Rust ones: Zola, Cobalt.  Then I played with Hugo.  Lastly, I came back to Zola.

Yes, I never touched the JavaScript versions.  JavaScript belongs in a browser and that's about it.  If JavaScript exits the browser, I definitely wouldn't want to touch any part of the yuck that is NPM.  I only want to include code that I trust which either requires understanding it or trusting the authors.  The problem is that when you use a library that uses other libraries, your trust is spread across thost other libraries.  And it's recursive and sucks.  I don't trust NPM and I dislike the experience of installing and using JS cli tools.  A personal preference.  I've uninstalled Node so that it's not easy for me to slide back into that world.

To sum up my experience with static site generators I would say, I almost gave up.  My issue with Zola was the syntax highlighting.  I wanted to use css classes / variables so that I could have a light and dark mode in the code.  Well, Zola uses Syntect, and Syntect operates with Sublime theme and syntax files.  This is great because there's a lot of Sublime syntax and themes out there.  But I couldn't find a way to gain css control over it.  If you look at some other posts you'll notice that the syntax highlighting is dark themed.  That's the best I can do for now.

When I hit the syntax highlighting wall I started questioning if static generators were really what I should be using.  And... It's not ideal, but building my own workflow engine and building up a library of little programs that build sites isn't going to happen so I'm going to use a static site generator.

I started to like Hugo although I didn't care for the content layout.  That's the problem, everything is opinionated. Opinionated - which people use as a positive buzzword - when it's actually annoying.
