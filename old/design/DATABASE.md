# DATABASE Design

At time of writing, tokio.rs does not support targetting wasm, leaving use of sqlx non-functional (I've tried using async_std, but I can't seem to get it working)

Due to the distributed nature of this project, I want to research a database format that will allow for user-added columns or tables to the schema. I'll have to research what can do this, but it may be that sqlx was a bad decision anyway

Exact database architecture can come later, but, for now, I want to guarantee that this goal will work with potentially distributed tables across databases

In general, it seem important to ensure that there is a ground-truth database architecture,because users might have data locally, and users may well interface with different serves with potentially different architecture. How do we ensure that?

I'm worried this distirbuted idea will be very upload/downloaded gated. How will we deal with that

An example:
I use Server A and Server B, but as a user, I want a single-sign-on, even though A and B are administered by different hosts
So, I log into my Mosaic client, and this securely (how?) sends a Post of my password to all relevant servers, which shoudl work. So this is fine, we can just assume password reuse over the servers, which isn't ideal, but should be fine here.
    - How? HTTPS/SSL, apparently. Fortunately, the networking on all of this is, like, phase 3

For lack of overhead (and because, for now, I don't intend to target large user-bases), I'm using sqlite.

I'll create a boostrap database profile to instantiate a new, fresh server. We'll work on porting old stuff over. For now, we'll use this to set up the initial format.

So, now, what does a Database look like? Arguably, we might need two types, one for a server and one for a client, but I'd like, given the option, to have both work on the same framework.

A given database needs to know what:
A list of users by their UUID, username on this server

We'll have to generate UUID per server, and just allow cross-server connections to verify that someone own multiple UUIDs.

We'll want a secure way to sync known databases/UUIDs over the server. that is, if I log into my DMs and 3 servers on a client, I can choose which of those to automatically sync so that, when I log into one of the synced ones on a different platform, it automatically syncs from that server's stored data about me. Data will be duplicated. We have to be okay with that in this distributed stuff.
