= Why I Hated AMP and Why My Mind Has Changed
:keywords: AMP, Rant
:date: 2020-07-13T01:36:02Z
:draft:

By hated I mean… hated. Hated for good reasons.

My first thought when I heard about AMP was, “Great, it’s not enough for search engines to track what user’s do before they click our link, now they want to control the experience after the user clicks our link.”

# Why I hated it

When AMP came out, mobile friendliness was a big deal. Many websites had poor mobile experiences and it makes sense that user experience should count toward search ranking. I was afraid that AMP would be the definition of a good mobile user experience and become mandatory for good mobile ranking. AMP constrains developers in many ways. Limited JavaScript, limited CSS sizes, served from third party caches, mandatory third party scripts (something I almost never do otherwise), and removal of certain tags.

From a user standpoint, I disliked that I would get the AMP page in Google search results and then have to click the little button that takes you to the canonical website. I tried to find a setting in Google search that would stop showing me AMP links, but I couldn’t find one. I did find a new search engine which didn’t use AMP.

With AMP your analytics options are limited to what’s built in. As such, I would get off the AMP site as soon as I possibly could and onto the main site. I'd rather take my chances with the main site’s data collection and usage policies. But I could never get off as fast as I wanted considering it took two taps _after_ the AMP page loaded to do so.

In general, mobile shouldn’t be treated as a separate platform / target, but many of the AMP pages I’ve used were alternatives to their main site. My searching behavior may be different from the norm, but I spend more time reading through the search results and am pretty certain that the link click is the one I want. I’ll even open a few links in different tabs and evaluate them one at a time which would not get any of the AMP prerendering benefits. Having that open tab reflects my usage of browser features for task / focus management. I don’t feel the need to replace those with search engine based task management. That’s just me though. Cake browser is an example of a browser offering similar benefits as what the search engines were doing, though maybe not as efficiently because AMP's limits are designed to make loading fast.

# Why I don’t mind it so much anymore

Google’s web.dev/live event had a talk about AMP’s new direction: to become a high quality component library — a happy path. With Core Web Vitals, the open question of how user experience would be tied into search ranking has been answered. Answered in a data-driven, and well tooled way. This means that if you have a pleasing website — no matter the technology — your rank won’t be hampered. You can choose whatever analytics provider you respect. You can choose whichever commenting, cdn, etc. companies that you trust and believe in — even small projects without AMP integration. This is good news.

You won’t need to include a third party script — which you shouldn’t be doing without sub-resource integrity to begin with. Third party scripts represent an enormous risk. It takes a single malicious one to compromise a whole page — potentially more than a single page. This is why it’s important to compartmentalize your website’s login and payment processing using iframes with only very trusted third party scripts on those pages. When possible, self-host your scripts. There’s your sec-tip — I’ll say no more.

AMP has some really good technology though — technology that takes a long time to build right. Things like loading resources based on viewport priority, proper accessibility for carousels, and maintaining general performance is difficult. To be able to solve that using custom-elements and raw JavaScript is quite impressive.

Those are valuable pieces, as is the optimization technology that powers the AMP caches. Opening up that cache is very exciting.

Looks like signed HTTP exchanges could let AMP caches serve your site as if it came from your domain.  I'd heard about signed HTTP exchanges because I heard about offline page sharing.  That is, using near field communication, a user could share a page with another device.  The site is secure because the exchange is signed so the domain name is your domain name.  I was interested in this because I thought it could make for a very resilient means of distributing apps that would otherwise be censored.  If user's could spread the app via in person interaction it could pass borders easily and if it used WebRTC and WebPush to build a serverless peer to peer network, then it would be almost unstoppable.  It's cool that there's more uses of this technology than what I'd first thought.

# Plans for the Future

Not using JavaScript is both concerning and freeing. Constraint breeds creativity. With AMP’s content first approach and custom-element base, I think it’s very well suited to static site generators. I see it as a fast track to a fast, content-oriented design. Outputting custom-elements is the same as outputting HTML. My Github pages site is manual HTML but I’ve wanted to convert it to a static site generator for awhile. I’m thinking I’ll use AMP.

Some site visitors will never visit again and I think maintaining a low profile (loading quickly, maybe not caching your full site’s assets, maybe no service worker, etc.) for these users is very kind. AMP seems to do this. As long as I can also extend it to improve return user experience, I hope to have the AMP pages be canonical.

***

So that’s it. I judged a technology based on my fears of what I thought it was destined for. I switched search engines in large part to avoid it. Now that my fears have been mostly addressed, I’m planning on building a website with it. And I hope that the technology and user experience that it’s known for is brought to other websites.
