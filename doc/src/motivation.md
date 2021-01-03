# Motivation

I decided to create `leafbuild` because I am not happy with what
is already there:
- cmake(I don't like the docs at all)
- meson:
> Meson is an open source build system meant to be both extremely fast, and,
>even more importantly, as user friendly as possible.
>
> &mdash; <cite><a href="https://mesonbuild.com/index.html#overview">Meson overview</a></cite>

Well it's not really that fast. In fact it takes about 1.5 - 2 seconds to
*only print the version* in a new session, which I find outrageous.

So in the end I ended up creating my own build system.
