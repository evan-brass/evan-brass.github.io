+++
title = "Distributed Web Applications"
date = 2020-03-27T20:18:42Z

[taxonomies]
tags = [
	"JavaScript",
	"Distributed Systems",
	"WebRTC",
	"WebPush"
]
+++
I’m a big believer in distributed systems. It is easy to see the need for distributed systems when we see the existing digital infrastructure being tested by COVID-19 levels of load. Unfortunately, distributing a system is not usually a simple task. Consistency no longer comes naturally and horizontal scaling is difficult. Distributing a system requires careful thought. It can be worthwhile though.

# Why should we build distributed web apps?

There are at least three desirable properties that distributed web apps can achieve:

1. Fault tolerance: They can survive failing servers. With service workers, web apps are able to show something even if the server and client are disconnected (a server down, network partition, or just being offline), but it usually can’t progress. By progress I mean that it can’t show new content to users. If a web app were distributed than even if the server were removed, users could still interact and share fresh data.
2. Privacy: I’m terrified about the prospect of having to build or maintain a server that stores sensitive data. There are so many layers of software and hardware that could be breached. By distributing information across nodes, each node becomes a less valuable target.
3. Ease of Use: The thing that the web brings to the world of distributed systems is ease of use. Websites are easy to install and most devices can use them. User interfaces on the web are simple to build and work on each platform. The existence of API gateways for Ethereum, IPFS, and Stellar show that developers want easy integration with these systems.

# How would it work?

Building distributed systems in the browser isn’t easy but it’s getting easier. As you may expect, the foundation for distributed web applications is WebRTC. WebRTC let’s browsers talk to each other directly. WebRTC is for more than just video communication. WebTorrent and libp2p both use WebRTC.

The piece left out from WebRTC is signaling. It’s expected that the application developer will build a server that passes messages between the clients so that they can establish a direct connection. This is fine, but it means that WebRTC applications are not usually distributed. The way to handle this is usually to use WebRTC data channels to perform signaling after an initial connection with some well known servers. This creates tiers: some nodes can be bootstrapped from but others cannot. What we need for a more distributed system is an API for peers to send their signaling messages directly to each other during that first connection.

# Enter WebPush

A specification for sending messages to a browser exists as the web push specification. A website can get a subscription with a push service and the browser will manage all the communication necessary to deliver messages sent to that subscription to the website’s service worker.

In essence, what we are trying to do is to replace the single signaling server that the app developer would have built with a protocol that runs on whatever push service the browser vendor uses. This lets a browser bootstrap from another browser. A side consequence is that if two different web apps spoke the same protocol, then they would be able to communicate even if the developers didn’t connect their signaling servers together:  
Nosebook.com <-> Earbook.com

Using WebPush means that you could host a social media website using purely static servers — like on Github Pages. If we get web packaging, users could even share the app with other users using near field communication — Bluetooth perhaps. This would mean you could have a social media app that didn’t have any of it’s own servers at all! Such an app would have no central failure / censorship points.

In terms of privacy, WebRTC, and WebPush both use encryption. The push services don’t get access to the notification contents. With WebRTC, the stun servers get ICE information, but none of the data transmitted on the peer-connection.

I’ve been playing with this for a few months trying to bring all the pieces together: sending push messages from a browser, communicating between the service worker and client, encoding WebRTC signaling messages into a small binary format, etc. I’ve built a rudimentary video chat app using these pieces. I think it could be a reference if you wanted to build something along the lines of what I’ve described.

# Caveats

I’ve learned a bunch about WebRTC and WebPush as I’ve been working with them. There are some limits to what you can build using a webpush+webrtc based distributed system.

1. You still need a well known peer that you can bootstrap from — but it can be any peer. In my case, I just have the users copy and paste an introduction from one peer to the other. After that they can connect to that peer for \~12 hours. Once they connect, the peer will send them more JSON Web Tokens authorizing them to send messages to that peer’s push subscription for \~36 hours.
2. Web Push services only work with messages that are <4094 bytes. That’s 4kb-2b for the 16 bit unsigned padding length. In some notification bridging scenarios it could be even less, though using it as I’ve described shouldn’t encounter those. I always assumed that 4kb would be plenty for signaling until I got my first SDP offer from chrome which was +6kb. I used Pako to zip things down but then base64 encode it because I was getting null data properties on the push events. In my testing, I’m sending push subscription information, a peer public key, a few signatures for JWT’s, an SDP offer, and a few ICE candidates all in a single <4kb web push. Not sending whole JWTs made a huge improvement and it’s fine because the header is the same for all VAPID tokens and the audience is always the origin for the push-subscription endpoint. That just leaves the subscriber and expiration assuming you don’t put any unnecessary fields in the JWT body.
3. Google Cloud Messenger (the push service for Chrome) doesn’t send CORS headers so I had to use cors-anywhere. Firefox Autopush does send CORS headers. I believe the specification recommends that the push service use CORS so hopefully this won’t always be an issue.
4. Safari doesn’t have web push support. I looked at Mozilla’s Autopush and I think you could maybe subscribe over a web socket from the browser but I’m not sure. If you can then a work around for Safari could be manual subscription via Autopush. To be even more fault tolerant, not relying on the single push service that the browser includes would be a good idea.
5. The push service can end a subscription at any time which is when the onsubscriptionchanged handler is useful. In my testing, I’ve never seen one. But it could happen and I suspect if the push service thought a subscription was spam then it might revoke it.
6. Firefox nightly was unable to connect to autopush because of the increased sec-* headers but the issue is gone now I think.

# Conclusion

Web apps are great. They (mostly) turn off when you close your browser. They run in sophisticated sandboxing. They’re familiar to most people. They’re so good that I think distributed system creators who want the greatest reach should consider choosing the web as their native platform. Web Push allows WebRTC signaling across the browser’s native signaling network allowing a website to get fresh data even if its own servers are lost — or if it has no servers at all. Building such a website is tough but possible.

# Resources

[MIT 6.824: Distributed Systems](https://www.youtube.com/channel/UC_7WrbZTCODu1o_kfUMq88g) — If you’d like to learn how to design distributed systems, the lectures for MIT’s 6.824 are on YouTube. I’ve been really enjoying them.

[Perfect Negotiation in WebRTC/](https://blog.mozilla.org/webrtc/perfect-negotiation-in-webrtc/) — Super helpful in understanding WebRTC signaling.

[Web Push Protocol](https://developers.google.com/web/fundamentals/push-notifications/web-push-protocol) — This has everything about sending push notifications. The web push api has changed (the Voluntary Application server Identification is mostly mandatory now), but this information works.

[MDN Service Workers](https://developers.google.com/web/fundamentals/primers/service-workers) — Service workers are required for web push.

[Web3.0-test](https://github.com/evan-brass/web3.0-test) — This is where my code is. It might be the ugliest code I’ve written, but it has each of the pieces all in one place.

[Signaling Spec](https://github.com/evan-brass/web3.0-test/blob/master/sw/signaling-spec.md) — This is one way that you could map WebRTC signaling onto WebPush. It would need modification to support multiple encryption algorithms and any other changes in the Web Push or WebRTC specifications.

[Mozilla Push Server Docs](https://mozilla.github.io/application-services/docs/push/welcome.html) — If you wanted to try and support Safari, the information about Mozilla’s Autopush is here. I think this is an updated version of [these](https://mozilla-push-service.readthedocs.io/en/latest/) with autopush-rs and http2 API information.
