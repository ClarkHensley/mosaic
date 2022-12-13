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
